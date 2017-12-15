extern crate fixclient;

use std::rc::Rc;

use fixclient::{FixSessionConfig, FixDictionary};
//use fixclient::connector::*;
use fixclient::connector::statemachine::*;
use fixclient::connector::memstore::*;
use fixclient::builder;

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

    let res = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");

    assert_eq!("us logon - them logon", format!("{}", sm));
}

#[test]
fn test_logon_handshake_state() {
    let store = create_store(|_| {});
    let mut sm = FixSyncState::new( store );

    let send_res = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");
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
fn test_server_sending_a_resend_request() {
    let store = create_store(|_| {});
    let mut sm = FixSyncState::new( store );

    let send_res = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");
    sm.register_recv( &builder::build_logon( 1, None ) ).expect("success");
    let recv_res = sm.register_recv( &builder::build_resend_req( 2, 1, 10 ) ).expect("success");

    assert_eq!("us resync - them operational", format!("{}", sm));
    match recv_res {
        TransitionAction::ResendRange( range ) => {
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

    let send_res = sm.register_sent( &builder::build_logon( 1, None ) ).expect("success");
    sm.register_recv( &builder::build_logon( 1, None ) ).expect("success");
    sm.register_recv( &builder::build_resend_req( 2, 1, 10 ) ).expect("success");
    sm.register_sent( &builder::build_sequence_reset(2, 11, Some(true) ) );

    assert_eq!("us operational - them operational", format!("{}", sm));
}


fn create_store<F>( f : F ) -> Rc<MemoryMessageStore>
    where F : FnOnce(&mut MemoryMessageStore) -> () {

    let cfg = FixSessionConfig::new( "qualifier", "sender", "target", "hostname",
                                     1234, 60, "log", "store", FixDictionary::Fix42 );
    let mut store = MemoryMessageStore::new( &cfg ).unwrap();
    f(&mut store);
    let store = Rc::new( store );
    store
}


//#[test]
//fn test_initial_state() {
//    let mut sm = FixStateTransition::new();
//
//    assert_eq!("us connected - them connected", format!("{}", sm));
//}
//
//#[test]
//fn test_logon_sent() {
//    let mut sm = FixStateTransition::new();
//
//    assert!( sm.outgoing_logon().is_ok() );
//
//    assert_eq!("us logon - them connected", format!("{}", sm));
//}
//
//#[test]
//fn test_logon_sent_then_recv() {
//    let mut sm = FixStateTransition::new();
//
//    assert!( sm.outgoing_logon().is_ok() );
//    assert!( sm.incoming_logon().is_ok() );
//
//    assert_eq!("us operational - them operational", format!("{}", sm));
//}
//
//#[test]
//fn test_invalid_logon_recv() {
//    let mut sm = FixStateTransition::new();
//
//    assert!( sm.incoming_logon().is_err() );
//}
//
//#[test]
//fn test_seq_reset_recv() {
//    let mut sm = FixStateTransition::new();
//
//    assert!( sm.outgoing_logon().is_ok() );
//    assert!( sm.incoming_logon().is_ok() );
//    assert!( sm.incoming_seqreset().is_ok() );
//
//    assert_eq!("us operational - them operational", format!("{}", sm));
//}
//
//#[test]
//fn test_seq_reset_sent() {
//    let mut sm = FixStateTransition::new();
//
//    assert!( sm.outgoing_logon().is_ok() );
//    assert!( sm.incoming_logon().is_ok() );
//    assert!( sm.outgoing_seqreset().is_ok() );
//
//    assert_eq!("us operational - them operational", format!("{}", sm));
//}
//
//#[test]
//fn test_resend_recv() {
//    let mut sm = FixStateTransition::new();
//
//    assert!( sm.outgoing_logon().is_ok() );
//    assert!( sm.incoming_logon().is_ok() );
//    assert!( sm.incoming_resend().is_ok() );
//
//    assert_eq!("us operational - them await_resend", format!("{}", sm));
//}
//
//#[test]
//fn test_resend_sent() {
//    let mut sm = FixStateTransition::new();
//
//    assert!( sm.outgoing_logon().is_ok() );
//    assert!( sm.incoming_logon().is_ok() );
//    assert!( sm.outgoing_resend().is_ok() );
//
//    assert_eq!("us await_resend - them operational", format!("{}", sm));
//}
//
//#[test]
//fn test_reply_to_resend_recv() {
//    let mut sm = FixStateTransition::new();
//
//    assert!( sm.outgoing_logon().is_ok() );
//    assert!( sm.incoming_logon().is_ok() );
//    assert!( sm.incoming_resend().is_ok() );
//    assert!( sm.confirm_resent_sent().is_ok() );
//
//    assert_eq!("us operational - them operational", format!("{}", sm));
//}