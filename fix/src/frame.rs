use nom::*;
use std::str::{FromStr,from_utf8_unchecked};
use std::{io, str};
use std::borrow::Cow;
use chrono::prelude::*; 
use fixmessagegen::*;
use bytes::{BytesMut, BufMut};


#[derive(PartialEq,Debug)]
pub struct FieldVal<'a> {
    pub id  : u32, 
    pub val : &'a str
}

#[derive(PartialEq,Debug)]
pub struct RawFixFrame<'a> {
    begin_str: &'static str,
    len : usize,
    flds: Vec<FieldVal<'a>>,
}

#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub struct FixFrame {
    pub header  : FixHeader,
    pub message : FixMessage,
}

const FIX_BEGIN_42   : &str= "FIX.4.2";
const FIX_BEGIN_44   : &str= "FIX.4.4";

const FIX_BEGIN_42_B : &[u8]= b"FIX.4.2";
const FIX_BEGIN_44_B : &[u8]= b"FIX.4.4";

const FIX_MESSAGE_DELIMITER: char = '\u{1}';


impl FixFrame {

    pub fn new(seq: i32, sender: &str, target: &str, begin_str: &'static str, message: FixMessage) -> FixFrame {
        let ts = Utc::now();
        FixFrame {
            header: FixHeader {
                begin_string: Cow::from(begin_str),
                sending_time: UtcDateTime::new(ts),
                msg_seq_num: seq as i32,
                sender_comp_id: sender.to_owned(), // TODO: Consider making these 2 Cow too
                target_comp_id: target.to_owned(),
                msg_type: message.msg_type(),
                .. Default::default()
            },
            message,
        }
    }

    /// format:
    /// 8=begin_string|9=len of body|body|10=checksum
    /// body = extended header + specific message fields + trailer (except checksum)
    pub fn write(&self, buf : &mut BytesMut) -> Result<(), io::Error> {

        // message content
        let mut temp_buf = BytesMut::new(); // ::new() ::with_capacity(1024) -  experiment with initial capacity here 

        // delegates to code gen
        write_fix_header(&self.header, &mut temp_buf)?;
        write_fix_message(&self.message, &mut temp_buf)?;
        
        let prelude = format!("8={version}\u{1}9={len}\u{1}", len= temp_buf.len(), version=self.header.begin_string );
        let mut message_builder = BytesMut::with_capacity(prelude.len() + temp_buf.len());
        message_builder.put( prelude ); // buffer copy 1 - sad!
        message_builder.extend_from_slice( &temp_buf.freeze()[..] ); // buffer copy 2 - sad!
        let body = message_builder.freeze();

        // header
        buf.extend_from_slice( &body[..] ); // buffer copy 3 - sad!

        // trailler
        let checksum = FixFrame::checksum( &body[..] );
        let trailer = format!("10={:03}\u{1}", checksum);
        buf.extend_from_slice( &trailer.as_str().as_bytes()[..] ); // ugly!

        // debug!("raw generated {:?}", buf);

        Ok( () )
    }

    /// Just to make unit testing simpler
    pub fn write_to_str(&self) -> Result<String, io::Error> {
        let mut buffer = BytesMut::new();
        self.write( &mut buffer )?;
        unsafe {
            Ok(  String::from_utf8_unchecked(buffer.freeze().to_vec()) )
        }
    }

    // https://github.com/antklim/fix_checksum/blob/master/src/lib.rs
    fn checksum(message: &[u8]) -> u32 {
        let mut cs: u32 = 0;
        for b in message {
            cs += *b as u32;
        }
        cs %= 256;
        cs
    }
}


// begin -- only here to make unit testing easier
use std::string::ToString;
impl ToString for FixFrame {
    fn to_string(&self) -> String {
        let mut buf = BytesMut::new();
        self.write(&mut buf);
        String::from_utf8( buf.to_vec() ).unwrap()
    }
}
// end ---

// SOH = 0x01
// 20170627-14:23:04.690 : 8=FIX.4.4|9=98
// 20170627-19:06:54.551 : 8=FIX.4.4 | 9=98 | 35=A | 34=1 | 49=CCLRA301 | 52=20170627-19:06:54.522 | 56=OE101C | 95=6 | 96=YWEKNJ | 98=0 | 108=20 | 141=Y | 35002=0 | 10=111 | 

