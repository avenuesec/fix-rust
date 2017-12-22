
use std::io;
use std::rc::Rc;
use std::sync::Mutex;

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

pub struct FixSyncState2 <Store : MessageStore> {
    store:  Rc<Mutex<Store>>,
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

impl <Store : MessageStore> FixSyncState2 <Store> {

    pub fn new( store: Rc<Mutex<Store>> ) -> FixSyncState2 <Store> {
        FixSyncState2 {
            store,
            sender: PartyState::new(),
            target: PartyState::new(),
            sent_count: 0,
            recv_count: 0,
        }
    }

    pub fn register_sent(&mut self, frame: &FixFrame) -> io::Result<()> {

        self.incr_sent();

        // sender: if we're in sync, has the gap been filled?
        if self.sender.has_sync_pending() {

        } else {

            self.increment_sender_seq_num();
        }

        Ok( () )
    }

    pub fn register_recv(&mut self, frame: &FixFrame) -> io::Result<TransitionAction> {

        self.incr_recv();

        let is_poss_dup = frame.header.poss_dup_flag.map_or(false, |v| v);

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
                self.sender.gap.start_gap(start, end);
                Ok( self.sender.gap_as_range() )
            },
            _ => unreachable!("nope")
        }
    }

    /// other party requested a reset
    fn process_target_seq_reset(&mut self, frame: &FixFrame) -> io::Result<TransitionAction> {

        Ok( TransitionAction::None )
    }

    /// compare `received seq` with `expected` and determines action if different
    fn try_detect_gap_and_select_transition(&mut self, frame: &FixFrame) -> io::Result<TransitionAction> {

        let recv_target_seq_num = frame.header.msg_seq_num;
        let expected_seq        = self.expected_target_seq_num();

        let result = {
            if recv_target_seq_num > expected_seq {
                warn!("try_detect_gap_and_select_transition: expecting seq {} but received {}. \
                       Initiating resend request, holding recv/sending until synchronization is complete", expected_seq, recv_target_seq_num);

                self.target.gap_detected( expected_seq, recv_target_seq_num );

                TransitionAction::RequestResendRange( self.target.gap_as_range() )

            } else if recv_target_seq_num < expected_seq {
                // something very wrong. the spec tells us to disconnect and ask for manual intervention
                error!("detect_gap_and_choose_action: expecting seq {} but received {}. \
                        Too low detected, disconnecting.", expected_seq, recv_target_seq_num);

                TransitionAction::LogoutWith( "msg seq too low" )

            } else {

//                if self.target.has_sync_pending() && self.target.update_gap_check_filled(recv_target_seq_num, false) {
//                    self.target.sync_completed();
//                }

                // all good! transition to valid state if necessary
                if self.target.has_sync_pending() {
                    self.target.sync_completed();
                    // TODO: flush queued messaged if any
                }

                // since we're good, increment seq
                let new_seq = self.increment_target_seq_num()?;

                assert_eq!(new_seq, expected_seq + 1);

                TransitionAction::None
            }
        };

        Ok( result )
    }

    fn process_incoming_gap_fill(&mut self, frame: &FixFrame) -> io::Result<TransitionAction> {

        // a seqreset-RESET mode is especially handled as per spec:
        match &frame.message {
            &FixMessage::SequenceReset(ref flds) => {
                let is_reset = flds.gap_fill_flag.map_or(false, |v| v) == false;

                if is_reset {
                    // very serious situation where the target is saying we need
                    // to treat this as disaster recovery. we then forcefully overwrite
                    // the sequence going backwards or forward

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
        // the gap should be filled in order, either by proper app messages, reject or seq-resets:

        let is_poss_dup = frame.header.poss_dup_flag.map_or(false, |v| v);

        assert_eq!(is_poss_dup, true);

        match &frame.message {
            &FixMessage::SequenceReset(ref flds) => {
                let is_gap_fill = flds.gap_fill_flag.map_or(false, |v| v) == true;

                assert_eq!( is_gap_fill, true ); // must be gap fill at this point

                let new_sequence = flds.new_seq_no;

                if new_sequence < self.expected_target_seq_num() {
                    // Seq reset can only increase the seq, never go backwards
                    error!("SequenceReset incorrectly instructed lower seq, which by the spec definition should not be allowed. \
                            Current expected {} new seq: {}", new_sequence, self.expected_target_seq_num() );
                }

                // jumps ahead
                self.store.lock().unwrap().overwrite_target_seq( new_sequence )?;

            },
            _ => { /* nothing to do */ }
        }

//            } else {
//            // we should not be receiving ordinary messages while we're gap-filling, so what to do?
//            error!("received message with poss_dup_flag = false during gap filling process: {:?}", frame.message.msg_type());
//            // should we crash, reject, or save the message for later?
//            panic!("unexpected message received during gap filling process")
//            }

        Ok( TransitionAction::None )
    }

    fn expected_target_seq_num(&self) -> i32 {
        self.store.lock().unwrap().next_target_seq_num()
    }
    fn expected_sender_seq_num(&self) -> i32 {
        self.store.lock().unwrap().next_sender_seq_num()
    }

    fn increment_sender_seq_num(&mut self) -> io::Result<i32> {
        self.store.lock().unwrap().incr_sender_seq_num()
    }

    fn increment_target_seq_num(&mut self) -> io::Result<i32> {
        self.store.lock().unwrap().incr_target_seq_num()
    }

    fn incr_sent(&mut self) {
        self.sent_count = self.sent_count + 1;
    }
    fn incr_recv(&mut self) {
        self.recv_count = self.recv_count + 1;
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

    fn start_gap(&mut self, start: i32, end: i32) {
        self.gap = Some( (start, end) );
    }

    fn update_gap(&mut self, new_begin : i32, is_seq_reset: bool) -> bool {
        if let Some( (_start, end) ) = self.gap {
            // is it fully filled?
            if (is_seq_reset && new_begin > end) || (!is_seq_reset && new_begin == end) {
                // reset it
                self.gap = None;
                return true
            } else {
                self.gap = Some( (new_begin, end) );
            }
        } else {
            // Shouldn't happen!
            panic!("update_gap without a gap pre-established");
        }
        return false;
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

    fn gap_detected(&mut self, start: i32, end: i32) {
        self.gap.start_gap( start, end );
        self.state = FixPartyState::MessageSynchronization;
    }

    fn update_gap_check_filled(&mut self, latest_seq: i32, is_seq_reset: bool) -> bool {
        if self.gap.update_gap( latest_seq, is_seq_reset ) {
            self.state = FixPartyState::Operational;
            true
        } else {
            false
        }
    }

    fn has_sync_pending(&self) -> bool {
        match self.state {
            FixPartyState::MessageSynchronization => true,
            _ => false
        }
    }

    fn sync_completed(&mut self) {
        self.gap = None;
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