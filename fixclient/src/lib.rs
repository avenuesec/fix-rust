
// #![deny(warnings, missing_docs)]

extern crate nom;
#[macro_use] extern crate log;
// #[macro_use] extern crate serde_derive;

extern crate chrono;
extern crate mio;
extern crate mio_more;
extern crate slab;
extern crate bytes;
// extern crate byteorder;
// extern crate serde;
extern crate fix;

use std::{io, str};
use std::net::SocketAddr;

// use chrono::{DateTime, Utc};

use fix::frame;

//#[derive(Default)]
//pub struct FixConnectionConfig {
//    pub session   : FixSessionConfig, // Vec<FixSessionConfig>,
//}

pub enum DayOfTheWeek {
    Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday
}

pub struct DayTime {
    pub hour: u8,
    pub min:  u8,
    pub sec:  u8,
    pub day_of_the_week: Option<DayOfTheWeek>,
}
impl DayTime {
    pub fn new( hour: u8, min: u8, sec: u8, day_of_the_week : Option<DayOfTheWeek> ) -> DayTime {
        DayTime {
            hour, min, sec, day_of_the_week
        }
    }
}

// #[derive(Default)]
pub struct FixSessionConfig {
    pub qualifier: String,
    pub sender_comp: String,
    pub target_comp: String,
    pub hostname: String,
    pub port: u32,
    pub heart_beat: u32,
    pub log_dir   : String,
    pub store_dir : String,
    pub reset_seq_num: bool,
    pub use_local_time: bool, // start/end should be interpret as local time instead of UTC
    pub session_start : DayTime,
    pub session_end   : DayTime,
}

use mio::{Poll, Token};
use mio_more::{timer};
use std::net::ToSocketAddrs;

pub mod conn;
pub mod io_handler;
pub mod sender;
pub mod connector;

use self::io_handler::{IoHandler};
use self::sender::{Sender, AdvSender};

impl FixSessionConfig {

    pub fn new ( qualifier: &str, sender: &str, target: &str, hostname: &str,
                 port: u32, heart_beat: u32, log: &str, store: &str ) -> FixSessionConfig {

        FixSessionConfig {
            qualifier: qualifier.to_owned(),
            sender_comp: sender.to_owned(),
            target_comp: target.to_owned(),
            log_dir: log.to_owned(),
            store_dir: store.to_owned(),
            hostname: hostname.to_owned(),
            port,
            heart_beat,
            reset_seq_num: false,
            use_local_time: false,
            session_start: DayTime::new(0, 01, 0, None),   // 1 min past   midnight
            session_end:   DayTime::new(23, 59, 0, None),  // 1 min before midnight
        }
    }
}

pub struct FixApp <F : FixHandlerFactory> {
    io: IoHandler<F>,
}

impl<F : FixHandlerFactory> FixApp<F> {
    
    pub fn new(factory: F) -> FixApp<F> {
        FixApp {
            io: IoHandler::new( factory, Poll::new().unwrap() ),
        }
    }

    pub fn connect<A : ToSocketAddrs>(&mut self, addr: &A) -> io::Result<()> {
        let mut addr = addr.to_socket_addrs().unwrap();
        let first = addr.nth(0).unwrap();
        self.io.connect( &first )?;
        Ok( () )
    }

    pub fn create_adv_sender(&self) -> AdvSender {
        self.io.create_adv_sender()
    }

    // Starts
    pub fn run(&mut self) -> io::Result<()> {
        self.io.run()?;
        Ok( () )
    }
}

// best way to give the handler a single Sender, simplifies the rest of the impl
pub trait FixHandlerFactory {
    type Handler: FixHandler;

    fn on_connected(&mut self, destination: &SocketAddr, sender: Sender) -> Self::Handler;

    fn on_shutdown(&mut self) {
        debug!("Factory received WebSocket shutdown request.");
    }
}

pub trait FixHandler {
    
    fn on_message(&mut self, message: frame::FixFrame) -> io::Result<()>;

    fn on_timeout(&mut self, event_kind: Token) -> io::Result<()>;

    fn new_timeout(&mut self, timeout: timer::Timeout, event_kind: Token) {
    }

    fn on_network_error(&mut self); // <- add error info

    // back channel to allow extensions to send messages
    fn before_send(&mut self, message: fix::fixmessagegen::FixMessage);
}

// Super cool way of adapting a Fn to a trait
impl<F, H> FixHandlerFactory for F 
    where H : FixHandler,
          F : FnMut(Sender) -> H {
    type Handler = H;

    fn on_connected(&mut self, _destination: &SocketAddr, sender: Sender) -> Self::Handler {
        // when the "event" is called, we just invoke the FnMut which should return a handler
        self(sender)
    }
}



