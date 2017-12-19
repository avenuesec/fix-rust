extern crate fixclient;
extern crate fix;

use std::rc::Rc;
use std::sync::Mutex;

use fixclient::{FixSessionConfig, FixDictionary};
use fixclient::connector::statemachine::*;
use fixclient::connector::memstore::*;
use fixclient::connector::MessageStore;
use fixclient::builder;
use fix::fixmessagegen::*;


#[test]
fn test_initial_state() {
    let store = create_store(|_| {});
    let sm = FixSyncState::new( store );

    assert_eq!("us connected - them connected", format!("{}", sm));
}

#[test]
fn test_logon_sent_state() {
    let store = create_store(|_| {});
    let mut sm = FixSyncState::new( store );

    let _ = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");

    assert_eq!("us logon - them logon", format!("{}", sm));
}

#[test]
fn test_logon_handshake_state() {
    let store = create_store(|_| {});
    let mut sm = FixSyncState::new( store );

    let _        = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");
    let recv_res = sm.register_recv( &builder::build_logon( 1, None ) ).expect("success");

    assert_eq!("us operational - them operational", format!("{}", sm));
    match recv_res {
        TransitionAction::None => {
            // expected
        },
        _ => {
             panic!("expecting None, but got {:?}", recv_res);
        }
    }
}

