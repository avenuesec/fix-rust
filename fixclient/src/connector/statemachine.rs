use std::string::ToString;
use std::result::Result;
use std::io;
use std::sync::Mutex;
use std::fmt::{Display, Error, Formatter};
use std::rc::Rc;

// use chrono::{Utc, DateTime};

use super::MessageStore;

use fix::frame::FixFrame;
use fix::fixmessagegen::*;


pub struct MessageToReSend {
    pub seq: i32,
    pub orig_sending_time: UtcDateTime,
    pub message: FixMessage,
}

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
    them:       FixPartyState,
    us:         FixPartyState,
    store:      Rc<Mutex<Store>>,
    sent_count: u32,
    recv_count: u32,
    their_gap : Option<(i32, i32)>,
    our_gap   : Option<(i32, i32)>,
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

impl <Store : MessageStore> FixSyncState <Store> {

    pub fn new( store: Rc<Mutex<Store>> ) -> FixSyncState <Store> {
        FixSyncState {
            us:   FixPartyState::Connected,
            them: FixPartyState::Connected,
            store,
            sent_count: 0,
            recv_count: 0,
            their_gap: None,
            our_gap:   None,
        }
    }

    pub fn register_sent(&mut self, frame: &FixFrame) -> io::Result<()> {

        self.incr_sent();

        // are we in the middle of a re-sync?
        // if so, we should only be sending poss_dup_flag = true or sequence resets (gap fills)
        if self.us == FixPartyState::MessageSynchronization {
            // we need to identify when we're done with resend

            let msg_seq = frame.header.msg_seq_num;
            let is_resend = frame.header.poss_dup_flag.map_or(false, |v| v);
            let is_seq_reset = frame.message.msg_type() == FieldMsgTypeEnum::SequenceReset;

            if is_seq_reset {
                match &frame.message {
                    &FixMessage::SequenceReset(ref flds) => {
                        let new_seq = flds.new_seq_no;
                        if self.update_our_gap( new_seq ) {
                            self.us_notify_resend_completed();
                        }
                    }
                    _ => { }
                }
            } else if is_resend {
                if self.update_our_gap( msg_seq ) {
                    self.us_notify_resend_completed();
                }
            } else {
                // we shouldn't be sending anything else!
                panic!("sending other message than allowed during re-sync: {:?}", frame);
            }

            return Ok( () );
        }

        match &frame.message {
            &FixMessage::Logon(ref flds) => {
                self.outgoing_logon()?;

                if flds.reset_seq_num_flag.map_or(false, |v| v) {
                    // we have requested a seq reset.
                    // assumes that SessionImpl already reset the
                    // sequences before sending this message
                }
            },

            _ => { }
        }

        Ok( () )
    }

    pub fn register_recv(&mut self, frame: &FixFrame) -> io::Result<TransitionAction> {

        self.incr_recv();

        let is_logon = frame.message.msg_type() == FieldMsgTypeEnum::Logon;

        // seq shouldnt be validated before for these logon/reset
        // and should be "ignored" when we're getting resends
        let should_pre_confirm_seq = !is_logon &&
                                      frame.message.msg_type() != FieldMsgTypeEnum::SequenceReset &&
                                     !frame.header.poss_dup_flag.map_or(false, |v| v);

        if should_pre_confirm_seq {
            let res = self.detect_gap_and_choose_action(&frame.header, &frame.message);
            if !res.is_none() {
                return Ok( res );
            }
        }

        match &frame.message {
            &FixMessage::Logon(ref flds) => {
                self.incoming_logon()?;

                if flds.reset_seq_num_flag.map_or(false, |v| v) {
                    // other party instructed reset

                    let recv_seq = frame.header.msg_seq_num;
                    if let Ok(mut store) = self.store.try_lock() {
                        store.overwrite_target_seq(recv_seq);
                    }

                    // no point in trying to determine gap, so transition
                    self.them = FixPartyState::Operational;
                }
            },

            &FixMessage::ResendRequest(ref flds) => {
                // The other party identified a message gap

                self.acknoledge_our_gap( flds.begin_seq_no, flds.end_seq_no );

                // Instruct to resend
                return Ok( TransitionAction::DoResendRange( self.our_gap.unwrap() ) );
            }

            &FixMessage::SequenceReset(ref flds) => {
                let gap_filling = flds.gap_fill_flag.map_or(false, |v| v);
                let new_seq = flds.new_seq_no;

                if !gap_filling {
                    // Reset in no gap filling mode = disaster recovery
                    warn!("Disaster recovery SequenceReset received. {} new seq: {}", gap_filling, new_seq);
                }

                let next_target_seq_num = self.get_next_target_seq_num();

                if new_seq < next_target_seq_num {
                    // Seq reset can only increase the seq, never go backwards
                    error!("SequenceReset incorrectly instructed lower seq, which by the spec definition should not be allowed. \
                            Current expected {} new seq: {}", next_target_seq_num, new_seq);
                }

                if let Ok(mut store) = self.store.try_lock() {
                    store.overwrite_target_seq(new_seq);
                }

                if let Some(gap) = self.their_gap {
                    if new_seq >= gap.0 {
                        // gap "filled" by reset
                        self.them = FixPartyState::Operational;
                    }
                }
            },
            _ => { }
        }

        // Special handling, we need to process the logon seq after accepting/processing the message
        // We're assuming the other party is being a good citizen and not resending a logon msg
        if is_logon {
            let res = self.detect_gap_and_choose_action(&frame.header, &frame.message);
            return Ok( res );
        }

        Ok( TransitionAction::None )
    }

