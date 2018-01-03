//! Manages fix party status and handles message synchronization

extern crate core;

use std::io;
// use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::fmt::{Display, Formatter, Error};

use super::MessageStore;

use fix::frame::FixFrame;
use fix::fixmessagegen::*;

/// Indicates the state of each party:
/// (sender, target) or (innitator, acceptor)
#[derive(PartialEq)]
enum FixPartyState {
    /// Initial state
    Connected,
    /// Processing logon stage
    Logon,
    /// Logout initiated
    Logout,
    /// Gap detected, so negotiating and awaiting resend or reset
    /// At this point, no new app messages should be sent
    /// and messages past gap should not be delivered to application code
    MessageSynchronization,
    /// All is fine
    Operational,
}

pub struct FixSyncState <Store : MessageStore> {
    store:  Arc<Mutex<Store>>,
    sender: PartyState,
    target: PartyState,
    sent_count: u32,
    recv_count: u32,
}

pub struct PartyState {
    state: FixPartyState,
    gap: MessageGap,
}

#[derive(Debug,Copy,Clone)]
pub struct MessageGap {
    gap: Option<(i32, i32)>,
}

impl <Store : MessageStore> FixSyncState <Store> {
    pub fn new( store: Arc<Mutex<Store>> ) -> FixSyncState <Store> {
        FixSyncState {
            store,
            sender: PartyState::new(),
            target: PartyState::new(),
            sent_count: 0,
            recv_count: 0,
        }
    }

    pub fn are_both_operational(&self) -> bool {
        self.sender.is_operational() && self.sender.is_operational()
    }

    pub fn register_sent(&mut self, frame: &FixFrame) -> io::Result<()> {
        self.incr_sent();

        let is_poss_dup = frame.header.poss_dup_flag.map_or(false, |v| v);

        // sender: if we're in sync, has the gap been filled?
        if self.sender.has_sync_pending() && frame.message.msg_type() != FieldMsgTypeEnum::Logout {
            // let msg_seq = frame.header.msg_seq_num;
            assert_eq!( is_poss_dup, true );
            let fill_range = FixSyncState::<Store>::extract_range( &frame )?;
            println!("sender: filling range {:?} gap {:?}", fill_range, self.sender.gap);
            self.sender.fill_range( fill_range );
        }

        if is_poss_dup == false {
            match &frame.message {
                &FixMessage::Logon(ref flds) => {
                    self.outgoing_logon()?;

                    if flds.reset_seq_num_flag.map_or(false, |v| v) {
                        self.store.lock().unwrap().overwrite_sender_seq(1)?;
                    }
                },
                _ => { /*nothing to do*/ }
            }
        }

        Ok( () )
    }

    pub fn register_recv(&mut self, frame: &FixFrame) -> io::Result<TransitionAction> {
        self.incr_recv();

        // first, is there a gap?
        let action_if_gap_in_target = {
            // target: if we're in sync, has the gap been filled?
            if self.target.has_sync_pending() {
                self.process_incoming_gap_fill( &frame )?
            } else {
                // we're in good state: is there a gap?
                self.try_detect_gap_and_select_transition( &frame )?
            }
        };

        let is_poss_dup = frame.header.poss_dup_flag.map_or(false, |v| v);

        // now process the message
        let composed_action = {
            // sender: has the target found a gap in our seq?
            if !is_poss_dup && frame.message.msg_type() == FieldMsgTypeEnum::ResendRequest {
                let resend_range = self.register_sender_gap_detected_by_target( &frame )?;

                match action_if_gap_in_target {
                    TransitionAction::RequestResendRange( request_range ) => {
                        // augment previous action
                        TransitionAction::DoResendAndRequestRange( (resend_range, request_range) )
                    },
                    TransitionAction::None => {
                        TransitionAction::DoResendRange( resend_range )
                    },
                    _ => action_if_gap_in_target // logout
                }
            } else {
                action_if_gap_in_target
            }
        };

        if is_poss_dup == false {
            match &frame.message {
                // Logon needs to be processed
                // even if a gap was detected
                &FixMessage::Logon(ref flds) => {
                    self.incoming_logon()?;

                    if flds.reset_seq_num_flag.map_or(false, |v| v) {
                        // other party instructed reset
                        // let recv_seq = frame.header.msg_seq_num;
                        if let Ok(mut store) = self.store.try_lock() {
                            store.overwrite_target_seq(1)?;
                        }
                    }
                },
                _ => {}
            }
        }

        Ok( composed_action )
    }

    fn register_sender_gap_detected_by_target(&mut self, frame: &FixFrame) -> io::Result<(i32, i32)> {
        match &frame.message {
            &FixMessage::ResendRequest(ref flds) => {
                let start = flds.begin_seq_no;
                let end   =
                    if flds.end_seq_no != 0 {
                        flds.end_seq_no
                    } else {
                        self.expected_sender_seq_num()
                    };
                self.sender.gap_detected(start, end);
                Ok( self.sender.gap_as_range() )
            },
            _ => unreachable!("nope")
        }
    }

