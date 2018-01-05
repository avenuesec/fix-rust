use std::io;
use std::borrow::Cow;
// use std::rc::Rc;
use std::sync::Arc;
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
use super::syncstate::*;
use super::resendresponse::*;

const EVKIND_SENDER_TIMEOUT : Token = Token(0);
const EVKIND_RCV_TIMEOUT    : Token = Token(1);

pub struct SessionStateImpl <Store>
    where Store : MessageStore {

    config: FixSessionConfig,
    store: Arc<Mutex<Store>>,
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
        let store = Arc::new( Mutex::new(store) );
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
        self.state_machine.are_both_operational()
    }

    fn update_last_sent(&mut self) {
        self.last_sent = Some(Utc::now()); // sys call? need to check
        debug!("update_last_sent is operational? {}", self.is_operational());
        if !self.is_operational() {
            return
        }
        // Cancel existing, if any
        if let Some(timeout) = self.send_timeout.take() {
            self.sender.as_ref().map(move |s| s.cancel_timeout(timeout));
        }
        let hb_in_ms = self.hearbeat_in_ms as u32;
        self.sender.as_ref().map(move |s| s.set_timeout(hb_in_ms, EVKIND_SENDER_TIMEOUT));
    }
    fn update_last_recv(&mut self) {
        self.last_recv = Some(Utc::now()); // sys call? need to check
        debug!("update_last_recv is operational? {}", self.is_operational());
        if !self.is_operational() {
            return
        }
        // Cancel existing, if any
        if let Some(timeout) = self.recv_timeout.take() {
            self.sender.as_ref().map(move |s| s.cancel_timeout(timeout));
        }
        let hb_in_ms = self.hearbeat_in_ms as u32;
        self.sender.as_ref().map(move |s| s.set_timeout(hb_in_ms, EVKIND_RCV_TIMEOUT));
    }

    fn send_hearbeat_in_response(&mut self, test_req_id: &str) {
        info!("received test request with {} sending heartbeat in response", test_req_id);

        let hb_flds = HeartbeatFields {
            test_req_id: Some(test_req_id.to_owned()),
        };
        let _ = self.post_send( FixMessage::Heartbeat(Box::new(hb_flds)) );
    }

    fn ack_hearbeat_received(&mut self, test_req_id: &Option<String>) {
        info!("received heartbeat with {:?}", test_req_id );
    }

    fn ack_logout_received(&mut self, _flds: &LogoutFields) {

    }

    fn ack_logon_received(&mut self, flds: &LogonFields) {
        info!("received server logon with {:?}", flds );

//        if flds.reset_seq_num_flag.unwrap_or(false) && !self.config.reset_seq_num {
//            info!("reseting seqs nums as per server request");
//            if let Ok(mut store) = self.store.try_lock() {
//                let _ = store.reset_seqs();
//            }
//        }

        if flds.heart_bt_int != self.config.heart_beat as i32 {
            info!("server asked for a different hearbeat. our cfg {} - server {}", self.config.heart_beat, flds.heart_bt_int);
        }
    }

    fn post_send(&self, message: FixMessage) {
        self.sender.as_ref().map(move |s| s.send_self(message) );
    }

    fn post_resend(&self, frame: FixFrame) {
        self.sender.as_ref().map(move |s| s.send_self_frame(frame) );
    }

    /// Gracefully disconnects by sending a logout message
    /// with the reason and then closing the socket
    fn post_disconnect(&mut self, reason : &str) {
        let flds = LogoutFields {
            text: Some(reason.to_owned()),
            .. Default::default()
        };
        self.post_send( FixMessage::Logout(Box::new(flds)) );
        self.sender.as_ref().map(move |s| s.disconnect() );
    }

    /// resends messages within the given range
    fn do_resend(&mut self, start : i32, end : i32) -> io::Result<()> {
        let mut entries = {
            if let Ok(mut store) = self.store.try_lock() {
                build_resend_request_response::<Store>( &mut store, start, end )?
            } else {
                panic!("could not obtain lock");
            }
        };

        for entry in entries.drain(0..) {
            let frame = self.build_frame_for_resend( entry )?;
            self.post_resend( frame );
        }

        Ok( () )
    }

    fn build_frame_for_resend(&mut self, message: MessageToReSend) -> io::Result<FixFrame> {
        let mut frame = self.build_frame(message.message, false)?;
        frame.header.msg_seq_num = message.seq;
        frame.header.orig_sending_time = Some( message.orig_sending_time );
        frame.header.poss_dup_flag = Some(true);
        Ok( frame )
    }

    /// sends a ResendRequest message request a range (gap fill) from the other party
    fn request_resend(&mut self, start: i32, end: i32) -> io::Result<()> {
        let flds = ResendRequestFields {
            begin_seq_no: start,
            end_seq_no: end,
//            end_seq_no: 0,
        };
        let message = FixMessage::ResendRequest(Box::new(flds));
        self.post_send(message);
        Ok( () )
    }

    fn enable_timeouts(&mut self) {
        // enable heartbeats/test reqs
        if self.recv_timeout.is_none() {
            self.update_last_recv();
        }
        if self.send_timeout.is_none() {
            self.update_last_sent();
        }
    }
}

