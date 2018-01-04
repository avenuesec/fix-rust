extern crate fix;
extern crate nom;
extern crate chrono;

use std::str;
use std::borrow::Cow;
use std::default::Default;

use chrono::*;
use nom::IResult;
use fix::*;
use fix::frame::*;
use fix::fixmessagegen::*;


#[test]
#[cfg(feature="fix42")]
fn test_logon_message() {
    let line = b"8=FIX.4.2\x019=70\x0135=A\x0134=1\x0149=SOME\x0152=20171129-22:28:53.000\x0156=SINGU\x0198=0\x01108=60\x01141=Y\x0110=226\x01";
	if let IResult::Done(_, b) = frame::parse(line) {
        assert_eq!(b,
                   FixFrame {
                       header: FixHeader {
                           begin_string: Cow::from("FIX.4.2"),
                           msg_type: FieldMsgTypeEnum::Logon,
                           sender_comp_id: "SOME".to_owned(),
                           target_comp_id: "SINGU".to_owned(),
                           sending_time: UtcDateTime::new( Utc.ymd(2017, 11, 29).and_hms_milli(22, 28, 53, 0) ),
                           msg_seq_num: 1,
                           .. Default::default()
                       },
                       message: FixMessage::Logon(Box::new(LogonFields {
                           encrypt_method: FieldEncryptMethodEnum::None,
                           heart_bt_int: 60,
                           reset_seq_num_flag: Some(true),
                           .. Default::default()
                       }))
                   });
    } else {
        panic!("expecting Done but got somethign else");
    }
}

#[test]
#[cfg(feature="fix42")]
fn test_buffer_with_multiple_messages_should_parse_them_in_order() {
    let line = b"8=FIX.4.2\x019=70\x0135=A\x0134=1\x0149=SOME1\x0152=20171129-22:28:53.000\x0156=SINGU\x0198=0\x01108=60\x01141=Y\x0110=226\x018=FIX.4.2\x019=70\x0135=A\x0134=2\x0149=SOME2\x0152=20171129-22:28:53.000\x0156=SINGU\x0198=0\x01108=60\x01141=Y\x0110=226\x01";

    let res = frame::parse(line);
    if let IResult::Done(remaining, b) = res {
        assert_eq!(remaining.len(), 93);
        assert_eq!(1, b.header.msg_seq_num);

        let res = frame::parse(remaining);
        if let IResult::Done(remaining, b) = res {
            assert_eq!(remaining.len(), 0);
            assert_eq!(2, b.header.msg_seq_num);
        } else {
            panic!("did not get DONE! {:?}", res);
        }

    } else {
        panic!("did not get DONE! {:?}", res);
    }
}

/// It's mandatory that incomplete frames yield IResult::Incomplete by the parser
#[test]
#[cfg(feature="fix42")]
fn test_fragments_should_yield_not_complete() {
    let line = b"8=FIX.4.2\x019=70\x0135=A\x0134=1\x0149=SOME\x0152=20171129-22:28:53.000\x0156=SINGU\x0198=0\x01108=60\x01141=Y\x0110=226\x01";

    for i in 0..line.len() + 1  {
        let slice = &line[..i];
        let b = frame::parse(slice);

        if i == line.len() {
            match b {
                IResult::Done(_, _) => {
                    // all good!
                }
                _ => panic!("expecting complete frame to parse successfully but got {:?}", b)
            }
        } else {
            match b {
                IResult::Incomplete(_) => {
                    // all good!
                },
                _ => unsafe {
                    println!("{} {:?} {:?}", i, b, str::from_utf8_unchecked(slice) );
                    panic!("Expecting IResult::Incomplete when parsing incomplete frames")
                }
            }
        }
//        unsafe {
//            println!("{} {:?} {:?}", i, b, str::from_utf8_unchecked(slice) );
//        }
    }
}

#[test]
#[cfg(feature="fix44")]
fn test_heartbeat_message() {
    let line = "8=FIX.4.4|9=57|35=0|49=OE101C|56=CCLRA301|34=2|52=20170627-14:23:54.802|10=065|".replace("|", "\x01");
	let b = frame::parse(line.as_bytes());
    println!("{:?}", b);
}

#[test]
#[cfg(feature="fix44")]
fn test_new_order_single_message() {
    let line = "8=FIX.4.4|9=206|35=D|34=2|49=CCLRA301|52=20170627-19:32:13.105|56=OE101C|1=4004|11=30011_0|38=100|40=2|44=5|54=1|55=PETR4|59=0|60=20170627-16:32:13|453=3|448=CCLRA301|447=D|452=36|448=308|447=D|452=7|448=DMA1|447=D|452=31|10=207|\r\n".replace("|", "\x01");
	let b = frame::parse(line.as_bytes());
}
