

extern crate fix;
extern crate nom;

use fix::frame;
use nom::IResult;


#[test]
fn begin_string_parsing_42() {
    let res = frame::begin_string( b"8=FIX.4.2\x01" );
    // assert_eq!( "FIX.4.2", res );
    assert_eq!(res, IResult::Done( &b"\x01"[..], "FIX.4.2") );
}

#[test]
fn begin_string_parsing_44() {
    let res = frame::begin_string( b"8=FIX.4.4\x01" );
    // assert_eq!( "FIX.4.2", res );
    assert_eq!(res, IResult::Done( &b"\x01"[..], "FIX.4.4") );
}

#[test]
fn begin_string_parsing_44_() {
    let res = frame::begin_string( b"8=FIX.4.4" );
    // assert_eq!( "FIX.4.2", res );
    assert_eq!(res, IResult::Done( &b""[..], "FIX.4.4") );
}


