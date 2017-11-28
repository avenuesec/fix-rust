
extern crate fixclient;
extern crate env_logger;

use fixclient::FixSessionConfig;
use fixclient::connector::store::FSMessageStore;

// Starts brand new store, change its seqs, drop it and start new
// old to confirm restored state
#[test]
fn test_seq_nums_persistence_and_init() {

    env_logger::init().unwrap();

    FSMessageStore::delete_files( "./store" ); // <- clean slate

    let cfg = FixSessionConfig::new( "123", "sender", "target", "hostname", 1234, 30, "./logs", "./store" );

    let mut store = FSMessageStore::new ( &cfg ).expect("store created");
    store.incr_sender_seq_num();
    store.incr_sender_seq_num();
    store.incr_target_seq_num();
    drop(store);

    let mut store = FSMessageStore::new ( &cfg ).expect("store created");
    assert_eq!( store.get_sender_seq(), 3 );
    assert_eq!( store.get_target_seq(), 2 );
}