named!(timestamp<DateTime<Utc>>,
    do_parse!(
        // 20170627-14:23:04.690
        y: apply!(to_u32_sized, 4) >> 
        m: apply!(to_u32_sized, 2) >> 
        d: apply!(to_u32_sized, 2) >> 
        tag!(b"-") >> 
        h: apply!(to_u32_sized, 2)  >> tag!(b":") >>
        mm: apply!(to_u32_sized, 2) >> tag!(b":") >> 
        s: apply!(to_u32_sized, 2)  >> tag!(b".") >>
        ms: apply!(to_u32_sized, 3) >> 
        // assembles with parsed pieces:
        ( Utc.ymd(y as i32, m, d).and_hms_milli(h, mm, s, ms) )
    )
);
named!(fieldid, take_while1!(is_digit));
named!(fieldval, take_until!( "\x01" )); // consume all up to SOH
named!(field<FieldVal>, do_parse!(
     id:  fieldid >> 
          tag!(b"=") >>
     val: fieldval >> 
          tag!("\x01") >>
     ( FieldVal { 
         id:  buf_to_u32_2(id),
         val: to_string_2(val),
    } )
) ); 
named!(fields <Vec<FieldVal>>, many1!( field ) );
named!(fld_value_usize<usize>, do_parse!(
    raw : take_while1!(is_digit) >> 
    ( 
        buf_to_u32_2(raw) as usize
    )
));
named!(pub begin_string<&'static str>, do_parse!(
    // eg: 8=FIX.4.4
    bstr : alt!( tag!(b"FIX.4.4") | tag!(b"FIX.4.2") ) >>
    ( to_fix_version(bstr) )
));

// raw parser entry point
// 20170627-19:06:54.551 : 8=FIX.4.4 | 9=98 | 35=A | 34=1 | 49=CCLRA301 | 52=20170627-19:06:54.522 | 56=OE101C | 95=6 | 96=YWEKNJ | 98=0 | 108=20 | 141=Y | 35002=0 | 10=111 | 
// 20170713-22:04:24.685 : 8=FIX.4.4|9=58|35=0|49=OE103C|56=RCLRA310|34=54|52=20170713-22:04:24.696|10=140|
const CHECKSUM_SIZE : usize = 7; // 10=012\u{1}
named!(raw_frame<RawFixFrame>,
  do_parse!(
              tag!(b"8=") >>
      bstr  : begin_string >>
              tag!("\x019=") >>
              // tag!(b"8=FIX.4.2\x019=") >>
      lenw  : fld_value_usize >> 
              tag!("\x01") >>
              call!(ensure_size, lenw + CHECKSUM_SIZE) >> // is there enough bytes to consume given the msg length?
      flds  : fields >>
    (RawFixFrame {
        begin_str: bstr,
        len      : lenw,
        flds     : flds
    })
  )
);

const MSG_SEQ:        u32 = 34;
const MSG_TYPE:       u32 = 35;
const SENDER_COMP_ID: u32 = 49;
const TARGET_COMP_ID: u32 = 56;
const SEND_TIME:      u32 = 52;

/// Assemble FixFrame containinig proper message
pub fn parse(buffer: &[u8]) -> IResult<&[u8], FixFrame> {
    let (remaining, raw) = try_parse!(buffer, raw_frame);

    let header = build_fix_header( &raw.begin_str, &raw.flds );
    let msg_type = header.msg_type;
    let message = build_fix_message( &msg_type.to_string(), &raw.flds );

    let fixframe = FixFrame {
        header,
        message,
    };

    IResult::Done(remaining, fixframe)
}

// why is this not part of nom in the first place?
#[inline(always)]
fn to_u32_sized(input: &[u8], size: usize) -> IResult<&[u8], u32> {
    match take!(input, size) {
        IResult::Error(e)      => IResult::Error(e),
        IResult::Incomplete(e) => IResult::Incomplete(e),
        IResult::Done(i,o) => {
            let mut res = 0u32;
            for &e in o {
                let digit = e as char;
                let value = digit.to_digit(10).unwrap_or(0);
                res = value + (res * 10);
            }
            IResult::Done(i, res)
        }
    }
}

// utilities:
#[inline(always)]
fn to_string_2(s: &[u8]) -> &str {
    unsafe { from_utf8_unchecked(s) }
}

#[inline(always)]
fn to_fix_version(s: &[u8]) -> &'static str {

    if s == FIX_BEGIN_42_B {
        FIX_BEGIN_42
    } else if s == FIX_BEGIN_44_B {
        FIX_BEGIN_44
    } else {
        panic!("unsupported version");
    }

}

// #[inline(always)]
// fn to_i32_2(s: &str) -> i32 {
//     FromStr::from_str(s).unwrap()
// }
#[inline(always)]
fn to_u32_2(s: &str) -> u32 {
    FromStr::from_str(s).unwrap()
}
#[inline(always)]
fn buf_to_u32_2(s: &[u8]) -> u32 {
    to_u32_2(to_string_2(s))
}
// #[inline(always)]
// fn buf_to_i32_2(s: &[u8]) -> i32 {
//     to_i32_2(to_string_2(s))
// }
#[inline(always)]
// pub required for benchmark code
pub fn timestamp_val(buffer: &[u8]) -> Option<DateTime<Utc>>  {
    match timestamp(buffer) {
        IResult::Done(_, dt) => {
            Some( dt )
        },
        _ => return None
    }
}

