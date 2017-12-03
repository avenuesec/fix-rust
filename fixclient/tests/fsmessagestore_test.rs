
extern crate fixclient;
extern crate env_logger;
extern crate fix;

use std::default::Default;
use std::string::ToString;

use fix::fixmessagegen::*;
use fix::frame::FixFrame;

use fixclient::{FixSessionConfig, FixDictionary};
use fixclient::connector::MessageStore;
use fixclient::connector::fsstore::FSMessageStore;


// Starts brand new store, change its seqs, drop it and start new
// old to confirm restored state
#[test]
fn test_seq_nums_persistence_and_init() {
    // env_logger::init().unwrap();

    let store_dir = "./temptest/store2";

    FSMessageStore::delete_files( "fix.4.2_target_sender", store_dir ); // <- clean slate

    let cfg = FixSessionConfig::new( "123", "sender", "target", "hostname",
                                     1234, 30, "./temptest/logs", store_dir,
                                     FixDictionary::Fix42 );

    let mut store = FSMessageStore::new ( &cfg ).expect("store created");
    store.incr_sender_seq_num();
    store.incr_sender_seq_num();
    store.incr_target_seq_num();
    drop(store);

    let mut store = FSMessageStore::new ( &cfg ).expect("store created");
    assert_eq!( store.get_sender_seq(), 3 );
    assert_eq!( store.get_target_seq(), 2 );
}

#[test]
fn test_recreation() {
    // env_logger::init().unwrap();

    let store_dir = "./temptest/store1";

    FSMessageStore::delete_files( "fix.4.2_target_sender", store_dir ); // <- clean slate

    let cfg = FixSessionConfig::new( "123", "sender", "target", "hostname",
                                     1234, 30, "./temptest/logs", store_dir,
                                     FixDictionary::Fix42 );

    for i in 0..10 {
        let mut store = FSMessageStore::new ( &cfg ).expect(&format!("store created - iteration {}", i));
        assert_eq!( store.get_sender_seq(), 1 );
        assert_eq!( store.get_target_seq(), 1 );
    }
}

#[test]
fn test_message_persistence_and_query_post_recreation() {
    // env_logger::init().unwrap();

    let store_dir = "./temptest/store3";

    FSMessageStore::delete_files( "fix.4.2_target_sender", store_dir ); // <- clean slate

    let cfg = FixSessionConfig::new( "123", "sender", "target", "hostname",
                                     1234, 30, "./temptest/logs", store_dir,
                                     FixDictionary::Fix42 );

    let mut logon = None;
    let mut test_req = None;
    let mut hb_req = None;

    {
        let mut store = FSMessageStore::new ( &cfg ).expect("store created");
        logon = Some(FixFrame::new(store.incr_sender_seq_num().unwrap(), "sender", "target", "FIX.4.2", FixMessage::Logon(Box::new(LogonFields {
            encrypt_method: FieldEncryptMethodEnum::None,
            heart_bt_int: 60,
            reset_seq_num_flag: Some(true),
            .. Default::default()
        }))));
        test_req = Some(FixFrame::new(store.incr_sender_seq_num().unwrap(), "sender", "target", "FIX.4.2", FixMessage::TestRequest(Box::new(TestRequestFields {
            test_req_id: "TEST".to_owned(),
        }))));
        hb_req = Some(FixFrame::new(store.incr_sender_seq_num().unwrap(), "sender", "target", "FIX.4.2", FixMessage::Heartbeat(Box::new(HeartbeatFields {
            test_req_id: Some("TEST".to_owned()),
        }))));

        store.sent( logon.as_ref().unwrap() );
        store.sent( test_req.as_ref().unwrap() );
        store.sent( hb_req.as_ref().unwrap() );
    }

    {
        let mut store = FSMessageStore::new ( &cfg ).expect("store created");
        let messages = store.query(1, 0).expect("expecting messages");
        assert_eq!(messages.len(), 3);
        assert_eq!(logon.unwrap().to_string(),    messages[0].to_string());
        assert_eq!(test_req.unwrap().to_string(), messages[1].to_string());
        assert_eq!(hb_req.unwrap().to_string(),   messages[2].to_string());
    }
}

#[test]
fn test_message_persistence_and_query() {
    // env_logger::init().unwrap();

    let store_dir = "./temptest/store4";

    FSMessageStore::delete_files( "fix.4.2_target_sender", store_dir ); // <- clean slate

    let cfg = FixSessionConfig::new( "123", "sender", "target", "hostname",
                                     1234, 30, "./temptest/logs", store_dir,
                                     FixDictionary::Fix42 );

    let mut store = FSMessageStore::new ( &cfg ).expect("store created");
    let logon = FixFrame::new(store.incr_sender_seq_num().unwrap(), "sender", "target", "FIX.4.2", FixMessage::Logon(Box::new(LogonFields {
        encrypt_method: FieldEncryptMethodEnum::None,
        heart_bt_int: 60,
        reset_seq_num_flag: Some(true),
        .. Default::default()
    })));
    let test_req = FixFrame::new(store.incr_sender_seq_num().unwrap(), "sender", "target", "FIX.4.2", FixMessage::TestRequest(Box::new(TestRequestFields {
        test_req_id: "TEST".to_owned(),
    })));
    let hb_req = FixFrame::new(store.incr_sender_seq_num().unwrap(), "sender", "target", "FIX.4.2", FixMessage::Heartbeat(Box::new(HeartbeatFields {
        test_req_id: Some("TEST@".to_owned())
    })));

    store.sent( &logon );
    store.sent( &test_req );
    store.sent( &hb_req );

    let messages = store.query(1, 0).expect("expecting messages");
    assert_eq!(messages.len(), 3);

    assert_eq!(&logon.to_string(),    &messages[0].to_string());
    assert_eq!(&test_req.to_string(), &messages[1].to_string());
    assert_eq!(&hb_req.to_string(),   &messages[2].to_string());
}