use std::io;
use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Mutex;
use super::SessionState;

use chrono::{Utc, DateTime};
use mio::{Token};
use mio_more::timer;

use fix::fixmessagegen::*;
use fix::frame::FixFrame;
use super::super::FixSessionConfig;
use super::MessageStore;
use super::super::Sender;
use super::statemachine::*;


const EVKIND_SENDER_TIMEOUT : Token = Token(0);
const EVKIND_RCV_TIMEOUT    : Token = Token(1);


pub struct SessionStateImpl <Store>
    where Store : MessageStore {

    config: FixSessionConfig,
    store: Rc<Mutex<Store>>,
    last_sent: Option<DateTime<Utc>>,
    last_recv: Option<DateTime<Utc>>,
    sender: Option<Sender>,

    send_timeout: Option<timer::Timeout>,
    recv_timeout: Option<timer::Timeout>,
    hearbeat_in_ms: i32, // heartbeat in milliseconds
    begin_string: Cow<'static, str>,

    state_machine : FixSyncState <Store>,
}

impl <Store> SessionStateImpl <Store>
    where Store : MessageStore {

    pub fn new( cfg: &FixSessionConfig, store: Store ) -> SessionStateImpl<Store> {

        let store = Rc::new( Mutex::new(store) );

        SessionStateImpl {
            config: cfg.clone(),
            store: store.clone(),
            last_sent: None,
            last_recv: None,
            sender: None,
            send_timeout: None,
            recv_timeout: None,
            hearbeat_in_ms: (cfg.heart_beat as f32 * 1000.0 * 0.9) as i32, // converts it to ms and also lowers it a bit
            begin_string: Cow::from(cfg.begin_string.to_owned()),
            state_machine : FixSyncState::new( store.clone() ),
        }
    }

    pub fn is_operational(&self) -> bool {
        self.state_machine.is_operational()
    }

    fn update_last_sent(&mut self) {
        if !self.is_operational() {
            return
        }
//        if !self.logon_sent || !self.logon_recv {
//            return
//        }
        self.last_sent = Some(Utc::now()); // sys call? need to check

        // Cancel existing, if any
        if let Some(timeout) = self.send_timeout.take() {
            self.sender.as_ref().map(move |s| s.cancel_timeout(timeout));
        }
        let hb_in_ms = self.hearbeat_in_ms as u32;
        self.sender.as_ref().map(move |s| s.set_timeout(hb_in_ms, EVKIND_SENDER_TIMEOUT));
    }
    fn update_last_recv(&mut self) {
        if !self.is_operational() {
            return
        }
//        if !self.logon_sent || !self.logon_recv {
//            return
//        }
        self.last_recv = Some(Utc::now()); // sys call? need to check

        // Cancel existing, if any
        if let Some(timeout) = self.recv_timeout.take() {
            self.sender.as_ref().map(move |s| s.cancel_timeout(timeout));
        }
        let hb_in_ms = self.hearbeat_in_ms as u32;
        self.sender.as_ref().map(move |s| s.set_timeout(hb_in_ms, EVKIND_RCV_TIMEOUT));
    }

    fn send_hearbeat_in_response(&mut self, test_req_id: &str) {
        info!("received test request with {}", test_req_id);

        let hb_flds = HeartbeatFields {
            test_req_id: Some(test_req_id.to_owned()),
        };
        let _ = self.post_send( FixMessage::Heartbeat(Box::new(hb_flds)) );
    }

    fn ack_hearbeat_received(&mut self, test_req_id: &Option<String>) {
        info!("received heartbeat with {:?}", test_req_id );
    }

    fn ack_logout_received(&mut self, flds: &LogoutFields) {

    }

    fn ack_logon_received(&mut self, flds: &LogonFields) {
        info!("received server logon with {:?}", flds );

        if flds.reset_seq_num_flag.unwrap_or(false) && !self.config.reset_seq_num {
            info!("reseting seqs nums as per server request");

            // let _ = Rc::get_mut(&mut self.store).unwrap().reset_seqs();
            if let Ok(mut store) = self.store.try_lock() {
                store.reset_seqs();
            }
        }

        if flds.heart_bt_int != self.config.heart_beat as i32 {
            info!("server asked for a different hearbeat. our cfg {} - server {}", self.config.heart_beat, flds.heart_bt_int);
        }

//        self.logon_recv = true;

        // enable heartbeats/test reqs

        if self.recv_timeout.is_none() {
            self.update_last_recv();
        }
        if self.send_timeout.is_none() {
            self.update_last_sent();
        }
    }

    fn post_send(&self, message: FixMessage) {

        self.sender.as_ref().map(move |s| s.send_self(message) );
    }

    fn post_resend(&self, message: FixFrame) {

        self.sender.as_ref().map(move |s| s.send_self_frame(message) );
    }

    fn post_disconnect(&mut self, reason : &str) -> io::Result<()> {

        // TODO: Send logout with given reason, then disconnect

        Ok( () )
    }

    fn resend(&mut self, mut message: FixFrame) -> io::Result<()> {
        message.header.poss_dup_flag = Some(true);
        message.header.orig_sending_time = Some( message.header.sending_time.clone() );
        message.header.sending_time = UtcDateTime::new( Utc::now() );

        self.post_resend( message );

        Ok( () )
    }

    fn validate_incoming(&self, frame: &FixFrame) -> io::Result<()> {

        // TODO: validate sender, target, time within session timespan
        //

        Ok( () )
    }

    fn ack_sequence_reset(&mut self, new_seq_number: i32, gap_fill : Option<bool>) -> io::Result<()> {

        Ok( () )
    }

    /// tries to fulfill request from the other side to supply missing messages
    /// we may send them or generate a sequence reset if there's a gap
    fn ack_resend_request(&mut self, start : i32, end : i32) -> io::Result<()> {

        let mut entries = self.state_machine.build_resend_request_response( start, end );



        Ok( () )
    }

    fn send_sequence_reset(&mut self, seq_to_use: i32, new_seq_no: i32) -> io::Result<()> {
        let flds = SequenceResetFields {
            gap_fill_flag: Some(true),
            new_seq_no,
        };

        let message = FixMessage::SequenceReset(Box::new(flds));
        let frame = self.build(message, false)?;
        let mut frame = self.build_for_resend(frame)?;
        frame.header.msg_seq_num = seq_to_use;

        self.post_resend( frame );

        Ok( () )
    }

    fn send_resend_request(&mut self, start: i32) -> io::Result<()> {
        let flds = ResendRequestFields {
            begin_seq_no: start,
            end_seq_no: 0,
        };
        let message = FixMessage::ResendRequest(Box::new(flds));
        let frame = self.build(message, true)?;

//        let mut frame = self.build_for_resend(frame)?;
//        frame.header.msg_seq_num = start;

        // self.post_send( frame );

        Ok( () )
    }
}

