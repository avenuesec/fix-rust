

extern crate fix;
extern crate nom;

use fix::frame;
use nom::IResult;


#[test]
fn begin_string_parsing_42_should_succeed() {
    let res = frame::begin_string( b"FIX.4.2\x01" );
    assert_eq!(res, IResult::Done( &b"\x01"[..], "FIX.4.2") );
}

#[test]
fn begin_string_parsing_44_should_succeed() {
    let res = frame::begin_string( b"FIX.4.4\x01" );
    assert_eq!(res, IResult::Done( &b"\x01"[..], "FIX.4.4") );
}

#[test]
fn begin_string_parsing_43_should_fail() {
    let res = frame::begin_string( b"FIX.4.3\x01" );
    match res  {
        IResult::Error(_) => {
            // all good!
        },
        _ => panic!("expecting an error!")
    }
}


