
extern crate fixclient;
extern crate env_logger;

use fixclient::{FixSessionConfig, FixDictionary};
use fixclient::connector::MessageStore;
use fixclient::connector::fsstore::FSMessageStore;

// Starts brand new store, change its seqs, drop it and start new
// old to confirm restored state
#[test]
fn test_seq_nums_persistence_and_init() {
    env_logger::init().unwrap();

    let store_dir = "./temptest/store";

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
    env_logger::init().unwrap();

    let store_dir = "./temptest/store";

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