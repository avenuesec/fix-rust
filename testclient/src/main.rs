#[macro_use] extern crate log;

extern crate env_logger;
extern crate fix;
extern crate fixclient;
extern crate chrono;

use std::io;
use std::net::{SocketAddr, ToSocketAddrs};
use std::env;
use std::default::Default;
use std::thread;
use env_logger::LogBuilder;
use log::{LogRecord, LogLevelFilter};
use chrono::{Local};

use fix::fixmessagegen::*;

use fixclient::*;
use fixclient::sender::{Sender};
use fixclient::connector::handler::DefaultHandler;
use fixclient::connector::fsstore::FSMessageStore;
use fixclient::connector::fslogger::FSLogger;
use fixclient::connector::session::SessionStateImpl;
use fixclient::connector::{UserSender,UserHandler,UserHandlerFactory};


fn main() {
    set_up_logger();

    let addr = to_addr("127.0.0.1:8000").expect("Could not parse address for fixserver");
    println!("addr resolve to {:?}", addr);

    let factory = FixCustomHandlerFactory { };

    let mut fix_app = FixApp::new(factory);

    let adv_sender = fix_app.create_adv_sender();

    let _ = fix_app.connect(&addr);

    thread::spawn(move || {
        let _ = fix_app.run(); // blocks
    });

    println!("> Type ? for help");

    loop {
        let mut cmd = String::new();
        if let Err(e) = io::stdin().read_line(&mut cmd) {
            // something's wrong
            break;
        }

        match &cmd[..cmd.len() - 1] {
            "test_req" => {
                let flds = TestRequestFields {
                    test_req_id: "TEST".to_owned()
                };
                let msg = FixMessage::TestRequest(Box::new(flds));
                adv_sender.send( msg );
            },
            "new_order" => {
                let flds = NewOrderSingleFields {
                    cl_ord_id: "cldordid1".to_owned(),
                    account: Some("account".to_owned()),
                    handl_inst: FieldHandlInstEnum::AutomatedExecutionOrderPublicBrokerInterventionOk,
                    symbol: "AAPL".to_owned(),
                    side: FieldSideEnum::Buy,
                    ord_type: FieldOrdTypeEnum::Limit,
                    price: Some(300.2),
                    order_qty: Some(100.0),
                    .. Default::default()
                };
                let msg = FixMessage::NewOrderSingle(Box::new(flds));
                adv_sender.send( msg );
            },
            "exit" => {
                println!("Bye");
                break;
            },
            "?" => {
                println!("enter command and press enter. Commands available: ");
                println!("new_order  - sends a new LIMIT order");
                println!("test_req   - sends a TestRequest");
                println!("exit");
            }
            _ => {
                println!("Huh? Try again. Didn't recognized {}", cmd.as_str() );
            }
        }
    }

    // adv_sender.shutdown();
}


fn set_up_logger() {
    let format = |record: &LogRecord| {
        format!("{:5} {}: {}", record.level(), Local::now().format("%H:%M:%S%.3f"), record.args() )
    };

    let mut builder = LogBuilder::new();
    builder.format(format).filter(None, LogLevelFilter::Info);

    if env::var("RUST_LOG").is_ok() {
        builder.parse(&env::var("RUST_LOG").unwrap());
    } else {
        builder.parse("debug");
    }

    builder.init().unwrap();
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
    type Handler = DefaultHandler<SessionStateImpl<FSMessageStore, FSLogger>, UserHandlerFactory2>;

    fn on_started(&mut self, _destination: &SocketAddr, sender: Sender) -> Self::Handler {
        // needs to map SocketAddr to fixsessionconfig

        let settings = FixSessionConfig {
            qualifier:     "mainsess".to_owned(),
            sender_comp:   "SINGU".to_owned(),
            target_comp:   "AAAA".to_owned(),
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
        let fslogger = FSLogger::new()
        let state   = SessionStateImpl::new( &settings, fsstore, fslogger );

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

    fn on_new_order_single(&mut self, message: &NewOrderSingleFields, header: &FixHeader) -> io::Result<()> {
        info!("on_new_order_single received {:?}", message);

        Ok( () )
    }

    fn on_reject(&mut self, message: &RejectFields, header: &FixHeader) -> io::Result<()> {
        info!("on_reject received {:?}", message);
        Ok( () )
    }

    fn on_execution_report(&mut self, message: &ExecutionReportFields, header: &FixHeader) -> io::Result<()> {
        info!("on_execution_report received {:?}", message);
        Ok( () )
    }
}