impl <Store> SessionState for SessionStateImpl <Store> where Store : MessageStore {

    fn init(&mut self, sender: Sender) {
        if let Ok(mut store) = self.store.try_lock() {
            store.init( sender.clone() );
        }

        self.sender = Some(sender);

        let reset_seq_num_flag = self.config.reset_seq_num;

        if reset_seq_num_flag {
            if let Ok(mut store) = self.store.try_lock() {
                let _ = store.reset_seqs();
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

    fn build_frame(&mut self, message: FixMessage, fill_seq: bool) -> io::Result<FixFrame> {
        let next_seq = {
            if fill_seq {
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

    fn received(&mut self, frame: &FixFrame) -> io::Result<()> {
        // record incoming
        if let Ok(mut store) = self.store.try_lock() {
            store.received( frame )?
        } else {
            return Err( io::Error::new(io::ErrorKind::Other, "could not obtain lock") );
        }

        self.update_last_recv();

        // Logon needs special treatment
        if frame.message.msg_type() == FieldMsgTypeEnum::Logon {
            match &frame.message {
                &FixMessage::Logon(ref flds) => { self.ack_logon_received(flds); },
                _ => { }
            }
        }

        // if it's a session level:
        match self.state_machine.register_recv( &frame )? {
            TransitionAction::RequestResendRange( range ) => {
                self.request_resend( range.0, range.1 )?;
                return Ok( () )
            },
            TransitionAction::DoResendRange( range ) => {
                self.do_resend( range.0, range.1 )?;
                return Ok( () )
            },
            TransitionAction::DoResendAndRequestRange( (send_range, req_range) ) => {
                self.do_resend( send_range.0, send_range.1 )?;
                self.request_resend( req_range.0, req_range.1 )?;
                return Ok( () )
            },
            TransitionAction::LogoutWith( reason ) => {
                self.post_disconnect( reason );
                return Ok( () )
            },
            TransitionAction::None => {}
        }

        match &frame.message {
            &FixMessage::Logon(_)              => self.enable_timeouts(),
            &FixMessage::Logout(ref flds)      => self.ack_logout_received( flds ),
            &FixMessage::TestRequest(ref flds) => self.send_hearbeat_in_response( &flds.test_req_id ),
            &FixMessage::Heartbeat(ref flds)   => self.ack_hearbeat_received( &flds.test_req_id ),
            _ => { }
        }

        Ok( () )
    }

    fn sent(&mut self, frame: &FixFrame) -> io::Result<()> {

        // record outgoing
        self.state_machine.register_sent( &frame )?;

        self.update_last_sent();

        if let Ok(mut store) = self.store.try_lock() {
            store.sent( frame )?
        } else {
            return Err( io::Error::new(io::ErrorKind::Other, "could not obtain lock") );
        }

        Ok( () )
    }

    fn new_timeout(&mut self, timeout: &timer::Timeout, event_kind: Token) {

        match event_kind {
            EVKIND_SENDER_TIMEOUT => {
                self.send_timeout = Some(timeout.clone());
            },
            EVKIND_RCV_TIMEOUT => {
                self.recv_timeout = Some(timeout.clone());
            },
            _ => { }
        }
    }

    fn on_timeout(&mut self, event_kind: Token) {
        debug!("on_timeout - kind {:?}", event_kind);

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

        if let Ok(store) = Arc::try_unwrap(self.store) {
            let _ = store.into_inner().unwrap().close();
        }

        Ok( () )
    }
}