#[test]
fn test_vanilla_communication_no_gaps() {
    let store = create_store(|_| {});
    let mut sm = FixSyncState::new( store );

    let _  = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");
    assert!( sm.register_recv( &builder::build_logon( 1, None ) ).expect("success").is_none() );
    assert_eq!("us operational - them operational", format!("{}", sm));

    let _  = sm.register_sent( &builder::build_test_req(2, "TEST") ).expect("success");
    assert!( sm.register_recv( &builder::build_heartbeat(2, Some("TEST") ) ).expect("success").is_none() );
    assert_eq!("us operational - them operational", format!("{}", sm));

    assert!( sm.register_recv( &builder::build_test_req(3, "TEST2" ) ).expect("success").is_none() );
    let _  = sm.register_sent( &builder::build_heartbeat(3, Some("TEST2") ) ).expect("success");
    assert_eq!("us operational - them operational", format!("{}", sm));

    let _ = sm.register_sent( &builder::build_new_order_single(4, false, "cl1", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
    assert!( sm.register_recv( &builder::build_exec_report(4, false, "cl1", "ord1", "exec1", "AAPL", 0.0, 100.0, 594.2, FieldSideEnum::Buy, FieldExecTypeEnum::Fill, FieldExecTransTypeEnum::Status, FieldOrdStatusEnum::Filled ) ).expect("success").is_none() );
    assert_eq!("us operational - them operational", format!("{}", sm));
}

#[test]
fn test_server_sending_a_resend_request() {
    let store = create_store(|_| {});
    let mut sm = FixSyncState::new( store );

    let _ = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");
    let _ = sm.register_recv( &builder::build_logon( 1, None ) ).expect("success");
    let recv_res = sm.register_recv( &builder::build_resend_req( 2, 1, 10 ) ).expect("success");

    assert_eq!("us resync - them operational", format!("{}", sm));
    match recv_res {
        TransitionAction::DoResendRange( range ) => {
            assert_eq!( (1,10), range );
            // expected
        },
        _ => {
            panic!("expecting None, but got {:?}", recv_res);
        }
    }
}

#[test]
fn test_responding_to_a_resend_request_with_a_full_gap_fill() {
    let store = create_store(|_| {});
    let mut sm = FixSyncState::new( store );

    let _ = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");
    let _ = sm.register_recv( &builder::build_logon( 1, None ) ).expect("success");
    let _ = sm.register_recv( &builder::build_resend_req( 2, 1, 10 ) ).expect("success");
    let _ = sm.register_sent( &builder::build_sequence_reset(2, 11, Some(true) ) );

    assert_eq!("us operational - them operational", format!("{}", sm));
}

#[test]
fn test_responding_to_a_resend_request_with_individual_messages() {
    let store = create_store(|_| {});
    let mut sm = FixSyncState::new( store );

    let _ = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");
    let _ = sm.register_recv( &builder::build_logon( 1, None ) ).expect("success");
    let _ = sm.register_recv( &builder::build_resend_req( 2, 2, 5 ) ).expect("success");
    let _ = sm.register_sent( &builder::build_new_order_single(2, true, "cl1", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
    let _ = sm.register_sent( &builder::build_new_order_single(3, true, "cl2", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
    let _ = sm.register_sent( &builder::build_new_order_single(4, true, "cl3", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );

    assert_eq!("us resync - them operational", format!("{}", sm));
}

#[test]
fn test_responding_to_a_resend_request_with_individual_messages_complete() {
    let store = create_store(|_| {});
    let mut sm = FixSyncState::new( store );

    let _ = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");
    let _ = sm.register_recv( &builder::build_logon( 1, None ) ).expect("success");
    let _ = sm.register_recv( &builder::build_resend_req( 2, 2, 5 ) ).expect("success");
    let _ = sm.register_sent( &builder::build_new_order_single(2, true, "cl1", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
    let _ = sm.register_sent( &builder::build_new_order_single(3, true, "cl2", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
    let _ = sm.register_sent( &builder::build_new_order_single(4, true, "cl3", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
    let _ = sm.register_sent( &builder::build_new_order_single(5, true, "cl4", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );

    assert_eq!("us operational - them operational", format!("{}", sm));
}

#[test]
fn test_detecting_gap_from_server() {
    let store = create_store(|_| {});
    let mut sm = FixSyncState::new( store );

    let _  = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");
    let rc = sm.register_recv( &builder::build_logon( 10, None ) ).expect("success");

    assert_eq!("us operational - them resync", format!("{}", sm));
    match rc {
        TransitionAction::RequestResendRange(range) => {
            assert_eq!( (1,10), range );
        },
        _ => panic!("expecting RequestResendRange")
    }
}

#[test]
fn test_logon_with_reset() {
    let store = create_store(|s| { s.overwrite_seqs(10, 20); });
    let mut sm = FixSyncState::new( store.clone() );

    assert_eq!(store.lock().unwrap().next_sender_seq_num(), 10);
    assert_eq!(store.lock().unwrap().next_target_seq_num(), 20);

    let _ = sm.register_sent( &builder::build_logon( 1, Some(true) ) ).expect("success");
    let _ = sm.register_recv( &builder::build_logon( 1, Some(true) ) ).expect("success");

    assert_eq!(store.lock().unwrap().next_sender_seq_num(), 10);
    assert_eq!(store.lock().unwrap().next_target_seq_num(), 2);
    assert_eq!("us operational - them operational", format!("{}", sm));
}


#[test]
fn test_processing_resends_not_complete() {
    let store = create_store(|_| {});
    let mut sm = FixSyncState::new( store );

    let _ = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");
    let _ = sm.register_recv( &builder::build_logon( 1, None ) ).expect("success");
    let _ = sm.register_recv( &builder::build_resend_req( 2, 2, 5 ) ).expect("success");

    assert_eq!("us resync - them operational", format!("{}", sm));
}

#[test]
fn test_processing_resends_with_seq_reset() {
    let store = create_store(|_| {});
    let mut sm = FixSyncState::new( store );

    let _ = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");
    let _ = sm.register_recv( &builder::build_logon( 1, None ) ).expect("success");
    let _ = sm.register_recv( &builder::build_resend_req( 2, 2, 5 ) ).expect("success");
    let _ = sm.register_sent( &builder::build_sequence_reset( 2, 6, Some(true) ) ).expect("success");

    assert_eq!("us operational - them operational", format!("{}", sm));
}

fn create_store<F>( f : F ) -> Rc<Mutex<MemoryMessageStore>>
    where F : FnOnce(&mut MemoryMessageStore) -> () {

    let cfg = FixSessionConfig::new( "qualifier", "sender", "target", "hostname",
                                     1234, 60, "log", "store", FixDictionary::Fix42 );
    let mut store = MemoryMessageStore::new( &cfg ).unwrap();
    f(&mut store);
    let store = Rc::new( Mutex::new(store) );
    store
}



