#[macro_use]
extern crate bencher;
extern crate fix;
extern crate chrono;
extern crate bytes;

use bencher::Bencher;
use chrono::prelude::*;
use bytes::{BytesMut, BufMut};

use fix::*;
use fix::frame::{FixFrame};
use fix::fixmessagegen::*;


fn new_order_single_parse(bench: &mut Bencher) {
    let line = "8=FIX.4.2|9=123|35=D|34=479|49=OMS|52=20170809-11:48:59.413|56=SING|11=1234567|1=1|21=2|55=GOOGL|54=1|60=20170809-11:48:59.413|38=100|40=2|10=029|".replace("|", "\x01");
    let b = line.as_bytes();

    bench.iter(|| {
        frame::parse(b);
    });
}

fn logon_message_parse(bench: &mut Bencher) {
    let line = "8=FIX.4.2|9=98|35=A|34=1|49=CCLRA301|52=20170627-14:23:04.660|56=OE101C|95=6|96=YWEKNJ|98=0|108=20|141=Y|35002=0|10=103|".replace("|", "\x01");
    let b = line.as_bytes();

    bench.iter(|| {
        frame::parse(b);
    });
}

fn heartbeat_message_parse(bench: &mut Bencher) {
    let line = "8=FIX.4.2|9=57|35=0|49=OE101C|56=CCLRA301|34=2|52=20170627-14:23:54.802|10=065|".replace("|", "\x01");
    let b = line.as_bytes();

    bench.iter(|| {
        frame::parse(b);
    });
}

fn heartbeat_message_build(bench: &mut Bencher) {
    let frame = FixFrame {
        header: FixHeader {
            sending_time: UtcDateTime::new(Utc.ymd(2017, 08, 09).and_hms_milli(11, 48, 59, 413)),
            msg_seq_num: 479,
            sender_comp_id: "OMS".to_string(),
            target_comp_id: "SING".to_string(),
            msg_type: FieldMsgTypeEnum::Heartbeat,
            .. Default::default()
        },
        message: FixMessage::Heartbeat(Box::new(HeartbeatFields {
            test_req_id: Some("req".to_string())
        }))
    };
    let mut buffer = BytesMut::with_capacity(1024);

    bench.iter(|| {
        unsafe { buffer.set_len(0); }
        frame.write( &mut buffer );
    });
}

benchmark_group!(benches, heartbeat_message_parse, logon_message_parse, new_order_single_parse, heartbeat_message_build);
benchmark_main!(benches);