    fn extract_range(frame: &FixFrame) -> io::Result<(i32, i32)> {
        // if message is seqreset-gapfill, range is (seq..new_begin)
        // if message is something else,   range is (seq..seq)

        let seq = frame.header.msg_seq_num;

        let range = {
            match &frame.message {
                &FixMessage::SequenceReset(ref flds) => {
                    let gap_fill = flds.gap_fill_flag.map_or(false, |v| v);

                    assert_eq!(gap_fill, true); // assert not a seqreset-hard_reset

                    (seq, flds.new_seq_no - 1)
                },
                _ => {
                    (seq, seq)
                }
            }
        };

        Ok( range )
    }

    /// compare `received seq` with `expected` and determines action if different
    fn try_detect_gap_and_select_transition(&mut self, frame: &FixFrame) -> io::Result<TransitionAction> {
        let recv_target_seq_num = frame.header.msg_seq_num;
        let expected_seq        = self.expected_target_seq_num();

        let result = {
            if recv_target_seq_num > expected_seq {
                warn!("try_detect_gap_and_select_transition: expecting seq {} but received {}. \
                       Initiating resend request, holding recv/sending until synchronization is complete", expected_seq, recv_target_seq_num);

                println!("target : gap_detected e {} r {}", expected_seq, recv_target_seq_num );
                self.target.gap_detected( expected_seq, recv_target_seq_num );

                TransitionAction::RequestResendRange( self.target.gap_as_range() )

            } else if recv_target_seq_num < expected_seq {
                // something very wrong. the spec tells us to disconnect and ask for manual intervention
                error!("detect_gap_and_choose_action: expecting seq {} but received {}. \
                        Too low detected, disconnecting.", expected_seq, recv_target_seq_num);

                println!("msg seq too low: expected {} recv {}", expected_seq, recv_target_seq_num);

                TransitionAction::LogoutWith( "msg seq too low" )

            } else {

                // since we're good, increment seq
                let _ = self.increment_target_seq_num()?;

                // assert_eq!(new_seq + 1, expected_seq + 1);
                // println!("increment_target_seq_num  new_seq: {}", new_seq);

                // transition if necessary
                if self.target.has_sync_pending() {
                    self.target.sync_completed();
                    // flush queue (TODO)
                }

                TransitionAction::None
            }
        };

        Ok( result )
    }

    fn process_incoming_gap_fill(&mut self, frame: &FixFrame) -> io::Result<TransitionAction> {
        assert_eq!(self.target.has_sync_pending(), true);

        // a seqreset-RESET mode is especially handled as per spec:
        match &frame.message {
            &FixMessage::SequenceReset(ref flds) => {
                let is_reset = flds.gap_fill_flag.map_or(false, |v| v) == false;

                if is_reset {
                    // very serious situation where the target is saying we need
                    // to treat this as disaster recovery. we then forcefully overwrite
                    // the sequence
                    let new_sequence = flds.new_seq_no;
                    self.store.lock().unwrap().overwrite_target_seq( new_sequence )?;
                    self.target.sync_completed();

                    return Ok( TransitionAction::None );
                }
            },
            _ => { /* nothing to do */ }
        }

        // now we handle SeqReset gap fill and ordinary messages.
        // the other party should be a good citizen:
        // - the gap should be filled in order, either by proper app messages, reject or seq-resets
        // - they must be poss_dup = true

        let is_poss_dup = frame.header.poss_dup_flag.map_or(false, |v| v);
        if is_poss_dup {
            let fill_range = FixSyncState::<Store>::extract_range( &frame )?;

            println!("target: filling range {:?} gap {:?} next {}", fill_range, self.target.gap.gap, self.expected_target_seq_num() );
            let next = self.target.fill_range( fill_range );

            self.store.lock().unwrap().overwrite_target_seq( next )?;

            Ok( TransitionAction::None )

        } else {
            // in this case, during gap fill (bad citizen)
            // we received an ordinary msg (not poss_dup)
            // so we adjust the gap

            self.target.gap_adjust( frame.header.msg_seq_num );

            Ok( TransitionAction::RequestResendRange( self.target.gap_as_range() ) )
        }
    }

    fn expected_target_seq_num(&self) -> i32 {
        self.store.lock().unwrap().next_target_seq_num()
    }
    fn expected_sender_seq_num(&self) -> i32 {
        self.store.lock().unwrap().next_sender_seq_num()
    }
    fn increment_target_seq_num(&mut self) -> io::Result<i32> {
        let new = self.store.lock().unwrap().incr_target_seq_num()?;
        info!("increment_target_seq_num called: {}", new);
        Ok( new )
    }
    fn incr_sent(&mut self) {
        self.sent_count = self.sent_count + 1;
    }
    fn incr_recv(&mut self) {
        self.recv_count = self.recv_count + 1;
    }

