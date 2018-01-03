extern crate fix;
extern crate fixclient;
extern crate mio;
extern crate mio_more;

use std::default::Default;
use std::sync::Mutex;

use mio_more::{channel};
use mio_more::channel::Receiver;
use mio::Token;

use fix::frame::*;
use fix::fixmessagegen::*;

use fixclient::{FixSessionConfig, FixDictionary};
use fixclient::connector::*;
use fixclient::connector::MessageStore;
use fixclient::connector::session::*;
use fixclient::connector::memstore::*;
use fixclient::sender::Sender;
use fixclient::io_handler::{Command, CommandAction};
use fixclient::builder;


fn basic_init<F>( f : F ) -> (SessionStateImpl <MemoryMessageStore>, Receiver<Command>)
    where F : FnOnce(&mut MemoryMessageStore) -> ()
{
    let cfg = FixSessionConfig::new("qualifier", "sender", "target", "host",
                                    5000, 60, "logfolder", "store", FixDictionary::Fix42);
    let mut session = {
        let mut store = MemoryMessageStore::new(&cfg).expect("expecting store");
        f( &mut store );

        SessionStateImpl::new( &cfg, store )
    };
    let (tx, rx) = channel::sync_channel(100);
    let sender = Sender::new( Token(1), tx );
    session.init( sender );
    redispath_to_session( &rx, &mut session );
    (session, rx)
}

// #[test]
fn test_logon_handshake() {

    // Arrange
    let (mut session, _) = basic_init( |_| {  } );

    // Act
    confirm_logon( 1, &mut session );

    // Assert
    assert!( session.is_operational() );

}

// #[test]
fn test_resend_req_from_server() {
    // Arrange
    let (mut session, rx) = basic_init( |store| {
        store.add_to_store( builder::build_new_order_single( 2, false, "cl1", "AAPL",  100.0, 172.2,  FieldSideEnum::Buy, FieldOrdTypeEnum::Market) );
        store.add_to_store( builder::build_new_order_single( 3, false, "cl2", "MSFT",  200.0, 81.4,   FieldSideEnum::Buy, FieldOrdTypeEnum::Market) );
        store.add_to_store( builder::build_new_order_single( 4, false, "cl3", "GOOGL", 100.0, 1001.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Market) );
        store.add_to_store( builder::build_heartbeat( 5, Some("test") ) );
        store.add_to_store( builder::build_heartbeat( 6, Some("test") ) );
        store.add_to_store( builder::build_new_order_single( 7, false, "cl4", "YHOO",  100.0, 53.0,   FieldSideEnum::Buy, FieldOrdTypeEnum::Market) );
        store.add_to_store( builder::build_heartbeat( 8, Some("test2") ) );
    } );
    confirm_logon( 1, &mut session );

    // Act
    session.received( &builder::build_resend_req(2, 2, 0) ).unwrap();

    // Assert
    let outgoing = extract_outgoing(&rx);
    for m in outgoing.iter() {
        println!("{:?}", m.header);

        match &m.message {
            &FixMessage::SequenceReset(ref flds) => {
                println!("\t{:?}\r\n", flds);
            },
            _ => {
                println!("");
            }
        }
    }
    assert!( session.is_operational() );
}


fn extract_outgoing(rx : &Receiver<Command>) -> Vec<FixFrame> {
    let mut vec = vec![];

    loop {
        if let Ok(cmd) = rx.try_recv() {
            match cmd.action {
                CommandAction::SendFrameBackToHandler(f) => {
                    vec.push(f)
                },
                CommandAction::SendBackToHandler(m) => {
                    println!("found SendBackToHandler {:?}", m);
                },
                _ => {

                }
            }
        } else {
            break;
        }
    }

    vec
}

fn redispath_to_session( rx : &Receiver<Command>, session: &mut SessionStateImpl <MemoryMessageStore> ) {

    let cmd = rx.try_recv().unwrap();

    let message = match cmd.action {
        CommandAction::SendBackToHandler(m) => {
            m
        },
        _ => unreachable!(),
    };

    let final_message = session.build_frame(message, true).unwrap();

    session.sent( &final_message ).unwrap();
}

fn confirm_logon( seq: i32, session: &mut SessionStateImpl <MemoryMessageStore> ) {

    let flds = LogonFields {
        encrypt_method: FieldEncryptMethodEnum::None,
        heart_bt_int: 60,
        .. Default::default()
    };
    let logon = FixMessage::Logon(Box::new(flds));
    let server_frame = FixFrame::new(seq, "sender", "target", "FIX.4.2", logon);
    session.received( &server_frame ).unwrap();
}

