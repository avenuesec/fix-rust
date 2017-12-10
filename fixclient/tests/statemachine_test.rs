extern crate fixclient;

use fixclient::connector::statemachine::*;

#[test]
fn test_initial_state() {
    let mut sm = FixStateTransition::new();

    assert_eq!("us connected - them connected", format!("{}", sm));
}

#[test]
fn test_logon_sent() {
    let mut sm = FixStateTransition::new();

    assert!( sm.outgoing_logon().is_ok() );

    assert_eq!("us logon - them connected", format!("{}", sm));
}

#[test]
fn test_logon_sent_then_recv() {
    let mut sm = FixStateTransition::new();

    assert!( sm.outgoing_logon().is_ok() );
    assert!( sm.incoming_logon().is_ok() );

    assert_eq!("us operational - them operational", format!("{}", sm));
}

#[test]
fn test_invalid_logon_recv() {
    let mut sm = FixStateTransition::new();

    assert!( sm.incoming_logon().is_err() );
}

#[test]
fn test_seq_reset_recv() {
    let mut sm = FixStateTransition::new();

    assert!( sm.outgoing_logon().is_ok() );
    assert!( sm.incoming_logon().is_ok() );
    assert!( sm.incoming_seqreset().is_ok() );

    assert_eq!("us operational - them operational", format!("{}", sm));
}

#[test]
fn test_seq_reset_sent() {
    let mut sm = FixStateTransition::new();

    assert!( sm.outgoing_logon().is_ok() );
    assert!( sm.incoming_logon().is_ok() );
    assert!( sm.outgoing_seqreset().is_ok() );

    assert_eq!("us operational - them operational", format!("{}", sm));
}

#[test]
fn test_resend_recv() {
    let mut sm = FixStateTransition::new();

    assert!( sm.outgoing_logon().is_ok() );
    assert!( sm.incoming_logon().is_ok() );
    assert!( sm.incoming_resend().is_ok() );

    assert_eq!("us operational - them await_resend", format!("{}", sm));
}

#[test]
fn test_resend_sent() {
    let mut sm = FixStateTransition::new();

    assert!( sm.outgoing_logon().is_ok() );
    assert!( sm.incoming_logon().is_ok() );
    assert!( sm.outgoing_resend().is_ok() );

    assert_eq!("us await_resend - them operational", format!("{}", sm));
}

#[test]
fn test_reply_to_resend_recv() {
    let mut sm = FixStateTransition::new();

    assert!( sm.outgoing_logon().is_ok() );
    assert!( sm.incoming_logon().is_ok() );
    assert!( sm.incoming_resend().is_ok() );
    assert!( sm.confirm_resent_sent().is_ok() );

    assert_eq!("us operational - them operational", format!("{}", sm));
}