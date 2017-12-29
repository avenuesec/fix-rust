
use std::io;

use super::MessageStore;
use super::UserHandler;

use fix::frame::FixFrame;
use fix::fixmessagegen::*;

pub struct MessageToReSend {
    pub seq: i32,
    pub orig_sending_time: UtcDateTime,
    pub message: FixMessage,
}


/// gathers the messages that need to be resent
pub fn build_resend_request_response<Store>(store : &mut Store, /*f : F,*/ start: i32, end: i32) -> io::Result<Vec<MessageToReSend>>
    where Store : MessageStore/*, F : Fn(&FixFrame) -> bool*/ {

    let mut entries = store.query(start, end)?;

    let mut result = Vec::with_capacity( entries.len() );

    let mut temp_gap_to_fill : Option<i32> = None;
    let mut expected_seq = start;
    let mut cur_seq = 0;

    for frame in entries.drain(0..) {
        cur_seq = frame.header.msg_seq_num;

        if temp_gap_to_fill.is_none() && cur_seq != expected_seq  {
            // is there a gap between the seq_num expected and the one we're dealing now?
            temp_gap_to_fill = Some(cur_seq);
        }

        if is_admin_message( &frame.message ) && frame.message.msg_type() != FieldMsgTypeEnum::Reject /*|| f( &frame ) == false*/ {
            // we cannot resend admin messages (with the exception of rejects)
            // so we need to adjust the new start seq

            if temp_gap_to_fill.is_none() {
                temp_gap_to_fill = Some(cur_seq);
            }

        } else {
            // It's a valid message that can be re-sent

            // is there gap?
            if temp_gap_to_fill.is_some() {
                let new_start_seq = temp_gap_to_fill.take().unwrap();
                result.push( build_gap_fill( new_start_seq, cur_seq ) );
            }

            result.push( build_resend( frame ) );
        }

        expected_seq = cur_seq + 1;
    }

    if temp_gap_to_fill.is_some() {
        // gap left to be filled
        let new_start_seq = temp_gap_to_fill.take().unwrap();
        result.push( build_gap_fill( new_start_seq, expected_seq ) );
    }

    let new_end = {
        if end > cur_seq {
            // we didnt have all requested messages. fill the gap
            let new_end = end + 1;
            result.push( build_gap_fill( expected_seq, new_end ) );
            new_end
        } else {
            end
        }
    };

//    if new_end > self.get_next_sender_seq_num() {
//        if let Ok(mut store) = self.store.try_lock() {
//            info!("overwrite_sender_seq to new seq {}", new_end);
//            store.overwrite_sender_seq( new_end );
//        }
//    }

    Ok( result )
}

fn build_gap_fill( other_party_expected_seq: i32, new_seq_no: i32 ) -> MessageToReSend {
    let flds = SequenceResetFields {
        gap_fill_flag: Some(true),
        new_seq_no,
    };
    MessageToReSend {
        seq: other_party_expected_seq,
        orig_sending_time: UtcDateTime::now(),
        message: FixMessage::SequenceReset( Box::new(flds) ),
    }
}

fn build_resend( frame : FixFrame ) -> MessageToReSend {
    MessageToReSend {
        seq: frame.header.msg_seq_num,
        orig_sending_time: frame.header.sending_time.clone(),
        message: frame.message,
    }
}
