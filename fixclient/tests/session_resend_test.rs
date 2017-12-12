extern crate fix;
extern crate fixclient;
extern crate mio;
extern crate mio_more;

use std::default::Default;

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


fn basic_init<F>( f : F ) -> (SessionStateImpl <MemoryMessageStore>, Receiver<Command>)
    where F : FnOnce(&mut MemoryMessageStore) -> ()
{
    let cfg = FixSessionConfig::new("qualifier", "sender", "target", "host",
                                    5000, 60, "logfolder", "store", FixDictionary::Fix42);
    let mut store = MemoryMessageStore::new(&cfg).expect("expecting store");
    f( &mut store );

    let mut session = SessionStateImpl::new( &cfg, store );
    let (tx, rx) = channel::sync_channel(100);
    let sender = Sender::new( Token(1), tx );
    session.init( sender );
    redispath_to_session( &rx, &mut session );
    (session, rx)
}

#[test]
fn test_logon_handshake() {

    // Arrange
    let (mut session, _) = basic_init( |_| {  } );

    // Act
    confirm_logon( 1, &mut session );

    // Assert
    assert!( session.is_operational() );

}

//use std::thread;
//use std::time;

#[test]
fn test_resend_req_from_server() {
    // Arrange
    let (mut session, rx) = basic_init( |store| {
        store.add_to_store( build_new_order_single( 2, "cl1", "AAPL",  100.0, 172.2,  FieldSideEnum::Buy, FieldOrdTypeEnum::Market) );
        store.add_to_store( build_new_order_single( 3, "cl2", "MSFT",  200.0, 81.4,   FieldSideEnum::Buy, FieldOrdTypeEnum::Market) );
        store.add_to_store( build_new_order_single( 4, "cl3", "GOOGL", 100.0, 1001.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Market) );
        store.add_to_store( build_heartbeat( 5, Some("test") ) );
        store.add_to_store( build_heartbeat( 6, Some("test") ) );
        store.add_to_store( build_new_order_single( 7, "cl4", "YHOO",  100.0, 53.0,   FieldSideEnum::Buy, FieldOrdTypeEnum::Market) );
        store.add_to_store( build_heartbeat( 8, Some("test2") ) );
        // thread::sleep(time::Duration::from_millis(500));
    } );
    confirm_logon( 1, &mut session );

    // Act

    session.received( &build_resend_req(2, 2, 0) ).unwrap();

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

    let final_message = session.build(message).unwrap();

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

fn build_logon( seq: i32 ) -> FixFrame {
    let flds = LogonFields {
        encrypt_method: FieldEncryptMethodEnum::None,
        heart_bt_int: 60,
        .. Default::default()
    };
    let logon = FixMessage::Logon(Box::new(flds));
    FixFrame::new(seq, "sender", "target", "FIX.4.2", logon)
}

fn build_logout( seq: i32, text: Option<String> ) -> FixFrame {
    let flds = LogoutFields {
        text,
        .. Default::default()
    };
    let msg = FixMessage::Logout(Box::new(flds));
    FixFrame::new(seq, "sender", "target", "FIX.4.2", msg)
}

fn build_resend_req( seq: i32, begin: i32, end: i32 ) -> FixFrame {
    let flds = ResendRequestFields {
        begin_seq_no: begin,
        end_seq_no: end,
        .. Default::default()
    };
    let msg = FixMessage::ResendRequest(Box::new(flds));
    FixFrame::new(seq, "sender", "target", "FIX.4.2", msg)
}

fn build_sequence_reset( seq: i32, new_seq: i32, gap_fill: Option<bool> ) -> FixFrame {
    let flds = SequenceResetFields {
        gap_fill_flag: gap_fill,
        new_seq_no: new_seq,
        .. Default::default()
    };
    let msg = FixMessage::SequenceReset(Box::new(flds));
    FixFrame::new(seq, "sender", "target", "FIX.4.2", msg)
}

fn build_test_req( seq: i32, test_req_id: &str ) -> FixFrame {
    let flds = TestRequestFields {
        test_req_id: test_req_id.to_owned(),
        .. Default::default()
    };
    let msg = FixMessage::TestRequest(Box::new(flds));
    FixFrame::new(seq, "sender", "target", "FIX.4.2", msg)
}

fn build_heartbeat( seq: i32, test_req_id: Option<&str> ) -> FixFrame {
    let flds = HeartbeatFields {
        test_req_id: test_req_id.map_or( None, |v| Some(v.to_owned()) ),
        .. Default::default()
    };
    let msg = FixMessage::Heartbeat(Box::new(flds));
    FixFrame::new(seq, "sender", "target", "FIX.4.2", msg)
}

fn build_new_order_single( seq: i32, cl_ord_id: &str, symbol: &str, qty: f32, price: f32,
                           side: FieldSideEnum, ord_type: FieldOrdTypeEnum ) -> FixFrame {
    let flds = NewOrderSingleFields {
        cl_ord_id: cl_ord_id.to_owned(),
        symbol: symbol.to_owned(),
        order_qty: Some(qty),
        price: Some(price),
        side,
        ord_type,
        handl_inst: FieldHandlInstEnum::AutomatedExecutionOrderPrivateNoBrokerIntervention,
        .. Default::default()
    };
    let msg = FixMessage::NewOrderSingle(Box::new(flds));
    FixFrame::new(seq, "sender", "target", "FIX.4.2", msg)
}

fn build_exec_report( seq: i32, cl_ord_id: &str, order_id: &str, exec_id: &str, symbol: &str,
                      leaves_qty: f32, cum_qty: f32, avg_px: f32,
                      side: FieldSideEnum,
                      exec_type: FieldExecTypeEnum, trans_type: FieldExecTransTypeEnum,
                      ord_status: FieldOrdStatusEnum ) -> FixFrame {

    let flds = ExecutionReportFields {
        order_id: order_id.to_owned(),
        cl_ord_id: Some(cl_ord_id.to_owned()),
        exec_id: exec_id.to_owned(),
        exec_trans_type: trans_type,
        symbol: symbol.to_owned(),
        leaves_qty, cum_qty, avg_px,
        ord_status,
        exec_type,
        .. Default::default()
    };
    let msg = FixMessage::ExecutionReport(Box::new(flds));
    FixFrame::new(seq, "sender", "target", "FIX.4.2", msg)
}