impl <Store> SessionState for SessionStateImpl <Store> where Store : MessageStore {

    fn init(&mut self, sender: Sender) {

        // Rc::get_mut(&mut self.store).unwrap().init( sender.clone() );
        if let Ok(mut store) = self.store.try_lock() {
            store.init( sender.clone() );
        }

        self.sender = Some(sender);

        let reset_seq_num_flag = self.config.reset_seq_num;

        if reset_seq_num_flag {
            // Rc::get_mut(&mut self.store).unwrap().reset_seqs();
            if let Ok(mut store) = self.store.try_lock() {
                store.reset_seqs();
            } else {
                panic!( "could not obtain lock" );
            }
        }

        // Start login process
        let flds = LogonFields {
            encrypt_method: FieldEncryptMethodEnum::None,
            heart_bt_int: self.config.heart_beat,
            reset_seq_num_flag: Some(reset_seq_num_flag),
            .. Default::default()
        };

        let logon_message = FixMessage::Logon(Box::new(flds));

        self.post_send( logon_message );
    }

    fn build(&mut self, message: FixMessage, fill_seq: bool) -> io::Result<FixFrame> {

        let next_seq = {
            if fill_seq {
                // Rc::get_mut(&mut self.store).unwrap().incr_sender_seq_num()?
                if let Ok(mut store) = self.store.try_lock() {
                    store.incr_sender_seq_num()?
                } else {
                    return Err( io::Error::new(io::ErrorKind::Other, "could not obtain lock") );
                }
            } else {
                0
            }
        };

        let frame = FixFrame {
            header: FixHeader {
                msg_seq_num: next_seq,
                msg_type : message.msg_type(),
                sending_time: UtcDateTime::now(),
                sender_comp_id: self.config.sender_comp.to_owned(),
                target_comp_id: self.config.target_comp.to_owned(),
                begin_string: self.begin_string.clone(),
                .. Default::default()
            },
            message,
        };
        Ok ( frame )
    }

    fn build_for_resend(&mut self, original: FixFrame) -> io::Result<FixFrame> {

        let mut copy = original.clone();
        copy.header.poss_dup_flag = Some(true);
        copy.header.orig_sending_time = Some(original.header.sending_time.clone());
        copy.header.sending_time = UtcDateTime::now();

        Ok ( copy )
    }

