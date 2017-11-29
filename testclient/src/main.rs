
#[macro_use] extern crate log;

extern crate env_logger;
extern crate fix;
extern crate fixclient;
extern crate chrono;


use std::io;
use std::net::{SocketAddr, ToSocketAddrs};

use fix::fixmessagegen::*;

use fixclient::*;
use fixclient::sender::{Sender};
use fixclient::connector::handler::DefaultHandler;
use fixclient::connector::fsstore::FSMessageStore;
use fixclient::connector::session::SessionStateImpl;
use fixclient::connector::{UserSender,UserHandler,UserHandlerFactory};


fn main() {
    env_logger::init().unwrap();


    let addr = to_addr("localhost:8000").expect("Could not parse address for fixserver");

    let factory = FixCustomHandlerFactory { };

    let mut fix_app = FixApp::new(factory);

    let _ = fix_app.connect(&addr);

    fix_app.run(); // blocks

}


fn to_addr<A: ToSocketAddrs>(add_representation : A) -> Result<SocketAddr, io::Error> {
    match add_representation.to_socket_addrs() {
        Ok(mut addrs) => {
            let v4 = addrs.nth(0).expect("expecting parsing of at least 1 V4 address");
            Ok(v4)
        }
        Err(e) => Err(e)
    }
}

pub struct FixCustomHandlerFactory { }

impl FixHandlerFactory for FixCustomHandlerFactory {
    type Handler = DefaultHandler<SessionStateImpl<FSMessageStore>, UserHandlerFactory2>;

    fn on_connected(&mut self, destination: &SocketAddr, sender: Sender) -> Self::Handler {
        // needs to map SocketAddr to fixsessionconfig

        let settings = FixSessionConfig {
            qualifier:     "mainsess".to_owned(),
            sender_comp:   "SINGU".to_owned(),
            target_comp:   "SOME".to_owned(),
            hostname:      "127.0.0.1".to_owned(),
            port:          8000,
            heart_beat:    60,
            log_dir:       "./logs".to_owned(),
            store_dir:     "./store".to_owned(),
            reset_seq_num: false,
            use_local_time: true,
            session_end:   DayTime::new(  0,  0, 0, None ),
            session_start: DayTime::new( 23, 59, 0, None ),
            begin_string : "FIX.4.2".to_owned(),
        };

        let fsstore = FSMessageStore::new( &settings ).unwrap(); // Better error handling here
        let state   = SessionStateImpl::new( &settings, fsstore );

        // |usersender| {  ServiceFixHandler { sender: usersender } }
        let f = UserHandlerFactory2 { };

        let mut handler = DefaultHandler::new(sender, settings, state, f);
        handler.init();
        handler
    }

    fn on_shutdown(&mut self) {
        debug!("Factory received shutdown request.");
    }
}

pub struct UserHandlerFactory2 { }

impl UserHandlerFactory for UserHandlerFactory2 {
    type Handler = ServiceFixHandler;

    fn build(&mut self, sender: UserSender) -> Self::Handler {
        ServiceFixHandler {
            sender
        }
    }
}

pub struct ServiceFixHandler {
    sender: UserSender,
}

impl UserHandler for ServiceFixHandler {

    fn on_new_order_single(&mut self, message: &NewOrderSingleFields) -> io::Result<()> {

        Ok( () )
    }
}