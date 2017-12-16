//! Tests focused on the potential ways to reply to a ResendRequest

extern crate fixclient;
extern crate fix;

use std::rc::Rc;

use fixclient::{FixSessionConfig, FixDictionary};
use fixclient::connector::statemachine::*;
use fixclient::connector::memstore::*;
use fixclient::connector::MessageStore;
use fixclient::builder;
use fix::fixmessagegen::*;


#[test]
fn test_retrieving_app_level_messagens_slice() {
    let store = create_store(|s| {
        s.add_to_store( builder::build_new_order_single(1, true, "cl1", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
        s.add_to_store( builder::build_new_order_single(2, true, "cl2", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
        s.add_to_store( builder::build_new_order_single(3, true, "cl3", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
        s.add_to_store( builder::build_new_order_single(4, true, "cl4", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
        s.add_to_store( builder::build_new_order_single(5, true, "cl5", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
        s.add_to_store( builder::build_new_order_single(6, true, "cl6", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
        s.add_to_store( builder::build_new_order_single(7, true, "cl7", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
    });
    let mut sm = FixSyncState::new( store.clone() );

    let entries = sm.build_resend_request_response( 2, 5 ).expect("success");

    assert_eq!(entries.len(), 4);
    // assert_eq!(store.next_sender_seq_num(), 8);
    assert_eq!(entries.get(0).unwrap().seq, 2);
    assert_eq!(entries.get(1).unwrap().seq, 3);
    assert_eq!(entries.get(2).unwrap().seq, 4);
    assert_eq!(entries.get(3).unwrap().seq, 5);
    assert_eq!(entries.get(0).unwrap().message.msg_type(), FieldMsgTypeEnum::OrderSingle);
    assert_eq!(entries.get(1).unwrap().message.msg_type(), FieldMsgTypeEnum::OrderSingle);
    assert_eq!(entries.get(2).unwrap().message.msg_type(), FieldMsgTypeEnum::OrderSingle);
    assert_eq!(entries.get(3).unwrap().message.msg_type(), FieldMsgTypeEnum::OrderSingle);
}


//#[test]
fn test_retrieving_app_level_messagens_slice_with_missing_end() {
    let store = create_store(|s| {
        s.add_to_store( builder::build_new_order_single(1, true, "cl1", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
        s.add_to_store( builder::build_new_order_single(2, true, "cl2", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
        s.add_to_store( builder::build_new_order_single(3, true, "cl3", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
        s.add_to_store( builder::build_new_order_single(4, true, "cl4", "AAPL", 100.0, 594.2, FieldSideEnum::Buy, FieldOrdTypeEnum::Limit ) );
    });
    let mut sm = FixSyncState::new( store );

    let entries = sm.build_resend_request_response( 2, 5 ).expect("success");

    assert_eq!(entries.len(), 4);
    assert_eq!(entries.get(0).unwrap().seq, 2);
    assert_eq!(entries.get(1).unwrap().seq, 3);
    assert_eq!(entries.get(2).unwrap().seq, 4);
    assert_eq!(entries.get(3).unwrap().seq, 5);
    assert_eq!(entries.get(0).unwrap().message.msg_type(), FieldMsgTypeEnum::OrderSingle);
    assert_eq!(entries.get(1).unwrap().message.msg_type(), FieldMsgTypeEnum::OrderSingle);
    assert_eq!(entries.get(2).unwrap().message.msg_type(), FieldMsgTypeEnum::OrderSingle);
    assert_eq!(entries.get(3).unwrap().message.msg_type(), FieldMsgTypeEnum::SequenceReset);
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