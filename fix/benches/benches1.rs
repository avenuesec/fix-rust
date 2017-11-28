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


fn timestamp_parse_utc_datetime_from_str(bench: &mut Bencher) {
    bench.iter(|| {
        // (0..1000).fold(0, |x, y| x + y)
        Utc.datetime_from_str("20170627-14:24:14.804", "%Y%m%d-%H:%M:%S%.3f").unwrap()
    })
}

fn timestamp_parse_combinator(bench: &mut Bencher) {
    bench.iter(|| {
        frame::timestamp_val(b"20170627-14:24:14.804")
    });
}

fn new_order_single_parse(bench: &mut Bencher) {
    let line = "8=FIX.4.4|9=206|35=D|34=2|49=CCLRA301|52=20170627-19:32:13.105|56=OE101C|1=4004|11=30011_0|38=100|40=2|44=5|54=1|55=PETR4|59=0|60=20170627-16:32:13|453=3|448=CCLRA301|447=D|452=36|448=308|447=D|452=7|448=DMA1|447=D|452=31|10=207|\r\n".replace("|", "\x01");
    let b = line.as_bytes();

    bench.iter(|| {
        frame::parse(b);
    });
}

fn logon_message_parse(bench: &mut Bencher) {
    let line = "8=FIX.4.4|9=98|35=A|34=1|49=CCLRA301|52=20170627-14:23:04.660|56=OE101C|95=6|96=YWEKNJ|98=0|108=20|141=Y|35002=0|10=103|\r\n".replace("|", "\x01");
    let b = line.as_bytes();

    bench.iter(|| {
        frame::parse(b);
    });
}

fn heartbeat_message_parse(bench: &mut Bencher) {
    let line = "8=FIX.4.4|9=57|35=0|49=OE101C|56=CCLRA301|34=2|52=20170627-14:23:54.802|10=065|\r\n".replace("|", "\x01");
    let b = line.as_bytes();

    bench.iter(|| {
        frame::parse(b);
    });
}

fn heartbeat_message_build(bench: &mut Bencher) {
    let frame = FixFrame {
        ts: Utc.ymd(2017, 08, 09).and_hms_milli(11, 48, 59, 388),
        sending: Utc.ymd(2017, 08, 09).and_hms_milli(11, 48, 59, 413),
        seq: 479,
        sender_comp_id: "XPOMS".to_string(),
        target_comp_id: "CLEAR".to_string(),
        msg: FixMessage::Heartbeat(Box::new(HeartbeatFields { 
            test_req_id: Some("req".to_string())
        }))
    };
    let mut buffer = BytesMut::with_capacity(1024);

    bench.iter(|| {
        unsafe { buffer.set_len(0); }
        frame.write( &mut buffer );
    });
}

benchmark_group!(benches, timestamp_parse_combinator, timestamp_parse_utc_datetime_from_str, heartbeat_message_parse, logon_message_parse, new_order_single_parse, heartbeat_message_build);
benchmark_main!(benches);