    fn received(&mut self, frame: &FixFrame) -> io::Result<()> {
        // record incoming
        if let Ok(mut store) = self.store.try_lock() {
            store.received( frame )?
        } else {
            return Err( io::Error::new(io::ErrorKind::Other, "could not obtain lock") );
        }

        self.update_last_recv();

        match self.state_machine.register_recv( &frame )? {
            TransitionAction::RequestResendRange( range ) => {
                // self.send_resend_request( start )?;
                return Ok( () )
            },
            TransitionAction::DoResendRange( range ) => {
                // self.resend( range.0, range.1 )?;
                return Ok( () )
            },
            TransitionAction::DoResendAndRequestRange( (send_range, req_range) ) => {
                // self.resend( range.0, range.1 )?;
                // self.send_resend_request( start )?;
                return Ok( () )
            },
            TransitionAction::LogoutWith( reason ) => {
                // self.post_disconnect( reason )?;
                return Ok( () )
            },
            TransitionAction::None => {}
        }

        // we have to process admin level messages, and pass thru app level messages
//        match &frame.message {
//            &FixMessage::Logon(ref flds) => {
//                self.ack_logon_received( flds.as_ref() );
//            },
//            &FixMessage::Logout(ref flds) => {
//                self.ack_logout_received(flds.as_ref());
//            },
//            &FixMessage::ResendRequest(ref flds) => {
//                self.ack_resend_request(flds.begin_seq_no, flds.end_seq_no)?;
//            },
//            &FixMessage::SequenceReset(ref flds) => {
//                self.ack_sequence_reset( flds.new_seq_no, flds.gap_fill_flag )?;
//            },
//            &FixMessage::TestRequest(ref flds) => {
//                self.validate_incoming(&frame)?;
//                self.send_hearbeat_in_response(&flds.test_req_id)
//            },
//            &FixMessage::Heartbeat(ref flds) => {
//                self.validate_incoming(&frame)?;
//                self.ack_hearbeat_received(&flds.test_req_id);
//            }
//            _ => {
//                self.validate_incoming(&frame)?;
//            }
//        }

        Ok( () )
    }

    fn sent(&mut self, frame: &FixFrame) -> io::Result<()> {

        // record outgoing
        self.state_machine.register_sent( &frame )?;

//        match &frame.message {
//
//            &FixMessage::Logon(ref flds) => {
//                // self.state_machine.outgoing_logon()?;
//            },
//            &FixMessage::Logout(_) => {
//                // self.state_machine.outgoing_logout()?;
//            },
//            &FixMessage::ResendRequest(_) => {
//                // self.state_machine.outgoing_resend()?;
//            },
//            &FixMessage::SequenceReset(_) => {
//                // self.state_machine.outgoing_seqreset()?;
//            },
//            _ => {
//                // nothing to do
//            }
//        }

        self.update_last_sent();

        // Rc::get_mut(&mut self.store).unwrap().sent( frame )?;
        if let Ok(mut store) = self.store.try_lock() {
            store.sent( frame )?
        } else {
            return Err( io::Error::new(io::ErrorKind::Other, "could not obtain lock") );
        }

        Ok( () )
    }

    fn new_timeout(&mut self, _timeout: &timer::Timeout, _event_kind: Token) {

    }

    fn on_timeout(&mut self, event_kind: Token) {

        if event_kind == EVKIND_SENDER_TIMEOUT {

            if let Some(last) = self.last_sent {
                let now = Utc::now();
                let duration_since_last_sent = now.signed_duration_since(last);
                debug!("duration_since_last_sent {}", duration_since_last_sent);


                let flds = TestRequestFields {
                    test_req_id: "TEST".to_owned()
                };
                let _ = self.post_send(FixMessage::TestRequest(Box::new(flds)));
            }

        } else if event_kind == EVKIND_RCV_TIMEOUT {

            if let Some(last) = self.last_recv {
                let now = Utc::now();
                let duration_since_last_rcv  = now.signed_duration_since(last);
                debug!("duration_since_last_rcv {}", duration_since_last_rcv);

                let flds = HeartbeatFields {
                    test_req_id: None,
                    // test_req_id: Some("b".to_owned())
                };
                let _ = self.post_send(FixMessage::Heartbeat( Box::new(flds)));
            }
        }
    }

    fn close(self) -> io::Result<()> {
        info!("session close");

        drop(self.state_machine);

        if let Ok(mut store) = Rc::try_unwrap(self.store) {
            store.into_inner().unwrap().close();
        }

        Ok( () )
    }
}
