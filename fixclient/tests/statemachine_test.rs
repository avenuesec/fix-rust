extern crate fixclient;
extern crate fix;

// use std::rc::Rc;
use std::sync::{Arc, Mutex};

use fixclient::{FixSessionConfig, FixDictionary};
use fixclient::connector::syncstate::*;
use fixclient::connector::memstore::*;
use fixclient::connector::MessageStore;
use fixclient::builder;
use fix::fixmessagegen::*;

#[test]
fn test_initial_state() {
    let store = create_store(|_| {});
    let sm = FixSyncState::new( store );

    assert_eq!("sender connected - target connected", format!("{}", sm));
}

#[test]
fn test_logon_sent_state() {
    let store = create_store(|_| {});
    let mut sm = FixSyncState::new( store );

    let _ = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");

    assert_eq!("sender logon - target logon", format!("{}", sm));
}

#[test]
fn test_logon_handshake_completed_should_transition_to_operational() {
    let store = create_store(|_| {});
    let mut sm = FixSyncState::new( store );

    let _        = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");
    let recv_res = sm.register_recv( &builder::build_logon( 1, None ) ).expect("success");

    assert_eq!("sender operational - target operational", format!("{}", sm));
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
fn test_vanilla_communication_no_gaps_should_always_be_operational() {
    let store = create_store(|_| {});
    let mut sm = FixSyncState::new( store );

    let _  = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");
    assert!( sm.register_recv( &builder::build_logon( 1, None ) ).expect("success").is_none() );
    assert_eq!("sender operational - target operational", format!("{}", sm));

    let _  = sm.register_sent( &builder::build_test_req(2, "TEST") ).expect("success");
    assert!( sm.register_recv( &builder::build_heartbeat(2, Some("TEST") ) ).expect("success").is_none() );
    assert_eq!("sender operational - target operational", format!("{}", sm));

    assert!( sm.register_recv( &builder::build_test_req(3, "TEST2" ) ).expect("success").is_none() );
    let _  = sm.register_sent( &builder::build_heartbeat(3, Some("TEST2") ) ).expect("success");
    assert_eq!("sender operational - target operational", format!("{}", sm));

    let _ = sm.register_sent( &builder::build_new_order_single(4, false, "cl1", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
    assert!( sm.register_recv( &builder::build_exec_report(4, false, "cl1", "ord1", "exec1", "AAPL", 0.0, 100.0, 594.2, FieldSideEnum::Buy, FieldExecTypeEnum::Fill, FieldExecTransTypeEnum::Status, FieldOrdStatusEnum::Filled ) ).expect("success").is_none() );
    assert_eq!("sender operational - target operational", format!("{}", sm));
}

#[test]
fn test_server_sending_a_resend_request_should_put_us_on_resync() {
    let store = create_store(|_| {});
    let mut sm = FixSyncState::new( store );

    let _ = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");
    let _ = sm.register_recv( &builder::build_logon( 1, None ) ).expect("success");
    let recv_res = sm.register_recv( &builder::build_resend_req( 2, 1, 10 ) ).expect("success");

    assert_eq!("sender resync (1, 10) - target operational", format!("{}", sm));
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
fn test_responding_to_a_resend_request_with_a_full_gap_fill_should_make_us_operational() {
    let store = create_store(|_| {});
    let mut sm = FixSyncState::new( store );

    let _ = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");
    let _ = sm.register_recv( &builder::build_logon( 1, None ) ).expect("success");
    let _ = sm.register_recv( &builder::build_resend_req( 2, 2, 10 ) ).expect("success");
    let _ = sm.register_sent( &builder::build_sequence_reset(2, 11, Some(true) ) );

    assert_eq!("sender operational - target operational", format!("{}", sm));
}

#[test]
fn test_responding_to_a_resend_request_with_individual_messages_should_fill_gap() {
    let store = create_store(|_| {});
    let mut sm = FixSyncState::new( store );

    let _ = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");
    let _ = sm.register_recv( &builder::build_logon( 1, None ) ).expect("success");
    let _ = sm.register_recv( &builder::build_resend_req( 2, 2, 5 ) ).expect("success");
    let _ = sm.register_sent( &builder::build_new_order_single(2, true, "cl1", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
    let _ = sm.register_sent( &builder::build_new_order_single(3, true, "cl2", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
    let _ = sm.register_sent( &builder::build_new_order_single(4, true, "cl3", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );

    assert_eq!("sender resync (5, 5) - target operational", format!("{}", sm));
}

#[test]
fn test_responding_to_a_resend_request_with_individual_messages_completeshould_make_us_operational() {
    let store = create_store(|_| {});
    let mut sm = FixSyncState::new( store );

    let _ = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");
    let _ = sm.register_recv( &builder::build_logon( 1, None ) ).expect("success");
    let _ = sm.register_recv( &builder::build_resend_req( 2, 2, 5 ) ).expect("success");
    let _ = sm.register_sent( &builder::build_new_order_single(2, true, "cl1", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
    let _ = sm.register_sent( &builder::build_new_order_single(3, true, "cl2", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
    let _ = sm.register_sent( &builder::build_new_order_single(4, true, "cl3", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
    let _ = sm.register_sent( &builder::build_new_order_single(5, true, "cl4", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );

    assert_eq!("sender operational - target operational", format!("{}", sm));
}

#[test]
fn test_detecting_gap_from_server_should_transition_them_to_resync() {
    let store = create_store(|_| {});
    let mut sm = FixSyncState::new( store );

    let _  = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");
    let rc = sm.register_recv( &builder::build_logon( 10, None ) ).expect("success");

    assert_eq!("sender operational - target resync (1, 10)", format!("{}", sm));
    match rc {
        TransitionAction::RequestResendRange(range) => {
            assert_eq!( (1,10), range );
        },
        _ => panic!("expecting RequestResendRange")
    }
}

#[test]
fn test_when_server_in_resync_receiving_seq_reset_should_make_them_operational() {
    let store = create_store(|_| {});
    let mut sm = FixSyncState::new( store );

    let _ = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");
    let _ = sm.register_recv( &builder::build_logon( 10, None ) ).expect("success");
    let _ = sm.register_sent( &builder::build_resend_req( 1, 1, 0 ) ).expect("success");
    assert_eq!("sender operational - target resync (1, 10)", format!("{}", sm));
    let a = sm.register_recv( &builder::build_sequence_reset( 1, 11, Some(true) ) ).expect("success");

    println!("{:?}", a);
    assert_eq!("sender operational - target operational", format!("{}", sm));
}

#[test]
fn test_when_server_in_resync_receiving_out_of_order_should_adjust_gap() {
    let store = create_store(|_| {});
    let mut sm = FixSyncState::new( store );

    let _    = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");
    let gap1 = sm.register_recv( &builder::build_logon( 5, None ) ).expect("success");
    let _    = sm.register_sent( &builder::build_resend_req( 2, 1, 0 ) ).expect("success");

    assert_eq!("sender operational - target resync (1, 5)", format!("{}", sm));
    let gap2 = sm.register_recv( &builder::build_new_order_single( 6, false, "cl1", "AAPL",  100.0, 172.2,  FieldSideEnum::Buy, FieldOrdTypeEnum::Market) ).expect("success");
    assert_eq!("sender operational - target resync (1, 6)", format!("{}", sm));

    match gap1 {
        TransitionAction::RequestResendRange(range) => {
            assert_eq!( (1, 5), range );
        },
        _ => panic!("expecting RequestResendRange")
    }
    match gap2 {
        TransitionAction::RequestResendRange(range) => {
            assert_eq!( (1, 6), range );
        },
        _ => panic!("expecting RequestResendRange")
    }
}

#[test]
fn test_when_server_in_resync_receiving_ordered_should_fill_gap() {
    let store = create_store(|_| {});
    let mut sm = FixSyncState::new( store );

    let _  = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");
    let _  = sm.register_recv( &builder::build_logon( 5, None ) ).expect("success");
    assert_eq!("sender operational - target resync (1, 5)", format!("{}", sm));
    let _  = sm.register_sent( &builder::build_resend_req( 2, 1, 0 ) ).expect("success");
    assert_eq!("sender operational - target resync (1, 5)", format!("{}", sm));

    let r1 = sm.register_recv( &builder::build_new_order_single( 1, true, "cl1", "AAPL",  100.0, 172.2,  FieldSideEnum::Buy, FieldOrdTypeEnum::Market) ).expect("success");
    let r2 = sm.register_recv( &builder::build_new_order_single( 2, true, "cl2", "AAPL",  100.0, 172.2,  FieldSideEnum::Buy, FieldOrdTypeEnum::Market) ).expect("success");
    let r3 = sm.register_recv( &builder::build_new_order_single( 3, true, "cl3", "AAPL",  100.0, 172.2,  FieldSideEnum::Buy, FieldOrdTypeEnum::Market) ).expect("success");
    let r4 = sm.register_recv( &builder::build_new_order_single( 4, true, "cl4", "AAPL",  100.0, 172.2,  FieldSideEnum::Buy, FieldOrdTypeEnum::Market) ).expect("success");
    let r5 = sm.register_recv( &builder::build_new_order_single( 5, true, "cl5", "AAPL",  100.0, 172.2,  FieldSideEnum::Buy, FieldOrdTypeEnum::Market) ).expect("success");
    let r6 = sm.register_recv( &builder::build_new_order_single( 6, false, "cl6", "AAPL",  100.0, 172.2,  FieldSideEnum::Buy, FieldOrdTypeEnum::Market) ).expect("success");

    assert_eq!("sender operational - target operational", format!("{}", sm));
    assert!( r1.is_none() );
    assert!( r2.is_none() );
    assert!( r3.is_none() );
    assert!( r4.is_none() );
    assert!( r5.is_none() );
    assert!( r6.is_none() );
}

#[test]
fn test_when_server_in_resync_receiving_gap_should_make_them_operational() {
    let store = create_store(|_| {});
    let mut sm = FixSyncState::new( store );

    let _ = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");
    let _ = sm.register_recv( &builder::build_logon( 5, None ) ).expect("success");
    let _ = sm.register_sent( &builder::build_resend_req( 2, 1, 0 ) ).expect("success");
    let _ = sm.register_recv( &builder::build_new_order_single( 1, true, "cl1", "AAPL",  100.0, 172.2,  FieldSideEnum::Buy, FieldOrdTypeEnum::Market) ).expect("success");
    let _ = sm.register_recv( &builder::build_new_order_single( 2, true, "cl2", "AAPL",  100.0, 172.2,  FieldSideEnum::Buy, FieldOrdTypeEnum::Market) ).expect("success");
    let _ = sm.register_recv( &builder::build_new_order_single( 3, true, "cl3", "AAPL",  100.0, 172.2,  FieldSideEnum::Buy, FieldOrdTypeEnum::Market) ).expect("success");
    let _ = sm.register_recv( &builder::build_new_order_single( 4, true, "cl4", "AAPL",  100.0, 172.2,  FieldSideEnum::Buy, FieldOrdTypeEnum::Market) ).expect("success");
    let _ = sm.register_recv( &builder::build_new_order_single( 5, true, "cl5", "AAPL",  100.0, 172.2,  FieldSideEnum::Buy, FieldOrdTypeEnum::Market) ).expect("success");

    assert_eq!("sender operational - target operational", format!("{}", sm));
}

#[test]
fn test_logon_with_reset_should_reset_target_seq_as_requested() {
    let store = create_store(|s| { s.overwrite_seqs(10, 20); });
    let mut sm = FixSyncState::new( store.clone() );

    assert_eq!(store.lock().unwrap().next_sender_seq_num(), 10);
    assert_eq!(store.lock().unwrap().next_target_seq_num(), 20);

    let _ = sm.register_sent( &builder::build_logon( 10, Some(true) ) ).expect("success");
    let _ = sm.register_recv( &builder::build_logon( 20, Some(true) ) ).expect("success");

    assert_eq!(store.lock().unwrap().next_target_seq_num(), 1);
    assert_eq!(store.lock().unwrap().next_sender_seq_num(), 1);
    assert_eq!("sender operational - target operational", format!("{}", sm));
}

#[test]
fn test_processing_resends_not_complete() {
    let store = create_store(|_| {});
    let mut sm = FixSyncState::new( store );

    let _ = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");
    let _ = sm.register_recv( &builder::build_logon( 1, None ) ).expect("success");
    let _ = sm.register_recv( &builder::build_resend_req( 2, 2, 5 ) ).expect("success");

    assert_eq!("sender resync (2, 5) - target operational", format!("{}", sm));
}

#[test]
fn test_responding_to_a_resend_request_with_a_full_gap_fill_should_make_us_operational_2() {
    let store = create_store(|_| {});
    let mut sm = FixSyncState::new( store );

    let _ = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");
    let _ = sm.register_recv( &builder::build_logon( 1, None ) ).expect("success");
    let _ = sm.register_recv( &builder::build_resend_req( 2, 2, 5 ) ).expect("success");
    let _ = sm.register_sent( &builder::build_sequence_reset( 2, 6, Some(true) ) ).expect("success");

    assert_eq!("sender operational - target operational", format!("{}", sm));
}

#[test]
fn test_processing_resends_with_partial_seq_reset_should_not_consider_it_operational() {
    let store = create_store(|_| {});
    let mut sm = FixSyncState::new( store );

    let _ = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");
    let _ = sm.register_recv( &builder::build_logon( 1, None ) ).expect("success");
    let _ = sm.register_recv( &builder::build_resend_req( 2, 2, 5 ) ).expect("success");
    let _ = sm.register_sent( &builder::build_sequence_reset( 2, 5, Some(true) ) ).expect("success");

    assert_eq!("sender resync (5, 5) - target operational", format!("{}", sm));
}

fn create_store<F>( f : F ) -> Arc<Mutex<MemoryMessageStore>>
    where F : FnOnce(&mut MemoryMessageStore) -> () {

    let cfg = FixSessionConfig::new( "qualifier", "sender", "target", "hostname",
                                     1234, 60, "log", "store", FixDictionary::Fix42 );
    let mut store = MemoryMessageStore::new( &cfg ).unwrap();
    f(&mut store);
    let store = Arc::new( Mutex::new(store) );
    store
}



