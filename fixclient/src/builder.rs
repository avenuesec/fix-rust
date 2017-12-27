
use fix::frame::*;
use fix::fixmessagegen::*;


pub fn build_logon( seq: i32, reset_seq_num_flag: Option<bool> ) -> FixFrame {
    let flds = LogonFields {
        encrypt_method: FieldEncryptMethodEnum::None,
        heart_bt_int: 60,
        reset_seq_num_flag,
        .. Default::default()
    };
    let logon = FixMessage::Logon(Box::new(flds));
    FixFrame::new(seq, "sender", "target", "FIX.4.2", logon)
}

pub fn build_logout( seq: i32, text: Option<String> ) -> FixFrame {
    let flds = LogoutFields {
        text,
        .. Default::default()
    };
    let msg = FixMessage::Logout(Box::new(flds));
    FixFrame::new(seq, "sender", "target", "FIX.4.2", msg)
}

pub fn build_resend_req( seq: i32, begin: i32, end: i32 ) -> FixFrame {
    let flds = ResendRequestFields {
        begin_seq_no: begin,
        end_seq_no: end,
        .. Default::default()
    };
    let msg = FixMessage::ResendRequest(Box::new(flds));
    FixFrame::new(seq, "sender", "target", "FIX.4.2", msg)
}

pub fn build_sequence_reset( seq: i32, new_seq: i32, gap_fill: Option<bool> ) -> FixFrame {
    let flds = SequenceResetFields {
        gap_fill_flag: gap_fill,
        new_seq_no: new_seq,
        .. Default::default()
    };
    let msg = FixMessage::SequenceReset(Box::new(flds));
    let mut frame = FixFrame::new(seq, "sender", "target", "FIX.4.2", msg);
    frame.header.poss_dup_flag = Some(true);
    frame
}

pub fn build_test_req( seq: i32, test_req_id: &str ) -> FixFrame {
    let flds = TestRequestFields {
        test_req_id: test_req_id.to_owned(),
        .. Default::default()
    };
    let msg = FixMessage::TestRequest(Box::new(flds));
    FixFrame::new(seq, "sender", "target", "FIX.4.2", msg)
}

pub fn build_heartbeat( seq: i32, test_req_id: Option<&str> ) -> FixFrame {
    let flds = HeartbeatFields {
        test_req_id: test_req_id.map_or( None, |v| Some(v.to_owned()) ),
        .. Default::default()
    };
    let msg = FixMessage::Heartbeat(Box::new(flds));
    FixFrame::new(seq, "sender", "target", "FIX.4.2", msg)
}

pub fn build_new_order_single( seq: i32, poss_dup: bool, cl_ord_id: &str, symbol: &str, qty: f32, price: f32,
                               side: FieldSideEnum, ord_type: FieldOrdTypeEnum ) -> FixFrame {
    let flds = NewOrderSingleFields {
        cl_ord_id: cl_ord_id.to_owned(),
        symbol: symbol.to_owned(),
        order_qty: Some(qty),
        price: Some(price),
        side,
        ord_type,
        handl_inst: FieldHandlInstEnum::AutomatedExecutionOrderPrivateNoBrokerIntervention,
        .. Default::default()
    };
    let msg = FixMessage::NewOrderSingle(Box::new(flds));
    let mut frame = FixFrame::new(seq, "sender", "target", "FIX.4.2", msg);
    if poss_dup {
        frame.header.poss_dup_flag = Some(true);
    }
    frame
}

pub fn build_exec_report( seq: i32, poss_dup: bool, cl_ord_id: &str, order_id: &str, exec_id: &str, symbol: &str,
                          leaves_qty: f32, cum_qty: f32, avg_px: f32,
                          side: FieldSideEnum,
                          exec_type: FieldExecTypeEnum, trans_type: FieldExecTransTypeEnum,
                          ord_status: FieldOrdStatusEnum ) -> FixFrame {

    let flds = ExecutionReportFields {
        order_id: order_id.to_owned(),
        cl_ord_id: Some(cl_ord_id.to_owned()),
        exec_id: exec_id.to_owned(),
        exec_trans_type: trans_type,
        symbol: symbol.to_owned(),
        leaves_qty, cum_qty, avg_px,
        ord_status,
        exec_type,
        .. Default::default()
    };
    let msg = FixMessage::ExecutionReport(Box::new(flds));
    let mut frame = FixFrame::new(seq, "sender", "target", "FIX.4.2", msg);
    if poss_dup {
        frame.header.poss_dup_flag = Some(true);
    }
    frame
}