/// Just ensures the stream has enough bytes, doesnt consume anything
fn ensure_size(i:&[u8], len: usize) -> IResult<&[u8], &[u8]>{
  if i.len() < len {
    IResult::Incomplete(Needed::Size(len))
  } else {
    // no changes!
    IResult::Done(i, i)
  }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_1() {
        let res = parse(b"20170627-14:23:04.690 : 8=FIX.4.4\x019=57\x0135=0\x0134=3\x0149=RCLRA310\x0152=20170627-14:24:14.804\x0156=OE103C\x0110=082\x01\n");
        println!("parse_1 {:?}", res);
    }

    #[test]
    fn timestamp_parsing() {
        let res = timestamp( b"20170627-14:23:04.690 : 8=FIX.4.4\x019=98" ) ;
        // println!("timestamp_parsing {:?}", res);
        assert_eq!(res, IResult::Done(&b" : 8=FIX.4.4\x019=98"[..], Utc.ymd(2017, 06, 27).and_hms_milli(14, 23, 4, 690)));
    }

    #[test]
    fn rawframe_parsing_42() {
        let res = raw_frame( b"8=FIX.4.2\x019=57\x0135=0\x0134=3\x0149=RCLRA310\x0152=20170627-14:24:14.804\x0156=OE103C\x0110=082\x01" ) ;
        // println!("rawframe_parsing {:?}", res);
        let expected_frame = RawFixFrame {
            begin_str: "FIX.4.2",
            len: 57,
            flds: vec![ FieldVal { id: 35, val: "0" },
                        FieldVal { id: 34, val: "3" },
                        FieldVal { id: 49, val: "RCLRA310" },
                        FieldVal { id: 52, val: "20170627-14:24:14.804" },
                        FieldVal { id: 56, val: "OE103C" },
                        FieldVal { id: 10, val: "082" }]
        };
        assert_eq!(res, IResult::Done(&b""[..], expected_frame));
    }

    #[test]
    fn rawframe_parsing() {
        let res = raw_frame( b"8=FIX.4.4\x019=57\x0135=0\x0134=3\x0149=RCLRA310\x0152=20170627-14:24:14.804\x0156=OE103C\x0110=082\x01" ) ;
        // println!("rawframe_parsing {:?}", res);
        let expected_frame = RawFixFrame {
            begin_str: "FIX.4.4",
            len: 57, 
            flds: vec![ FieldVal { id: 35, val: "0" }, 
                        FieldVal { id: 34, val: "3" }, 
                        FieldVal { id: 49, val: "RCLRA310" }, 
                        FieldVal { id: 52, val: "20170627-14:24:14.804" }, 
                        FieldVal { id: 56, val: "OE103C" }, 
                        FieldVal { id: 10, val: "082" }] 
        };
        assert_eq!(res, IResult::Done(&b""[..], expected_frame));
    }

    #[test]
    fn rawframe_parsing_failure1() {
        let res = raw_frame( b"8=FIX.4.4\x01" ) ;
        assert_eq!(res, IResult::Incomplete(Needed::Size(12)));
    }

    #[test]
    fn rawframe_parsing_failure2() {
        let res = raw_frame( b"8=FIX.4.2\x01" );
        println!("result {:?}", res);
        assert_eq!(res, IResult::Incomplete(Needed::Size(12)));
    }

    #[test]
    fn field_parsing() {
        let res =  field( b"9=98\x0110=something" ) ;
        assert_eq!(res, IResult::Done(&b"10=something"[..], FieldVal { id: 9, val: "98" }));
    }

    #[test] // TODO: proper asserts
    fn fields_parsing() {
        let res = fields( b"9=57\x0135=0\x0134=3\x0149=RCLRA310\x0152=20170627-14:24:14.804\x0156=OE103C\x0110=082\x01\n9=57\x0135=0\x0134=3\x0149=RCLRA310\x0152=20170627-14:24:14.804\x0156=OE103C\x0110=082\x01\n" ) ;
        // println!("fields_parsing {:?}", res);
        assert_eq!(res, IResult::Done(
            &b"\n9=57\x0135=0\x0134=3\x0149=RCLRA310\x0152=20170627-14:24:14.804\x0156=OE103C\x0110=082\x01\n"[..], 
            vec![   FieldVal { id:  9, val: "57" },
                    FieldVal { id: 35, val: "0" },
                    FieldVal { id: 34, val: "3" },
                    FieldVal { id: 49, val: "RCLRA310" },
                    FieldVal { id: 52, val: "20170627-14:24:14.804" },
                    FieldVal { id: 56, val: "OE103C" },
                    FieldVal { id: 10, val: "082" } ]));
    }

    #[test]
    fn fld_value_usize_parsing_ok() {
        let res = fld_value_usize( b"57\x019=12\x01" );
        // println!("fld_value_usize_parsing_ok {:?}", res);
        assert_eq!(res, IResult::Done(&b"\x019=12\x01"[..], 57)); 
    }

    // #[test]
    // fn new_order_single() {
    //     let line = "20170627-19:32:13.106 : 8=FIX.4.4|9=206|35=D|34=2|49=CCLRA301|52=20170627-19:32:13.105|56=OE101C|1=4004|11=30011_0|38=100|40=2|44=5|54=1|55=PETR4|59=0|60=20170627-16:32:13|453=3|448=CCLRA301|447=D|452=36|448=308|447=D|452=7|448=DMA1|447=D|452=54|10=207|\r\n".replace("|", "\x01");
    //     let b = parse(line.as_bytes());
    //     println!("new_order_single {:?}", b);
    // }
}
