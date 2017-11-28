use std::io;
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

    sender_comp_id : String,
    target_comp_id : String,
    store: Store,
    logon_sent : bool,
    logon_recv : bool,
    last_sent: Option<DateTime<Utc>>,
    last_recv: Option<DateTime<Utc>>,
    sender: Option<Sender>,

    send_timeout: Option<timer::Timeout>,
    recv_timeout: Option<timer::Timeout>,
    hearbeat_in_ms: u32, // heartbeat in milliseconds
    config_heartbeat: u32, // original config, unless server overrides it
}

impl <Store> SessionStateImpl <Store>
    where Store : MessageStore {

    pub fn new( cfg: &FixSessionConfig, store: Store ) -> SessionStateImpl<Store> {

        SessionStateImpl {
            sender_comp_id: cfg.sender_comp.to_owned(),
            target_comp_id: cfg.target_comp.to_owned(),
            logon_sent: false,
            logon_recv: false,
            store,
            last_sent: None,
            last_recv: None,
            sender: None,
            send_timeout: None,
            recv_timeout: None,
            config_heartbeat: cfg.heart_beat,
            hearbeat_in_ms: (cfg.heart_beat as f32 * 1000.0 * 0.8) as u32, // converts it to ms and also lowers it a bit
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
        let _ = self.sender.as_ref().map(move |s| s.send_self( FixMessage::Heartbeat(Box::new(hb_flds)) ));
    }

    fn ack_hearbeat_received(&mut self, test_req_id: &Option<String>) {
        info!("received heartbeat with {:?}", test_req_id );
    }

    fn ack_logon_received(&mut self, flds: &LogonFields) {
        info!("received server logon with {:?}", flds );

        if flds.reset_seq_num_flag.unwrap_or(false) {
            info!("reseting seqs nums as per server request");

            self.store.reset_seqs();
        }

        if flds.heart_bt_int != self.config_heartbeat as i32 {
            info!("server asked for a different hearbeat. our cfg {} - server {}", self.config_heartbeat, flds.heart_bt_int);

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
}

impl <Store> SessionState for SessionStateImpl <Store> where Store : MessageStore {

    fn init(&mut self, sender: Sender) {
        self.store.init( sender.clone() );
        self.sender = Some(sender);
    }

    fn build(&mut self, message: FixMessage) -> io::Result<FixFrame> {

        let seq = self.store.incr_sender_seq_num()?;

        let frame = FixFrame {
            seq,
            message,
            sending: Utc::now(),
            sender_comp_id: self.sender_comp_id.to_owned(),
            target_comp_id: self.target_comp_id.to_owned(),
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

        match &frame.message {
            &FixMessage::Logon(ref flds) => {
                self.ack_logon_received(flds.as_ref());
            },

            &FixMessage::TestRequest(ref flds) => {
                self.send_hearbeat_in_response(&flds.test_req_id)
            },

            &FixMessage::Heartbeat(ref flds) => {
                self.ack_hearbeat_received(&flds.test_req_id);
            }
            _ => {}
        }

        Ok( () )
    }

    fn new_timeout(&mut self, timeout: &timer::Timeout, event_kind: Token) {

    }

    fn on_timeout(&mut self, event_kind: Token) {

        if event_kind == EVKIND_SENDER_TIMEOUT {

            if let Some(last) = self.last_sent {

                let now = Utc::now();
                let duration_since_last_sent = now.signed_duration_since(last);

                let flds = TestRequestFields {
                    test_req_id: "TEST".to_owned()
                };
                let _ = self.sender.as_ref().map(move |s| s.send_self(FixMessage::TestRequest(Box::new(flds))));
            }

        } else if event_kind == EVKIND_RCV_TIMEOUT {

            if let Some(last) = self.last_recv {
                let now = Utc::now();
                let duration_since_last_rcv  = now.signed_duration_since(last);

                let flds = HeartbeatFields {
                    test_req_id: None,
                    // test_req_id: Some("b".to_owned())
                };
                let _ = self.sender.as_ref().map(move |s| s.send_self(FixMessage::Heartbeat( Box::new(flds))));
            }
        }
    }
}