    /// logon message sent
    fn outgoing_logon(&mut self ) -> io::Result<()> {
        if self.sender.state == FixPartyState::Connected {
            self.sender.state = FixPartyState::Logon;
            self.target.state = FixPartyState::Logon;
            Ok( () )
        } else {
            Err( io::Error::new(io::ErrorKind::Other, format!("cannot not go to logon_sent from {}", self.to_string()).as_str() ) )
        }
    }

    /// logon message recv (confirmation)
    fn incoming_logon(&mut self ) -> io::Result<()> {
        if self.target.state == FixPartyState::Logon {
            self.target.state = FixPartyState::Operational;
        }
        if self.sender.state == FixPartyState::Logon {
            // there's room for a race condition here:
            // The spec actually recommends that we send a TestRequest or wait a few seconds
            // before considering the connection op, as the server may still be figuring out
            // if there's a gap, and sending a ResendRequest soon.
            // For now, let's be optimistic.
            self.sender.state = FixPartyState::Operational;
            Ok( () )
        } else {
            Err( io::Error::new(io::ErrorKind::Other, format!("logon_recv without sending a logon? current: {}", self).as_str() ) )
        }
    }
}

#[derive(Debug,Copy,Clone)]
pub enum TransitionAction {
    None,
    /// Instruct graceful disconnection
    LogoutWith(&'static str),         // <- reason
    /// Instruct a resend of messages to the other party
    DoResendRange( (i32, i32) ),      // <- start,end,
    /// Instruct requesting missing messages from the other party
    RequestResendRange( (i32, i32) ), // <- start,end,
    /// Both resends and requests range
    DoResendAndRequestRange( ((i32,i32), (i32,i32)) )
}

impl MessageGap {
    fn new() -> MessageGap {
        MessageGap { gap : None }
    }

    fn clear(&mut self) {
        self.gap = None;
    }

    fn start_gap(&mut self, start: i32, end: i32) {
        self.gap = Some( (start, end) );
    }

    fn update_gap(&mut self, range : (i32, i32) ) -> bool {
        let (rstart, rend) = range;

        if let Some( (start, end) ) = self.gap {
            if rstart != start {
                let msg = format!("Expecting start to match current start of gap: expecting {} got {}", start, rstart);
                panic!(msg);
            }
            // is it fully filled?
            if rend == end {
                // reset it
                self.gap = None;
                return true
            } else {
                self.gap = Some( (rend + 1, end) );
            }
        } else {
            // Shouldn't happen!
            panic!("update_gap without a gap pre-established");
        }
        return false;
    }

    fn update_gap_end(&mut self, new_end: i32) {
        if let Some( (start, end) ) = self.gap {
            if new_end > end {
                self.gap = Some((start, new_end));
            }
        }
    }

    fn as_range(&self) -> (i32, i32) {
        if let Some(range) = self.gap {
            range
        } else {
            panic!("no gap set");
        }
    }
}

impl PartyState {
    fn new() -> PartyState {
        PartyState {
            state: FixPartyState::Connected,
            gap: MessageGap::new(),
        }
    }

    fn is_operational(&self) -> bool {
        match self.state {
            FixPartyState::Operational => true,
            _ => false
        }
    }

    fn gap_detected(&mut self, start: i32, end: i32) {
        self.gap.start_gap( start, end );
        self.state = FixPartyState::MessageSynchronization;
    }

    fn gap_adjust(&mut self, new_end: i32) {
        self.gap.update_gap_end( new_end );
    }

    fn fill_range(&mut self, fill_range : (i32, i32) ) -> i32 {
        assert_eq!( self.has_sync_pending(), true );

        if self.gap.update_gap( fill_range ) {
            self.state = FixPartyState::Operational;
        }

        let expected_next = fill_range.1 + 1;
        expected_next
    }

    fn has_sync_pending(&self) -> bool {
        match self.state {
            FixPartyState::MessageSynchronization => true,
            _ => false
        }
    }

    fn sync_completed(&mut self) {
        self.gap.clear();
        self.state = FixPartyState::Operational;
    }

    fn gap_as_range(&self) -> (i32, i32) {
        self.gap.as_range()
    }
}

/// API sugar
impl TransitionAction {
    pub fn is_none(&self) -> bool {
        match self {
            &TransitionAction::None => true,
            _ => false
        }
    }
}

impl <Store : MessageStore> Display for FixSyncState <Store> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str( &format!("sender {} - target {}", self.sender, self.target) )
    }
}

impl Display for PartyState {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str( format!("{}{}", self.state.to_string(), self.gap).as_str() )
    }
}

impl Display for MessageGap {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        if self.gap.is_none() {
            f.write_str( "" )
        } else {
            let range = self.gap.as_ref().unwrap();
            f.write_str( format!(" {:?}", range).as_str() )
        }
    }
}

impl ToString for FixPartyState {
    fn to_string(&self) -> String {
        match self {
            &FixPartyState::Connected               => "connected",
            &FixPartyState::Logon                   => "logon",
            &FixPartyState::Logout                  => "logout",
            &FixPartyState::MessageSynchronization  => "resync",
            &FixPartyState::Operational             => "operational",
        }.to_owned()
    }
}