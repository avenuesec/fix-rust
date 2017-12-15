
use std::string::ToString;
use std::result::Result;
use std::io;
use std::fmt::{Display, Error, Formatter};
use std::rc::Rc;

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
    them:       FixPartyState,
    us:         FixPartyState,
    store:      Rc<Store>,
    sent_count: u32,
    recv_count: u32,
    their_gap : Option<(i32, i32)>,
    our_gap   : Option<(i32, i32)>,
}

#[derive(Debug,Copy,Clone)]
pub enum TransitionAction {
    None,
    LogoutWith(&'static str),  // <- reason
    RequestResendFrom(i32),    // <- start seq
    ResendRange( (i32, i32) )  // <- start,end
}

impl TransitionAction {
    fn is_none(&self) -> bool {
        match self {
            &TransitionAction::None => true,
            _ => false
        }
    }
}

impl <Store : MessageStore> FixSyncState <Store> {

    pub fn new( store: Rc<Store> ) -> FixSyncState <Store> {
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
                    Rc::get_mut(&mut self.store).unwrap().overwrite_target_seq(recv_seq);

                    // no point in trying to determine gap, so transition
                    self.them = FixPartyState::Operational;
                }
            },

            &FixMessage::ResendRequest(ref flds) => {
                // The other party identified a message gap

                // what's the gap range?
                let end_seq_no =
                    if flds.end_seq_no == 0 {
                        self.store.next_sender_seq_num() - 1
                    } else {
                        flds.end_seq_no
                    };
                // Save our gap so we can tell when it's done
                self.our_gap = Some( (flds.begin_seq_no, end_seq_no) );
                self.us = FixPartyState::MessageSynchronization;

                // Instruct to resend
                return Ok( TransitionAction::ResendRange( self.our_gap.unwrap() ) );
            }

            &FixMessage::SequenceReset(ref flds) => {
                let gap_filling = flds.gap_fill_flag.map_or(false, |v| v);
                let new_seq = flds.new_seq_no;

                if !gap_filling {
                    // Reset in no gap filling mode = disaster recovery
                    warn!("Disaster recovery SequenceReset received. {} new seq: {}", gap_filling, new_seq)
                }

                Rc::get_mut(&mut self.store).unwrap().overwrite_target_seq(new_seq);

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

    /// check if there's a gap in the incoming messages
    fn detect_gap_and_choose_action(&mut self, header: &FixHeader, message: &FixMessage) -> TransitionAction {

        // we shouldn't process poss_dup messages
        assert_eq!(header.poss_dup_flag.map_or(false, |v| v), false);

        // incr_target_seq_num mutates the store
        let expected_seq        = Rc::get_mut(&mut self.store)
                                      .map(|s| s.incr_target_seq_num() ).unwrap().unwrap();
        let recv_target_seq_num = header.msg_seq_num;

        if expected_seq > recv_target_seq_num {
            warn!("detect_gap_and_choose_action: expecting seq {} but received {}. \
                   Initiating resend request, holding recv/sending until synchronization is complete", expected_seq, recv_target_seq_num);

            self.them = FixPartyState::MessageSynchronization;
            self.their_gap = Some( (expected_seq, recv_target_seq_num) );

            // request missing messages
            if message.msg_type() == FieldMsgTypeEnum::ResendRequest {
                return TransitionAction::ResendRange( self.their_gap.unwrap() ) ;
            }

            return TransitionAction::RequestResendFrom( expected_seq ) ;

        } else if expected_seq < recv_target_seq_num {
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

    // Experimental
//    pub fn can_send_app_messages(&self) -> bool {
//        true
//    }
//
//    pub fn can_receive_app_messages(&self) -> bool {
//        true
//    }
    // End Experimental

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

//    pub fn incoming_sync_complete(&mut self) {
//        self.them = FixPartyState::Operational;
//    }
//
//    pub fn outgoing_sync_complete(&mut self) {
//        self.us = FixPartyState::Operational;
//    }

    /// other side requested resend of missing messages (gap fill)
//    pub fn incoming_resend(&mut self) -> io::Result<()> {
//        if self.is_operational() {
//            self.them = FixPartyState::AwaitingResend;
//            Ok( () )
//        } else {
//            Err( io::Error::new(io::ErrorKind::Other, format!("incoming_resend not valid at this point. current: {}", self).as_str() ) )
//        }
//    }

    /// we requested resend of missing messages when a gap is detected
//    pub fn outgoing_resend(&mut self) -> io::Result<()> {
//        if self.is_operational() {
//            self.us = FixPartyState::AwaitingResend;
//            Ok( () )
//        } else {
//            Err( io::Error::new(io::ErrorKind::Other, format!("outgoing_resend not valid at this point. current: {}", self).as_str() ) )
//        }
//    }

//    pub fn confirm_resent_sent(&mut self) -> io::Result<()>  {
//        if self.them == FixPartyState::AwaitingResend {
//            self.them = FixPartyState::Operational;
//            Ok( () )
//        } else {
//            Err( io::Error::new(io::ErrorKind::Other, format!("confirm_resent_sent not valid at this point. current: {}", self).as_str() ) )
//        }
//    }

//    pub fn confirm_resent_recv(&mut self) -> io::Result<()>  {
//        if self.us == FixPartyState::AwaitingResend {
//            self.us = FixPartyState::Operational;
//            Ok( () )
//        } else {
//            Err( io::Error::new(io::ErrorKind::Other, format!("confirm_resent_recv not valid at this point. current: {}", self).as_str() ) )
//        }
//    }

//    pub fn incoming_seqreset(&mut self) -> io::Result<()> {
//        // since this can happen in many situations, it's always acceptable
//        // Operational and AwaitingResend would be the most obvious
//
//        Ok( () )
//    }
//
//    pub fn outgoing_seqreset(&mut self) -> io::Result<()> {
//        // since this can happen in many situations, it's always acceptable
//        // Operational and AwaitingResend would be the most obvious
//
//        Ok( () )
//    }

//    pub fn disconnect(&mut self) {
//        self.them = FixPartyState::Disconnected;
//        self.us = FixPartyState::Disconnected;
//    }

    pub fn is_operational(&self) -> bool {
        self.us == FixPartyState::Operational && self.them == FixPartyState::Operational
    }
}

impl <Store : MessageStore> Display for FixSyncState <Store> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str( &format!("us {} - them {}", self.us.to_string(), self.them.to_string()) )
    }
}

impl ToString for FixPartyState {
    fn to_string(&self) -> String {
        match self {
            &FixPartyState::Connected               => "connected",
            // &FixPartyState::Disconnected     => "disconnected",
            &FixPartyState::Logon                   => "logon",
            &FixPartyState::Logout                  => "logout",
            &FixPartyState::MessageSynchronization  => "resync",
            &FixPartyState::Operational             => "operational",
            _ => unreachable!()
        }.to_owned()
    }
}