    /// gathers the messages that need to be resent
    pub fn build_resend_request_response(&mut self, start: i32, end: i32) -> io::Result<Vec<MessageToReSend>> {

        // let mut entries = Rc::get_mut(&mut self.store).unwrap().query(start, end)?;
        let mut entries = {
            if let Ok(mut store) = self.store.try_lock() {
                store.query(start, end)?
            } else {
                return Err( io::Error::new(io::ErrorKind::Other, "could not obtain lock") );
            }
        };

        let mut result = Vec::with_capacity( entries.len() );

        let mut temp_gap_to_fill : Option<i32> = None;
        let mut expected_seq = start;
        let mut cur_seq = 0;

        for frame in entries.drain(0..) {
            cur_seq = frame.header.msg_seq_num;

            if temp_gap_to_fill.is_none() && cur_seq != expected_seq  {
                // is there a gap between the seq_num expected and the one we're dealing now?
                temp_gap_to_fill = Some(cur_seq);
            }

            if is_admin_message( &frame.message ) && frame.message.msg_type() != FieldMsgTypeEnum::Reject {
                // we cannot resend admin messages (with the exception of rejects)
                // so we need to adjust the new start seq

                if temp_gap_to_fill.is_none() {
                    temp_gap_to_fill = Some(cur_seq);
                }

            } else {
                // It's a valid message that can be re-sent

                // is there gap?
                if temp_gap_to_fill.is_some() {
                    let new_start_seq = temp_gap_to_fill.take().unwrap();
                    result.push( FixSyncState::<Store>::build_gap_fill( new_start_seq, cur_seq ) );
                }

                result.push( FixSyncState::<Store>::build_resend( frame ) );
            }

            expected_seq = cur_seq + 1;
        }

        if temp_gap_to_fill.is_some() {
            // gap left to be filled
            let new_start_seq = temp_gap_to_fill.take().unwrap();
            result.push( FixSyncState::<Store>::build_gap_fill( new_start_seq, expected_seq ) );
        }

        let new_end = {
            if end > cur_seq {
                // we didnt have all requested messages. fill the gap
                let new_end = end + 1;
                result.push( FixSyncState::<Store>::build_gap_fill( expected_seq, new_end ) );
                new_end
            } else {
                end
            }
        };

        if new_end > self.get_next_sender_seq_num() {
            if let Ok(mut store) = self.store.try_lock() {
                info!("overwrite_sender_seq to new seq {}", new_end);
                store.overwrite_sender_seq( new_end );
            }
        }

        Ok( result )
    }

    fn build_gap_fill( other_party_expected_seq: i32, new_seq_no: i32 ) -> MessageToReSend {
        let flds = SequenceResetFields {
            gap_fill_flag: Some(true),
            new_seq_no,
        };
        MessageToReSend {
            seq: other_party_expected_seq,
            orig_sending_time: UtcDateTime::now(),
            message: FixMessage::SequenceReset( Box::new(flds) ),
        }
    }

    fn build_resend( frame : FixFrame ) -> MessageToReSend {
        MessageToReSend {
            seq: frame.header.msg_seq_num,
            orig_sending_time: frame.header.sending_time.clone(),
            message: frame.message,
        }
    }

    fn incr_sent(&mut self) {
        self.sent_count = self.sent_count + 1;
    }
    fn incr_recv(&mut self) {
        self.recv_count = self.recv_count + 1;
    }

    fn update_our_gap(&mut self, new_begin : i32) -> bool {
        if let Some( (b, e) ) = self.our_gap {
            // is it fully filled?
            if new_begin >= e {
                // reset it
                self.our_gap = None;
                return true
            } else {
                self.our_gap = Some( (new_begin, e) );
            }
        } else {
            // Shouldn't happen!
            panic!("update_our_gap without a gap pre-established");
        }
        return false;
    }

    fn us_notify_resend_completed(&mut self) {
        self.us = FixPartyState::Operational;
    }

