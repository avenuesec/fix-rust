use nom::*;
use std::str::{FromStr,from_utf8_unchecked};
use std::{io, str};
use std::borrow::Cow;
use chrono::prelude::*; 
use fixmessagegen::*;
use bytes::{BytesMut, BufMut};
use serde::{Serialize,Deserialize};


#[derive(PartialEq,Debug)]
pub struct FieldVal<'a> {
	pub id  : u32, 
	pub val : &'a str
}

#[derive(PartialEq,Debug)]
pub struct RawFixFrame<'a> {
	// ts  : DateTime<Utc>,
	len : usize,
	flds: Vec<FieldVal<'a>>, // TODO checksum
}

#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub struct FixFrame {
	pub begin_string : Cow<'static, str>, // &'static str,
	pub sending  : DateTime<Utc>, // field 52
	pub seq : u32,  // field 34
	pub sender_comp_id: String, // 49
	pub target_comp_id: String, // 56
	// OnBehalfOfCompID: String, 115 
	// DeliverToCompID: String,  128
	pub message : FixMessage,
}

const FIX_BEGIN : &str= "FIX.4.4";
const FIX_MESSAGE_DELIMITER: char = '\u{1}';
// const FIX_EQUAL: char = '=';

impl FixFrame {

	pub fn new(seq: u32, sender: &str, target: &str, begin_str: &'static str, message: FixMessage) -> FixFrame {
		let ts = Utc::now();
		FixFrame {
            begin_string: Cow::from(begin_str),
			sending: ts,
			seq,
			sender_comp_id: sender.to_string(),
			target_comp_id: target.to_string(),
			message
		}
	}

