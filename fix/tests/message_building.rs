
extern crate fix;
extern crate chrono;
extern crate bytes;

use std::default::{Default};
use std::borrow::Cow;

use chrono::prelude::*; 
use bytes::{BytesMut, BufMut};

use fix::*;
use fix::frame::{FixFrame};
use fix::fixmessagegen::*;


#[test]
fn test_build_heartbeat() {
    let frame = FixFrame {
        begin_string: Cow::from("FIX.4.2"),
        sending: Utc.ymd(2017, 08, 09).and_hms_milli(11, 48, 59, 413),
        seq: 479,
        sender_comp_id: "XPOMS".to_string(),
        target_comp_id: "CLEAR".to_string(),
        orig_sending: None,
        poss_duplicate: None,
        poss_resend: None,
        message: FixMessage::Heartbeat(Box::new(HeartbeatFields { 
            // test_req_id: Some("req".to_string())
            test_req_id: None
        }))
    };
    let mut buffer = BytesMut::new();
    let _ = frame.write( &mut buffer );
    let message = unsafe {
        String::from_utf8_unchecked(buffer.freeze().to_vec())
    };
    // original:  8=FIX.4.4|9=55|35=0|34=479|49=XPOMS|52=20170809-11:48:59.413|56=CLEAR|10=086| 
    // generated: 8=FIX.4.4|9=55|35=0|34=479|49=XPOMS|52=20170809-11:48:59.413|56=CLEAR|10=086|
    assert_eq!("8=FIX.4.2|9=55|35=0|34=479|49=XPOMS|52=20170809-11:48:59.413|56=CLEAR|10=084|",
        message.replace("\u{1}", "|"));
}

#[test]
#[cfg(fix42)]
fn test_build_logon() {
    let frame = FixFrame {
        begin_string: Cow::from("FIX.4.2"),
        sending: Utc.ymd(2017, 08, 09).and_hms_milli(11, 48, 59, 413),
        seq: 100321,
        sender_comp_id: "XPOMS".to_string(),
        target_comp_id: "CLEAR".to_string(),
        orig_sending: None,
        poss_duplicate: None,
        poss_resend: None,
        message: FixMessage::Logon(Box::new(LogonFields {
            encrypt_method: Field_EncryptMethod_Enum::NONEOTHER,
            heart_bt_int: 60,
            reset_seq_num_flag: Some(true),
            .. Default::default()
        }))
    };
    let mut buffer = BytesMut::new();
    let _ = frame.write( &mut buffer );
    let message = unsafe {
        String::from_utf8_unchecked(buffer.freeze().to_vec())
    };

    assert_eq!("8=FIX.4.2|9=99|35=A|34=100321|49=XPOMS|52=20170809-11:48:59.413|56=CLEAR|98=0|108=60|141=Y|464=Y|553=Hey|554=John|10=220|",
               message.replace("\u{1}", "|"));
}

#[test]
#[cfg(fix44)]
fn test_build_logon() {
    let frame = FixFrame {
        begin_string: Cow::from("FIX.4.4"),
        sending: Utc.ymd(2017, 08, 09).and_hms_milli(11, 48, 59, 413),
        seq: 100321,
        sender_comp_id: "XPOMS".to_string(),
        target_comp_id: "CLEAR".to_string(),
        orig_sending: None,
        poss_duplicate: None,
        poss_resend: None,
        message: FixMessage::Logon(Box::new(LogonFields { 
            encrypt_method: Field_EncryptMethod_Enum::NONEOTHER,
            heart_bt_int: 60,
            reset_seq_num_flag: Some(true),
            username: Some("Hey".to_string()),
            password: Some("John".to_string()),
            test_message_indicator: Some(true),
            .. Default::default()
        }))
    };
    let mut buffer = BytesMut::new();
    let _ = frame.write( &mut buffer );
    let message = unsafe {
        String::from_utf8_unchecked(buffer.freeze().to_vec())
    };

    assert_eq!("8=FIX.4.4|9=99|35=A|34=100321|49=XPOMS|52=20170809-11:48:59.413|56=CLEAR|98=0|108=60|141=Y|464=Y|553=Hey|554=John|10=220|", 
        message.replace("\u{1}", "|"));
}

#[test]
#[cfg(fix44)]
fn test_build_new_order_single() {
    let frame = FixFrame {
        begin_string: Cow::from("FIX.4.4"),
        sending: Utc.ymd(2017, 08, 09).and_hms_milli(13, 44, 16, 182),
        seq: 70827,
        sender_comp_id: "CLEAR".to_string(),
        target_comp_id: "XPOMS".to_string(),
        orig_sending: None,
        poss_duplicate: None,
        poss_resend: None,
        message: FixMessage::NewOrderSingle(Box::new(NewOrderSingleFields { 
            cl_ord_id: "53887733_0".to_string(),
            account: Some("31334".to_string()),
            handl_inst: Some(Field_HandlInst_Enum::AUTOEXECPRIV),
            order_qty_data: OrderQtyDataFields { 
                order_qty: Some(5.0), 
                .. Default::default()
            },
            ord_type: Field_OrdType_Enum::LIMIT,
            price: Some(3160.5),
            side: Field_Side_Enum::BUY,
            instrument: InstrumentFields {
                symbol: Some("WDOU17".to_string()), 
                security_exchange: Some("XBMF".to_string()),
                maturity_month_year: Some( UtcDate::new( Utc.ymd(2017, 08, 09).and_hms(0, 0, 0) ) ),
                .. Default::default()
            },
            time_in_force: Some(Field_TimeInForce_Enum::DAY),
            transact_time: UtcDateTime::new(Utc.ymd(2017, 08, 09).and_hms(13, 44, 16)),
            .. Default::default()
        }))
    };
    let mut buffer = BytesMut::new();
    let _ = frame.write( &mut buffer );
    let message = unsafe {
        String::from_utf8_unchecked(buffer.freeze().to_vec())
    };
    // 20170809-13:44:16.182 : 8=FIX.4.4|9=154|35=D|34=70827|49=CLEAR|52=20170809-13:44:16.182|56=XPOMS|1=31334|11=53887733_0|21=1|38=5|40=2|44=3160.5|54=1|55=WDOU17|59=0|60=20170809-13:44:16|207=XBMF|10=177|
    assert_eq!("8=FIX.4.4|9=171|35=D|34=70827|49=CLEAR|52=20170809-13:44:16.182|56=XPOMS|11=53887733_0|1=31334|21=1|55=WDOU17|200=20170809|207=XBMF|54=1|60=20170809-13:44:16.000|38=5|40=2|44=3160.5|59=0|10=217|", 
        message.replace("\u{1}", "|"));
}
