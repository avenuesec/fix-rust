extern crate fixclient;
extern crate mio;
extern crate mio_more;

use mio_more::{channel};
use mio::Token;

use fixclient::{FixSessionConfig, FixDictionary};
use fixclient::connector::*;
use fixclient::connector::MessageStore;
use fixclient::connector::session::*;
use fixclient::connector::memstore::*;
use fixclient::sender::Sender;


#[test]
fn test_initial_state() {
    let cfg = FixSessionConfig::new("qualifier", "sender", "target", "host",
                                    5000, 60, "logfolder", "store", FixDictionary::Fix42);

    let store = MemoryMessageStore::new(&cfg).expect("expecting store");

    let mut session = SessionStateImpl::new( &cfg, store );

    let (tx, rx) = channel::sync_channel(100);
    let sender = Sender::new( Token(1), tx );

    session.init( sender );

    let cmd = rx.try_recv().unwrap();

    // println!("cmd {:?}", cmd);

    // session.received(  );

    // assert_eq!("us connected - them connected", format!("{}", sm));
}