	pub fn write(&self, buf : &mut BytesMut) -> Result<(), io::Error> {
		// message content
		let mut temp_buf = BytesMut::new(); // ::new() ::with_capacity(1024) -  experiment with initial capacity here 
		//delegates to code gen
		write_fix_message(&self.message, &UtcDateTime::new(self.sending), self.seq,
                          &self.sender_comp_id, &self.target_comp_id, &mut temp_buf)?;
		
		let prelude = format!("8={version}\u{1}9={len}\u{1}", len= temp_buf.len(), version=self.begin_string );
		let mut message_builder = BytesMut::with_capacity(prelude.len() + temp_buf.len());
		message_builder.put( prelude ); // buffer copy 1 - sad!
		message_builder.extend_from_slice( &temp_buf.freeze()[..] ); // buffer copy 2 - sad!
		let body = message_builder.freeze();

		// header
		buf.extend_from_slice( &body[..] ); // buffer copy 3 - sad!

		// trailler
        let checksum = FixFrame::checksum( &body[..] );
        //let trailer = format!("10={:03}{}\r\n", checksum, FIX_MESSAGE_DELIMITER);
        let trailer = format!("10={:03}{}", checksum, FIX_MESSAGE_DELIMITER);
		// buf.reserve( trailer.len() );
		buf.extend_from_slice( &trailer.as_str().as_bytes()[..] ); // ugly!

		debug!("raw generated {:?}", buf);

		Ok( () )
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
		h: apply!(to_u32_sized, 2) >> tag!(b":") >> 
		mm: apply!(to_u32_sized, 2) >> tag!(b":") >> 
		s: apply!(to_u32_sized, 2) >> tag!(b".") >> 
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

// raw parser entry point
// 20170627-19:06:54.551 : 8=FIX.4.4 | 9=98 | 35=A | 34=1 | 49=CCLRA301 | 52=20170627-19:06:54.522 | 56=OE101C | 95=6 | 96=YWEKNJ | 98=0 | 108=20 | 141=Y | 35002=0 | 10=111 | 
// 20170713-22:04:24.685 : 8=FIX.4.4|9=58|35=0|49=OE103C|56=RCLRA310|34=54|52=20170713-22:04:24.696|10=140|
const CHECKSUM_SIZE : usize = 7; // 10=012\u{1}
named!(raw_frame<RawFixFrame>,
  do_parse!(
	  //ts    : timestamp >> 
		      tag!(b"8=FIX.4.4\x019=") >>
      lenw  : fld_value_usize >> 
	  		  tag!("\x01") >>
			  call!(ensure_size, lenw + CHECKSUM_SIZE) >> // is there enough bytes to consume given the msg length?
	  flds  : fields >> // ideally should not parse checksum field..
	  		  // line_ending >> 
	(RawFixFrame { 
		// ts: ts,
		len: lenw,
		flds: flds
		// ..Default::default()
	})
  )
);

const MSG_SEQ:        u32 = 34;
const MSG_TYPE:       u32 = 35;
const SENDER_COMP_ID: u32 = 49;
const TARGET_COMP_ID: u32 = 56;
const SEND_TIME:      u32 = 52;

pub fn parse(buffer: &[u8]) -> IResult<&[u8], FixFrame> {
	let (remaining, raw) = try_parse!(buffer, raw_frame);
	
	// Assemble FixFrame containinig proper message

	// StandardHeader    https://www.onixs.biz/fix-dictionary/4.4/compBlock_StandardHeader.html
	// msg type  // 35
	// seq : u32,   34	MsgSeqNum
	// sender_comp_id: String, // 49
	// target_comp_id: String, // 56
	// 52	SendingTime

	let mut msg_type : Option<String> = None;
	let mut msg_seq  : Option<u32> = None;
	let mut sender   : Option<String> = None;
	let mut target   : Option<String> = None;
	let mut snd_time : Option<DateTime<Utc>> = None;

	let standard_headers = &raw.flds;
	
	// not sure we need to filter. these will be the first fields anyways
	for fld in standard_headers.iter().filter(|f| { f.id == 34 || f.id == 35 || f.id == 49 || f.id == 52 || f.id == 56 }) {
		// to_string sends them to the heap
		match fld { 
			&FieldVal { id: MSG_SEQ,        val: v } => { msg_seq  = Some(u32::from_str(v).unwrap());  },
			&FieldVal { id: MSG_TYPE,       val: v } => { msg_type = Some(v.to_string());  },
			&FieldVal { id: SENDER_COMP_ID, val: v } => { sender   = Some(v.to_string());  },
			&FieldVal { id: TARGET_COMP_ID, val: v } => { target   = Some(v.to_string());  },
			&FieldVal { id: SEND_TIME,      val: v } => { snd_time = timestamp_val( v.as_bytes() );  },
			_ => {
				return IResult::Error(error_code!(ErrorKind::Custom(42))); // TODO: better errors!
			}
		}
	}

	let fixframe = FixFrame {
        begin_string: Cow::from(FIX_BEGIN),
		sending  : snd_time.unwrap(),
		seq : msg_seq.unwrap(),
		sender_comp_id: sender.unwrap(),
		target_comp_id: target.unwrap(),
		// OnBehalfOfCompID: String, 115 
		// DeliverToCompID: String,  128
		message : build_fix_message( &msg_type.unwrap(), &raw.flds ), // delegates to code gen
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
	fn rawframe_parsing() {
		let res = raw_frame( b"8=FIX.4.4\x019=57\x0135=0\x0134=3\x0149=RCLRA310\x0152=20170627-14:24:14.804\x0156=OE103C\x0110=082\x01" ) ;
		// println!("rawframe_parsing {:?}", res);
		let expected_frame = RawFixFrame { 
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
		let res =  raw_frame( b"8=FIX.4.4\x01" ) ;
		// println!("rawframe_parsing_failure1 {:?}", res);
		assert_eq!(res, IResult::Incomplete(Needed::Size(12)));
	}

	#[test]
	fn field_parsing() {
		let res =  field( b"9=98\x0110=something" ) ;
		// println!("field_parsing {:?}", res);
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
	// 	let line = "20170627-19:32:13.106 : 8=FIX.4.4|9=206|35=D|34=2|49=CCLRA301|52=20170627-19:32:13.105|56=OE101C|1=4004|11=30011_0|38=100|40=2|44=5|54=1|55=PETR4|59=0|60=20170627-16:32:13|453=3|448=CCLRA301|447=D|452=36|448=308|447=D|452=7|448=DMA1|447=D|452=54|10=207|\r\n".replace("|", "\x01");
	// 	let b = parse(line.as_bytes());
	// 	println!("new_order_single {:?}", b);
	// }
}
