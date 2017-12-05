use std::io;
use std::borrow::Cow;
use super::SessionState;

use chrono::{Utc, DateTime};
use mio::{Token};
use mio_more::timer;

use fix::fixmessagegen::*;
use fix::frame::FixFrame;
use super::super::FixSessionConfig;
use super::MessageStore;
use super::super::Sender;


const EVKIND_SENDER_TIMEOUT : Token = Token(0);
const EVKIND_RCV_TIMEOUT    : Token = Token(1);


pub struct SessionStateImpl <Store>
    where Store : MessageStore {

    config: FixSessionConfig,
    store: Store,
    logon_sent : bool,
    logon_recv : bool,
    seq_reset_sent: bool,
    last_sent: Option<DateTime<Utc>>,
    last_recv: Option<DateTime<Utc>>,
    sender: Option<Sender>,

    send_timeout: Option<timer::Timeout>,
    recv_timeout: Option<timer::Timeout>,
    hearbeat_in_ms: u32, // heartbeat in milliseconds
    begin_string: Cow<'static, str>,
}

impl <Store> SessionStateImpl <Store>
    where Store : MessageStore {

    pub fn new( cfg: &FixSessionConfig, store: Store ) -> SessionStateImpl<Store> {

        SessionStateImpl {
            config: cfg.clone(),
            logon_sent: false,
            logon_recv: false,
            seq_reset_sent: false,
            store,
            last_sent: None,
            last_recv: None,
            sender: None,
            send_timeout: None,
            recv_timeout: None,
            hearbeat_in_ms: (cfg.heart_beat as f32 * 1000.0 * 0.8) as u32, // converts it to ms and also lowers it a bit
            begin_string: Cow::from(cfg.begin_string.to_owned()),
        }
    }

    fn update_last_sent(&mut self) {
        if !self.logon_sent || !self.logon_recv {
            return
        }
        self.last_sent = Some(Utc::now()); // sys call? need to check

        // Cancel existing, if any
        if let Some(timeout) = self.send_timeout.take() {
            self.sender.as_ref().map(move |s| s.cancel_timeout(timeout));
        }
        let hb = self.hearbeat_in_ms;
        self.sender.as_ref().map(move |s| s.set_timeout(hb, EVKIND_SENDER_TIMEOUT));
    }
    fn update_last_recv(&mut self) {
        if !self.logon_sent || !self.logon_recv {
            return
        }
        self.last_recv = Some(Utc::now()); // sys call? need to check

        // Cancel existing, if any
        if let Some(timeout) = self.recv_timeout.take() {
            self.sender.as_ref().map(move |s| s.cancel_timeout(timeout));
        }
        let hb = self.hearbeat_in_ms;
        self.sender.as_ref().map(move |s| s.set_timeout(hb, EVKIND_RCV_TIMEOUT));
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

    fn ack_logon_received(&mut self, flds: &LogonFields) {
        info!("received server logon with {:?}", flds );

        if flds.reset_seq_num_flag.unwrap_or(false) && !self.seq_reset_sent {
            info!("reseting seqs nums as per server request");

            let _ = self.store.reset_seqs();
        }

        if flds.heart_bt_int != self.config.heart_beat as i32 {
            info!("server asked for a different hearbeat. our cfg {} - server {}", self.config.heart_beat, flds.heart_bt_int);
        }

        self.logon_recv = true;

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

    fn validate_incoming(&self, frame: &FixFrame) -> io::Result<()> {

        // TODO: validate sender, target, time within session timespan
        //

        Ok( () )
    }

    /// tries to fulfill request from the other side to supply missing messages
    /// we may send them or generate a sequence reset if there's a gap
    fn ack_resend_request(&mut self, start : u32, end : u32) {
        let entries = self.store.query(start, end)?;

        for i in 0..entries.len() {

            let frame = &entries[i];
            let seq = frame.seq;

            if fixmessagegen::is_admin_message( frame.message ) {
                // we dont resend admin messages

            } else {

            }

        }

        let reset_flds = SequenceResetFields {
            gap_fill_flag: Some(true),
            new_seq_no: new_seq_start,
        };
        self.post_send( FixMessage::SequenceReset(Box::new(reset_flds)) );

    }
}

impl <Store> SessionState for SessionStateImpl <Store> where Store : MessageStore {

    fn init(&mut self, sender: Sender) {
        self.store.init( sender.clone() );
        self.sender = Some(sender);

        let reset_seq_num_flag = self.config.reset_seq_num;

        if reset_seq_num_flag {
            self.seq_reset_sent = true;
            self.store.reset_seqs();
        }

        // Start login process
        let flds = LogonFields {
            encrypt_method: FieldEncryptMethodEnum::None,
            heart_bt_int: self.config.heart_beat as i32,
            reset_seq_num_flag: Some(reset_seq_num_flag),
            .. Default::default()
        };
        let logon_message = FixMessage::Logon(Box::new(flds));

        self.post_send( logon_message );
    }

    fn build(&mut self, message: FixMessage) -> io::Result<FixFrame> {

        let seq = self.store.incr_sender_seq_num()?;

        let frame = FixFrame {
            seq,
            message,
            sending: Utc::now(),
            sender_comp_id: self.config.sender_comp.to_owned(),
            target_comp_id: self.config.target_comp.to_owned(),
            begin_string: self.begin_string.clone(),
        };
        Ok ( frame )
    }

    fn sent(&mut self, frame: &FixFrame) -> io::Result<()> {

        match &frame.message {
            &FixMessage::Logon(ref bflds) => {
                self.logon_sent = true;
            },
            _ => {
                // nothing to do
            }
        }

        self.update_last_sent();

        self.store.sent( frame )?;

        Ok( () )
    }

    fn received(&mut self, frame: &FixFrame) -> io::Result<()> {

        self.store.received( frame )?;

        self.update_last_recv();

        // TODO: validate income seq number, and request missing frames in case there's a difference

        match &frame.message {
            &FixMessage::Logon(ref flds) => {
                self.ack_logon_received(flds.as_ref());
            },

            &FixMessage::ResendRequest(ref flds) => {
                let flds = flds.as_ref();
                self.ack_resend_request(flds.begin_seq_no, flds.end_seq_no)?
            },

            &FixMessage::SequenceReset(ref flds) => {

            },

            &FixMessage::TestRequest(ref flds) => {
                self.validate_incoming(&frame)?;

                self.send_hearbeat_in_response(&flds.test_req_id)
            },

            &FixMessage::Heartbeat(ref flds) => {

                self.validate_incoming(&frame)?;

                self.ack_hearbeat_received(&flds.test_req_id);
            }
            _ => {
                self.validate_incoming(&frame)?;

            }
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
}