    fn get_next_sender_seq_num(&self) -> i32 {
        if let Ok(mut store) = self.store.try_lock() {
            store.next_sender_seq_num()
        } else {
            panic!("could not obtain lock");
        }
    }
    fn get_next_target_seq_num(&self) -> i32 {
        if let Ok(mut store) = self.store.try_lock() {
            store.next_target_seq_num()
        } else {
            panic!("could not obtain lock");
        }
    }

    fn acknoledge_our_gap(&mut self, start: i32, end: i32) {
        // what's the gap range?
        let end =
            if end == 0 {
                self.get_next_sender_seq_num() - 1
            } else {
                end
            };
        // Save our gap so we can tell when it's done
        self.our_gap = Some( (start, end) );
        self.us = FixPartyState::MessageSynchronization;
    }

    fn acknoledge_their_gap(&mut self, start: i32, end: i32 ) {
        self.them = FixPartyState::MessageSynchronization;
        self.their_gap = Some( (start, end) );
    }

    /// check if there's a gap in the incoming messages
    fn detect_gap_and_choose_action(&mut self, header: &FixHeader, message: &FixMessage) -> TransitionAction {

        // we shouldn't process poss_dup messages
        assert_eq!(header.poss_dup_flag.map_or(false, |v| v), false);

        // incr_target_seq_num mutates the store
        let expected_seq = {
            if let Ok(mut store) = self.store.try_lock() {
                store.incr_target_seq_num().unwrap()
            } else {
                panic!("could not obtain lock");
            }
        };

        let recv_target_seq_num = header.msg_seq_num;

        if recv_target_seq_num > expected_seq {
            warn!("detect_gap_and_choose_action: expecting seq {} but received {}. \
                   Initiating resend request, holding recv/sending until synchronization is complete", expected_seq, recv_target_seq_num);

            self.acknoledge_their_gap( expected_seq, recv_target_seq_num );

            match message {
                // another special case:
                // we're responding to a gap fill and we also noticed a gap in the other party
                &FixMessage::ResendRequest(ref flds) => {
                    self.acknoledge_our_gap( flds.begin_seq_no, flds.end_seq_no );
                    return TransitionAction::DoResendAndRequestRange( (self.our_gap.unwrap(), self.their_gap.unwrap() ) );
                },
                _ => {}
            }

            // in this case, the gap lies only on our side
            return TransitionAction::RequestResendRange( self.their_gap.unwrap() ) ;

        } else if recv_target_seq_num < expected_seq {
            // something very wrong. the spec tells us to disconnect and manual intervention is necessary
            error!("detect_gap_and_choose_action: expecting seq {} but received {}. \
                    Too low detected, disconnecting.", expected_seq, recv_target_seq_num);

            // self.post_disconnect("msg seq too low")?;
            return TransitionAction::LogoutWith( "msg seq too low" ) ;

        } else {
            // all good! transition to valid state if necessary
            if self.them == FixPartyState::MessageSynchronization {
                self.them = FixPartyState::Operational;
                // TODO: flush queued messaged if any
            }
            TransitionAction::None
        }
    }

    /// logon message sent
    fn outgoing_logon(&mut self ) -> io::Result<()> {
        if self.us == FixPartyState::Connected {
            self.us = FixPartyState::Logon;
            self.them = FixPartyState::Logon;
            Ok( () )
        } else {
            Err( io::Error::new(io::ErrorKind::Other, format!("cannot not go to logon_sent from {}", self.to_string()).as_str() ) )
        }
    }

    /// logon message recv (confirmation)
    fn incoming_logon(&mut self ) -> io::Result<()> {
        if self.us == FixPartyState::Logon {
            self.them = FixPartyState::MessageSynchronization;
            // self.us = FixPartyState::MessageSynchronization;

            // there's room for a race condition here:
            // The spec actually recommends that we send a TestRequest or wait a few seconds
            // before considering the connection op, as the server may still be figuring out
            // if there's a gap, and sending a ResendRequest soon.
            // For now, let's be optimistic.
            self.us = FixPartyState::Operational;
            Ok( () )
        } else {
            Err( io::Error::new(io::ErrorKind::Other, format!("logon_recv without sending a logon? current: {}", self).as_str() ) )
        }
        // Ok( () )
    }

    /// logout message sent
    fn outgoing_logout(&mut self) -> io::Result<()> {
        self.us = FixPartyState::Logout;
        Ok( () )
    }

    /// logout message recv
    fn incoming_logout(&mut self) -> io::Result<()> {
        self.us = FixPartyState::Logout;
        // transition to disconnected?
        Ok( () )
    }

    pub fn is_operational(&self) -> bool {
        self.us == FixPartyState::Operational && self.them == FixPartyState::Operational
    }
}

impl <Store : MessageStore> Display for FixSyncState <Store> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str( &format!("us {} - them {}", self.us.to_string(), self.them.to_string()) )
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
