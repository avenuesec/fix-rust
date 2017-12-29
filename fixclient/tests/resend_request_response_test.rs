//! Tests focused on the potential ways to reply to a ResendRequest

extern crate fixclient;
extern crate fix;

use std::rc::Rc;
use std::sync::Mutex;

use fixclient::{FixSessionConfig, FixDictionary};
use fixclient::connector::statemac2::*;
use fixclient::connector::memstore::*;
use fixclient::connector::MessageStore;
use fixclient::connector::resendresponse::*;
use fixclient::builder;
use fix::fixmessagegen::*;


#[test]
fn test_retrieving_app_level_messagens_slice() {
    let mut store = create_store(|s| {
        s.add_to_store( builder::build_new_order_single(1, true, "cl1", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
        s.add_to_store( builder::build_new_order_single(2, true, "cl2", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
        s.add_to_store( builder::build_new_order_single(3, true, "cl3", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
        s.add_to_store( builder::build_new_order_single(4, true, "cl4", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
        s.add_to_store( builder::build_new_order_single(5, true, "cl5", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
        s.add_to_store( builder::build_new_order_single(6, true, "cl6", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
        s.add_to_store( builder::build_new_order_single(7, true, "cl7", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
    });
    // let mut sm = FixSyncState::new( store.clone() );

    let entries = build_resend_request_response( &mut store, 2, 5 ).expect("success");

    assert_eq!(entries.len(), 4);
    assert_eq!(entries.get(0).unwrap().seq, 2);
    assert_eq!(entries.get(1).unwrap().seq, 3);
    assert_eq!(entries.get(2).unwrap().seq, 4);
    assert_eq!(entries.get(3).unwrap().seq, 5);
    assert_eq!(entries.get(0).unwrap().message.msg_type(), FieldMsgTypeEnum::OrderSingle);
    assert_eq!(entries.get(1).unwrap().message.msg_type(), FieldMsgTypeEnum::OrderSingle);
    assert_eq!(entries.get(2).unwrap().message.msg_type(), FieldMsgTypeEnum::OrderSingle);
    assert_eq!(entries.get(3).unwrap().message.msg_type(), FieldMsgTypeEnum::OrderSingle);
    assert_eq!(store.next_target_seq_num(), 1);
    assert_eq!(store.next_sender_seq_num(), 8);
}

#[test]
fn test_retrieving_app_level_messagens_slice_with_missing_end() {
    let mut store = create_store(|s| {
        s.add_to_store( builder::build_new_order_single(1, true, "cl1", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
        s.add_to_store( builder::build_new_order_single(2, true, "cl2", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
        s.add_to_store( builder::build_new_order_single(3, true, "cl3", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
        s.add_to_store( builder::build_new_order_single(4, true, "cl4", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
    });

    let entries = build_resend_request_response( &mut store, 2, 5 ).expect("success");

    assert_eq!(entries.len(), 4);
    assert_eq!(entries.get(0).unwrap().seq, 2);
    assert_eq!(entries.get(1).unwrap().seq, 3);
    assert_eq!(entries.get(2).unwrap().seq, 4);
    assert_eq!(entries.get(3).unwrap().seq, 5);
    assert_eq!(entries.get(0).unwrap().message.msg_type(), FieldMsgTypeEnum::OrderSingle);
    assert_eq!(entries.get(1).unwrap().message.msg_type(), FieldMsgTypeEnum::OrderSingle);
    assert_eq!(entries.get(2).unwrap().message.msg_type(), FieldMsgTypeEnum::OrderSingle);
    assert_eq!(entries.get(3).unwrap().message.msg_type(), FieldMsgTypeEnum::SequenceReset);
    match &entries.get(3).unwrap().message {
        &FixMessage::SequenceReset(ref flds) => {
            assert_eq!(flds.new_seq_no, 6);
        }
        _ => panic!("expecting seq reset")
    }
    assert_eq!(store.next_target_seq_num(), 1);
    assert_eq!(store.next_sender_seq_num(), 6);
}

#[test]
fn test_gap_filling_session_level_gap_at_the_start() {
    let mut store = create_store(|s| {
        s.add_to_store( builder::build_test_req(1, "TEST1") );
        s.add_to_store( builder::build_test_req(2, "TEST2") );
        s.add_to_store( builder::build_new_order_single(3, true, "cl1", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
        s.add_to_store( builder::build_new_order_single(4, true, "cl2", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
    });
    // let mut sm = FixSyncState::new( store.clone() );

    let entries = build_resend_request_response( &mut store, 1, 0 ).expect("success");

    assert_eq!(entries.len(), 3);
    assert_eq!(entries.get(0).unwrap().seq, 1);
    assert_eq!(entries.get(1).unwrap().seq, 3);
    assert_eq!(entries.get(2).unwrap().seq, 4);
    assert_eq!(entries.get(0).unwrap().message.msg_type(), FieldMsgTypeEnum::SequenceReset);
    assert_eq!(entries.get(1).unwrap().message.msg_type(), FieldMsgTypeEnum::OrderSingle);
    assert_eq!(entries.get(2).unwrap().message.msg_type(), FieldMsgTypeEnum::OrderSingle);

    match &entries.get(0).unwrap().message {
        &FixMessage::SequenceReset(ref flds) => {
            assert_eq!(flds.new_seq_no, 3);
        }
        _ => panic!("expecting seq reset")
    }

    assert_eq!(store.next_target_seq_num(), 1);
    assert_eq!(store.next_sender_seq_num(), 5);
}

#[test]
fn test_gap_filling_session_level_gap_at_the_end() {
    let mut store = create_store(|s| {
        s.add_to_store( builder::build_new_order_single(1, true, "cl1", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
        s.add_to_store( builder::build_new_order_single(2, true, "cl2", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
        s.add_to_store( builder::build_test_req(3, "TEST1") );
    });

    let entries = build_resend_request_response( &mut store, 1, 0 ).expect("success");

    assert_eq!(entries.len(), 3);
    assert_eq!(entries.get(0).unwrap().seq, 1);
    assert_eq!(entries.get(1).unwrap().seq, 2);
    assert_eq!(entries.get(2).unwrap().seq, 3);
    assert_eq!(entries.get(0).unwrap().message.msg_type(), FieldMsgTypeEnum::OrderSingle);
    assert_eq!(entries.get(1).unwrap().message.msg_type(), FieldMsgTypeEnum::OrderSingle);
    assert_eq!(entries.get(2).unwrap().message.msg_type(), FieldMsgTypeEnum::SequenceReset);

    match &entries.get(2).unwrap().message {
        &FixMessage::SequenceReset(ref flds) => {
            assert_eq!(flds.new_seq_no, 4);
        }
        _ => panic!("expecting seq reset")
    }

    assert_eq!(store.next_target_seq_num(), 1);
    assert_eq!(store.next_sender_seq_num(), 4);
}

#[test]
fn test_gap_filling_session_level_gap_at_the_middle() {
    let mut store = create_store(|s| {
        s.add_to_store( builder::build_new_order_single(1, true, "cl1", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
        s.add_to_store( builder::build_test_req(2, "TEST1") );
        s.add_to_store( builder::build_test_req(3, "TEST1") );
        s.add_to_store( builder::build_new_order_single(4, true, "cl2", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
    });

    let entries = build_resend_request_response( &mut store, 1, 0 ).expect("success");

    assert_eq!(entries.len(), 3);
    assert_eq!(entries.get(0).unwrap().seq, 1);
    assert_eq!(entries.get(1).unwrap().seq, 2);
    assert_eq!(entries.get(2).unwrap().seq, 4);
    assert_eq!(entries.get(0).unwrap().message.msg_type(), FieldMsgTypeEnum::OrderSingle);
    assert_eq!(entries.get(1).unwrap().message.msg_type(), FieldMsgTypeEnum::SequenceReset);
    assert_eq!(entries.get(2).unwrap().message.msg_type(), FieldMsgTypeEnum::OrderSingle);

    match &entries.get(1).unwrap().message {
        &FixMessage::SequenceReset(ref flds) => {
            assert_eq!(flds.new_seq_no, 4);
        }
        _ => panic!("expecting seq reset")
    }

    assert_eq!(store.next_target_seq_num(), 1);
    assert_eq!(store.next_sender_seq_num(), 5);
}


#[test]
fn test_gap_filling_session_level_full_gap() {
    let mut store = create_store(|s| {
        s.add_to_store( builder::build_test_req(1, "TEST1") );
        s.add_to_store( builder::build_test_req(2, "TEST1") );
        s.add_to_store( builder::build_test_req(3, "TEST1") );
    });

    let entries = build_resend_request_response( &mut store, 1, 0 ).expect("success");

    assert_eq!(entries.len(), 1);
    assert_eq!(entries.get(0).unwrap().seq, 1);
    assert_eq!(entries.get(0).unwrap().message.msg_type(), FieldMsgTypeEnum::SequenceReset);

    match &entries.get(0).unwrap().message {
        &FixMessage::SequenceReset(ref flds) => {
            assert_eq!(flds.new_seq_no, 4);
        }
        _ => panic!("expecting seq reset")
    }

    assert_eq!(store.next_target_seq_num(), 1);
    assert_eq!(store.next_sender_seq_num(), 4);
}


fn create_store<F>( f : F ) -> MemoryMessageStore // Rc<Mutex<MemoryMessageStore>>
    where F : FnOnce(&mut MemoryMessageStore) -> () {

    let cfg = FixSessionConfig::new( "qualifier", "sender", "target", "hostname",
                                     1234, 60, "log", "store", FixDictionary::Fix42 );
    let mut store = MemoryMessageStore::new( &cfg ).unwrap();
    f(&mut store);
    // let store = Rc::new( Mutex::new(store) );
    store
}