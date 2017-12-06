// Generated file.
// Structs, enums to support parsing/generation of fix messages

#![ignore(unused_imports)]
#![ignore(unused_variables)]
#![ignore(dead_code)]

use std::str::{FromStr};
use std::{io, str, char, i32};
use std::io::Write;
use std::default::{Default};
use std::borrow::Cow;

use bytes::BytesMut;
use chrono::prelude::*;  // DateTime
// use serde::{Serialize,Deserialize,Deserializer,Serializer};


use frame::{FieldVal};
use fixmessage::*;


use std::fmt::{Debug, Formatter, Display, Error};

#[derive(PartialEq,Debug,Serialize,Deserialize,Default)]
pub struct FixHeader {
    pub begin_string : Cow<'static, str>,

    pub msg_type : FieldMsgTypeEnum, // FIELD_MSGTYPE 35
    pub sender_comp_id : String, // FIELD_SENDERCOMPID 49
    pub target_comp_id : String, // FIELD_TARGETCOMPID 56
    pub on_behalf_of_comp_id : Option<String>, // FIELD_ONBEHALFOFCOMPID 115
    pub deliver_to_comp_id : Option<String>, // FIELD_DELIVERTOCOMPID 128
    pub secure_data_len : Option<usize>, // FIELD_SECUREDATALEN 90
    pub secure_data : Option<String>, // FIELD_SECUREDATA 91
    pub msg_seq_num : i32, // FIELD_MSGSEQNUM 34
    pub sender_sub_id : Option<String>, // FIELD_SENDERSUBID 50
    pub sender_location_id : Option<String>, // FIELD_SENDERLOCATIONID 142
    pub target_sub_id : Option<String>, // FIELD_TARGETSUBID 57
    pub target_location_id : Option<String>, // FIELD_TARGETLOCATIONID 143
    pub on_behalf_of_sub_id : Option<String>, // FIELD_ONBEHALFOFSUBID 116
    pub on_behalf_of_location_id : Option<String>, // FIELD_ONBEHALFOFLOCATIONID 144
    pub deliver_to_sub_id : Option<String>, // FIELD_DELIVERTOSUBID 129
    pub deliver_to_location_id : Option<String>, // FIELD_DELIVERTOLOCATIONID 145
    pub poss_dup_flag : Option<bool>, // FIELD_POSSDUPFLAG 43
    pub poss_resend : Option<bool>, // FIELD_POSSRESEND 97
    pub sending_time : UtcDateTime, // FIELD_SENDINGTIME 52
    pub orig_sending_time : Option<UtcDateTime>, // FIELD_ORIGSENDINGTIME 122
    pub xml_data_len : Option<usize>, // FIELD_XMLDATALEN 212
    pub xml_data : Option<String>, // FIELD_XMLDATA 213
    pub message_encoding : Option<FieldMessageEncodingEnum>, // FIELD_MESSAGEENCODING 347
    pub last_msg_seq_num_processed : Option<i32>, // FIELD_LASTMSGSEQNUMPROCESSED 369
    pub on_behalf_of_sending_time : Option<UtcDateTime>, // FIELD_ONBEHALFOFSENDINGTIME 370
}

// size of enum = tag + size of largest message, so we box the fields
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum FixMessage {
    // type: 0
    Heartbeat(Box<HeartbeatFields>),
    // type: 1
    TestRequest(Box<TestRequestFields>),
    // type: 2
    ResendRequest(Box<ResendRequestFields>),
    // type: 3
    Reject(Box<RejectFields>),
    // type: 4
    SequenceReset(Box<SequenceResetFields>),
    // type: 5
    Logout(Box<LogoutFields>),
    // type: 7
    Advertisement(Box<AdvertisementFields>),
    // type: 8
    ExecutionReport(Box<ExecutionReportFields>),
    // type: 9
    OrderCancelReject(Box<OrderCancelRejectFields>),
    // type: A
    Logon(Box<LogonFields>),
    // type: D
    NewOrderSingle(Box<NewOrderSingleFields>),
    // type: E
    NewOrderList(Box<NewOrderListFields>),
    // type: F
    OrderCancelRequest(Box<OrderCancelRequestFields>),
    // type: G
    OrderCancelReplaceRequest(Box<OrderCancelReplaceRequestFields>),
    // type: H
    OrderStatusRequest(Box<OrderStatusRequestFields>),
    // fixed, for when the parser can't figure out the message type
    UndefinedMessage,
}

impl FixMessage {
    pub fn msg_type(&self) -> FieldMsgTypeEnum {
        match self {
            &FixMessage::Heartbeat(_) => FieldMsgTypeEnum::Heartbeat,      // 0
            &FixMessage::TestRequest(_) => FieldMsgTypeEnum::TestRequest,      // 1
            &FixMessage::ResendRequest(_) => FieldMsgTypeEnum::ResendRequest,      // 2
            &FixMessage::Reject(_) => FieldMsgTypeEnum::Reject,      // 3
            &FixMessage::SequenceReset(_) => FieldMsgTypeEnum::SequenceReset,      // 4
            &FixMessage::Logout(_) => FieldMsgTypeEnum::Logout,      // 5
            &FixMessage::Advertisement(_) => FieldMsgTypeEnum::Advertisement,      // 7
            &FixMessage::ExecutionReport(_) => FieldMsgTypeEnum::ExecutionReport,      // 8
            &FixMessage::OrderCancelReject(_) => FieldMsgTypeEnum::OrderCancelReject,      // 9
            &FixMessage::Logon(_) => FieldMsgTypeEnum::Logon,      // A
            &FixMessage::NewOrderSingle(_) => FieldMsgTypeEnum::OrderSingle,      // D
            &FixMessage::NewOrderList(_) => FieldMsgTypeEnum::OrderList,      // E
            &FixMessage::OrderCancelRequest(_) => FieldMsgTypeEnum::OrderCancelRequest,      // F
            &FixMessage::OrderCancelReplaceRequest(_) => FieldMsgTypeEnum::OrderCancelReplaceRequest,      // G
            &FixMessage::OrderStatusRequest(_) => FieldMsgTypeEnum::OrderStatusRequest,      // H
            _ => unreachable!()
        }
    }
}

// message struct Fields

#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct HeartbeatFields {
    pub test_req_id : Option<String>, // FIELD_TESTREQID 112

}

#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct TestRequestFields {
    pub test_req_id : String, // FIELD_TESTREQID 112

}

#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct ResendRequestFields {
    pub begin_seq_no : i32, // FIELD_BEGINSEQNO 7
    pub end_seq_no : i32, // FIELD_ENDSEQNO 16

}

#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct RejectFields {
    pub ref_seq_num : i32, // FIELD_REFSEQNUM 45
    pub ref_tag_id : Option<i32>, // FIELD_REFTAGID 371
    pub ref_msg_type : Option<String>, // FIELD_REFMSGTYPE 372
    pub session_reject_reason : Option<FieldSessionRejectReasonEnum>, // FIELD_SESSIONREJECTREASON 373
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}

#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct SequenceResetFields {
    pub gap_fill_flag : Option<bool>, // FIELD_GAPFILLFLAG 123
    pub new_seq_no : i32, // FIELD_NEWSEQNO 36

}

#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct LogoutFields {
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}

#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct AdvertisementFields {
    pub adv_id : String, // FIELD_ADVID 2
    pub adv_trans_type : FieldAdvTransTypeEnum, // FIELD_ADVTRANSTYPE 5
    pub adv_ref_id : Option<String>, // FIELD_ADVREFID 3
    pub symbol : String, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<FieldIDSourceEnum>, // FIELD_IDSOURCE 22
    pub security_type : Option<FieldSecurityTypeEnum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<FieldPutOrCallEnum>, // FIELD_PUTORCALL 201
    pub strike_price : Option<f32>, // FIELD_STRIKEPRICE 202
    pub opt_attribute : Option<char>, // FIELD_OPTATTRIBUTE 206
    pub contract_multiplier : Option<f32>, // FIELD_CONTRACTMULTIPLIER 231
    pub coupon_rate : Option<f32>, // FIELD_COUPONRATE 223
    pub security_exchange : Option<String>, // FIELD_SECURITYEXCHANGE 207
    pub issuer : Option<String>, // FIELD_ISSUER 106
    pub encoded_issuer_len : Option<usize>, // FIELD_ENCODEDISSUERLEN 348
    pub encoded_issuer : Option<String>, // FIELD_ENCODEDISSUER 349
    pub security_desc : Option<String>, // FIELD_SECURITYDESC 107
    pub encoded_security_desc_len : Option<usize>, // FIELD_ENCODEDSECURITYDESCLEN 350
    pub encoded_security_desc : Option<String>, // FIELD_ENCODEDSECURITYDESC 351
    pub adv_side : FieldAdvSideEnum, // FIELD_ADVSIDE 4
    pub shares : f32, // FIELD_SHARES 53
    pub price : Option<f32>, // FIELD_PRICE 44
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub trade_date : Option<UtcDateTime>, // FIELD_TRADEDATE 75
    pub transact_time : Option<UtcDateTime>, // FIELD_TRANSACTTIME 60
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355
    pub urllink : Option<String>, // FIELD_URLLINK 149
    pub last_mkt : Option<String>, // FIELD_LASTMKT 30
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336

}

#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct ExecutionReportFields {
    pub order_id : String, // FIELD_ORDERID 37
    pub secondary_order_id : Option<String>, // FIELD_SECONDARYORDERID 198
    pub cl_ord_id : Option<String>, // FIELD_CLORDID 11
    pub orig_cl_ord_id : Option<String>, // FIELD_ORIGCLORDID 41
    pub client_id : Option<String>, // FIELD_CLIENTID 109
    pub exec_broker : Option<String>, // FIELD_EXECBROKER 76
    pub no_contra_brokers : Option<Vec<NoContraBrokers2Fields>>, // FIELD_NOCONTRABROKERS 0
    pub list_id : Option<String>, // FIELD_LISTID 66
    pub exec_id : String, // FIELD_EXECID 17
    pub exec_trans_type : FieldExecTransTypeEnum, // FIELD_EXECTRANSTYPE 20
    pub exec_ref_id : Option<String>, // FIELD_EXECREFID 19
    pub exec_type : FieldExecTypeEnum, // FIELD_EXECTYPE 150
    pub ord_status : FieldOrdStatusEnum, // FIELD_ORDSTATUS 39
    pub ord_rej_reason : Option<FieldOrdRejReasonEnum>, // FIELD_ORDREJREASON 103
    pub exec_restatement_reason : Option<FieldExecRestatementReasonEnum>, // FIELD_EXECRESTATEMENTREASON 378
    pub account : Option<String>, // FIELD_ACCOUNT 1
    pub settlmnt_typ : Option<FieldSettlmntTypEnum>, // FIELD_SETTLMNTTYP 63
    pub fut_sett_date : Option<UtcDateTime>, // FIELD_FUTSETTDATE 64
    pub symbol : String, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<FieldIDSourceEnum>, // FIELD_IDSOURCE 22
    pub security_type : Option<FieldSecurityTypeEnum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<FieldPutOrCallEnum>, // FIELD_PUTORCALL 201
    pub strike_price : Option<f32>, // FIELD_STRIKEPRICE 202
    pub opt_attribute : Option<char>, // FIELD_OPTATTRIBUTE 206
    pub contract_multiplier : Option<f32>, // FIELD_CONTRACTMULTIPLIER 231
    pub coupon_rate : Option<f32>, // FIELD_COUPONRATE 223
    pub security_exchange : Option<String>, // FIELD_SECURITYEXCHANGE 207
    pub issuer : Option<String>, // FIELD_ISSUER 106
    pub encoded_issuer_len : Option<usize>, // FIELD_ENCODEDISSUERLEN 348
    pub encoded_issuer : Option<String>, // FIELD_ENCODEDISSUER 349
    pub security_desc : Option<String>, // FIELD_SECURITYDESC 107
    pub encoded_security_desc_len : Option<usize>, // FIELD_ENCODEDSECURITYDESCLEN 350
    pub encoded_security_desc : Option<String>, // FIELD_ENCODEDSECURITYDESC 351
    pub side : FieldSideEnum, // FIELD_SIDE 54
    pub order_qty : Option<f32>, // FIELD_ORDERQTY 38
    pub cash_order_qty : Option<f32>, // FIELD_CASHORDERQTY 152
    pub ord_type : Option<FieldOrdTypeEnum>, // FIELD_ORDTYPE 40
    pub price : Option<f32>, // FIELD_PRICE 44
    pub stop_px : Option<f32>, // FIELD_STOPPX 99
    pub peg_difference : Option<f32>, // FIELD_PEGDIFFERENCE 211
    pub discretion_inst : Option<FieldDiscretionInstEnum>, // FIELD_DISCRETIONINST 388
    pub discretion_offset : Option<f32>, // FIELD_DISCRETIONOFFSET 389
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub compliance_id : Option<String>, // FIELD_COMPLIANCEID 376
    pub solicited_flag : Option<bool>, // FIELD_SOLICITEDFLAG 377
    pub time_in_force : Option<FieldTimeInForceEnum>, // FIELD_TIMEINFORCE 59
    pub effective_time : Option<UtcDateTime>, // FIELD_EFFECTIVETIME 168
    pub expire_date : Option<UtcDateTime>, // FIELD_EXPIREDATE 432
    pub expire_time : Option<UtcDateTime>, // FIELD_EXPIRETIME 126
    pub exec_inst : Option<FieldExecInstEnum>, // FIELD_EXECINST 18
    pub rule80_a : Option<FieldRule80AEnum>, // FIELD_RULE80A 47
    pub last_shares : Option<f32>, // FIELD_LASTSHARES 32
    pub last_px : Option<f32>, // FIELD_LASTPX 31
    pub last_spot_rate : Option<f32>, // FIELD_LASTSPOTRATE 194
    pub last_forward_points : Option<f32>, // FIELD_LASTFORWARDPOINTS 195
    pub last_mkt : Option<String>, // FIELD_LASTMKT 30
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub last_capacity : Option<FieldLastCapacityEnum>, // FIELD_LASTCAPACITY 29
    pub leaves_qty : f32, // FIELD_LEAVESQTY 151
    pub cum_qty : f32, // FIELD_CUMQTY 14
    pub avg_px : f32, // FIELD_AVGPX 6
    pub day_order_qty : Option<f32>, // FIELD_DAYORDERQTY 424
    pub day_cum_qty : Option<f32>, // FIELD_DAYCUMQTY 425
    pub day_avg_px : Option<f32>, // FIELD_DAYAVGPX 426
    pub gtbooking_inst : Option<FieldGTBookingInstEnum>, // FIELD_GTBOOKINGINST 427
    pub trade_date : Option<UtcDateTime>, // FIELD_TRADEDATE 75
    pub transact_time : Option<UtcDateTime>, // FIELD_TRANSACTTIME 60
    pub report_to_exch : Option<bool>, // FIELD_REPORTTOEXCH 113
    pub commission : Option<f32>, // FIELD_COMMISSION 12
    pub comm_type : Option<FieldCommTypeEnum>, // FIELD_COMMTYPE 13
    pub gross_trade_amt : Option<f32>, // FIELD_GROSSTRADEAMT 381
    pub settl_curr_amt : Option<f32>, // FIELD_SETTLCURRAMT 119
    pub settl_currency : Option<f32>, // FIELD_SETTLCURRENCY 120
    pub settl_curr_fx_rate : Option<f32>, // FIELD_SETTLCURRFXRATE 155
    pub settl_curr_fx_rate_calc : Option<FieldSettlCurrFxRateCalcEnum>, // FIELD_SETTLCURRFXRATECALC 156
    pub handl_inst : Option<FieldHandlInstEnum>, // FIELD_HANDLINST 21
    pub min_qty : Option<f32>, // FIELD_MINQTY 110
    pub max_floor : Option<f32>, // FIELD_MAXFLOOR 111
    pub open_close : Option<FieldOpenCloseEnum>, // FIELD_OPENCLOSE 77
    pub max_show : Option<f32>, // FIELD_MAXSHOW 210
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355
    pub fut_sett_date2 : Option<UtcDateTime>, // FIELD_FUTSETTDATE2 193
    pub order_qty2 : Option<f32>, // FIELD_ORDERQTY2 192
    pub clearing_firm : Option<String>, // FIELD_CLEARINGFIRM 439
    pub clearing_account : Option<String>, // FIELD_CLEARINGACCOUNT 440
    pub multi_leg_reporting_type : Option<FieldMultiLegReportingTypeEnum>, // FIELD_MULTILEGREPORTINGTYPE 442

}

#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct OrderCancelRejectFields {
    pub order_id : String, // FIELD_ORDERID 37
    pub secondary_order_id : Option<String>, // FIELD_SECONDARYORDERID 198
    pub cl_ord_id : String, // FIELD_CLORDID 11
    pub orig_cl_ord_id : String, // FIELD_ORIGCLORDID 41
    pub ord_status : FieldOrdStatusEnum, // FIELD_ORDSTATUS 39
    pub client_id : Option<String>, // FIELD_CLIENTID 109
    pub exec_broker : Option<String>, // FIELD_EXECBROKER 76
    pub list_id : Option<String>, // FIELD_LISTID 66
    pub account : Option<String>, // FIELD_ACCOUNT 1
    pub transact_time : Option<UtcDateTime>, // FIELD_TRANSACTTIME 60
    pub cxl_rej_response_to : FieldCxlRejResponseToEnum, // FIELD_CXLREJRESPONSETO 434
    pub cxl_rej_reason : Option<FieldCxlRejReasonEnum>, // FIELD_CXLREJREASON 102
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}

#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct LogonFields {
    pub encrypt_method : FieldEncryptMethodEnum, // FIELD_ENCRYPTMETHOD 98
    pub heart_bt_int : i32, // FIELD_HEARTBTINT 108
    pub raw_data_length : Option<usize>, // FIELD_RAWDATALENGTH 95
    pub raw_data : Option<String>, // FIELD_RAWDATA 96
    pub reset_seq_num_flag : Option<bool>, // FIELD_RESETSEQNUMFLAG 141
    pub max_message_size : Option<i32>, // FIELD_MAXMESSAGESIZE 383
    pub no_msg_types : Option<Vec<NoMsgTypes3Fields>>, // FIELD_NOMSGTYPES 0

}

#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NewOrderSingleFields {
    pub cl_ord_id : String, // FIELD_CLORDID 11
    pub client_id : Option<String>, // FIELD_CLIENTID 109
    pub exec_broker : Option<String>, // FIELD_EXECBROKER 76
    pub account : Option<String>, // FIELD_ACCOUNT 1
    pub no_allocs : Option<Vec<NoAllocs1Fields>>, // FIELD_NOALLOCS 0
    pub settlmnt_typ : Option<FieldSettlmntTypEnum>, // FIELD_SETTLMNTTYP 63
    pub fut_sett_date : Option<UtcDateTime>, // FIELD_FUTSETTDATE 64
    pub handl_inst : FieldHandlInstEnum, // FIELD_HANDLINST 21
    pub exec_inst : Option<FieldExecInstEnum>, // FIELD_EXECINST 18
    pub min_qty : Option<f32>, // FIELD_MINQTY 110
    pub max_floor : Option<f32>, // FIELD_MAXFLOOR 111
    pub ex_destination : Option<String>, // FIELD_EXDESTINATION 100
    pub no_trading_sessions : Option<Vec<NoTradingSessions5Fields>>, // FIELD_NOTRADINGSESSIONS 0
    pub process_code : Option<FieldProcessCodeEnum>, // FIELD_PROCESSCODE 81
    pub symbol : String, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<FieldIDSourceEnum>, // FIELD_IDSOURCE 22
    pub security_type : Option<FieldSecurityTypeEnum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<FieldPutOrCallEnum>, // FIELD_PUTORCALL 201
    pub strike_price : Option<f32>, // FIELD_STRIKEPRICE 202
    pub opt_attribute : Option<char>, // FIELD_OPTATTRIBUTE 206
    pub contract_multiplier : Option<f32>, // FIELD_CONTRACTMULTIPLIER 231
    pub coupon_rate : Option<f32>, // FIELD_COUPONRATE 223
    pub security_exchange : Option<String>, // FIELD_SECURITYEXCHANGE 207
    pub issuer : Option<String>, // FIELD_ISSUER 106
    pub encoded_issuer_len : Option<usize>, // FIELD_ENCODEDISSUERLEN 348
    pub encoded_issuer : Option<String>, // FIELD_ENCODEDISSUER 349
    pub security_desc : Option<String>, // FIELD_SECURITYDESC 107
    pub encoded_security_desc_len : Option<usize>, // FIELD_ENCODEDSECURITYDESCLEN 350
    pub encoded_security_desc : Option<String>, // FIELD_ENCODEDSECURITYDESC 351
    pub prev_close_px : Option<f32>, // FIELD_PREVCLOSEPX 140
    pub side : FieldSideEnum, // FIELD_SIDE 54
    pub locate_reqd : Option<bool>, // FIELD_LOCATEREQD 114
    pub transact_time : UtcDateTime, // FIELD_TRANSACTTIME 60
    pub order_qty : Option<f32>, // FIELD_ORDERQTY 38
    pub cash_order_qty : Option<f32>, // FIELD_CASHORDERQTY 152
    pub ord_type : FieldOrdTypeEnum, // FIELD_ORDTYPE 40
    pub price : Option<f32>, // FIELD_PRICE 44
    pub stop_px : Option<f32>, // FIELD_STOPPX 99
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub compliance_id : Option<String>, // FIELD_COMPLIANCEID 376
    pub solicited_flag : Option<bool>, // FIELD_SOLICITEDFLAG 377
    pub ioiid : Option<String>, // FIELD_IOIID 23
    pub quote_id : Option<String>, // FIELD_QUOTEID 117
    pub time_in_force : Option<FieldTimeInForceEnum>, // FIELD_TIMEINFORCE 59
    pub effective_time : Option<UtcDateTime>, // FIELD_EFFECTIVETIME 168
    pub expire_date : Option<UtcDateTime>, // FIELD_EXPIREDATE 432
    pub expire_time : Option<UtcDateTime>, // FIELD_EXPIRETIME 126
    pub gtbooking_inst : Option<FieldGTBookingInstEnum>, // FIELD_GTBOOKINGINST 427
    pub commission : Option<f32>, // FIELD_COMMISSION 12
    pub comm_type : Option<FieldCommTypeEnum>, // FIELD_COMMTYPE 13
    pub rule80_a : Option<FieldRule80AEnum>, // FIELD_RULE80A 47
    pub forex_req : Option<bool>, // FIELD_FOREXREQ 121
    pub settl_currency : Option<f32>, // FIELD_SETTLCURRENCY 120
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355
    pub fut_sett_date2 : Option<UtcDateTime>, // FIELD_FUTSETTDATE2 193
    pub order_qty2 : Option<f32>, // FIELD_ORDERQTY2 192
    pub open_close : Option<FieldOpenCloseEnum>, // FIELD_OPENCLOSE 77
    pub covered_or_uncovered : Option<FieldCoveredOrUncoveredEnum>, // FIELD_COVEREDORUNCOVERED 203
    pub customer_or_firm : Option<FieldCustomerOrFirmEnum>, // FIELD_CUSTOMERORFIRM 204
    pub max_show : Option<f32>, // FIELD_MAXSHOW 210
    pub peg_difference : Option<f32>, // FIELD_PEGDIFFERENCE 211
    pub discretion_inst : Option<FieldDiscretionInstEnum>, // FIELD_DISCRETIONINST 388
    pub discretion_offset : Option<f32>, // FIELD_DISCRETIONOFFSET 389
    pub clearing_firm : Option<String>, // FIELD_CLEARINGFIRM 439
    pub clearing_account : Option<String>, // FIELD_CLEARINGACCOUNT 440

}

#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NewOrderListFields {
    pub list_id : String, // FIELD_LISTID 66
    pub bid_id : Option<String>, // FIELD_BIDID 390
    pub client_bid_id : Option<String>, // FIELD_CLIENTBIDID 391
    pub prog_rpt_reqs : Option<FieldProgRptReqsEnum>, // FIELD_PROGRPTREQS 414
    pub bid_type : i32, // FIELD_BIDTYPE 394
    pub prog_period_interval : Option<i32>, // FIELD_PROGPERIODINTERVAL 415
    pub list_exec_inst_type : Option<FieldListExecInstTypeEnum>, // FIELD_LISTEXECINSTTYPE 433
    pub list_exec_inst : Option<String>, // FIELD_LISTEXECINST 69
    pub encoded_list_exec_inst_len : Option<usize>, // FIELD_ENCODEDLISTEXECINSTLEN 352
    pub encoded_list_exec_inst : Option<String>, // FIELD_ENCODEDLISTEXECINST 353
    pub tot_no_orders : i32, // FIELD_TOTNOORDERS 68
    pub no_orders : Vec<NoOrders4Fields>, // FIELD_NOORDERS 0

}

#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct OrderCancelRequestFields {
    pub orig_cl_ord_id : String, // FIELD_ORIGCLORDID 41
    pub order_id : Option<String>, // FIELD_ORDERID 37
    pub cl_ord_id : String, // FIELD_CLORDID 11
    pub list_id : Option<String>, // FIELD_LISTID 66
    pub account : Option<String>, // FIELD_ACCOUNT 1
    pub client_id : Option<String>, // FIELD_CLIENTID 109
    pub exec_broker : Option<String>, // FIELD_EXECBROKER 76
    pub symbol : String, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<FieldIDSourceEnum>, // FIELD_IDSOURCE 22
    pub security_type : Option<FieldSecurityTypeEnum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<FieldPutOrCallEnum>, // FIELD_PUTORCALL 201
    pub strike_price : Option<f32>, // FIELD_STRIKEPRICE 202
    pub opt_attribute : Option<char>, // FIELD_OPTATTRIBUTE 206
    pub contract_multiplier : Option<f32>, // FIELD_CONTRACTMULTIPLIER 231
    pub coupon_rate : Option<f32>, // FIELD_COUPONRATE 223
    pub security_exchange : Option<String>, // FIELD_SECURITYEXCHANGE 207
    pub issuer : Option<String>, // FIELD_ISSUER 106
    pub encoded_issuer_len : Option<usize>, // FIELD_ENCODEDISSUERLEN 348
    pub encoded_issuer : Option<String>, // FIELD_ENCODEDISSUER 349
    pub security_desc : Option<String>, // FIELD_SECURITYDESC 107
    pub encoded_security_desc_len : Option<usize>, // FIELD_ENCODEDSECURITYDESCLEN 350
    pub encoded_security_desc : Option<String>, // FIELD_ENCODEDSECURITYDESC 351
    pub side : FieldSideEnum, // FIELD_SIDE 54
    pub transact_time : UtcDateTime, // FIELD_TRANSACTTIME 60
    pub order_qty : Option<f32>, // FIELD_ORDERQTY 38
    pub cash_order_qty : Option<f32>, // FIELD_CASHORDERQTY 152
    pub compliance_id : Option<String>, // FIELD_COMPLIANCEID 376
    pub solicited_flag : Option<bool>, // FIELD_SOLICITEDFLAG 377
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}

#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct OrderCancelReplaceRequestFields {
    pub order_id : Option<String>, // FIELD_ORDERID 37
    pub client_id : Option<String>, // FIELD_CLIENTID 109
    pub exec_broker : Option<String>, // FIELD_EXECBROKER 76
    pub orig_cl_ord_id : String, // FIELD_ORIGCLORDID 41
    pub cl_ord_id : String, // FIELD_CLORDID 11
    pub list_id : Option<String>, // FIELD_LISTID 66
    pub account : Option<String>, // FIELD_ACCOUNT 1
    pub no_allocs : Option<Vec<NoAllocs1Fields>>, // FIELD_NOALLOCS 0
    pub settlmnt_typ : Option<FieldSettlmntTypEnum>, // FIELD_SETTLMNTTYP 63
    pub fut_sett_date : Option<UtcDateTime>, // FIELD_FUTSETTDATE 64
    pub handl_inst : FieldHandlInstEnum, // FIELD_HANDLINST 21
    pub exec_inst : Option<FieldExecInstEnum>, // FIELD_EXECINST 18
    pub min_qty : Option<f32>, // FIELD_MINQTY 110
    pub max_floor : Option<f32>, // FIELD_MAXFLOOR 111
    pub ex_destination : Option<String>, // FIELD_EXDESTINATION 100
    pub no_trading_sessions : Option<Vec<NoTradingSessions5Fields>>, // FIELD_NOTRADINGSESSIONS 0
    pub symbol : String, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<FieldIDSourceEnum>, // FIELD_IDSOURCE 22
    pub security_type : Option<FieldSecurityTypeEnum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<FieldPutOrCallEnum>, // FIELD_PUTORCALL 201
    pub strike_price : Option<f32>, // FIELD_STRIKEPRICE 202
    pub opt_attribute : Option<char>, // FIELD_OPTATTRIBUTE 206
    pub contract_multiplier : Option<f32>, // FIELD_CONTRACTMULTIPLIER 231
    pub coupon_rate : Option<f32>, // FIELD_COUPONRATE 223
    pub security_exchange : Option<String>, // FIELD_SECURITYEXCHANGE 207
    pub issuer : Option<String>, // FIELD_ISSUER 106
    pub encoded_issuer_len : Option<usize>, // FIELD_ENCODEDISSUERLEN 348
    pub encoded_issuer : Option<String>, // FIELD_ENCODEDISSUER 349
    pub security_desc : Option<String>, // FIELD_SECURITYDESC 107
    pub encoded_security_desc_len : Option<usize>, // FIELD_ENCODEDSECURITYDESCLEN 350
    pub encoded_security_desc : Option<String>, // FIELD_ENCODEDSECURITYDESC 351
    pub side : FieldSideEnum, // FIELD_SIDE 54
    pub transact_time : UtcDateTime, // FIELD_TRANSACTTIME 60
    pub order_qty : Option<f32>, // FIELD_ORDERQTY 38
    pub cash_order_qty : Option<f32>, // FIELD_CASHORDERQTY 152
    pub ord_type : FieldOrdTypeEnum, // FIELD_ORDTYPE 40
    pub price : Option<f32>, // FIELD_PRICE 44
    pub stop_px : Option<f32>, // FIELD_STOPPX 99
    pub peg_difference : Option<f32>, // FIELD_PEGDIFFERENCE 211
    pub discretion_inst : Option<FieldDiscretionInstEnum>, // FIELD_DISCRETIONINST 388
    pub discretion_offset : Option<f32>, // FIELD_DISCRETIONOFFSET 389
    pub compliance_id : Option<String>, // FIELD_COMPLIANCEID 376
    pub solicited_flag : Option<bool>, // FIELD_SOLICITEDFLAG 377
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub time_in_force : Option<FieldTimeInForceEnum>, // FIELD_TIMEINFORCE 59
    pub effective_time : Option<UtcDateTime>, // FIELD_EFFECTIVETIME 168
    pub expire_date : Option<UtcDateTime>, // FIELD_EXPIREDATE 432
    pub expire_time : Option<UtcDateTime>, // FIELD_EXPIRETIME 126
    pub gtbooking_inst : Option<FieldGTBookingInstEnum>, // FIELD_GTBOOKINGINST 427
    pub commission : Option<f32>, // FIELD_COMMISSION 12
    pub comm_type : Option<FieldCommTypeEnum>, // FIELD_COMMTYPE 13
    pub rule80_a : Option<FieldRule80AEnum>, // FIELD_RULE80A 47
    pub forex_req : Option<bool>, // FIELD_FOREXREQ 121
    pub settl_currency : Option<f32>, // FIELD_SETTLCURRENCY 120
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355
    pub fut_sett_date2 : Option<UtcDateTime>, // FIELD_FUTSETTDATE2 193
    pub order_qty2 : Option<f32>, // FIELD_ORDERQTY2 192
    pub open_close : Option<FieldOpenCloseEnum>, // FIELD_OPENCLOSE 77
    pub covered_or_uncovered : Option<FieldCoveredOrUncoveredEnum>, // FIELD_COVEREDORUNCOVERED 203
    pub customer_or_firm : Option<FieldCustomerOrFirmEnum>, // FIELD_CUSTOMERORFIRM 204
    pub max_show : Option<f32>, // FIELD_MAXSHOW 210
    pub locate_reqd : Option<bool>, // FIELD_LOCATEREQD 114
    pub clearing_firm : Option<String>, // FIELD_CLEARINGFIRM 439
    pub clearing_account : Option<String>, // FIELD_CLEARINGACCOUNT 440

}

#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct OrderStatusRequestFields {
    pub order_id : Option<String>, // FIELD_ORDERID 37
    pub cl_ord_id : String, // FIELD_CLORDID 11
    pub client_id : Option<String>, // FIELD_CLIENTID 109
    pub account : Option<String>, // FIELD_ACCOUNT 1
    pub exec_broker : Option<String>, // FIELD_EXECBROKER 76
    pub symbol : String, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<FieldIDSourceEnum>, // FIELD_IDSOURCE 22
    pub security_type : Option<FieldSecurityTypeEnum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<FieldPutOrCallEnum>, // FIELD_PUTORCALL 201
    pub strike_price : Option<f32>, // FIELD_STRIKEPRICE 202
    pub opt_attribute : Option<char>, // FIELD_OPTATTRIBUTE 206
    pub contract_multiplier : Option<f32>, // FIELD_CONTRACTMULTIPLIER 231
    pub coupon_rate : Option<f32>, // FIELD_COUPONRATE 223
    pub security_exchange : Option<String>, // FIELD_SECURITYEXCHANGE 207
    pub issuer : Option<String>, // FIELD_ISSUER 106
    pub encoded_issuer_len : Option<usize>, // FIELD_ENCODEDISSUERLEN 348
    pub encoded_issuer : Option<String>, // FIELD_ENCODEDISSUER 349
    pub security_desc : Option<String>, // FIELD_SECURITYDESC 107
    pub encoded_security_desc_len : Option<usize>, // FIELD_ENCODEDSECURITYDESCLEN 350
    pub encoded_security_desc : Option<String>, // FIELD_ENCODEDSECURITYDESC 351
    pub side : FieldSideEnum, // FIELD_SIDE 54

}


// components struct Fields





// groups struct Fields



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoContraBrokers2Fields {
    pub contra_broker : Option<String>, // FIELD_CONTRABROKER 375
    pub contra_trader : Option<String>, // FIELD_CONTRATRADER 337
    pub contra_trade_qty : Option<f32>, // FIELD_CONTRATRADEQTY 437
    pub contra_trade_time : Option<UtcDateTime>, // FIELD_CONTRATRADETIME 438

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoAllocs1Fields {
    pub alloc_account : Option<String>, // FIELD_ALLOCACCOUNT 79
    pub alloc_shares : Option<f32>, // FIELD_ALLOCSHARES 80

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoMsgTypes3Fields {
    pub ref_msg_type : Option<String>, // FIELD_REFMSGTYPE 372
    pub msg_direction : Option<FieldMsgDirectionEnum>, // FIELD_MSGDIRECTION 385

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoTradingSessions5Fields {
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoOrders4Fields {
    pub cl_ord_id : String, // FIELD_CLORDID 11
    pub list_seq_no : i32, // FIELD_LISTSEQNO 67
    pub settl_inst_mode : Option<FieldSettlInstModeEnum>, // FIELD_SETTLINSTMODE 160
    pub client_id : Option<String>, // FIELD_CLIENTID 109
    pub exec_broker : Option<String>, // FIELD_EXECBROKER 76
    pub account : Option<String>, // FIELD_ACCOUNT 1
    pub no_allocs : Option<Vec<NoAllocs1Fields>>, // FIELD_NOALLOCS 0
    pub settlmnt_typ : Option<FieldSettlmntTypEnum>, // FIELD_SETTLMNTTYP 63
    pub fut_sett_date : Option<UtcDateTime>, // FIELD_FUTSETTDATE 64
    pub handl_inst : Option<FieldHandlInstEnum>, // FIELD_HANDLINST 21
    pub exec_inst : Option<FieldExecInstEnum>, // FIELD_EXECINST 18
    pub min_qty : Option<f32>, // FIELD_MINQTY 110
    pub max_floor : Option<f32>, // FIELD_MAXFLOOR 111
    pub ex_destination : Option<String>, // FIELD_EXDESTINATION 100
    pub no_trading_sessions : Option<Vec<NoTradingSessions5Fields>>, // FIELD_NOTRADINGSESSIONS 0
    pub process_code : Option<FieldProcessCodeEnum>, // FIELD_PROCESSCODE 81
    pub symbol : String, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<FieldIDSourceEnum>, // FIELD_IDSOURCE 22
    pub security_type : Option<FieldSecurityTypeEnum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<FieldPutOrCallEnum>, // FIELD_PUTORCALL 201
    pub strike_price : Option<f32>, // FIELD_STRIKEPRICE 202
    pub opt_attribute : Option<char>, // FIELD_OPTATTRIBUTE 206
    pub contract_multiplier : Option<f32>, // FIELD_CONTRACTMULTIPLIER 231
    pub coupon_rate : Option<f32>, // FIELD_COUPONRATE 223
    pub security_exchange : Option<String>, // FIELD_SECURITYEXCHANGE 207
    pub issuer : Option<String>, // FIELD_ISSUER 106
    pub encoded_issuer_len : Option<usize>, // FIELD_ENCODEDISSUERLEN 348
    pub encoded_issuer : Option<String>, // FIELD_ENCODEDISSUER 349
    pub security_desc : Option<String>, // FIELD_SECURITYDESC 107
    pub encoded_security_desc_len : Option<usize>, // FIELD_ENCODEDSECURITYDESCLEN 350
    pub encoded_security_desc : Option<String>, // FIELD_ENCODEDSECURITYDESC 351
    pub prev_close_px : Option<f32>, // FIELD_PREVCLOSEPX 140
    pub side : FieldSideEnum, // FIELD_SIDE 54
    pub side_value_ind : Option<i32>, // FIELD_SIDEVALUEIND 401
    pub locate_reqd : Option<bool>, // FIELD_LOCATEREQD 114
    pub transact_time : Option<UtcDateTime>, // FIELD_TRANSACTTIME 60
    pub order_qty : Option<f32>, // FIELD_ORDERQTY 38
    pub cash_order_qty : Option<f32>, // FIELD_CASHORDERQTY 152
    pub ord_type : Option<FieldOrdTypeEnum>, // FIELD_ORDTYPE 40
    pub price : Option<f32>, // FIELD_PRICE 44
    pub stop_px : Option<f32>, // FIELD_STOPPX 99
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub compliance_id : Option<String>, // FIELD_COMPLIANCEID 376
    pub solicited_flag : Option<bool>, // FIELD_SOLICITEDFLAG 377
    pub ioiid : Option<String>, // FIELD_IOIID 23
    pub quote_id : Option<String>, // FIELD_QUOTEID 117
    pub time_in_force : Option<FieldTimeInForceEnum>, // FIELD_TIMEINFORCE 59
    pub effective_time : Option<UtcDateTime>, // FIELD_EFFECTIVETIME 168
    pub expire_date : Option<UtcDateTime>, // FIELD_EXPIREDATE 432
    pub expire_time : Option<UtcDateTime>, // FIELD_EXPIRETIME 126
    pub gtbooking_inst : Option<FieldGTBookingInstEnum>, // FIELD_GTBOOKINGINST 427
    pub commission : Option<f32>, // FIELD_COMMISSION 12
    pub comm_type : Option<FieldCommTypeEnum>, // FIELD_COMMTYPE 13
    pub rule80_a : Option<FieldRule80AEnum>, // FIELD_RULE80A 47
    pub forex_req : Option<bool>, // FIELD_FOREXREQ 121
    pub settl_currency : Option<f32>, // FIELD_SETTLCURRENCY 120
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355
    pub fut_sett_date2 : Option<UtcDateTime>, // FIELD_FUTSETTDATE2 193
    pub order_qty2 : Option<f32>, // FIELD_ORDERQTY2 192
    pub open_close : Option<FieldOpenCloseEnum>, // FIELD_OPENCLOSE 77
    pub covered_or_uncovered : Option<FieldCoveredOrUncoveredEnum>, // FIELD_COVEREDORUNCOVERED 203
    pub customer_or_firm : Option<FieldCustomerOrFirmEnum>, // FIELD_CUSTOMERORFIRM 204
    pub max_show : Option<f32>, // FIELD_MAXSHOW 210
    pub peg_difference : Option<f32>, // FIELD_PEGDIFFERENCE 211
    pub discretion_inst : Option<FieldDiscretionInstEnum>, // FIELD_DISCRETIONINST 388
    pub discretion_offset : Option<f32>, // FIELD_DISCRETIONOFFSET 389
    pub clearing_firm : Option<String>, // FIELD_CLEARINGFIRM 439
    pub clearing_account : Option<String>, // FIELD_CLEARINGACCOUNT 440

}




// Fields Constants / enums

const FIELD_ACCOUNT : u32 = 1; // STRING


const FIELD_ADVID : u32 = 2; // STRING


const FIELD_ADVREFID : u32 = 3; // STRING


const FIELD_ADVSIDE : u32 = 4; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldAdvSideEnum {
    Buy, // = "B"
    Sell, // = "S"
    Trade, // = "T"
    Cross, // = "X"

    Undefined
}

impl FromStr for FieldAdvSideEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "B" => Ok(FieldAdvSideEnum::Buy),
            "S" => Ok(FieldAdvSideEnum::Sell),
            "T" => Ok(FieldAdvSideEnum::Trade),
            "X" => Ok(FieldAdvSideEnum::Cross),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldAdvSideEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldAdvSideEnum::Buy => {
                f.write_str( "B" )
            },
            &FieldAdvSideEnum::Sell => {
                f.write_str( "S" )
            },
            &FieldAdvSideEnum::Trade => {
                f.write_str( "T" )
            },
            &FieldAdvSideEnum::Cross => {
                f.write_str( "X" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldAdvSideEnum {
    fn default() -> Self {
        FieldAdvSideEnum::Undefined
    }
}
const FIELD_ADVTRANSTYPE : u32 = 5; // STRING


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldAdvTransTypeEnum {
    Cancel, // = "C"
    New, // = "N"
    Replace, // = "R"

    Undefined
}

impl FromStr for FieldAdvTransTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(FieldAdvTransTypeEnum::Cancel),
            "N" => Ok(FieldAdvTransTypeEnum::New),
            "R" => Ok(FieldAdvTransTypeEnum::Replace),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldAdvTransTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldAdvTransTypeEnum::Cancel => {
                f.write_str( "C" )
            },
            &FieldAdvTransTypeEnum::New => {
                f.write_str( "N" )
            },
            &FieldAdvTransTypeEnum::Replace => {
                f.write_str( "R" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldAdvTransTypeEnum {
    fn default() -> Self {
        FieldAdvTransTypeEnum::Undefined
    }
}
const FIELD_AVGPX : u32 = 6; // PRICE


const FIELD_BEGINSEQNO : u32 = 7; // INT


const FIELD_BEGINSTRING : u32 = 8; // STRING


const FIELD_BODYLENGTH : u32 = 9; // INT


const FIELD_CHECKSUM : u32 = 10; // STRING


const FIELD_CLORDID : u32 = 11; // STRING


const FIELD_COMMISSION : u32 = 12; // AMT


const FIELD_COMMTYPE : u32 = 13; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldCommTypeEnum {
    PerShare, // = "1"
    Percentage, // = "2"
    Absolute, // = "3"

    Undefined
}

impl FromStr for FieldCommTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldCommTypeEnum::PerShare),
            "2" => Ok(FieldCommTypeEnum::Percentage),
            "3" => Ok(FieldCommTypeEnum::Absolute),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldCommTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldCommTypeEnum::PerShare => {
                f.write_str( "1" )
            },
            &FieldCommTypeEnum::Percentage => {
                f.write_str( "2" )
            },
            &FieldCommTypeEnum::Absolute => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldCommTypeEnum {
    fn default() -> Self {
        FieldCommTypeEnum::Undefined
    }
}
const FIELD_CUMQTY : u32 = 14; // QTY


const FIELD_CURRENCY : u32 = 15; // CURRENCY


const FIELD_ENDSEQNO : u32 = 16; // INT


const FIELD_EXECID : u32 = 17; // STRING


const FIELD_EXECINST : u32 = 18; // MULTIPLEVALUESTRING


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldExecInstEnum {
    StayOnOfferside, // = "0"
    NotHeld, // = "1"
    Work, // = "2"
    GoAlong, // = "3"
    OverTheDay, // = "4"
    Held, // = "5"
    ParticipateDontInitiate, // = "6"
    StrictScale, // = "7"
    TryToScale, // = "8"
    StayOnBidside, // = "9"
    NoCross, // = "A"
    OkToCross, // = "B"
    CallFirst, // = "C"
    PercentOfVolume, // = "D"
    DoNotIncrease, // = "E"
    DoNotReduce, // = "F"
    AllOrNone, // = "G"
    InstitutionsOnly, // = "I"
    LastPeg, // = "L"
    MidPricePeg, // = "M"
    NonNegotiable, // = "N"
    OpeningPeg, // = "O"
    MarketPeg, // = "P"
    PrimaryPeg, // = "R"
    Suspend, // = "S"
    FixedPegToLocalBestBidOrOfferAtTimeOfOrder, // = "T"
    CustomerDisplayInstruction, // = "U"
    Netting, // = "V"
    PegToVwap, // = "W"

    Undefined
}

impl FromStr for FieldExecInstEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldExecInstEnum::StayOnOfferside),
            "1" => Ok(FieldExecInstEnum::NotHeld),
            "2" => Ok(FieldExecInstEnum::Work),
            "3" => Ok(FieldExecInstEnum::GoAlong),
            "4" => Ok(FieldExecInstEnum::OverTheDay),
            "5" => Ok(FieldExecInstEnum::Held),
            "6" => Ok(FieldExecInstEnum::ParticipateDontInitiate),
            "7" => Ok(FieldExecInstEnum::StrictScale),
            "8" => Ok(FieldExecInstEnum::TryToScale),
            "9" => Ok(FieldExecInstEnum::StayOnBidside),
            "A" => Ok(FieldExecInstEnum::NoCross),
            "B" => Ok(FieldExecInstEnum::OkToCross),
            "C" => Ok(FieldExecInstEnum::CallFirst),
            "D" => Ok(FieldExecInstEnum::PercentOfVolume),
            "E" => Ok(FieldExecInstEnum::DoNotIncrease),
            "F" => Ok(FieldExecInstEnum::DoNotReduce),
            "G" => Ok(FieldExecInstEnum::AllOrNone),
            "I" => Ok(FieldExecInstEnum::InstitutionsOnly),
            "L" => Ok(FieldExecInstEnum::LastPeg),
            "M" => Ok(FieldExecInstEnum::MidPricePeg),
            "N" => Ok(FieldExecInstEnum::NonNegotiable),
            "O" => Ok(FieldExecInstEnum::OpeningPeg),
            "P" => Ok(FieldExecInstEnum::MarketPeg),
            "R" => Ok(FieldExecInstEnum::PrimaryPeg),
            "S" => Ok(FieldExecInstEnum::Suspend),
            "T" => Ok(FieldExecInstEnum::FixedPegToLocalBestBidOrOfferAtTimeOfOrder),
            "U" => Ok(FieldExecInstEnum::CustomerDisplayInstruction),
            "V" => Ok(FieldExecInstEnum::Netting),
            "W" => Ok(FieldExecInstEnum::PegToVwap),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldExecInstEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldExecInstEnum::StayOnOfferside => {
                f.write_str( "0" )
            },
            &FieldExecInstEnum::NotHeld => {
                f.write_str( "1" )
            },
            &FieldExecInstEnum::Work => {
                f.write_str( "2" )
            },
            &FieldExecInstEnum::GoAlong => {
                f.write_str( "3" )
            },
            &FieldExecInstEnum::OverTheDay => {
                f.write_str( "4" )
            },
            &FieldExecInstEnum::Held => {
                f.write_str( "5" )
            },
            &FieldExecInstEnum::ParticipateDontInitiate => {
                f.write_str( "6" )
            },
            &FieldExecInstEnum::StrictScale => {
                f.write_str( "7" )
            },
            &FieldExecInstEnum::TryToScale => {
                f.write_str( "8" )
            },
            &FieldExecInstEnum::StayOnBidside => {
                f.write_str( "9" )
            },
            &FieldExecInstEnum::NoCross => {
                f.write_str( "A" )
            },
            &FieldExecInstEnum::OkToCross => {
                f.write_str( "B" )
            },
            &FieldExecInstEnum::CallFirst => {
                f.write_str( "C" )
            },
            &FieldExecInstEnum::PercentOfVolume => {
                f.write_str( "D" )
            },
            &FieldExecInstEnum::DoNotIncrease => {
                f.write_str( "E" )
            },
            &FieldExecInstEnum::DoNotReduce => {
                f.write_str( "F" )
            },
            &FieldExecInstEnum::AllOrNone => {
                f.write_str( "G" )
            },
            &FieldExecInstEnum::InstitutionsOnly => {
                f.write_str( "I" )
            },
            &FieldExecInstEnum::LastPeg => {
                f.write_str( "L" )
            },
            &FieldExecInstEnum::MidPricePeg => {
                f.write_str( "M" )
            },
            &FieldExecInstEnum::NonNegotiable => {
                f.write_str( "N" )
            },
            &FieldExecInstEnum::OpeningPeg => {
                f.write_str( "O" )
            },
            &FieldExecInstEnum::MarketPeg => {
                f.write_str( "P" )
            },
            &FieldExecInstEnum::PrimaryPeg => {
                f.write_str( "R" )
            },
            &FieldExecInstEnum::Suspend => {
                f.write_str( "S" )
            },
            &FieldExecInstEnum::FixedPegToLocalBestBidOrOfferAtTimeOfOrder => {
                f.write_str( "T" )
            },
            &FieldExecInstEnum::CustomerDisplayInstruction => {
                f.write_str( "U" )
            },
            &FieldExecInstEnum::Netting => {
                f.write_str( "V" )
            },
            &FieldExecInstEnum::PegToVwap => {
                f.write_str( "W" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldExecInstEnum {
    fn default() -> Self {
        FieldExecInstEnum::Undefined
    }
}
const FIELD_EXECREFID : u32 = 19; // STRING


const FIELD_EXECTRANSTYPE : u32 = 20; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldExecTransTypeEnum {
    New, // = "0"
    Cancel, // = "1"
    Correct, // = "2"
    Status, // = "3"

    Undefined
}

impl FromStr for FieldExecTransTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldExecTransTypeEnum::New),
            "1" => Ok(FieldExecTransTypeEnum::Cancel),
            "2" => Ok(FieldExecTransTypeEnum::Correct),
            "3" => Ok(FieldExecTransTypeEnum::Status),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldExecTransTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldExecTransTypeEnum::New => {
                f.write_str( "0" )
            },
            &FieldExecTransTypeEnum::Cancel => {
                f.write_str( "1" )
            },
            &FieldExecTransTypeEnum::Correct => {
                f.write_str( "2" )
            },
            &FieldExecTransTypeEnum::Status => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldExecTransTypeEnum {
    fn default() -> Self {
        FieldExecTransTypeEnum::Undefined
    }
}
const FIELD_HANDLINST : u32 = 21; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldHandlInstEnum {
    AutomatedExecutionOrderPrivateNoBrokerIntervention, // = "1"
    AutomatedExecutionOrderPublicBrokerInterventionOk, // = "2"
    ManualOrderBestExecution, // = "3"

    Undefined
}

impl FromStr for FieldHandlInstEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldHandlInstEnum::AutomatedExecutionOrderPrivateNoBrokerIntervention),
            "2" => Ok(FieldHandlInstEnum::AutomatedExecutionOrderPublicBrokerInterventionOk),
            "3" => Ok(FieldHandlInstEnum::ManualOrderBestExecution),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldHandlInstEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldHandlInstEnum::AutomatedExecutionOrderPrivateNoBrokerIntervention => {
                f.write_str( "1" )
            },
            &FieldHandlInstEnum::AutomatedExecutionOrderPublicBrokerInterventionOk => {
                f.write_str( "2" )
            },
            &FieldHandlInstEnum::ManualOrderBestExecution => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldHandlInstEnum {
    fn default() -> Self {
        FieldHandlInstEnum::Undefined
    }
}
const FIELD_IDSOURCE : u32 = 22; // STRING


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldIDSourceEnum {
    Cusip, // = "1"
    Sedol, // = "2"
    Quik, // = "3"
    IsinNumber, // = "4"
    RicCode, // = "5"
    IsoCurrencyCode, // = "6"
    IsoCountryCode, // = "7"
    ExchangeSymbol, // = "8"
    ConsolidatedTapeAssociation, // = "9"

    Undefined
}

impl FromStr for FieldIDSourceEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldIDSourceEnum::Cusip),
            "2" => Ok(FieldIDSourceEnum::Sedol),
            "3" => Ok(FieldIDSourceEnum::Quik),
            "4" => Ok(FieldIDSourceEnum::IsinNumber),
            "5" => Ok(FieldIDSourceEnum::RicCode),
            "6" => Ok(FieldIDSourceEnum::IsoCurrencyCode),
            "7" => Ok(FieldIDSourceEnum::IsoCountryCode),
            "8" => Ok(FieldIDSourceEnum::ExchangeSymbol),
            "9" => Ok(FieldIDSourceEnum::ConsolidatedTapeAssociation),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldIDSourceEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldIDSourceEnum::Cusip => {
                f.write_str( "1" )
            },
            &FieldIDSourceEnum::Sedol => {
                f.write_str( "2" )
            },
            &FieldIDSourceEnum::Quik => {
                f.write_str( "3" )
            },
            &FieldIDSourceEnum::IsinNumber => {
                f.write_str( "4" )
            },
            &FieldIDSourceEnum::RicCode => {
                f.write_str( "5" )
            },
            &FieldIDSourceEnum::IsoCurrencyCode => {
                f.write_str( "6" )
            },
            &FieldIDSourceEnum::IsoCountryCode => {
                f.write_str( "7" )
            },
            &FieldIDSourceEnum::ExchangeSymbol => {
                f.write_str( "8" )
            },
            &FieldIDSourceEnum::ConsolidatedTapeAssociation => {
                f.write_str( "9" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldIDSourceEnum {
    fn default() -> Self {
        FieldIDSourceEnum::Undefined
    }
}
const FIELD_IOIID : u32 = 23; // STRING


const FIELD_IOIOTHSVC : u32 = 24; // CHAR


const FIELD_IOIQLTYIND : u32 = 25; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldIOIQltyIndEnum {
    High, // = "H"
    Low, // = "L"
    Medium, // = "M"

    Undefined
}

impl FromStr for FieldIOIQltyIndEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "H" => Ok(FieldIOIQltyIndEnum::High),
            "L" => Ok(FieldIOIQltyIndEnum::Low),
            "M" => Ok(FieldIOIQltyIndEnum::Medium),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldIOIQltyIndEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldIOIQltyIndEnum::High => {
                f.write_str( "H" )
            },
            &FieldIOIQltyIndEnum::Low => {
                f.write_str( "L" )
            },
            &FieldIOIQltyIndEnum::Medium => {
                f.write_str( "M" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldIOIQltyIndEnum {
    fn default() -> Self {
        FieldIOIQltyIndEnum::Undefined
    }
}
const FIELD_IOIREFID : u32 = 26; // STRING


const FIELD_IOISHARES : u32 = 27; // STRING


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldIOISharesEnum {
    Large, // = "L"
    Medium, // = "M"
    Small, // = "S"

    Undefined
}

impl FromStr for FieldIOISharesEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(FieldIOISharesEnum::Large),
            "M" => Ok(FieldIOISharesEnum::Medium),
            "S" => Ok(FieldIOISharesEnum::Small),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldIOISharesEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldIOISharesEnum::Large => {
                f.write_str( "L" )
            },
            &FieldIOISharesEnum::Medium => {
                f.write_str( "M" )
            },
            &FieldIOISharesEnum::Small => {
                f.write_str( "S" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldIOISharesEnum {
    fn default() -> Self {
        FieldIOISharesEnum::Undefined
    }
}
const FIELD_IOITRANSTYPE : u32 = 28; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldIOITransTypeEnum {
    Cancel, // = "C"
    New, // = "N"
    Replace, // = "R"

    Undefined
}

impl FromStr for FieldIOITransTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(FieldIOITransTypeEnum::Cancel),
            "N" => Ok(FieldIOITransTypeEnum::New),
            "R" => Ok(FieldIOITransTypeEnum::Replace),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldIOITransTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldIOITransTypeEnum::Cancel => {
                f.write_str( "C" )
            },
            &FieldIOITransTypeEnum::New => {
                f.write_str( "N" )
            },
            &FieldIOITransTypeEnum::Replace => {
                f.write_str( "R" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldIOITransTypeEnum {
    fn default() -> Self {
        FieldIOITransTypeEnum::Undefined
    }
}
const FIELD_LASTCAPACITY : u32 = 29; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldLastCapacityEnum {
    Agent, // = "1"
    CrossAsAgent, // = "2"
    CrossAsPrincipal, // = "3"
    Principal, // = "4"

    Undefined
}

impl FromStr for FieldLastCapacityEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldLastCapacityEnum::Agent),
            "2" => Ok(FieldLastCapacityEnum::CrossAsAgent),
            "3" => Ok(FieldLastCapacityEnum::CrossAsPrincipal),
            "4" => Ok(FieldLastCapacityEnum::Principal),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldLastCapacityEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldLastCapacityEnum::Agent => {
                f.write_str( "1" )
            },
            &FieldLastCapacityEnum::CrossAsAgent => {
                f.write_str( "2" )
            },
            &FieldLastCapacityEnum::CrossAsPrincipal => {
                f.write_str( "3" )
            },
            &FieldLastCapacityEnum::Principal => {
                f.write_str( "4" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldLastCapacityEnum {
    fn default() -> Self {
        FieldLastCapacityEnum::Undefined
    }
}
const FIELD_LASTMKT : u32 = 30; // EXCHANGE


const FIELD_LASTPX : u32 = 31; // PRICE


const FIELD_LASTSHARES : u32 = 32; // QTY


const FIELD_LINESOFTEXT : u32 = 33; // INT


const FIELD_MSGSEQNUM : u32 = 34; // INT


const FIELD_MSGTYPE : u32 = 35; // STRING


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldMsgTypeEnum {
    Heartbeat, // = "0"
    TestRequest, // = "1"
    ResendRequest, // = "2"
    Reject, // = "3"
    SequenceReset, // = "4"
    Logout, // = "5"
    IndicationOfInterest, // = "6"
    Advertisement, // = "7"
    ExecutionReport, // = "8"
    OrderCancelReject, // = "9"
    QuoteStatusRequest, // = "a"
    Logon, // = "A"
    News, // = "B"
    QuoteAcknowledgement, // = "b"
    Email, // = "C"
    SecurityDefinitionRequest, // = "c"
    OrderSingle, // = "D"
    SecurityDefinition, // = "d"
    OrderList, // = "E"
    SecurityStatusRequest, // = "e"
    SecurityStatus, // = "f"
    OrderCancelRequest, // = "F"
    OrderCancelReplaceRequest, // = "G"
    TradingSessionStatusRequest, // = "g"
    OrderStatusRequest, // = "H"
    TradingSessionStatus, // = "h"
    MassQuote, // = "i"
    BusinessMessageReject, // = "j"
    Allocation, // = "J"
    ListCancelRequest, // = "K"
    BidRequest, // = "k"
    BidResponse, // = "l"
    ListExecute, // = "L"
    ListStrikePrice, // = "m"
    ListStatusRequest, // = "M"
    ListStatus, // = "N"
    AllocationAck, // = "P"
    DontKnowTrade, // = "Q"
    QuoteRequest, // = "R"
    Quote, // = "S"
    SettlementInstructions, // = "T"
    MarketDataRequest, // = "V"
    MarketDataSnapshotFullRefresh, // = "W"
    MarketDataIncrementalRefresh, // = "X"
    MarketDataRequestReject, // = "Y"
    QuoteCancel, // = "Z"

    Undefined
}

impl FromStr for FieldMsgTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldMsgTypeEnum::Heartbeat),
            "1" => Ok(FieldMsgTypeEnum::TestRequest),
            "2" => Ok(FieldMsgTypeEnum::ResendRequest),
            "3" => Ok(FieldMsgTypeEnum::Reject),
            "4" => Ok(FieldMsgTypeEnum::SequenceReset),
            "5" => Ok(FieldMsgTypeEnum::Logout),
            "6" => Ok(FieldMsgTypeEnum::IndicationOfInterest),
            "7" => Ok(FieldMsgTypeEnum::Advertisement),
            "8" => Ok(FieldMsgTypeEnum::ExecutionReport),
            "9" => Ok(FieldMsgTypeEnum::OrderCancelReject),
            "a" => Ok(FieldMsgTypeEnum::QuoteStatusRequest),
            "A" => Ok(FieldMsgTypeEnum::Logon),
            "B" => Ok(FieldMsgTypeEnum::News),
            "b" => Ok(FieldMsgTypeEnum::QuoteAcknowledgement),
            "C" => Ok(FieldMsgTypeEnum::Email),
            "c" => Ok(FieldMsgTypeEnum::SecurityDefinitionRequest),
            "D" => Ok(FieldMsgTypeEnum::OrderSingle),
            "d" => Ok(FieldMsgTypeEnum::SecurityDefinition),
            "E" => Ok(FieldMsgTypeEnum::OrderList),
            "e" => Ok(FieldMsgTypeEnum::SecurityStatusRequest),
            "f" => Ok(FieldMsgTypeEnum::SecurityStatus),
            "F" => Ok(FieldMsgTypeEnum::OrderCancelRequest),
            "G" => Ok(FieldMsgTypeEnum::OrderCancelReplaceRequest),
            "g" => Ok(FieldMsgTypeEnum::TradingSessionStatusRequest),
            "H" => Ok(FieldMsgTypeEnum::OrderStatusRequest),
            "h" => Ok(FieldMsgTypeEnum::TradingSessionStatus),
            "i" => Ok(FieldMsgTypeEnum::MassQuote),
            "j" => Ok(FieldMsgTypeEnum::BusinessMessageReject),
            "J" => Ok(FieldMsgTypeEnum::Allocation),
            "K" => Ok(FieldMsgTypeEnum::ListCancelRequest),
            "k" => Ok(FieldMsgTypeEnum::BidRequest),
            "l" => Ok(FieldMsgTypeEnum::BidResponse),
            "L" => Ok(FieldMsgTypeEnum::ListExecute),
            "m" => Ok(FieldMsgTypeEnum::ListStrikePrice),
            "M" => Ok(FieldMsgTypeEnum::ListStatusRequest),
            "N" => Ok(FieldMsgTypeEnum::ListStatus),
            "P" => Ok(FieldMsgTypeEnum::AllocationAck),
            "Q" => Ok(FieldMsgTypeEnum::DontKnowTrade),
            "R" => Ok(FieldMsgTypeEnum::QuoteRequest),
            "S" => Ok(FieldMsgTypeEnum::Quote),
            "T" => Ok(FieldMsgTypeEnum::SettlementInstructions),
            "V" => Ok(FieldMsgTypeEnum::MarketDataRequest),
            "W" => Ok(FieldMsgTypeEnum::MarketDataSnapshotFullRefresh),
            "X" => Ok(FieldMsgTypeEnum::MarketDataIncrementalRefresh),
            "Y" => Ok(FieldMsgTypeEnum::MarketDataRequestReject),
            "Z" => Ok(FieldMsgTypeEnum::QuoteCancel),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldMsgTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldMsgTypeEnum::Heartbeat => {
                f.write_str( "0" )
            },
            &FieldMsgTypeEnum::TestRequest => {
                f.write_str( "1" )
            },
            &FieldMsgTypeEnum::ResendRequest => {
                f.write_str( "2" )
            },
            &FieldMsgTypeEnum::Reject => {
                f.write_str( "3" )
            },
            &FieldMsgTypeEnum::SequenceReset => {
                f.write_str( "4" )
            },
            &FieldMsgTypeEnum::Logout => {
                f.write_str( "5" )
            },
            &FieldMsgTypeEnum::IndicationOfInterest => {
                f.write_str( "6" )
            },
            &FieldMsgTypeEnum::Advertisement => {
                f.write_str( "7" )
            },
            &FieldMsgTypeEnum::ExecutionReport => {
                f.write_str( "8" )
            },
            &FieldMsgTypeEnum::OrderCancelReject => {
                f.write_str( "9" )
            },
            &FieldMsgTypeEnum::QuoteStatusRequest => {
                f.write_str( "a" )
            },
            &FieldMsgTypeEnum::Logon => {
                f.write_str( "A" )
            },
            &FieldMsgTypeEnum::News => {
                f.write_str( "B" )
            },
            &FieldMsgTypeEnum::QuoteAcknowledgement => {
                f.write_str( "b" )
            },
            &FieldMsgTypeEnum::Email => {
                f.write_str( "C" )
            },
            &FieldMsgTypeEnum::SecurityDefinitionRequest => {
                f.write_str( "c" )
            },
            &FieldMsgTypeEnum::OrderSingle => {
                f.write_str( "D" )
            },
            &FieldMsgTypeEnum::SecurityDefinition => {
                f.write_str( "d" )
            },
            &FieldMsgTypeEnum::OrderList => {
                f.write_str( "E" )
            },
            &FieldMsgTypeEnum::SecurityStatusRequest => {
                f.write_str( "e" )
            },
            &FieldMsgTypeEnum::SecurityStatus => {
                f.write_str( "f" )
            },
            &FieldMsgTypeEnum::OrderCancelRequest => {
                f.write_str( "F" )
            },
            &FieldMsgTypeEnum::OrderCancelReplaceRequest => {
                f.write_str( "G" )
            },
            &FieldMsgTypeEnum::TradingSessionStatusRequest => {
                f.write_str( "g" )
            },
            &FieldMsgTypeEnum::OrderStatusRequest => {
                f.write_str( "H" )
            },
            &FieldMsgTypeEnum::TradingSessionStatus => {
                f.write_str( "h" )
            },
            &FieldMsgTypeEnum::MassQuote => {
                f.write_str( "i" )
            },
            &FieldMsgTypeEnum::BusinessMessageReject => {
                f.write_str( "j" )
            },
            &FieldMsgTypeEnum::Allocation => {
                f.write_str( "J" )
            },
            &FieldMsgTypeEnum::ListCancelRequest => {
                f.write_str( "K" )
            },
            &FieldMsgTypeEnum::BidRequest => {
                f.write_str( "k" )
            },
            &FieldMsgTypeEnum::BidResponse => {
                f.write_str( "l" )
            },
            &FieldMsgTypeEnum::ListExecute => {
                f.write_str( "L" )
            },
            &FieldMsgTypeEnum::ListStrikePrice => {
                f.write_str( "m" )
            },
            &FieldMsgTypeEnum::ListStatusRequest => {
                f.write_str( "M" )
            },
            &FieldMsgTypeEnum::ListStatus => {
                f.write_str( "N" )
            },
            &FieldMsgTypeEnum::AllocationAck => {
                f.write_str( "P" )
            },
            &FieldMsgTypeEnum::DontKnowTrade => {
                f.write_str( "Q" )
            },
            &FieldMsgTypeEnum::QuoteRequest => {
                f.write_str( "R" )
            },
            &FieldMsgTypeEnum::Quote => {
                f.write_str( "S" )
            },
            &FieldMsgTypeEnum::SettlementInstructions => {
                f.write_str( "T" )
            },
            &FieldMsgTypeEnum::MarketDataRequest => {
                f.write_str( "V" )
            },
            &FieldMsgTypeEnum::MarketDataSnapshotFullRefresh => {
                f.write_str( "W" )
            },
            &FieldMsgTypeEnum::MarketDataIncrementalRefresh => {
                f.write_str( "X" )
            },
            &FieldMsgTypeEnum::MarketDataRequestReject => {
                f.write_str( "Y" )
            },
            &FieldMsgTypeEnum::QuoteCancel => {
                f.write_str( "Z" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldMsgTypeEnum {
    fn default() -> Self {
        FieldMsgTypeEnum::Undefined
    }
}
const FIELD_NEWSEQNO : u32 = 36; // INT


const FIELD_ORDERID : u32 = 37; // STRING


const FIELD_ORDERQTY : u32 = 38; // QTY


const FIELD_ORDSTATUS : u32 = 39; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldOrdStatusEnum {
    New, // = "0"
    PartiallyFilled, // = "1"
    Filled, // = "2"
    DoneForDay, // = "3"
    Canceled, // = "4"
    Replaced, // = "5"
    PendingCancel, // = "6"
    Stopped, // = "7"
    Rejected, // = "8"
    Suspended, // = "9"
    PendingNew, // = "A"
    Calculated, // = "B"
    Expired, // = "C"
    AcceptedForBidding, // = "D"
    PendingReplace, // = "E"

    Undefined
}

impl FromStr for FieldOrdStatusEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldOrdStatusEnum::New),
            "1" => Ok(FieldOrdStatusEnum::PartiallyFilled),
            "2" => Ok(FieldOrdStatusEnum::Filled),
            "3" => Ok(FieldOrdStatusEnum::DoneForDay),
            "4" => Ok(FieldOrdStatusEnum::Canceled),
            "5" => Ok(FieldOrdStatusEnum::Replaced),
            "6" => Ok(FieldOrdStatusEnum::PendingCancel),
            "7" => Ok(FieldOrdStatusEnum::Stopped),
            "8" => Ok(FieldOrdStatusEnum::Rejected),
            "9" => Ok(FieldOrdStatusEnum::Suspended),
            "A" => Ok(FieldOrdStatusEnum::PendingNew),
            "B" => Ok(FieldOrdStatusEnum::Calculated),
            "C" => Ok(FieldOrdStatusEnum::Expired),
            "D" => Ok(FieldOrdStatusEnum::AcceptedForBidding),
            "E" => Ok(FieldOrdStatusEnum::PendingReplace),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldOrdStatusEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldOrdStatusEnum::New => {
                f.write_str( "0" )
            },
            &FieldOrdStatusEnum::PartiallyFilled => {
                f.write_str( "1" )
            },
            &FieldOrdStatusEnum::Filled => {
                f.write_str( "2" )
            },
            &FieldOrdStatusEnum::DoneForDay => {
                f.write_str( "3" )
            },
            &FieldOrdStatusEnum::Canceled => {
                f.write_str( "4" )
            },
            &FieldOrdStatusEnum::Replaced => {
                f.write_str( "5" )
            },
            &FieldOrdStatusEnum::PendingCancel => {
                f.write_str( "6" )
            },
            &FieldOrdStatusEnum::Stopped => {
                f.write_str( "7" )
            },
            &FieldOrdStatusEnum::Rejected => {
                f.write_str( "8" )
            },
            &FieldOrdStatusEnum::Suspended => {
                f.write_str( "9" )
            },
            &FieldOrdStatusEnum::PendingNew => {
                f.write_str( "A" )
            },
            &FieldOrdStatusEnum::Calculated => {
                f.write_str( "B" )
            },
            &FieldOrdStatusEnum::Expired => {
                f.write_str( "C" )
            },
            &FieldOrdStatusEnum::AcceptedForBidding => {
                f.write_str( "D" )
            },
            &FieldOrdStatusEnum::PendingReplace => {
                f.write_str( "E" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldOrdStatusEnum {
    fn default() -> Self {
        FieldOrdStatusEnum::Undefined
    }
}
const FIELD_ORDTYPE : u32 = 40; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldOrdTypeEnum {
    Market, // = "1"
    Limit, // = "2"
    Stop, // = "3"
    StopLimit, // = "4"
    MarketOnClose, // = "5"
    WithOrWithout, // = "6"
    LimitOrBetter, // = "7"
    LimitWithOrWithout, // = "8"
    OnBasis, // = "9"
    OnClose, // = "A"
    LimitOnClose, // = "B"
    ForexC, // = "C"
    PreviouslyQuoted, // = "D"
    PreviouslyIndicated, // = "E"
    ForexF, // = "F"
    ForexG, // = "G"
    ForexH, // = "H"
    Funari, // = "I"
    Pegged, // = "P"

    Undefined
}

impl FromStr for FieldOrdTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldOrdTypeEnum::Market),
            "2" => Ok(FieldOrdTypeEnum::Limit),
            "3" => Ok(FieldOrdTypeEnum::Stop),
            "4" => Ok(FieldOrdTypeEnum::StopLimit),
            "5" => Ok(FieldOrdTypeEnum::MarketOnClose),
            "6" => Ok(FieldOrdTypeEnum::WithOrWithout),
            "7" => Ok(FieldOrdTypeEnum::LimitOrBetter),
            "8" => Ok(FieldOrdTypeEnum::LimitWithOrWithout),
            "9" => Ok(FieldOrdTypeEnum::OnBasis),
            "A" => Ok(FieldOrdTypeEnum::OnClose),
            "B" => Ok(FieldOrdTypeEnum::LimitOnClose),
            "C" => Ok(FieldOrdTypeEnum::ForexC),
            "D" => Ok(FieldOrdTypeEnum::PreviouslyQuoted),
            "E" => Ok(FieldOrdTypeEnum::PreviouslyIndicated),
            "F" => Ok(FieldOrdTypeEnum::ForexF),
            "G" => Ok(FieldOrdTypeEnum::ForexG),
            "H" => Ok(FieldOrdTypeEnum::ForexH),
            "I" => Ok(FieldOrdTypeEnum::Funari),
            "P" => Ok(FieldOrdTypeEnum::Pegged),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldOrdTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldOrdTypeEnum::Market => {
                f.write_str( "1" )
            },
            &FieldOrdTypeEnum::Limit => {
                f.write_str( "2" )
            },
            &FieldOrdTypeEnum::Stop => {
                f.write_str( "3" )
            },
            &FieldOrdTypeEnum::StopLimit => {
                f.write_str( "4" )
            },
            &FieldOrdTypeEnum::MarketOnClose => {
                f.write_str( "5" )
            },
            &FieldOrdTypeEnum::WithOrWithout => {
                f.write_str( "6" )
            },
            &FieldOrdTypeEnum::LimitOrBetter => {
                f.write_str( "7" )
            },
            &FieldOrdTypeEnum::LimitWithOrWithout => {
                f.write_str( "8" )
            },
            &FieldOrdTypeEnum::OnBasis => {
                f.write_str( "9" )
            },
            &FieldOrdTypeEnum::OnClose => {
                f.write_str( "A" )
            },
            &FieldOrdTypeEnum::LimitOnClose => {
                f.write_str( "B" )
            },
            &FieldOrdTypeEnum::ForexC => {
                f.write_str( "C" )
            },
            &FieldOrdTypeEnum::PreviouslyQuoted => {
                f.write_str( "D" )
            },
            &FieldOrdTypeEnum::PreviouslyIndicated => {
                f.write_str( "E" )
            },
            &FieldOrdTypeEnum::ForexF => {
                f.write_str( "F" )
            },
            &FieldOrdTypeEnum::ForexG => {
                f.write_str( "G" )
            },
            &FieldOrdTypeEnum::ForexH => {
                f.write_str( "H" )
            },
            &FieldOrdTypeEnum::Funari => {
                f.write_str( "I" )
            },
            &FieldOrdTypeEnum::Pegged => {
                f.write_str( "P" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldOrdTypeEnum {
    fn default() -> Self {
        FieldOrdTypeEnum::Undefined
    }
}
const FIELD_ORIGCLORDID : u32 = 41; // STRING


const FIELD_ORIGTIME : u32 = 42; // UTCTIMESTAMP


const FIELD_POSSDUPFLAG : u32 = 43; // BOOLEAN


const FIELD_PRICE : u32 = 44; // PRICE


const FIELD_REFSEQNUM : u32 = 45; // INT


const FIELD_RELATDSYM : u32 = 46; // STRING


const FIELD_RULE80A : u32 = 47; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldRule80AEnum {
    AgencySingleOrder, // = "A"
    ShortExemptTransactionB, // = "B"
    ProgramOrderNonIndexArbForMemberFirmOrg, // = "C"
    ProgramOrderIndexArbForMemberFirmOrg, // = "D"
    RegisteredEquityMarketMakerTrades, // = "E"
    ShortExemptTransactionF, // = "F"
    ShortExemptTransactionH, // = "H"
    IndividualInvestorSingleOrder, // = "I"
    ProgramOrderIndexArbForIndividualCustomer, // = "J"
    ProgramOrderNonIndexArbForIndividualCustomer, // = "K"
    ShortExemptTransactionForMemberCompetingMarketMakerAffiliatedWithTheFirmClearingTheTrade, // = "L"
    ProgramOrderIndexArbForOtherMember, // = "M"
    ProgramOrderNonIndexArbForOtherMember, // = "N"
    CompetingDealerTradesO, // = "O"
    Principal, // = "P"
    CompetingDealerTradesR, // = "R"
    SpecialistTrades, // = "S"
    CompetingDealerTradesT, // = "T"
    ProgramOrderIndexArbForOtherAgency, // = "U"
    AllOtherOrdersAsAgentForOtherMember, // = "W"
    ShortExemptTransactionForMemberCompetingMarketMakerNotAffiliatedWithTheFirmClearingTheTrade, // = "X"
    ProgramOrderNonIndexArbForOtherAgency, // = "Y"
    ShortExemptTransactionForNonMemberCompetingMarketMaker, // = "Z"

    Undefined
}

impl FromStr for FieldRule80AEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(FieldRule80AEnum::AgencySingleOrder),
            "B" => Ok(FieldRule80AEnum::ShortExemptTransactionB),
            "C" => Ok(FieldRule80AEnum::ProgramOrderNonIndexArbForMemberFirmOrg),
            "D" => Ok(FieldRule80AEnum::ProgramOrderIndexArbForMemberFirmOrg),
            "E" => Ok(FieldRule80AEnum::RegisteredEquityMarketMakerTrades),
            "F" => Ok(FieldRule80AEnum::ShortExemptTransactionF),
            "H" => Ok(FieldRule80AEnum::ShortExemptTransactionH),
            "I" => Ok(FieldRule80AEnum::IndividualInvestorSingleOrder),
            "J" => Ok(FieldRule80AEnum::ProgramOrderIndexArbForIndividualCustomer),
            "K" => Ok(FieldRule80AEnum::ProgramOrderNonIndexArbForIndividualCustomer),
            "L" => Ok(FieldRule80AEnum::ShortExemptTransactionForMemberCompetingMarketMakerAffiliatedWithTheFirmClearingTheTrade),
            "M" => Ok(FieldRule80AEnum::ProgramOrderIndexArbForOtherMember),
            "N" => Ok(FieldRule80AEnum::ProgramOrderNonIndexArbForOtherMember),
            "O" => Ok(FieldRule80AEnum::CompetingDealerTradesO),
            "P" => Ok(FieldRule80AEnum::Principal),
            "R" => Ok(FieldRule80AEnum::CompetingDealerTradesR),
            "S" => Ok(FieldRule80AEnum::SpecialistTrades),
            "T" => Ok(FieldRule80AEnum::CompetingDealerTradesT),
            "U" => Ok(FieldRule80AEnum::ProgramOrderIndexArbForOtherAgency),
            "W" => Ok(FieldRule80AEnum::AllOtherOrdersAsAgentForOtherMember),
            "X" => Ok(FieldRule80AEnum::ShortExemptTransactionForMemberCompetingMarketMakerNotAffiliatedWithTheFirmClearingTheTrade),
            "Y" => Ok(FieldRule80AEnum::ProgramOrderNonIndexArbForOtherAgency),
            "Z" => Ok(FieldRule80AEnum::ShortExemptTransactionForNonMemberCompetingMarketMaker),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldRule80AEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldRule80AEnum::AgencySingleOrder => {
                f.write_str( "A" )
            },
            &FieldRule80AEnum::ShortExemptTransactionB => {
                f.write_str( "B" )
            },
            &FieldRule80AEnum::ProgramOrderNonIndexArbForMemberFirmOrg => {
                f.write_str( "C" )
            },
            &FieldRule80AEnum::ProgramOrderIndexArbForMemberFirmOrg => {
                f.write_str( "D" )
            },
            &FieldRule80AEnum::RegisteredEquityMarketMakerTrades => {
                f.write_str( "E" )
            },
            &FieldRule80AEnum::ShortExemptTransactionF => {
                f.write_str( "F" )
            },
            &FieldRule80AEnum::ShortExemptTransactionH => {
                f.write_str( "H" )
            },
            &FieldRule80AEnum::IndividualInvestorSingleOrder => {
                f.write_str( "I" )
            },
            &FieldRule80AEnum::ProgramOrderIndexArbForIndividualCustomer => {
                f.write_str( "J" )
            },
            &FieldRule80AEnum::ProgramOrderNonIndexArbForIndividualCustomer => {
                f.write_str( "K" )
            },
            &FieldRule80AEnum::ShortExemptTransactionForMemberCompetingMarketMakerAffiliatedWithTheFirmClearingTheTrade => {
                f.write_str( "L" )
            },
            &FieldRule80AEnum::ProgramOrderIndexArbForOtherMember => {
                f.write_str( "M" )
            },
            &FieldRule80AEnum::ProgramOrderNonIndexArbForOtherMember => {
                f.write_str( "N" )
            },
            &FieldRule80AEnum::CompetingDealerTradesO => {
                f.write_str( "O" )
            },
            &FieldRule80AEnum::Principal => {
                f.write_str( "P" )
            },
            &FieldRule80AEnum::CompetingDealerTradesR => {
                f.write_str( "R" )
            },
            &FieldRule80AEnum::SpecialistTrades => {
                f.write_str( "S" )
            },
            &FieldRule80AEnum::CompetingDealerTradesT => {
                f.write_str( "T" )
            },
            &FieldRule80AEnum::ProgramOrderIndexArbForOtherAgency => {
                f.write_str( "U" )
            },
            &FieldRule80AEnum::AllOtherOrdersAsAgentForOtherMember => {
                f.write_str( "W" )
            },
            &FieldRule80AEnum::ShortExemptTransactionForMemberCompetingMarketMakerNotAffiliatedWithTheFirmClearingTheTrade => {
                f.write_str( "X" )
            },
            &FieldRule80AEnum::ProgramOrderNonIndexArbForOtherAgency => {
                f.write_str( "Y" )
            },
            &FieldRule80AEnum::ShortExemptTransactionForNonMemberCompetingMarketMaker => {
                f.write_str( "Z" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldRule80AEnum {
    fn default() -> Self {
        FieldRule80AEnum::Undefined
    }
}
const FIELD_SECURITYID : u32 = 48; // STRING


const FIELD_SENDERCOMPID : u32 = 49; // STRING


const FIELD_SENDERSUBID : u32 = 50; // STRING


const FIELD_SENDINGDATE : u32 = 51; // LOCALMKTDATE


const FIELD_SENDINGTIME : u32 = 52; // UTCTIMESTAMP


const FIELD_SHARES : u32 = 53; // QTY


const FIELD_SIDE : u32 = 54; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldSideEnum {
    Buy, // = "1"
    Sell, // = "2"
    BuyMinus, // = "3"
    SellPlus, // = "4"
    SellShort, // = "5"
    SellShortExempt, // = "6"
    Undisclosed, // = "7"
    Cross, // = "8"
    CrossShort, // = "9"

    Undefined
}

impl FromStr for FieldSideEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldSideEnum::Buy),
            "2" => Ok(FieldSideEnum::Sell),
            "3" => Ok(FieldSideEnum::BuyMinus),
            "4" => Ok(FieldSideEnum::SellPlus),
            "5" => Ok(FieldSideEnum::SellShort),
            "6" => Ok(FieldSideEnum::SellShortExempt),
            "7" => Ok(FieldSideEnum::Undisclosed),
            "8" => Ok(FieldSideEnum::Cross),
            "9" => Ok(FieldSideEnum::CrossShort),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldSideEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldSideEnum::Buy => {
                f.write_str( "1" )
            },
            &FieldSideEnum::Sell => {
                f.write_str( "2" )
            },
            &FieldSideEnum::BuyMinus => {
                f.write_str( "3" )
            },
            &FieldSideEnum::SellPlus => {
                f.write_str( "4" )
            },
            &FieldSideEnum::SellShort => {
                f.write_str( "5" )
            },
            &FieldSideEnum::SellShortExempt => {
                f.write_str( "6" )
            },
            &FieldSideEnum::Undisclosed => {
                f.write_str( "7" )
            },
            &FieldSideEnum::Cross => {
                f.write_str( "8" )
            },
            &FieldSideEnum::CrossShort => {
                f.write_str( "9" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldSideEnum {
    fn default() -> Self {
        FieldSideEnum::Undefined
    }
}
const FIELD_SYMBOL : u32 = 55; // STRING


const FIELD_TARGETCOMPID : u32 = 56; // STRING


const FIELD_TARGETSUBID : u32 = 57; // STRING


const FIELD_TEXT : u32 = 58; // STRING


const FIELD_TIMEINFORCE : u32 = 59; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldTimeInForceEnum {
    Day, // = "0"
    GoodTillCancel, // = "1"
    AtTheOpening, // = "2"
    ImmediateOrCancel, // = "3"
    FillOrKill, // = "4"
    GoodTillCrossing, // = "5"
    GoodTillDate, // = "6"

    Undefined
}

impl FromStr for FieldTimeInForceEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldTimeInForceEnum::Day),
            "1" => Ok(FieldTimeInForceEnum::GoodTillCancel),
            "2" => Ok(FieldTimeInForceEnum::AtTheOpening),
            "3" => Ok(FieldTimeInForceEnum::ImmediateOrCancel),
            "4" => Ok(FieldTimeInForceEnum::FillOrKill),
            "5" => Ok(FieldTimeInForceEnum::GoodTillCrossing),
            "6" => Ok(FieldTimeInForceEnum::GoodTillDate),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldTimeInForceEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldTimeInForceEnum::Day => {
                f.write_str( "0" )
            },
            &FieldTimeInForceEnum::GoodTillCancel => {
                f.write_str( "1" )
            },
            &FieldTimeInForceEnum::AtTheOpening => {
                f.write_str( "2" )
            },
            &FieldTimeInForceEnum::ImmediateOrCancel => {
                f.write_str( "3" )
            },
            &FieldTimeInForceEnum::FillOrKill => {
                f.write_str( "4" )
            },
            &FieldTimeInForceEnum::GoodTillCrossing => {
                f.write_str( "5" )
            },
            &FieldTimeInForceEnum::GoodTillDate => {
                f.write_str( "6" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldTimeInForceEnum {
    fn default() -> Self {
        FieldTimeInForceEnum::Undefined
    }
}
const FIELD_TRANSACTTIME : u32 = 60; // UTCTIMESTAMP


const FIELD_URGENCY : u32 = 61; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldUrgencyEnum {
    Normal, // = "0"
    Flash, // = "1"
    Background, // = "2"

    Undefined
}

impl FromStr for FieldUrgencyEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldUrgencyEnum::Normal),
            "1" => Ok(FieldUrgencyEnum::Flash),
            "2" => Ok(FieldUrgencyEnum::Background),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldUrgencyEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldUrgencyEnum::Normal => {
                f.write_str( "0" )
            },
            &FieldUrgencyEnum::Flash => {
                f.write_str( "1" )
            },
            &FieldUrgencyEnum::Background => {
                f.write_str( "2" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldUrgencyEnum {
    fn default() -> Self {
        FieldUrgencyEnum::Undefined
    }
}
const FIELD_VALIDUNTILTIME : u32 = 62; // UTCTIMESTAMP


const FIELD_SETTLMNTTYP : u32 = 63; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldSettlmntTypEnum {
    Regular, // = "0"
    Cash, // = "1"
    NextDay, // = "2"
    TPlus2, // = "3"
    TPlus3, // = "4"
    TPlus4, // = "5"
    Future, // = "6"
    WhenIssued, // = "7"
    SellersOption, // = "8"
    TPlus5, // = "9"

    Undefined
}

impl FromStr for FieldSettlmntTypEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldSettlmntTypEnum::Regular),
            "1" => Ok(FieldSettlmntTypEnum::Cash),
            "2" => Ok(FieldSettlmntTypEnum::NextDay),
            "3" => Ok(FieldSettlmntTypEnum::TPlus2),
            "4" => Ok(FieldSettlmntTypEnum::TPlus3),
            "5" => Ok(FieldSettlmntTypEnum::TPlus4),
            "6" => Ok(FieldSettlmntTypEnum::Future),
            "7" => Ok(FieldSettlmntTypEnum::WhenIssued),
            "8" => Ok(FieldSettlmntTypEnum::SellersOption),
            "9" => Ok(FieldSettlmntTypEnum::TPlus5),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldSettlmntTypEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldSettlmntTypEnum::Regular => {
                f.write_str( "0" )
            },
            &FieldSettlmntTypEnum::Cash => {
                f.write_str( "1" )
            },
            &FieldSettlmntTypEnum::NextDay => {
                f.write_str( "2" )
            },
            &FieldSettlmntTypEnum::TPlus2 => {
                f.write_str( "3" )
            },
            &FieldSettlmntTypEnum::TPlus3 => {
                f.write_str( "4" )
            },
            &FieldSettlmntTypEnum::TPlus4 => {
                f.write_str( "5" )
            },
            &FieldSettlmntTypEnum::Future => {
                f.write_str( "6" )
            },
            &FieldSettlmntTypEnum::WhenIssued => {
                f.write_str( "7" )
            },
            &FieldSettlmntTypEnum::SellersOption => {
                f.write_str( "8" )
            },
            &FieldSettlmntTypEnum::TPlus5 => {
                f.write_str( "9" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldSettlmntTypEnum {
    fn default() -> Self {
        FieldSettlmntTypEnum::Undefined
    }
}
const FIELD_FUTSETTDATE : u32 = 64; // LOCALMKTDATE


const FIELD_SYMBOLSFX : u32 = 65; // STRING


const FIELD_LISTID : u32 = 66; // STRING


const FIELD_LISTSEQNO : u32 = 67; // INT


const FIELD_TOTNOORDERS : u32 = 68; // INT


const FIELD_LISTEXECINST : u32 = 69; // STRING


const FIELD_ALLOCID : u32 = 70; // STRING


const FIELD_ALLOCTRANSTYPE : u32 = 71; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldAllocTransTypeEnum {
    New, // = "0"
    Replace, // = "1"
    Cancel, // = "2"
    Preliminary, // = "3"
    Calculated, // = "4"
    CalculatedWithoutPreliminary, // = "5"

    Undefined
}

impl FromStr for FieldAllocTransTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldAllocTransTypeEnum::New),
            "1" => Ok(FieldAllocTransTypeEnum::Replace),
            "2" => Ok(FieldAllocTransTypeEnum::Cancel),
            "3" => Ok(FieldAllocTransTypeEnum::Preliminary),
            "4" => Ok(FieldAllocTransTypeEnum::Calculated),
            "5" => Ok(FieldAllocTransTypeEnum::CalculatedWithoutPreliminary),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldAllocTransTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldAllocTransTypeEnum::New => {
                f.write_str( "0" )
            },
            &FieldAllocTransTypeEnum::Replace => {
                f.write_str( "1" )
            },
            &FieldAllocTransTypeEnum::Cancel => {
                f.write_str( "2" )
            },
            &FieldAllocTransTypeEnum::Preliminary => {
                f.write_str( "3" )
            },
            &FieldAllocTransTypeEnum::Calculated => {
                f.write_str( "4" )
            },
            &FieldAllocTransTypeEnum::CalculatedWithoutPreliminary => {
                f.write_str( "5" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldAllocTransTypeEnum {
    fn default() -> Self {
        FieldAllocTransTypeEnum::Undefined
    }
}
const FIELD_REFALLOCID : u32 = 72; // STRING


const FIELD_NOORDERS : u32 = 73; // INT


const FIELD_AVGPRXPRECISION : u32 = 74; // INT


const FIELD_TRADEDATE : u32 = 75; // LOCALMKTDATE


const FIELD_EXECBROKER : u32 = 76; // STRING


const FIELD_OPENCLOSE : u32 = 77; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldOpenCloseEnum {
    Close, // = "C"
    Open, // = "O"

    Undefined
}

impl FromStr for FieldOpenCloseEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(FieldOpenCloseEnum::Close),
            "O" => Ok(FieldOpenCloseEnum::Open),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldOpenCloseEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldOpenCloseEnum::Close => {
                f.write_str( "C" )
            },
            &FieldOpenCloseEnum::Open => {
                f.write_str( "O" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldOpenCloseEnum {
    fn default() -> Self {
        FieldOpenCloseEnum::Undefined
    }
}
const FIELD_NOALLOCS : u32 = 78; // INT


const FIELD_ALLOCACCOUNT : u32 = 79; // STRING


const FIELD_ALLOCSHARES : u32 = 80; // QTY


const FIELD_PROCESSCODE : u32 = 81; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldProcessCodeEnum {
    Regular, // = "0"
    SoftDollar, // = "1"
    StepIn, // = "2"
    StepOut, // = "3"
    SoftDollarStepIn, // = "4"
    SoftDollarStepOut, // = "5"
    PlanSponsor, // = "6"

    Undefined
}

impl FromStr for FieldProcessCodeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldProcessCodeEnum::Regular),
            "1" => Ok(FieldProcessCodeEnum::SoftDollar),
            "2" => Ok(FieldProcessCodeEnum::StepIn),
            "3" => Ok(FieldProcessCodeEnum::StepOut),
            "4" => Ok(FieldProcessCodeEnum::SoftDollarStepIn),
            "5" => Ok(FieldProcessCodeEnum::SoftDollarStepOut),
            "6" => Ok(FieldProcessCodeEnum::PlanSponsor),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldProcessCodeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldProcessCodeEnum::Regular => {
                f.write_str( "0" )
            },
            &FieldProcessCodeEnum::SoftDollar => {
                f.write_str( "1" )
            },
            &FieldProcessCodeEnum::StepIn => {
                f.write_str( "2" )
            },
            &FieldProcessCodeEnum::StepOut => {
                f.write_str( "3" )
            },
            &FieldProcessCodeEnum::SoftDollarStepIn => {
                f.write_str( "4" )
            },
            &FieldProcessCodeEnum::SoftDollarStepOut => {
                f.write_str( "5" )
            },
            &FieldProcessCodeEnum::PlanSponsor => {
                f.write_str( "6" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldProcessCodeEnum {
    fn default() -> Self {
        FieldProcessCodeEnum::Undefined
    }
}
const FIELD_NORPTS : u32 = 82; // INT


const FIELD_RPTSEQ : u32 = 83; // INT


const FIELD_CXLQTY : u32 = 84; // QTY


const FIELD_NODLVYINST : u32 = 85; // INT


const FIELD_DLVYINST : u32 = 86; // STRING


const FIELD_ALLOCSTATUS : u32 = 87; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldAllocStatusEnum {
    Accepted, // = "0"
    Rejected, // = "1"
    PartialAccept, // = "2"
    Received, // = "3"

    Undefined
}

impl FromStr for FieldAllocStatusEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldAllocStatusEnum::Accepted),
            "1" => Ok(FieldAllocStatusEnum::Rejected),
            "2" => Ok(FieldAllocStatusEnum::PartialAccept),
            "3" => Ok(FieldAllocStatusEnum::Received),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldAllocStatusEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldAllocStatusEnum::Accepted => {
                f.write_str( "0" )
            },
            &FieldAllocStatusEnum::Rejected => {
                f.write_str( "1" )
            },
            &FieldAllocStatusEnum::PartialAccept => {
                f.write_str( "2" )
            },
            &FieldAllocStatusEnum::Received => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldAllocStatusEnum {
    fn default() -> Self {
        FieldAllocStatusEnum::Undefined
    }
}
const FIELD_ALLOCREJCODE : u32 = 88; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldAllocRejCodeEnum {
    UnknownAccount, // = "0"
    IncorrectQuantity, // = "1"
    IncorrectAveragePrice, // = "2"
    UnknownExecutingBrokerMnemonic, // = "3"
    CommissionDifference, // = "4"
    UnknownOrderid, // = "5"
    UnknownListid, // = "6"
    Other, // = "7"

    Undefined
}

impl FromStr for FieldAllocRejCodeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldAllocRejCodeEnum::UnknownAccount),
            "1" => Ok(FieldAllocRejCodeEnum::IncorrectQuantity),
            "2" => Ok(FieldAllocRejCodeEnum::IncorrectAveragePrice),
            "3" => Ok(FieldAllocRejCodeEnum::UnknownExecutingBrokerMnemonic),
            "4" => Ok(FieldAllocRejCodeEnum::CommissionDifference),
            "5" => Ok(FieldAllocRejCodeEnum::UnknownOrderid),
            "6" => Ok(FieldAllocRejCodeEnum::UnknownListid),
            "7" => Ok(FieldAllocRejCodeEnum::Other),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldAllocRejCodeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldAllocRejCodeEnum::UnknownAccount => {
                f.write_str( "0" )
            },
            &FieldAllocRejCodeEnum::IncorrectQuantity => {
                f.write_str( "1" )
            },
            &FieldAllocRejCodeEnum::IncorrectAveragePrice => {
                f.write_str( "2" )
            },
            &FieldAllocRejCodeEnum::UnknownExecutingBrokerMnemonic => {
                f.write_str( "3" )
            },
            &FieldAllocRejCodeEnum::CommissionDifference => {
                f.write_str( "4" )
            },
            &FieldAllocRejCodeEnum::UnknownOrderid => {
                f.write_str( "5" )
            },
            &FieldAllocRejCodeEnum::UnknownListid => {
                f.write_str( "6" )
            },
            &FieldAllocRejCodeEnum::Other => {
                f.write_str( "7" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldAllocRejCodeEnum {
    fn default() -> Self {
        FieldAllocRejCodeEnum::Undefined
    }
}
const FIELD_SIGNATURE : u32 = 89; // DATA


const FIELD_SECUREDATALEN : u32 = 90; // LENGTH


const FIELD_SECUREDATA : u32 = 91; // DATA


const FIELD_BROKEROFCREDIT : u32 = 92; // STRING


const FIELD_SIGNATURELENGTH : u32 = 93; // LENGTH


const FIELD_EMAILTYPE : u32 = 94; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldEmailTypeEnum {
    New, // = "0"
    Reply, // = "1"
    AdminReply, // = "2"

    Undefined
}

impl FromStr for FieldEmailTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldEmailTypeEnum::New),
            "1" => Ok(FieldEmailTypeEnum::Reply),
            "2" => Ok(FieldEmailTypeEnum::AdminReply),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldEmailTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldEmailTypeEnum::New => {
                f.write_str( "0" )
            },
            &FieldEmailTypeEnum::Reply => {
                f.write_str( "1" )
            },
            &FieldEmailTypeEnum::AdminReply => {
                f.write_str( "2" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldEmailTypeEnum {
    fn default() -> Self {
        FieldEmailTypeEnum::Undefined
    }
}
const FIELD_RAWDATALENGTH : u32 = 95; // LENGTH


const FIELD_RAWDATA : u32 = 96; // DATA


const FIELD_POSSRESEND : u32 = 97; // BOOLEAN


const FIELD_ENCRYPTMETHOD : u32 = 98; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldEncryptMethodEnum {
    None, // = "0"
    Pkcs, // = "1"
    Des, // = "2"
    PkcsDes, // = "3"
    PgpDes, // = "4"
    PgpDesMd5, // = "5"
    PemDesMd5, // = "6"

    Undefined
}

impl FromStr for FieldEncryptMethodEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldEncryptMethodEnum::None),
            "1" => Ok(FieldEncryptMethodEnum::Pkcs),
            "2" => Ok(FieldEncryptMethodEnum::Des),
            "3" => Ok(FieldEncryptMethodEnum::PkcsDes),
            "4" => Ok(FieldEncryptMethodEnum::PgpDes),
            "5" => Ok(FieldEncryptMethodEnum::PgpDesMd5),
            "6" => Ok(FieldEncryptMethodEnum::PemDesMd5),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldEncryptMethodEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldEncryptMethodEnum::None => {
                f.write_str( "0" )
            },
            &FieldEncryptMethodEnum::Pkcs => {
                f.write_str( "1" )
            },
            &FieldEncryptMethodEnum::Des => {
                f.write_str( "2" )
            },
            &FieldEncryptMethodEnum::PkcsDes => {
                f.write_str( "3" )
            },
            &FieldEncryptMethodEnum::PgpDes => {
                f.write_str( "4" )
            },
            &FieldEncryptMethodEnum::PgpDesMd5 => {
                f.write_str( "5" )
            },
            &FieldEncryptMethodEnum::PemDesMd5 => {
                f.write_str( "6" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldEncryptMethodEnum {
    fn default() -> Self {
        FieldEncryptMethodEnum::Undefined
    }
}
const FIELD_STOPPX : u32 = 99; // PRICE


const FIELD_EXDESTINATION : u32 = 100; // EXCHANGE


const FIELD_CXLREJREASON : u32 = 102; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldCxlRejReasonEnum {
    TooLateToCancel, // = "0"
    UnknownOrder, // = "1"
    BrokerOption, // = "2"
    OrderAlreadyInPendingCancelOrPendingReplaceStatus, // = "3"

    Undefined
}

impl FromStr for FieldCxlRejReasonEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldCxlRejReasonEnum::TooLateToCancel),
            "1" => Ok(FieldCxlRejReasonEnum::UnknownOrder),
            "2" => Ok(FieldCxlRejReasonEnum::BrokerOption),
            "3" => Ok(FieldCxlRejReasonEnum::OrderAlreadyInPendingCancelOrPendingReplaceStatus),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldCxlRejReasonEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldCxlRejReasonEnum::TooLateToCancel => {
                f.write_str( "0" )
            },
            &FieldCxlRejReasonEnum::UnknownOrder => {
                f.write_str( "1" )
            },
            &FieldCxlRejReasonEnum::BrokerOption => {
                f.write_str( "2" )
            },
            &FieldCxlRejReasonEnum::OrderAlreadyInPendingCancelOrPendingReplaceStatus => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldCxlRejReasonEnum {
    fn default() -> Self {
        FieldCxlRejReasonEnum::Undefined
    }
}
const FIELD_ORDREJREASON : u32 = 103; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldOrdRejReasonEnum {
    BrokerOption, // = "0"
    UnknownSymbol, // = "1"
    ExchangeClosed, // = "2"
    OrderExceedsLimit, // = "3"
    TooLateToEnter, // = "4"
    UnknownOrder, // = "5"
    DuplicateOrder, // = "6"
    DuplicateOfAVerballyCommunicatedOrder, // = "7"
    StaleOrder, // = "8"

    Undefined
}

impl FromStr for FieldOrdRejReasonEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldOrdRejReasonEnum::BrokerOption),
            "1" => Ok(FieldOrdRejReasonEnum::UnknownSymbol),
            "2" => Ok(FieldOrdRejReasonEnum::ExchangeClosed),
            "3" => Ok(FieldOrdRejReasonEnum::OrderExceedsLimit),
            "4" => Ok(FieldOrdRejReasonEnum::TooLateToEnter),
            "5" => Ok(FieldOrdRejReasonEnum::UnknownOrder),
            "6" => Ok(FieldOrdRejReasonEnum::DuplicateOrder),
            "7" => Ok(FieldOrdRejReasonEnum::DuplicateOfAVerballyCommunicatedOrder),
            "8" => Ok(FieldOrdRejReasonEnum::StaleOrder),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldOrdRejReasonEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldOrdRejReasonEnum::BrokerOption => {
                f.write_str( "0" )
            },
            &FieldOrdRejReasonEnum::UnknownSymbol => {
                f.write_str( "1" )
            },
            &FieldOrdRejReasonEnum::ExchangeClosed => {
                f.write_str( "2" )
            },
            &FieldOrdRejReasonEnum::OrderExceedsLimit => {
                f.write_str( "3" )
            },
            &FieldOrdRejReasonEnum::TooLateToEnter => {
                f.write_str( "4" )
            },
            &FieldOrdRejReasonEnum::UnknownOrder => {
                f.write_str( "5" )
            },
            &FieldOrdRejReasonEnum::DuplicateOrder => {
                f.write_str( "6" )
            },
            &FieldOrdRejReasonEnum::DuplicateOfAVerballyCommunicatedOrder => {
                f.write_str( "7" )
            },
            &FieldOrdRejReasonEnum::StaleOrder => {
                f.write_str( "8" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldOrdRejReasonEnum {
    fn default() -> Self {
        FieldOrdRejReasonEnum::Undefined
    }
}
const FIELD_IOIQUALIFIER : u32 = 104; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldIOIQualifierEnum {
    AllOrNone, // = "A"
    AtTheClose, // = "C"
    InTouchWith, // = "I"
    Limit, // = "L"
    MoreBehind, // = "M"
    AtTheOpen, // = "O"
    TakingAPosition, // = "P"
    AtTheMarket, // = "Q"
    ReadyToTrade, // = "R"
    PortfolioShowN, // = "S"
    ThroughTheDay, // = "T"
    Versus, // = "V"
    Indication, // = "W"
    CrossingOpportunity, // = "X"
    AtTheMidpoint, // = "Y"
    PreOpen, // = "Z"

    Undefined
}

impl FromStr for FieldIOIQualifierEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(FieldIOIQualifierEnum::AllOrNone),
            "C" => Ok(FieldIOIQualifierEnum::AtTheClose),
            "I" => Ok(FieldIOIQualifierEnum::InTouchWith),
            "L" => Ok(FieldIOIQualifierEnum::Limit),
            "M" => Ok(FieldIOIQualifierEnum::MoreBehind),
            "O" => Ok(FieldIOIQualifierEnum::AtTheOpen),
            "P" => Ok(FieldIOIQualifierEnum::TakingAPosition),
            "Q" => Ok(FieldIOIQualifierEnum::AtTheMarket),
            "R" => Ok(FieldIOIQualifierEnum::ReadyToTrade),
            "S" => Ok(FieldIOIQualifierEnum::PortfolioShowN),
            "T" => Ok(FieldIOIQualifierEnum::ThroughTheDay),
            "V" => Ok(FieldIOIQualifierEnum::Versus),
            "W" => Ok(FieldIOIQualifierEnum::Indication),
            "X" => Ok(FieldIOIQualifierEnum::CrossingOpportunity),
            "Y" => Ok(FieldIOIQualifierEnum::AtTheMidpoint),
            "Z" => Ok(FieldIOIQualifierEnum::PreOpen),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldIOIQualifierEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldIOIQualifierEnum::AllOrNone => {
                f.write_str( "A" )
            },
            &FieldIOIQualifierEnum::AtTheClose => {
                f.write_str( "C" )
            },
            &FieldIOIQualifierEnum::InTouchWith => {
                f.write_str( "I" )
            },
            &FieldIOIQualifierEnum::Limit => {
                f.write_str( "L" )
            },
            &FieldIOIQualifierEnum::MoreBehind => {
                f.write_str( "M" )
            },
            &FieldIOIQualifierEnum::AtTheOpen => {
                f.write_str( "O" )
            },
            &FieldIOIQualifierEnum::TakingAPosition => {
                f.write_str( "P" )
            },
            &FieldIOIQualifierEnum::AtTheMarket => {
                f.write_str( "Q" )
            },
            &FieldIOIQualifierEnum::ReadyToTrade => {
                f.write_str( "R" )
            },
            &FieldIOIQualifierEnum::PortfolioShowN => {
                f.write_str( "S" )
            },
            &FieldIOIQualifierEnum::ThroughTheDay => {
                f.write_str( "T" )
            },
            &FieldIOIQualifierEnum::Versus => {
                f.write_str( "V" )
            },
            &FieldIOIQualifierEnum::Indication => {
                f.write_str( "W" )
            },
            &FieldIOIQualifierEnum::CrossingOpportunity => {
                f.write_str( "X" )
            },
            &FieldIOIQualifierEnum::AtTheMidpoint => {
                f.write_str( "Y" )
            },
            &FieldIOIQualifierEnum::PreOpen => {
                f.write_str( "Z" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldIOIQualifierEnum {
    fn default() -> Self {
        FieldIOIQualifierEnum::Undefined
    }
}
const FIELD_WAVENO : u32 = 105; // STRING


const FIELD_ISSUER : u32 = 106; // STRING


const FIELD_SECURITYDESC : u32 = 107; // STRING


const FIELD_HEARTBTINT : u32 = 108; // INT


const FIELD_CLIENTID : u32 = 109; // STRING


const FIELD_MINQTY : u32 = 110; // QTY


const FIELD_MAXFLOOR : u32 = 111; // QTY


const FIELD_TESTREQID : u32 = 112; // STRING


const FIELD_REPORTTOEXCH : u32 = 113; // BOOLEAN


const FIELD_LOCATEREQD : u32 = 114; // BOOLEAN


const FIELD_ONBEHALFOFCOMPID : u32 = 115; // STRING


const FIELD_ONBEHALFOFSUBID : u32 = 116; // STRING


const FIELD_QUOTEID : u32 = 117; // STRING


const FIELD_NETMONEY : u32 = 118; // AMT


const FIELD_SETTLCURRAMT : u32 = 119; // AMT


const FIELD_SETTLCURRENCY : u32 = 120; // CURRENCY


const FIELD_FOREXREQ : u32 = 121; // BOOLEAN


const FIELD_ORIGSENDINGTIME : u32 = 122; // UTCTIMESTAMP


const FIELD_GAPFILLFLAG : u32 = 123; // BOOLEAN


const FIELD_NOEXECS : u32 = 124; // INT


const FIELD_CXLTYPE : u32 = 125; // CHAR


const FIELD_EXPIRETIME : u32 = 126; // UTCTIMESTAMP


const FIELD_DKREASON : u32 = 127; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldDKReasonEnum {
    UnknownSymbol, // = "A"
    WrongSide, // = "B"
    QuantityExceedsOrder, // = "C"
    NoMatchingOrder, // = "D"
    PriceExceedsLimit, // = "E"
    Other, // = "Z"

    Undefined
}

impl FromStr for FieldDKReasonEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(FieldDKReasonEnum::UnknownSymbol),
            "B" => Ok(FieldDKReasonEnum::WrongSide),
            "C" => Ok(FieldDKReasonEnum::QuantityExceedsOrder),
            "D" => Ok(FieldDKReasonEnum::NoMatchingOrder),
            "E" => Ok(FieldDKReasonEnum::PriceExceedsLimit),
            "Z" => Ok(FieldDKReasonEnum::Other),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldDKReasonEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldDKReasonEnum::UnknownSymbol => {
                f.write_str( "A" )
            },
            &FieldDKReasonEnum::WrongSide => {
                f.write_str( "B" )
            },
            &FieldDKReasonEnum::QuantityExceedsOrder => {
                f.write_str( "C" )
            },
            &FieldDKReasonEnum::NoMatchingOrder => {
                f.write_str( "D" )
            },
            &FieldDKReasonEnum::PriceExceedsLimit => {
                f.write_str( "E" )
            },
            &FieldDKReasonEnum::Other => {
                f.write_str( "Z" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldDKReasonEnum {
    fn default() -> Self {
        FieldDKReasonEnum::Undefined
    }
}
const FIELD_DELIVERTOCOMPID : u32 = 128; // STRING


const FIELD_DELIVERTOSUBID : u32 = 129; // STRING


const FIELD_IOINATURALFLAG : u32 = 130; // BOOLEAN


const FIELD_QUOTEREQID : u32 = 131; // STRING


const FIELD_BIDPX : u32 = 132; // PRICE


const FIELD_OFFERPX : u32 = 133; // PRICE


const FIELD_BIDSIZE : u32 = 134; // QTY


const FIELD_OFFERSIZE : u32 = 135; // QTY


const FIELD_NOMISCFEES : u32 = 136; // INT


const FIELD_MISCFEEAMT : u32 = 137; // AMT


const FIELD_MISCFEECURR : u32 = 138; // CURRENCY


const FIELD_MISCFEETYPE : u32 = 139; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldMiscFeeTypeEnum {
    Regulatory, // = "1"
    Tax, // = "2"
    LocalCommission, // = "3"
    ExchangeFees, // = "4"
    Stamp, // = "5"
    Levy, // = "6"
    Other, // = "7"
    Markup, // = "8"
    ConsumptionTax, // = "9"

    Undefined
}

impl FromStr for FieldMiscFeeTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldMiscFeeTypeEnum::Regulatory),
            "2" => Ok(FieldMiscFeeTypeEnum::Tax),
            "3" => Ok(FieldMiscFeeTypeEnum::LocalCommission),
            "4" => Ok(FieldMiscFeeTypeEnum::ExchangeFees),
            "5" => Ok(FieldMiscFeeTypeEnum::Stamp),
            "6" => Ok(FieldMiscFeeTypeEnum::Levy),
            "7" => Ok(FieldMiscFeeTypeEnum::Other),
            "8" => Ok(FieldMiscFeeTypeEnum::Markup),
            "9" => Ok(FieldMiscFeeTypeEnum::ConsumptionTax),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldMiscFeeTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldMiscFeeTypeEnum::Regulatory => {
                f.write_str( "1" )
            },
            &FieldMiscFeeTypeEnum::Tax => {
                f.write_str( "2" )
            },
            &FieldMiscFeeTypeEnum::LocalCommission => {
                f.write_str( "3" )
            },
            &FieldMiscFeeTypeEnum::ExchangeFees => {
                f.write_str( "4" )
            },
            &FieldMiscFeeTypeEnum::Stamp => {
                f.write_str( "5" )
            },
            &FieldMiscFeeTypeEnum::Levy => {
                f.write_str( "6" )
            },
            &FieldMiscFeeTypeEnum::Other => {
                f.write_str( "7" )
            },
            &FieldMiscFeeTypeEnum::Markup => {
                f.write_str( "8" )
            },
            &FieldMiscFeeTypeEnum::ConsumptionTax => {
                f.write_str( "9" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldMiscFeeTypeEnum {
    fn default() -> Self {
        FieldMiscFeeTypeEnum::Undefined
    }
}
const FIELD_PREVCLOSEPX : u32 = 140; // PRICE


const FIELD_RESETSEQNUMFLAG : u32 = 141; // BOOLEAN


const FIELD_SENDERLOCATIONID : u32 = 142; // STRING


const FIELD_TARGETLOCATIONID : u32 = 143; // STRING


const FIELD_ONBEHALFOFLOCATIONID : u32 = 144; // STRING


const FIELD_DELIVERTOLOCATIONID : u32 = 145; // STRING


const FIELD_NORELATEDSYM : u32 = 146; // INT


const FIELD_SUBJECT : u32 = 147; // STRING


const FIELD_HEADLINE : u32 = 148; // STRING


const FIELD_URLLINK : u32 = 149; // STRING


const FIELD_EXECTYPE : u32 = 150; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldExecTypeEnum {
    New, // = "0"
    PartialFill, // = "1"
    Fill, // = "2"
    DoneForDay, // = "3"
    Canceled, // = "4"
    Replace, // = "5"
    PendingCancel, // = "6"
    Stopped, // = "7"
    Rejected, // = "8"
    Suspended, // = "9"
    PendingNew, // = "A"
    Calculated, // = "B"
    Expired, // = "C"
    Restated, // = "D"
    PendingReplace, // = "E"

    Undefined
}

impl FromStr for FieldExecTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldExecTypeEnum::New),
            "1" => Ok(FieldExecTypeEnum::PartialFill),
            "2" => Ok(FieldExecTypeEnum::Fill),
            "3" => Ok(FieldExecTypeEnum::DoneForDay),
            "4" => Ok(FieldExecTypeEnum::Canceled),
            "5" => Ok(FieldExecTypeEnum::Replace),
            "6" => Ok(FieldExecTypeEnum::PendingCancel),
            "7" => Ok(FieldExecTypeEnum::Stopped),
            "8" => Ok(FieldExecTypeEnum::Rejected),
            "9" => Ok(FieldExecTypeEnum::Suspended),
            "A" => Ok(FieldExecTypeEnum::PendingNew),
            "B" => Ok(FieldExecTypeEnum::Calculated),
            "C" => Ok(FieldExecTypeEnum::Expired),
            "D" => Ok(FieldExecTypeEnum::Restated),
            "E" => Ok(FieldExecTypeEnum::PendingReplace),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldExecTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldExecTypeEnum::New => {
                f.write_str( "0" )
            },
            &FieldExecTypeEnum::PartialFill => {
                f.write_str( "1" )
            },
            &FieldExecTypeEnum::Fill => {
                f.write_str( "2" )
            },
            &FieldExecTypeEnum::DoneForDay => {
                f.write_str( "3" )
            },
            &FieldExecTypeEnum::Canceled => {
                f.write_str( "4" )
            },
            &FieldExecTypeEnum::Replace => {
                f.write_str( "5" )
            },
            &FieldExecTypeEnum::PendingCancel => {
                f.write_str( "6" )
            },
            &FieldExecTypeEnum::Stopped => {
                f.write_str( "7" )
            },
            &FieldExecTypeEnum::Rejected => {
                f.write_str( "8" )
            },
            &FieldExecTypeEnum::Suspended => {
                f.write_str( "9" )
            },
            &FieldExecTypeEnum::PendingNew => {
                f.write_str( "A" )
            },
            &FieldExecTypeEnum::Calculated => {
                f.write_str( "B" )
            },
            &FieldExecTypeEnum::Expired => {
                f.write_str( "C" )
            },
            &FieldExecTypeEnum::Restated => {
                f.write_str( "D" )
            },
            &FieldExecTypeEnum::PendingReplace => {
                f.write_str( "E" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldExecTypeEnum {
    fn default() -> Self {
        FieldExecTypeEnum::Undefined
    }
}
const FIELD_LEAVESQTY : u32 = 151; // QTY


const FIELD_CASHORDERQTY : u32 = 152; // QTY


const FIELD_ALLOCAVGPX : u32 = 153; // PRICE


const FIELD_ALLOCNETMONEY : u32 = 154; // AMT


const FIELD_SETTLCURRFXRATE : u32 = 155; // FLOAT


const FIELD_SETTLCURRFXRATECALC : u32 = 156; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldSettlCurrFxRateCalcEnum {
    Multiply, // = "M"
    Divide, // = "D"

    Undefined
}

impl FromStr for FieldSettlCurrFxRateCalcEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "M" => Ok(FieldSettlCurrFxRateCalcEnum::Multiply),
            "D" => Ok(FieldSettlCurrFxRateCalcEnum::Divide),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldSettlCurrFxRateCalcEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldSettlCurrFxRateCalcEnum::Multiply => {
                f.write_str( "M" )
            },
            &FieldSettlCurrFxRateCalcEnum::Divide => {
                f.write_str( "D" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldSettlCurrFxRateCalcEnum {
    fn default() -> Self {
        FieldSettlCurrFxRateCalcEnum::Undefined
    }
}
const FIELD_NUMDAYSINTEREST : u32 = 157; // INT


const FIELD_ACCRUEDINTERESTRATE : u32 = 158; // FLOAT


const FIELD_ACCRUEDINTERESTAMT : u32 = 159; // AMT


const FIELD_SETTLINSTMODE : u32 = 160; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldSettlInstModeEnum {
    Default, // = "0"
    StandingInstructionsProvided, // = "1"
    SpecificAllocationAccountOverriding, // = "2"
    SpecificAllocationAccountStanding, // = "3"

    Undefined
}

impl FromStr for FieldSettlInstModeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldSettlInstModeEnum::Default),
            "1" => Ok(FieldSettlInstModeEnum::StandingInstructionsProvided),
            "2" => Ok(FieldSettlInstModeEnum::SpecificAllocationAccountOverriding),
            "3" => Ok(FieldSettlInstModeEnum::SpecificAllocationAccountStanding),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldSettlInstModeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldSettlInstModeEnum::Default => {
                f.write_str( "0" )
            },
            &FieldSettlInstModeEnum::StandingInstructionsProvided => {
                f.write_str( "1" )
            },
            &FieldSettlInstModeEnum::SpecificAllocationAccountOverriding => {
                f.write_str( "2" )
            },
            &FieldSettlInstModeEnum::SpecificAllocationAccountStanding => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldSettlInstModeEnum {
    fn default() -> Self {
        FieldSettlInstModeEnum::Undefined
    }
}
const FIELD_ALLOCTEXT : u32 = 161; // STRING


const FIELD_SETTLINSTID : u32 = 162; // STRING


const FIELD_SETTLINSTTRANSTYPE : u32 = 163; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldSettlInstTransTypeEnum {
    Cancel, // = "C"
    New, // = "N"
    Replace, // = "R"

    Undefined
}

impl FromStr for FieldSettlInstTransTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(FieldSettlInstTransTypeEnum::Cancel),
            "N" => Ok(FieldSettlInstTransTypeEnum::New),
            "R" => Ok(FieldSettlInstTransTypeEnum::Replace),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldSettlInstTransTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldSettlInstTransTypeEnum::Cancel => {
                f.write_str( "C" )
            },
            &FieldSettlInstTransTypeEnum::New => {
                f.write_str( "N" )
            },
            &FieldSettlInstTransTypeEnum::Replace => {
                f.write_str( "R" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldSettlInstTransTypeEnum {
    fn default() -> Self {
        FieldSettlInstTransTypeEnum::Undefined
    }
}
const FIELD_EMAILTHREADID : u32 = 164; // STRING


const FIELD_SETTLINSTSOURCE : u32 = 165; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldSettlInstSourceEnum {
    BrokersInstructions, // = "1"
    InstitutionsInstructions, // = "2"

    Undefined
}

impl FromStr for FieldSettlInstSourceEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldSettlInstSourceEnum::BrokersInstructions),
            "2" => Ok(FieldSettlInstSourceEnum::InstitutionsInstructions),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldSettlInstSourceEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldSettlInstSourceEnum::BrokersInstructions => {
                f.write_str( "1" )
            },
            &FieldSettlInstSourceEnum::InstitutionsInstructions => {
                f.write_str( "2" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldSettlInstSourceEnum {
    fn default() -> Self {
        FieldSettlInstSourceEnum::Undefined
    }
}
const FIELD_SETTLLOCATION : u32 = 166; // STRING


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldSettlLocationEnum {
    Cedel, // = "CED"
    DepositoryTrustCompany, // = "DTC"
    Euroclear, // = "EUR"
    FederalBookEntry, // = "FED"
    LocalMarketSettleLocation, // = "ISO Country Code"
    Physical, // = "PNY"
    ParticipantTrustCompany, // = "PTC"

    Undefined
}

impl FromStr for FieldSettlLocationEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CED" => Ok(FieldSettlLocationEnum::Cedel),
            "DTC" => Ok(FieldSettlLocationEnum::DepositoryTrustCompany),
            "EUR" => Ok(FieldSettlLocationEnum::Euroclear),
            "FED" => Ok(FieldSettlLocationEnum::FederalBookEntry),
            "ISO Country Code" => Ok(FieldSettlLocationEnum::LocalMarketSettleLocation),
            "PNY" => Ok(FieldSettlLocationEnum::Physical),
            "PTC" => Ok(FieldSettlLocationEnum::ParticipantTrustCompany),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldSettlLocationEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldSettlLocationEnum::Cedel => {
                f.write_str( "CED" )
            },
            &FieldSettlLocationEnum::DepositoryTrustCompany => {
                f.write_str( "DTC" )
            },
            &FieldSettlLocationEnum::Euroclear => {
                f.write_str( "EUR" )
            },
            &FieldSettlLocationEnum::FederalBookEntry => {
                f.write_str( "FED" )
            },
            &FieldSettlLocationEnum::LocalMarketSettleLocation => {
                f.write_str( "ISO Country Code" )
            },
            &FieldSettlLocationEnum::Physical => {
                f.write_str( "PNY" )
            },
            &FieldSettlLocationEnum::ParticipantTrustCompany => {
                f.write_str( "PTC" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldSettlLocationEnum {
    fn default() -> Self {
        FieldSettlLocationEnum::Undefined
    }
}
const FIELD_SECURITYTYPE : u32 = 167; // STRING


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldSecurityTypeEnum {
    WildcardEntry, // = "?"
    BankersAcceptance, // = "BA"
    ConvertibleBond, // = "CB"
    CertificateOfDeposit, // = "CD"
    CollateralizeMortgageObligation, // = "CMO"
    CorporateBond, // = "CORP"
    CommercialPaper, // = "CP"
    CorporatePrivatePlacement, // = "CPP"
    CommonStock, // = "CS"
    FederalHousingAuthority, // = "FHA"
    FederalHomeLoan, // = "FHL"
    FederalNationalMortgageAssociation, // = "FN"
    ForeignExchangeContract, // = "FOR"
    Future, // = "FUT"
    GovernmentNationalMortgageAssociation, // = "GN"
    TreasuriesPlusAgencyDebenture, // = "GOVT"
    MortgageIoette, // = "IET"
    MutualFund, // = "MF"
    MortgageInterestOnly, // = "MIO"
    MortgagePrincipalOnly, // = "MPO"
    MortgagePrivatePlacement, // = "MPP"
    MiscellaneousPassThru, // = "MPT"
    MunicipalBond, // = "MUNI"
    NoIsitcSecurityType, // = "NONE"
    Option, // = "OPT"
    PreferredStock, // = "PS"
    RepurchaseAgreement, // = "RP"
    ReverseRepurchaseAgreement, // = "RVRP"
    StudentLoanMarketingAssociation, // = "SL"
    TimeDeposit, // = "TD"
    UsTreasuryBill, // = "USTB"
    Warrant, // = "WAR"
    CatsTigersLions, // = "ZOO"

    Undefined
}

impl FromStr for FieldSecurityTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "?" => Ok(FieldSecurityTypeEnum::WildcardEntry),
            "BA" => Ok(FieldSecurityTypeEnum::BankersAcceptance),
            "CB" => Ok(FieldSecurityTypeEnum::ConvertibleBond),
            "CD" => Ok(FieldSecurityTypeEnum::CertificateOfDeposit),
            "CMO" => Ok(FieldSecurityTypeEnum::CollateralizeMortgageObligation),
            "CORP" => Ok(FieldSecurityTypeEnum::CorporateBond),
            "CP" => Ok(FieldSecurityTypeEnum::CommercialPaper),
            "CPP" => Ok(FieldSecurityTypeEnum::CorporatePrivatePlacement),
            "CS" => Ok(FieldSecurityTypeEnum::CommonStock),
            "FHA" => Ok(FieldSecurityTypeEnum::FederalHousingAuthority),
            "FHL" => Ok(FieldSecurityTypeEnum::FederalHomeLoan),
            "FN" => Ok(FieldSecurityTypeEnum::FederalNationalMortgageAssociation),
            "FOR" => Ok(FieldSecurityTypeEnum::ForeignExchangeContract),
            "FUT" => Ok(FieldSecurityTypeEnum::Future),
            "GN" => Ok(FieldSecurityTypeEnum::GovernmentNationalMortgageAssociation),
            "GOVT" => Ok(FieldSecurityTypeEnum::TreasuriesPlusAgencyDebenture),
            "IET" => Ok(FieldSecurityTypeEnum::MortgageIoette),
            "MF" => Ok(FieldSecurityTypeEnum::MutualFund),
            "MIO" => Ok(FieldSecurityTypeEnum::MortgageInterestOnly),
            "MPO" => Ok(FieldSecurityTypeEnum::MortgagePrincipalOnly),
            "MPP" => Ok(FieldSecurityTypeEnum::MortgagePrivatePlacement),
            "MPT" => Ok(FieldSecurityTypeEnum::MiscellaneousPassThru),
            "MUNI" => Ok(FieldSecurityTypeEnum::MunicipalBond),
            "NONE" => Ok(FieldSecurityTypeEnum::NoIsitcSecurityType),
            "OPT" => Ok(FieldSecurityTypeEnum::Option),
            "PS" => Ok(FieldSecurityTypeEnum::PreferredStock),
            "RP" => Ok(FieldSecurityTypeEnum::RepurchaseAgreement),
            "RVRP" => Ok(FieldSecurityTypeEnum::ReverseRepurchaseAgreement),
            "SL" => Ok(FieldSecurityTypeEnum::StudentLoanMarketingAssociation),
            "TD" => Ok(FieldSecurityTypeEnum::TimeDeposit),
            "USTB" => Ok(FieldSecurityTypeEnum::UsTreasuryBill),
            "WAR" => Ok(FieldSecurityTypeEnum::Warrant),
            "ZOO" => Ok(FieldSecurityTypeEnum::CatsTigersLions),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldSecurityTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldSecurityTypeEnum::WildcardEntry => {
                f.write_str( "?" )
            },
            &FieldSecurityTypeEnum::BankersAcceptance => {
                f.write_str( "BA" )
            },
            &FieldSecurityTypeEnum::ConvertibleBond => {
                f.write_str( "CB" )
            },
            &FieldSecurityTypeEnum::CertificateOfDeposit => {
                f.write_str( "CD" )
            },
            &FieldSecurityTypeEnum::CollateralizeMortgageObligation => {
                f.write_str( "CMO" )
            },
            &FieldSecurityTypeEnum::CorporateBond => {
                f.write_str( "CORP" )
            },
            &FieldSecurityTypeEnum::CommercialPaper => {
                f.write_str( "CP" )
            },
            &FieldSecurityTypeEnum::CorporatePrivatePlacement => {
                f.write_str( "CPP" )
            },
            &FieldSecurityTypeEnum::CommonStock => {
                f.write_str( "CS" )
            },
            &FieldSecurityTypeEnum::FederalHousingAuthority => {
                f.write_str( "FHA" )
            },
            &FieldSecurityTypeEnum::FederalHomeLoan => {
                f.write_str( "FHL" )
            },
            &FieldSecurityTypeEnum::FederalNationalMortgageAssociation => {
                f.write_str( "FN" )
            },
            &FieldSecurityTypeEnum::ForeignExchangeContract => {
                f.write_str( "FOR" )
            },
            &FieldSecurityTypeEnum::Future => {
                f.write_str( "FUT" )
            },
            &FieldSecurityTypeEnum::GovernmentNationalMortgageAssociation => {
                f.write_str( "GN" )
            },
            &FieldSecurityTypeEnum::TreasuriesPlusAgencyDebenture => {
                f.write_str( "GOVT" )
            },
            &FieldSecurityTypeEnum::MortgageIoette => {
                f.write_str( "IET" )
            },
            &FieldSecurityTypeEnum::MutualFund => {
                f.write_str( "MF" )
            },
            &FieldSecurityTypeEnum::MortgageInterestOnly => {
                f.write_str( "MIO" )
            },
            &FieldSecurityTypeEnum::MortgagePrincipalOnly => {
                f.write_str( "MPO" )
            },
            &FieldSecurityTypeEnum::MortgagePrivatePlacement => {
                f.write_str( "MPP" )
            },
            &FieldSecurityTypeEnum::MiscellaneousPassThru => {
                f.write_str( "MPT" )
            },
            &FieldSecurityTypeEnum::MunicipalBond => {
                f.write_str( "MUNI" )
            },
            &FieldSecurityTypeEnum::NoIsitcSecurityType => {
                f.write_str( "NONE" )
            },
            &FieldSecurityTypeEnum::Option => {
                f.write_str( "OPT" )
            },
            &FieldSecurityTypeEnum::PreferredStock => {
                f.write_str( "PS" )
            },
            &FieldSecurityTypeEnum::RepurchaseAgreement => {
                f.write_str( "RP" )
            },
            &FieldSecurityTypeEnum::ReverseRepurchaseAgreement => {
                f.write_str( "RVRP" )
            },
            &FieldSecurityTypeEnum::StudentLoanMarketingAssociation => {
                f.write_str( "SL" )
            },
            &FieldSecurityTypeEnum::TimeDeposit => {
                f.write_str( "TD" )
            },
            &FieldSecurityTypeEnum::UsTreasuryBill => {
                f.write_str( "USTB" )
            },
            &FieldSecurityTypeEnum::Warrant => {
                f.write_str( "WAR" )
            },
            &FieldSecurityTypeEnum::CatsTigersLions => {
                f.write_str( "ZOO" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldSecurityTypeEnum {
    fn default() -> Self {
        FieldSecurityTypeEnum::Undefined
    }
}
const FIELD_EFFECTIVETIME : u32 = 168; // UTCTIMESTAMP


const FIELD_STANDINSTDBTYPE : u32 = 169; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldStandInstDbTypeEnum {
    Other, // = "0"
    DtcSid, // = "1"
    ThomsonAlert, // = "2"
    AGlobalCustodian, // = "3"

    Undefined
}

impl FromStr for FieldStandInstDbTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldStandInstDbTypeEnum::Other),
            "1" => Ok(FieldStandInstDbTypeEnum::DtcSid),
            "2" => Ok(FieldStandInstDbTypeEnum::ThomsonAlert),
            "3" => Ok(FieldStandInstDbTypeEnum::AGlobalCustodian),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldStandInstDbTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldStandInstDbTypeEnum::Other => {
                f.write_str( "0" )
            },
            &FieldStandInstDbTypeEnum::DtcSid => {
                f.write_str( "1" )
            },
            &FieldStandInstDbTypeEnum::ThomsonAlert => {
                f.write_str( "2" )
            },
            &FieldStandInstDbTypeEnum::AGlobalCustodian => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldStandInstDbTypeEnum {
    fn default() -> Self {
        FieldStandInstDbTypeEnum::Undefined
    }
}
const FIELD_STANDINSTDBNAME : u32 = 170; // STRING


const FIELD_STANDINSTDBID : u32 = 171; // STRING


const FIELD_SETTLDELIVERYTYPE : u32 = 172; // INT


const FIELD_SETTLDEPOSITORYCODE : u32 = 173; // STRING


const FIELD_SETTLBRKRCODE : u32 = 174; // STRING


const FIELD_SETTLINSTCODE : u32 = 175; // STRING


const FIELD_SECURITYSETTLAGENTNAME : u32 = 176; // STRING


const FIELD_SECURITYSETTLAGENTCODE : u32 = 177; // STRING


const FIELD_SECURITYSETTLAGENTACCTNUM : u32 = 178; // STRING


const FIELD_SECURITYSETTLAGENTACCTNAME : u32 = 179; // STRING


const FIELD_SECURITYSETTLAGENTCONTACTNAME : u32 = 180; // STRING


const FIELD_SECURITYSETTLAGENTCONTACTPHONE : u32 = 181; // STRING


const FIELD_CASHSETTLAGENTNAME : u32 = 182; // STRING


const FIELD_CASHSETTLAGENTCODE : u32 = 183; // STRING


const FIELD_CASHSETTLAGENTACCTNUM : u32 = 184; // STRING


const FIELD_CASHSETTLAGENTACCTNAME : u32 = 185; // STRING


const FIELD_CASHSETTLAGENTCONTACTNAME : u32 = 186; // STRING


const FIELD_CASHSETTLAGENTCONTACTPHONE : u32 = 187; // STRING


const FIELD_BIDSPOTRATE : u32 = 188; // PRICE


const FIELD_BIDFORWARDPOINTS : u32 = 189; // PRICEOFFSET


const FIELD_OFFERSPOTRATE : u32 = 190; // PRICE


const FIELD_OFFERFORWARDPOINTS : u32 = 191; // PRICEOFFSET


const FIELD_ORDERQTY2 : u32 = 192; // QTY


const FIELD_FUTSETTDATE2 : u32 = 193; // LOCALMKTDATE


const FIELD_LASTSPOTRATE : u32 = 194; // PRICE


const FIELD_LASTFORWARDPOINTS : u32 = 195; // PRICEOFFSET


const FIELD_ALLOCLINKID : u32 = 196; // STRING


const FIELD_ALLOCLINKTYPE : u32 = 197; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldAllocLinkTypeEnum {
    FXNetting, // = "0"
    FXSwap, // = "1"

    Undefined
}

impl FromStr for FieldAllocLinkTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldAllocLinkTypeEnum::FXNetting),
            "1" => Ok(FieldAllocLinkTypeEnum::FXSwap),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldAllocLinkTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldAllocLinkTypeEnum::FXNetting => {
                f.write_str( "0" )
            },
            &FieldAllocLinkTypeEnum::FXSwap => {
                f.write_str( "1" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldAllocLinkTypeEnum {
    fn default() -> Self {
        FieldAllocLinkTypeEnum::Undefined
    }
}
const FIELD_SECONDARYORDERID : u32 = 198; // STRING


const FIELD_NOIOIQUALIFIERS : u32 = 199; // INT


const FIELD_MATURITYMONTHYEAR : u32 = 200; // MONTHYEAR


const FIELD_PUTORCALL : u32 = 201; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldPutOrCallEnum {
    Put, // = "0"
    Call, // = "1"

    Undefined
}

impl FromStr for FieldPutOrCallEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldPutOrCallEnum::Put),
            "1" => Ok(FieldPutOrCallEnum::Call),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldPutOrCallEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldPutOrCallEnum::Put => {
                f.write_str( "0" )
            },
            &FieldPutOrCallEnum::Call => {
                f.write_str( "1" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldPutOrCallEnum {
    fn default() -> Self {
        FieldPutOrCallEnum::Undefined
    }
}
const FIELD_STRIKEPRICE : u32 = 202; // PRICE


const FIELD_COVEREDORUNCOVERED : u32 = 203; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldCoveredOrUncoveredEnum {
    Covered, // = "0"
    Uncovered, // = "1"

    Undefined
}

impl FromStr for FieldCoveredOrUncoveredEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldCoveredOrUncoveredEnum::Covered),
            "1" => Ok(FieldCoveredOrUncoveredEnum::Uncovered),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldCoveredOrUncoveredEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldCoveredOrUncoveredEnum::Covered => {
                f.write_str( "0" )
            },
            &FieldCoveredOrUncoveredEnum::Uncovered => {
                f.write_str( "1" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldCoveredOrUncoveredEnum {
    fn default() -> Self {
        FieldCoveredOrUncoveredEnum::Undefined
    }
}
const FIELD_CUSTOMERORFIRM : u32 = 204; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldCustomerOrFirmEnum {
    Customer, // = "0"
    Firm, // = "1"

    Undefined
}

impl FromStr for FieldCustomerOrFirmEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldCustomerOrFirmEnum::Customer),
            "1" => Ok(FieldCustomerOrFirmEnum::Firm),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldCustomerOrFirmEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldCustomerOrFirmEnum::Customer => {
                f.write_str( "0" )
            },
            &FieldCustomerOrFirmEnum::Firm => {
                f.write_str( "1" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldCustomerOrFirmEnum {
    fn default() -> Self {
        FieldCustomerOrFirmEnum::Undefined
    }
}
const FIELD_MATURITYDAY : u32 = 205; // DAYOFMONTH


const FIELD_OPTATTRIBUTE : u32 = 206; // CHAR


const FIELD_SECURITYEXCHANGE : u32 = 207; // EXCHANGE


const FIELD_NOTIFYBROKEROFCREDIT : u32 = 208; // BOOLEAN


const FIELD_ALLOCHANDLINST : u32 = 209; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldAllocHandlInstEnum {
    Match, // = "1"
    Forward, // = "2"
    ForwardAndMatch, // = "3"

    Undefined
}

impl FromStr for FieldAllocHandlInstEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldAllocHandlInstEnum::Match),
            "2" => Ok(FieldAllocHandlInstEnum::Forward),
            "3" => Ok(FieldAllocHandlInstEnum::ForwardAndMatch),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldAllocHandlInstEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldAllocHandlInstEnum::Match => {
                f.write_str( "1" )
            },
            &FieldAllocHandlInstEnum::Forward => {
                f.write_str( "2" )
            },
            &FieldAllocHandlInstEnum::ForwardAndMatch => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldAllocHandlInstEnum {
    fn default() -> Self {
        FieldAllocHandlInstEnum::Undefined
    }
}
const FIELD_MAXSHOW : u32 = 210; // QTY


const FIELD_PEGDIFFERENCE : u32 = 211; // PRICEOFFSET


const FIELD_XMLDATALEN : u32 = 212; // LENGTH


const FIELD_XMLDATA : u32 = 213; // DATA


const FIELD_SETTLINSTREFID : u32 = 214; // STRING


const FIELD_NOROUTINGIDS : u32 = 215; // INT


const FIELD_ROUTINGTYPE : u32 = 216; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldRoutingTypeEnum {
    TargetFirm, // = "1"
    TargetList, // = "2"
    BlockFirm, // = "3"
    BlockList, // = "4"

    Undefined
}

impl FromStr for FieldRoutingTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldRoutingTypeEnum::TargetFirm),
            "2" => Ok(FieldRoutingTypeEnum::TargetList),
            "3" => Ok(FieldRoutingTypeEnum::BlockFirm),
            "4" => Ok(FieldRoutingTypeEnum::BlockList),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldRoutingTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldRoutingTypeEnum::TargetFirm => {
                f.write_str( "1" )
            },
            &FieldRoutingTypeEnum::TargetList => {
                f.write_str( "2" )
            },
            &FieldRoutingTypeEnum::BlockFirm => {
                f.write_str( "3" )
            },
            &FieldRoutingTypeEnum::BlockList => {
                f.write_str( "4" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldRoutingTypeEnum {
    fn default() -> Self {
        FieldRoutingTypeEnum::Undefined
    }
}
const FIELD_ROUTINGID : u32 = 217; // STRING


const FIELD_SPREADTOBENCHMARK : u32 = 218; // PRICEOFFSET


const FIELD_BENCHMARK : u32 = 219; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldBenchmarkEnum {
    Curve, // = "1"
    A5Yr, // = "2"
    Old5, // = "3"
    A10Yr, // = "4"
    Old10, // = "5"
    A30Yr, // = "6"
    Old30, // = "7"
    A3MoLibor, // = "8"
    A6MoLibor, // = "9"

    Undefined
}

impl FromStr for FieldBenchmarkEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldBenchmarkEnum::Curve),
            "2" => Ok(FieldBenchmarkEnum::A5Yr),
            "3" => Ok(FieldBenchmarkEnum::Old5),
            "4" => Ok(FieldBenchmarkEnum::A10Yr),
            "5" => Ok(FieldBenchmarkEnum::Old10),
            "6" => Ok(FieldBenchmarkEnum::A30Yr),
            "7" => Ok(FieldBenchmarkEnum::Old30),
            "8" => Ok(FieldBenchmarkEnum::A3MoLibor),
            "9" => Ok(FieldBenchmarkEnum::A6MoLibor),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldBenchmarkEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldBenchmarkEnum::Curve => {
                f.write_str( "1" )
            },
            &FieldBenchmarkEnum::A5Yr => {
                f.write_str( "2" )
            },
            &FieldBenchmarkEnum::Old5 => {
                f.write_str( "3" )
            },
            &FieldBenchmarkEnum::A10Yr => {
                f.write_str( "4" )
            },
            &FieldBenchmarkEnum::Old10 => {
                f.write_str( "5" )
            },
            &FieldBenchmarkEnum::A30Yr => {
                f.write_str( "6" )
            },
            &FieldBenchmarkEnum::Old30 => {
                f.write_str( "7" )
            },
            &FieldBenchmarkEnum::A3MoLibor => {
                f.write_str( "8" )
            },
            &FieldBenchmarkEnum::A6MoLibor => {
                f.write_str( "9" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldBenchmarkEnum {
    fn default() -> Self {
        FieldBenchmarkEnum::Undefined
    }
}
const FIELD_COUPONRATE : u32 = 223; // FLOAT


const FIELD_CONTRACTMULTIPLIER : u32 = 231; // FLOAT


const FIELD_MDREQID : u32 = 262; // STRING


const FIELD_SUBSCRIPTIONREQUESTTYPE : u32 = 263; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldSubscriptionRequestTypeEnum {
    Snapshot, // = "0"
    SnapshotPlusUpdates, // = "1"
    DisablePreviousSnapshotPlusUpdateRequest, // = "2"

    Undefined
}

impl FromStr for FieldSubscriptionRequestTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldSubscriptionRequestTypeEnum::Snapshot),
            "1" => Ok(FieldSubscriptionRequestTypeEnum::SnapshotPlusUpdates),
            "2" => Ok(FieldSubscriptionRequestTypeEnum::DisablePreviousSnapshotPlusUpdateRequest),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldSubscriptionRequestTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldSubscriptionRequestTypeEnum::Snapshot => {
                f.write_str( "0" )
            },
            &FieldSubscriptionRequestTypeEnum::SnapshotPlusUpdates => {
                f.write_str( "1" )
            },
            &FieldSubscriptionRequestTypeEnum::DisablePreviousSnapshotPlusUpdateRequest => {
                f.write_str( "2" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldSubscriptionRequestTypeEnum {
    fn default() -> Self {
        FieldSubscriptionRequestTypeEnum::Undefined
    }
}
const FIELD_MARKETDEPTH : u32 = 264; // INT


const FIELD_MDUPDATETYPE : u32 = 265; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldMDUpdateTypeEnum {
    FullRefresh, // = "0"
    IncrementalRefresh, // = "1"

    Undefined
}

impl FromStr for FieldMDUpdateTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldMDUpdateTypeEnum::FullRefresh),
            "1" => Ok(FieldMDUpdateTypeEnum::IncrementalRefresh),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldMDUpdateTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldMDUpdateTypeEnum::FullRefresh => {
                f.write_str( "0" )
            },
            &FieldMDUpdateTypeEnum::IncrementalRefresh => {
                f.write_str( "1" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldMDUpdateTypeEnum {
    fn default() -> Self {
        FieldMDUpdateTypeEnum::Undefined
    }
}
const FIELD_AGGREGATEDBOOK : u32 = 266; // BOOLEAN


const FIELD_NOMDENTRYTYPES : u32 = 267; // INT


const FIELD_NOMDENTRIES : u32 = 268; // INT


const FIELD_MDENTRYTYPE : u32 = 269; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldMDEntryTypeEnum {
    Bid, // = "0"
    Offer, // = "1"
    Trade, // = "2"
    IndexValue, // = "3"
    OpeningPrice, // = "4"
    ClosingPrice, // = "5"
    SettlementPrice, // = "6"
    TradingSessionHighPrice, // = "7"
    TradingSessionLowPrice, // = "8"
    TradingSessionVwapPrice, // = "9"

    Undefined
}

impl FromStr for FieldMDEntryTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldMDEntryTypeEnum::Bid),
            "1" => Ok(FieldMDEntryTypeEnum::Offer),
            "2" => Ok(FieldMDEntryTypeEnum::Trade),
            "3" => Ok(FieldMDEntryTypeEnum::IndexValue),
            "4" => Ok(FieldMDEntryTypeEnum::OpeningPrice),
            "5" => Ok(FieldMDEntryTypeEnum::ClosingPrice),
            "6" => Ok(FieldMDEntryTypeEnum::SettlementPrice),
            "7" => Ok(FieldMDEntryTypeEnum::TradingSessionHighPrice),
            "8" => Ok(FieldMDEntryTypeEnum::TradingSessionLowPrice),
            "9" => Ok(FieldMDEntryTypeEnum::TradingSessionVwapPrice),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldMDEntryTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldMDEntryTypeEnum::Bid => {
                f.write_str( "0" )
            },
            &FieldMDEntryTypeEnum::Offer => {
                f.write_str( "1" )
            },
            &FieldMDEntryTypeEnum::Trade => {
                f.write_str( "2" )
            },
            &FieldMDEntryTypeEnum::IndexValue => {
                f.write_str( "3" )
            },
            &FieldMDEntryTypeEnum::OpeningPrice => {
                f.write_str( "4" )
            },
            &FieldMDEntryTypeEnum::ClosingPrice => {
                f.write_str( "5" )
            },
            &FieldMDEntryTypeEnum::SettlementPrice => {
                f.write_str( "6" )
            },
            &FieldMDEntryTypeEnum::TradingSessionHighPrice => {
                f.write_str( "7" )
            },
            &FieldMDEntryTypeEnum::TradingSessionLowPrice => {
                f.write_str( "8" )
            },
            &FieldMDEntryTypeEnum::TradingSessionVwapPrice => {
                f.write_str( "9" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldMDEntryTypeEnum {
    fn default() -> Self {
        FieldMDEntryTypeEnum::Undefined
    }
}
const FIELD_MDENTRYPX : u32 = 270; // PRICE


const FIELD_MDENTRYSIZE : u32 = 271; // QTY


const FIELD_MDENTRYDATE : u32 = 272; // UTCDATE


const FIELD_MDENTRYTIME : u32 = 273; // UTCTIMEONLY


const FIELD_TICKDIRECTION : u32 = 274; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldTickDirectionEnum {
    PlusTick, // = "0"
    ZeroPlusTick, // = "1"
    MinusTick, // = "2"
    ZeroMinusTick, // = "3"

    Undefined
}

impl FromStr for FieldTickDirectionEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldTickDirectionEnum::PlusTick),
            "1" => Ok(FieldTickDirectionEnum::ZeroPlusTick),
            "2" => Ok(FieldTickDirectionEnum::MinusTick),
            "3" => Ok(FieldTickDirectionEnum::ZeroMinusTick),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldTickDirectionEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldTickDirectionEnum::PlusTick => {
                f.write_str( "0" )
            },
            &FieldTickDirectionEnum::ZeroPlusTick => {
                f.write_str( "1" )
            },
            &FieldTickDirectionEnum::MinusTick => {
                f.write_str( "2" )
            },
            &FieldTickDirectionEnum::ZeroMinusTick => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldTickDirectionEnum {
    fn default() -> Self {
        FieldTickDirectionEnum::Undefined
    }
}
const FIELD_MDMKT : u32 = 275; // EXCHANGE


const FIELD_QUOTECONDITION : u32 = 276; // MULTIPLEVALUESTRING


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldQuoteConditionEnum {
    Open, // = "A"
    Closed, // = "B"
    ExchangeBest, // = "C"
    ConsolidatedBest, // = "D"
    Locked, // = "E"
    Crossed, // = "F"
    Depth, // = "G"
    FastTrading, // = "H"
    NonFirm, // = "I"

    Undefined
}

impl FromStr for FieldQuoteConditionEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(FieldQuoteConditionEnum::Open),
            "B" => Ok(FieldQuoteConditionEnum::Closed),
            "C" => Ok(FieldQuoteConditionEnum::ExchangeBest),
            "D" => Ok(FieldQuoteConditionEnum::ConsolidatedBest),
            "E" => Ok(FieldQuoteConditionEnum::Locked),
            "F" => Ok(FieldQuoteConditionEnum::Crossed),
            "G" => Ok(FieldQuoteConditionEnum::Depth),
            "H" => Ok(FieldQuoteConditionEnum::FastTrading),
            "I" => Ok(FieldQuoteConditionEnum::NonFirm),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldQuoteConditionEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldQuoteConditionEnum::Open => {
                f.write_str( "A" )
            },
            &FieldQuoteConditionEnum::Closed => {
                f.write_str( "B" )
            },
            &FieldQuoteConditionEnum::ExchangeBest => {
                f.write_str( "C" )
            },
            &FieldQuoteConditionEnum::ConsolidatedBest => {
                f.write_str( "D" )
            },
            &FieldQuoteConditionEnum::Locked => {
                f.write_str( "E" )
            },
            &FieldQuoteConditionEnum::Crossed => {
                f.write_str( "F" )
            },
            &FieldQuoteConditionEnum::Depth => {
                f.write_str( "G" )
            },
            &FieldQuoteConditionEnum::FastTrading => {
                f.write_str( "H" )
            },
            &FieldQuoteConditionEnum::NonFirm => {
                f.write_str( "I" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldQuoteConditionEnum {
    fn default() -> Self {
        FieldQuoteConditionEnum::Undefined
    }
}
const FIELD_TRADECONDITION : u32 = 277; // MULTIPLEVALUESTRING


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldTradeConditionEnum {
    Cash, // = "A"
    AveragePriceTrade, // = "B"
    CashTrade, // = "C"
    NextDay, // = "D"
    Opening, // = "E"
    IntradayTradeDetail, // = "F"
    Rule127Trade, // = "G"
    Rule155Trade, // = "H"
    SoldLast, // = "I"
    NextDayTrade, // = "J"
    Opened, // = "K"
    Seller, // = "L"
    Sold, // = "M"
    StoppedStock, // = "N"

    Undefined
}

impl FromStr for FieldTradeConditionEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(FieldTradeConditionEnum::Cash),
            "B" => Ok(FieldTradeConditionEnum::AveragePriceTrade),
            "C" => Ok(FieldTradeConditionEnum::CashTrade),
            "D" => Ok(FieldTradeConditionEnum::NextDay),
            "E" => Ok(FieldTradeConditionEnum::Opening),
            "F" => Ok(FieldTradeConditionEnum::IntradayTradeDetail),
            "G" => Ok(FieldTradeConditionEnum::Rule127Trade),
            "H" => Ok(FieldTradeConditionEnum::Rule155Trade),
            "I" => Ok(FieldTradeConditionEnum::SoldLast),
            "J" => Ok(FieldTradeConditionEnum::NextDayTrade),
            "K" => Ok(FieldTradeConditionEnum::Opened),
            "L" => Ok(FieldTradeConditionEnum::Seller),
            "M" => Ok(FieldTradeConditionEnum::Sold),
            "N" => Ok(FieldTradeConditionEnum::StoppedStock),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldTradeConditionEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldTradeConditionEnum::Cash => {
                f.write_str( "A" )
            },
            &FieldTradeConditionEnum::AveragePriceTrade => {
                f.write_str( "B" )
            },
            &FieldTradeConditionEnum::CashTrade => {
                f.write_str( "C" )
            },
            &FieldTradeConditionEnum::NextDay => {
                f.write_str( "D" )
            },
            &FieldTradeConditionEnum::Opening => {
                f.write_str( "E" )
            },
            &FieldTradeConditionEnum::IntradayTradeDetail => {
                f.write_str( "F" )
            },
            &FieldTradeConditionEnum::Rule127Trade => {
                f.write_str( "G" )
            },
            &FieldTradeConditionEnum::Rule155Trade => {
                f.write_str( "H" )
            },
            &FieldTradeConditionEnum::SoldLast => {
                f.write_str( "I" )
            },
            &FieldTradeConditionEnum::NextDayTrade => {
                f.write_str( "J" )
            },
            &FieldTradeConditionEnum::Opened => {
                f.write_str( "K" )
            },
            &FieldTradeConditionEnum::Seller => {
                f.write_str( "L" )
            },
            &FieldTradeConditionEnum::Sold => {
                f.write_str( "M" )
            },
            &FieldTradeConditionEnum::StoppedStock => {
                f.write_str( "N" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldTradeConditionEnum {
    fn default() -> Self {
        FieldTradeConditionEnum::Undefined
    }
}
const FIELD_MDENTRYID : u32 = 278; // STRING


const FIELD_MDUPDATEACTION : u32 = 279; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldMDUpdateActionEnum {
    New, // = "0"
    Change, // = "1"
    Delete, // = "2"

    Undefined
}

impl FromStr for FieldMDUpdateActionEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldMDUpdateActionEnum::New),
            "1" => Ok(FieldMDUpdateActionEnum::Change),
            "2" => Ok(FieldMDUpdateActionEnum::Delete),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldMDUpdateActionEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldMDUpdateActionEnum::New => {
                f.write_str( "0" )
            },
            &FieldMDUpdateActionEnum::Change => {
                f.write_str( "1" )
            },
            &FieldMDUpdateActionEnum::Delete => {
                f.write_str( "2" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldMDUpdateActionEnum {
    fn default() -> Self {
        FieldMDUpdateActionEnum::Undefined
    }
}
const FIELD_MDENTRYREFID : u32 = 280; // STRING


const FIELD_MDREQREJREASON : u32 = 281; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldMDReqRejReasonEnum {
    UnknownSymbol, // = "0"
    DuplicateMdreqid, // = "1"
    InsufficientBandwidth, // = "2"
    InsufficientPermissions, // = "3"
    UnsupportedSubscriptionrequesttype, // = "4"
    UnsupportedMarketdepth, // = "5"
    UnsupportedMdupdatetype, // = "6"
    UnsupportedAggregatedbook, // = "7"
    UnsupportedMdentrytype, // = "8"

    Undefined
}

impl FromStr for FieldMDReqRejReasonEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldMDReqRejReasonEnum::UnknownSymbol),
            "1" => Ok(FieldMDReqRejReasonEnum::DuplicateMdreqid),
            "2" => Ok(FieldMDReqRejReasonEnum::InsufficientBandwidth),
            "3" => Ok(FieldMDReqRejReasonEnum::InsufficientPermissions),
            "4" => Ok(FieldMDReqRejReasonEnum::UnsupportedSubscriptionrequesttype),
            "5" => Ok(FieldMDReqRejReasonEnum::UnsupportedMarketdepth),
            "6" => Ok(FieldMDReqRejReasonEnum::UnsupportedMdupdatetype),
            "7" => Ok(FieldMDReqRejReasonEnum::UnsupportedAggregatedbook),
            "8" => Ok(FieldMDReqRejReasonEnum::UnsupportedMdentrytype),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldMDReqRejReasonEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldMDReqRejReasonEnum::UnknownSymbol => {
                f.write_str( "0" )
            },
            &FieldMDReqRejReasonEnum::DuplicateMdreqid => {
                f.write_str( "1" )
            },
            &FieldMDReqRejReasonEnum::InsufficientBandwidth => {
                f.write_str( "2" )
            },
            &FieldMDReqRejReasonEnum::InsufficientPermissions => {
                f.write_str( "3" )
            },
            &FieldMDReqRejReasonEnum::UnsupportedSubscriptionrequesttype => {
                f.write_str( "4" )
            },
            &FieldMDReqRejReasonEnum::UnsupportedMarketdepth => {
                f.write_str( "5" )
            },
            &FieldMDReqRejReasonEnum::UnsupportedMdupdatetype => {
                f.write_str( "6" )
            },
            &FieldMDReqRejReasonEnum::UnsupportedAggregatedbook => {
                f.write_str( "7" )
            },
            &FieldMDReqRejReasonEnum::UnsupportedMdentrytype => {
                f.write_str( "8" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldMDReqRejReasonEnum {
    fn default() -> Self {
        FieldMDReqRejReasonEnum::Undefined
    }
}
const FIELD_MDENTRYORIGINATOR : u32 = 282; // STRING


const FIELD_LOCATIONID : u32 = 283; // STRING


const FIELD_DESKID : u32 = 284; // STRING


const FIELD_DELETEREASON : u32 = 285; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldDeleteReasonEnum {
    Cancelation, // = "0"
    Error, // = "1"

    Undefined
}

impl FromStr for FieldDeleteReasonEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldDeleteReasonEnum::Cancelation),
            "1" => Ok(FieldDeleteReasonEnum::Error),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldDeleteReasonEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldDeleteReasonEnum::Cancelation => {
                f.write_str( "0" )
            },
            &FieldDeleteReasonEnum::Error => {
                f.write_str( "1" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldDeleteReasonEnum {
    fn default() -> Self {
        FieldDeleteReasonEnum::Undefined
    }
}
const FIELD_OPENCLOSESETTLEFLAG : u32 = 286; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldOpenCloseSettleFlagEnum {
    DailyOpen, // = "0"
    SessionOpen, // = "1"
    DeliverySettlementPrice, // = "2"

    Undefined
}

impl FromStr for FieldOpenCloseSettleFlagEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldOpenCloseSettleFlagEnum::DailyOpen),
            "1" => Ok(FieldOpenCloseSettleFlagEnum::SessionOpen),
            "2" => Ok(FieldOpenCloseSettleFlagEnum::DeliverySettlementPrice),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldOpenCloseSettleFlagEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldOpenCloseSettleFlagEnum::DailyOpen => {
                f.write_str( "0" )
            },
            &FieldOpenCloseSettleFlagEnum::SessionOpen => {
                f.write_str( "1" )
            },
            &FieldOpenCloseSettleFlagEnum::DeliverySettlementPrice => {
                f.write_str( "2" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldOpenCloseSettleFlagEnum {
    fn default() -> Self {
        FieldOpenCloseSettleFlagEnum::Undefined
    }
}
const FIELD_SELLERDAYS : u32 = 287; // INT


const FIELD_MDENTRYBUYER : u32 = 288; // STRING


const FIELD_MDENTRYSELLER : u32 = 289; // STRING


const FIELD_MDENTRYPOSITIONNO : u32 = 290; // INT


const FIELD_FINANCIALSTATUS : u32 = 291; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldFinancialStatusEnum {
    Bankrupt, // = "1"

    Undefined
}

impl FromStr for FieldFinancialStatusEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldFinancialStatusEnum::Bankrupt),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldFinancialStatusEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldFinancialStatusEnum::Bankrupt => {
                f.write_str( "1" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldFinancialStatusEnum {
    fn default() -> Self {
        FieldFinancialStatusEnum::Undefined
    }
}
const FIELD_CORPORATEACTION : u32 = 292; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldCorporateActionEnum {
    ExDividend, // = "A"
    ExDistribution, // = "B"
    ExRights, // = "C"
    New, // = "D"
    ExInterest, // = "E"

    Undefined
}

impl FromStr for FieldCorporateActionEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(FieldCorporateActionEnum::ExDividend),
            "B" => Ok(FieldCorporateActionEnum::ExDistribution),
            "C" => Ok(FieldCorporateActionEnum::ExRights),
            "D" => Ok(FieldCorporateActionEnum::New),
            "E" => Ok(FieldCorporateActionEnum::ExInterest),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldCorporateActionEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldCorporateActionEnum::ExDividend => {
                f.write_str( "A" )
            },
            &FieldCorporateActionEnum::ExDistribution => {
                f.write_str( "B" )
            },
            &FieldCorporateActionEnum::ExRights => {
                f.write_str( "C" )
            },
            &FieldCorporateActionEnum::New => {
                f.write_str( "D" )
            },
            &FieldCorporateActionEnum::ExInterest => {
                f.write_str( "E" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldCorporateActionEnum {
    fn default() -> Self {
        FieldCorporateActionEnum::Undefined
    }
}
const FIELD_DEFBIDSIZE : u32 = 293; // QTY


const FIELD_DEFOFFERSIZE : u32 = 294; // QTY


const FIELD_NOQUOTEENTRIES : u32 = 295; // INT


const FIELD_NOQUOTESETS : u32 = 296; // INT


const FIELD_QUOTEACKSTATUS : u32 = 297; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldQuoteAckStatusEnum {
    Accepted, // = "0"
    CanceledForSymbol, // = "1"
    CanceledForSecurityType, // = "2"
    CanceledForUnderlying, // = "3"
    CanceledAll, // = "4"
    Rejected, // = "5"

    Undefined
}

impl FromStr for FieldQuoteAckStatusEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldQuoteAckStatusEnum::Accepted),
            "1" => Ok(FieldQuoteAckStatusEnum::CanceledForSymbol),
            "2" => Ok(FieldQuoteAckStatusEnum::CanceledForSecurityType),
            "3" => Ok(FieldQuoteAckStatusEnum::CanceledForUnderlying),
            "4" => Ok(FieldQuoteAckStatusEnum::CanceledAll),
            "5" => Ok(FieldQuoteAckStatusEnum::Rejected),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldQuoteAckStatusEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldQuoteAckStatusEnum::Accepted => {
                f.write_str( "0" )
            },
            &FieldQuoteAckStatusEnum::CanceledForSymbol => {
                f.write_str( "1" )
            },
            &FieldQuoteAckStatusEnum::CanceledForSecurityType => {
                f.write_str( "2" )
            },
            &FieldQuoteAckStatusEnum::CanceledForUnderlying => {
                f.write_str( "3" )
            },
            &FieldQuoteAckStatusEnum::CanceledAll => {
                f.write_str( "4" )
            },
            &FieldQuoteAckStatusEnum::Rejected => {
                f.write_str( "5" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldQuoteAckStatusEnum {
    fn default() -> Self {
        FieldQuoteAckStatusEnum::Undefined
    }
}
const FIELD_QUOTECANCELTYPE : u32 = 298; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldQuoteCancelTypeEnum {
    CancelForSymbol, // = "1"
    CancelForSecurityType, // = "2"
    CancelForUnderlyingSymbol, // = "3"
    CancelForAllQuotes, // = "4"

    Undefined
}

impl FromStr for FieldQuoteCancelTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldQuoteCancelTypeEnum::CancelForSymbol),
            "2" => Ok(FieldQuoteCancelTypeEnum::CancelForSecurityType),
            "3" => Ok(FieldQuoteCancelTypeEnum::CancelForUnderlyingSymbol),
            "4" => Ok(FieldQuoteCancelTypeEnum::CancelForAllQuotes),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldQuoteCancelTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldQuoteCancelTypeEnum::CancelForSymbol => {
                f.write_str( "1" )
            },
            &FieldQuoteCancelTypeEnum::CancelForSecurityType => {
                f.write_str( "2" )
            },
            &FieldQuoteCancelTypeEnum::CancelForUnderlyingSymbol => {
                f.write_str( "3" )
            },
            &FieldQuoteCancelTypeEnum::CancelForAllQuotes => {
                f.write_str( "4" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldQuoteCancelTypeEnum {
    fn default() -> Self {
        FieldQuoteCancelTypeEnum::Undefined
    }
}
const FIELD_QUOTEENTRYID : u32 = 299; // STRING


const FIELD_QUOTEREJECTREASON : u32 = 300; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldQuoteRejectReasonEnum {
    UnknownSymbol, // = "1"
    Exchange, // = "2"
    QuoteRequestExceedsLimit, // = "3"
    TooLateToEnter, // = "4"
    UnknownQuote, // = "5"
    DuplicateQuote, // = "6"
    InvalidBidAskSpread, // = "7"
    InvalidPrice, // = "8"
    NotAuthorizedToQuoteSecurity, // = "9"

    Undefined
}

impl FromStr for FieldQuoteRejectReasonEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldQuoteRejectReasonEnum::UnknownSymbol),
            "2" => Ok(FieldQuoteRejectReasonEnum::Exchange),
            "3" => Ok(FieldQuoteRejectReasonEnum::QuoteRequestExceedsLimit),
            "4" => Ok(FieldQuoteRejectReasonEnum::TooLateToEnter),
            "5" => Ok(FieldQuoteRejectReasonEnum::UnknownQuote),
            "6" => Ok(FieldQuoteRejectReasonEnum::DuplicateQuote),
            "7" => Ok(FieldQuoteRejectReasonEnum::InvalidBidAskSpread),
            "8" => Ok(FieldQuoteRejectReasonEnum::InvalidPrice),
            "9" => Ok(FieldQuoteRejectReasonEnum::NotAuthorizedToQuoteSecurity),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldQuoteRejectReasonEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldQuoteRejectReasonEnum::UnknownSymbol => {
                f.write_str( "1" )
            },
            &FieldQuoteRejectReasonEnum::Exchange => {
                f.write_str( "2" )
            },
            &FieldQuoteRejectReasonEnum::QuoteRequestExceedsLimit => {
                f.write_str( "3" )
            },
            &FieldQuoteRejectReasonEnum::TooLateToEnter => {
                f.write_str( "4" )
            },
            &FieldQuoteRejectReasonEnum::UnknownQuote => {
                f.write_str( "5" )
            },
            &FieldQuoteRejectReasonEnum::DuplicateQuote => {
                f.write_str( "6" )
            },
            &FieldQuoteRejectReasonEnum::InvalidBidAskSpread => {
                f.write_str( "7" )
            },
            &FieldQuoteRejectReasonEnum::InvalidPrice => {
                f.write_str( "8" )
            },
            &FieldQuoteRejectReasonEnum::NotAuthorizedToQuoteSecurity => {
                f.write_str( "9" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldQuoteRejectReasonEnum {
    fn default() -> Self {
        FieldQuoteRejectReasonEnum::Undefined
    }
}
const FIELD_QUOTERESPONSELEVEL : u32 = 301; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldQuoteResponseLevelEnum {
    NoAcknowledgement, // = "0"
    AcknowledgeOnlyNegativeOrErroneousQuotes, // = "1"
    AcknowledgeEachQuoteMessages, // = "2"

    Undefined
}

impl FromStr for FieldQuoteResponseLevelEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldQuoteResponseLevelEnum::NoAcknowledgement),
            "1" => Ok(FieldQuoteResponseLevelEnum::AcknowledgeOnlyNegativeOrErroneousQuotes),
            "2" => Ok(FieldQuoteResponseLevelEnum::AcknowledgeEachQuoteMessages),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldQuoteResponseLevelEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldQuoteResponseLevelEnum::NoAcknowledgement => {
                f.write_str( "0" )
            },
            &FieldQuoteResponseLevelEnum::AcknowledgeOnlyNegativeOrErroneousQuotes => {
                f.write_str( "1" )
            },
            &FieldQuoteResponseLevelEnum::AcknowledgeEachQuoteMessages => {
                f.write_str( "2" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldQuoteResponseLevelEnum {
    fn default() -> Self {
        FieldQuoteResponseLevelEnum::Undefined
    }
}
const FIELD_QUOTESETID : u32 = 302; // STRING


const FIELD_QUOTEREQUESTTYPE : u32 = 303; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldQuoteRequestTypeEnum {
    Manual, // = "1"
    Automatic, // = "2"

    Undefined
}

impl FromStr for FieldQuoteRequestTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldQuoteRequestTypeEnum::Manual),
            "2" => Ok(FieldQuoteRequestTypeEnum::Automatic),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldQuoteRequestTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldQuoteRequestTypeEnum::Manual => {
                f.write_str( "1" )
            },
            &FieldQuoteRequestTypeEnum::Automatic => {
                f.write_str( "2" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldQuoteRequestTypeEnum {
    fn default() -> Self {
        FieldQuoteRequestTypeEnum::Undefined
    }
}
const FIELD_TOTQUOTEENTRIES : u32 = 304; // INT


const FIELD_UNDERLYINGIDSOURCE : u32 = 305; // STRING


const FIELD_UNDERLYINGISSUER : u32 = 306; // STRING


const FIELD_UNDERLYINGSECURITYDESC : u32 = 307; // STRING


const FIELD_UNDERLYINGSECURITYEXCHANGE : u32 = 308; // EXCHANGE


const FIELD_UNDERLYINGSECURITYID : u32 = 309; // STRING


const FIELD_UNDERLYINGSECURITYTYPE : u32 = 310; // STRING


const FIELD_UNDERLYINGSYMBOL : u32 = 311; // STRING


const FIELD_UNDERLYINGSYMBOLSFX : u32 = 312; // STRING


const FIELD_UNDERLYINGMATURITYMONTHYEAR : u32 = 313; // MONTHYEAR


const FIELD_UNDERLYINGMATURITYDAY : u32 = 314; // DAYOFMONTH


const FIELD_UNDERLYINGPUTORCALL : u32 = 315; // INT


const FIELD_UNDERLYINGSTRIKEPRICE : u32 = 316; // PRICE


const FIELD_UNDERLYINGOPTATTRIBUTE : u32 = 317; // CHAR


const FIELD_UNDERLYINGCURRENCY : u32 = 318; // CURRENCY


const FIELD_RATIOQTY : u32 = 319; // QTY


const FIELD_SECURITYREQID : u32 = 320; // STRING


const FIELD_SECURITYREQUESTTYPE : u32 = 321; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldSecurityRequestTypeEnum {
    RequestSecurityIdentityAndSpecifications, // = "0"
    RequestSecurityIdentityForTheSpecificationsProvided, // = "1"
    RequestListSecurityTypes, // = "2"
    RequestListSecurities, // = "3"

    Undefined
}

impl FromStr for FieldSecurityRequestTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldSecurityRequestTypeEnum::RequestSecurityIdentityAndSpecifications),
            "1" => Ok(FieldSecurityRequestTypeEnum::RequestSecurityIdentityForTheSpecificationsProvided),
            "2" => Ok(FieldSecurityRequestTypeEnum::RequestListSecurityTypes),
            "3" => Ok(FieldSecurityRequestTypeEnum::RequestListSecurities),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldSecurityRequestTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldSecurityRequestTypeEnum::RequestSecurityIdentityAndSpecifications => {
                f.write_str( "0" )
            },
            &FieldSecurityRequestTypeEnum::RequestSecurityIdentityForTheSpecificationsProvided => {
                f.write_str( "1" )
            },
            &FieldSecurityRequestTypeEnum::RequestListSecurityTypes => {
                f.write_str( "2" )
            },
            &FieldSecurityRequestTypeEnum::RequestListSecurities => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldSecurityRequestTypeEnum {
    fn default() -> Self {
        FieldSecurityRequestTypeEnum::Undefined
    }
}
const FIELD_SECURITYRESPONSEID : u32 = 322; // STRING


const FIELD_SECURITYRESPONSETYPE : u32 = 323; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldSecurityResponseTypeEnum {
    AcceptSecurityProposalAsIs, // = "1"
    AcceptSecurityProposalWithRevisionsAsIndicatedInTheMessage, // = "2"
    ListOfSecurityTypesReturnedPerRequest, // = "3"
    ListOfSecuritiesReturnedPerRequest, // = "4"
    RejectSecurityProposal, // = "5"
    CanNotMatchSelectionCriteria, // = "6"

    Undefined
}

impl FromStr for FieldSecurityResponseTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldSecurityResponseTypeEnum::AcceptSecurityProposalAsIs),
            "2" => Ok(FieldSecurityResponseTypeEnum::AcceptSecurityProposalWithRevisionsAsIndicatedInTheMessage),
            "3" => Ok(FieldSecurityResponseTypeEnum::ListOfSecurityTypesReturnedPerRequest),
            "4" => Ok(FieldSecurityResponseTypeEnum::ListOfSecuritiesReturnedPerRequest),
            "5" => Ok(FieldSecurityResponseTypeEnum::RejectSecurityProposal),
            "6" => Ok(FieldSecurityResponseTypeEnum::CanNotMatchSelectionCriteria),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldSecurityResponseTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldSecurityResponseTypeEnum::AcceptSecurityProposalAsIs => {
                f.write_str( "1" )
            },
            &FieldSecurityResponseTypeEnum::AcceptSecurityProposalWithRevisionsAsIndicatedInTheMessage => {
                f.write_str( "2" )
            },
            &FieldSecurityResponseTypeEnum::ListOfSecurityTypesReturnedPerRequest => {
                f.write_str( "3" )
            },
            &FieldSecurityResponseTypeEnum::ListOfSecuritiesReturnedPerRequest => {
                f.write_str( "4" )
            },
            &FieldSecurityResponseTypeEnum::RejectSecurityProposal => {
                f.write_str( "5" )
            },
            &FieldSecurityResponseTypeEnum::CanNotMatchSelectionCriteria => {
                f.write_str( "6" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldSecurityResponseTypeEnum {
    fn default() -> Self {
        FieldSecurityResponseTypeEnum::Undefined
    }
}
const FIELD_SECURITYSTATUSREQID : u32 = 324; // STRING


const FIELD_UNSOLICITEDINDICATOR : u32 = 325; // BOOLEAN


const FIELD_SECURITYTRADINGSTATUS : u32 = 326; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldSecurityTradingStatusEnum {
    OpeningDelay, // = "1"
    MarketOnCloseImbalanceSell, // = "10"
    NoMarketImbalance, // = "12"
    NoMarketOnCloseImbalance, // = "13"
    ItsPreOpening, // = "14"
    NewPriceIndication, // = "15"
    TradeDisseminationTime, // = "16"
    ReadyToTrade, // = "17"
    NotAvailableForTrading, // = "18"
    NotTradedOnThisMarket, // = "19"
    TradingHalt, // = "2"
    UnknownOrInvalid, // = "20"
    Resume, // = "3"
    NoOpenNoResume, // = "4"
    PriceIndication, // = "5"
    TradingRangeIndication, // = "6"
    MarketImbalanceBuy, // = "7"
    MarketImbalanceSell, // = "8"
    MarketOnCloseImbalanceBuy, // = "9"

    Undefined
}

impl FromStr for FieldSecurityTradingStatusEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldSecurityTradingStatusEnum::OpeningDelay),
            "10" => Ok(FieldSecurityTradingStatusEnum::MarketOnCloseImbalanceSell),
            "12" => Ok(FieldSecurityTradingStatusEnum::NoMarketImbalance),
            "13" => Ok(FieldSecurityTradingStatusEnum::NoMarketOnCloseImbalance),
            "14" => Ok(FieldSecurityTradingStatusEnum::ItsPreOpening),
            "15" => Ok(FieldSecurityTradingStatusEnum::NewPriceIndication),
            "16" => Ok(FieldSecurityTradingStatusEnum::TradeDisseminationTime),
            "17" => Ok(FieldSecurityTradingStatusEnum::ReadyToTrade),
            "18" => Ok(FieldSecurityTradingStatusEnum::NotAvailableForTrading),
            "19" => Ok(FieldSecurityTradingStatusEnum::NotTradedOnThisMarket),
            "2" => Ok(FieldSecurityTradingStatusEnum::TradingHalt),
            "20" => Ok(FieldSecurityTradingStatusEnum::UnknownOrInvalid),
            "3" => Ok(FieldSecurityTradingStatusEnum::Resume),
            "4" => Ok(FieldSecurityTradingStatusEnum::NoOpenNoResume),
            "5" => Ok(FieldSecurityTradingStatusEnum::PriceIndication),
            "6" => Ok(FieldSecurityTradingStatusEnum::TradingRangeIndication),
            "7" => Ok(FieldSecurityTradingStatusEnum::MarketImbalanceBuy),
            "8" => Ok(FieldSecurityTradingStatusEnum::MarketImbalanceSell),
            "9" => Ok(FieldSecurityTradingStatusEnum::MarketOnCloseImbalanceBuy),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldSecurityTradingStatusEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldSecurityTradingStatusEnum::OpeningDelay => {
                f.write_str( "1" )
            },
            &FieldSecurityTradingStatusEnum::MarketOnCloseImbalanceSell => {
                f.write_str( "10" )
            },
            &FieldSecurityTradingStatusEnum::NoMarketImbalance => {
                f.write_str( "12" )
            },
            &FieldSecurityTradingStatusEnum::NoMarketOnCloseImbalance => {
                f.write_str( "13" )
            },
            &FieldSecurityTradingStatusEnum::ItsPreOpening => {
                f.write_str( "14" )
            },
            &FieldSecurityTradingStatusEnum::NewPriceIndication => {
                f.write_str( "15" )
            },
            &FieldSecurityTradingStatusEnum::TradeDisseminationTime => {
                f.write_str( "16" )
            },
            &FieldSecurityTradingStatusEnum::ReadyToTrade => {
                f.write_str( "17" )
            },
            &FieldSecurityTradingStatusEnum::NotAvailableForTrading => {
                f.write_str( "18" )
            },
            &FieldSecurityTradingStatusEnum::NotTradedOnThisMarket => {
                f.write_str( "19" )
            },
            &FieldSecurityTradingStatusEnum::TradingHalt => {
                f.write_str( "2" )
            },
            &FieldSecurityTradingStatusEnum::UnknownOrInvalid => {
                f.write_str( "20" )
            },
            &FieldSecurityTradingStatusEnum::Resume => {
                f.write_str( "3" )
            },
            &FieldSecurityTradingStatusEnum::NoOpenNoResume => {
                f.write_str( "4" )
            },
            &FieldSecurityTradingStatusEnum::PriceIndication => {
                f.write_str( "5" )
            },
            &FieldSecurityTradingStatusEnum::TradingRangeIndication => {
                f.write_str( "6" )
            },
            &FieldSecurityTradingStatusEnum::MarketImbalanceBuy => {
                f.write_str( "7" )
            },
            &FieldSecurityTradingStatusEnum::MarketImbalanceSell => {
                f.write_str( "8" )
            },
            &FieldSecurityTradingStatusEnum::MarketOnCloseImbalanceBuy => {
                f.write_str( "9" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldSecurityTradingStatusEnum {
    fn default() -> Self {
        FieldSecurityTradingStatusEnum::Undefined
    }
}
const FIELD_HALTREASONCHAR : u32 = 327; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldHaltReasonCharEnum {
    NewsDissemination, // = "D"
    OrderInflux, // = "E"
    OrderImbalance, // = "I"
    AdditionalInformation, // = "M"
    NewsPending, // = "P"
    EquipmentChangeover, // = "X"

    Undefined
}

impl FromStr for FieldHaltReasonCharEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "D" => Ok(FieldHaltReasonCharEnum::NewsDissemination),
            "E" => Ok(FieldHaltReasonCharEnum::OrderInflux),
            "I" => Ok(FieldHaltReasonCharEnum::OrderImbalance),
            "M" => Ok(FieldHaltReasonCharEnum::AdditionalInformation),
            "P" => Ok(FieldHaltReasonCharEnum::NewsPending),
            "X" => Ok(FieldHaltReasonCharEnum::EquipmentChangeover),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldHaltReasonCharEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldHaltReasonCharEnum::NewsDissemination => {
                f.write_str( "D" )
            },
            &FieldHaltReasonCharEnum::OrderInflux => {
                f.write_str( "E" )
            },
            &FieldHaltReasonCharEnum::OrderImbalance => {
                f.write_str( "I" )
            },
            &FieldHaltReasonCharEnum::AdditionalInformation => {
                f.write_str( "M" )
            },
            &FieldHaltReasonCharEnum::NewsPending => {
                f.write_str( "P" )
            },
            &FieldHaltReasonCharEnum::EquipmentChangeover => {
                f.write_str( "X" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldHaltReasonCharEnum {
    fn default() -> Self {
        FieldHaltReasonCharEnum::Undefined
    }
}
const FIELD_INVIEWOFCOMMON : u32 = 328; // BOOLEAN


const FIELD_DUETORELATED : u32 = 329; // BOOLEAN


const FIELD_BUYVOLUME : u32 = 330; // QTY


const FIELD_SELLVOLUME : u32 = 331; // QTY


const FIELD_HIGHPX : u32 = 332; // PRICE


const FIELD_LOWPX : u32 = 333; // PRICE


const FIELD_ADJUSTMENT : u32 = 334; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldAdjustmentEnum {
    Cancel, // = "1"
    Error, // = "2"
    Correction, // = "3"

    Undefined
}

impl FromStr for FieldAdjustmentEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldAdjustmentEnum::Cancel),
            "2" => Ok(FieldAdjustmentEnum::Error),
            "3" => Ok(FieldAdjustmentEnum::Correction),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldAdjustmentEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldAdjustmentEnum::Cancel => {
                f.write_str( "1" )
            },
            &FieldAdjustmentEnum::Error => {
                f.write_str( "2" )
            },
            &FieldAdjustmentEnum::Correction => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldAdjustmentEnum {
    fn default() -> Self {
        FieldAdjustmentEnum::Undefined
    }
}
const FIELD_TRADSESREQID : u32 = 335; // STRING


const FIELD_TRADINGSESSIONID : u32 = 336; // STRING


const FIELD_CONTRATRADER : u32 = 337; // STRING


const FIELD_TRADSESMETHOD : u32 = 338; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldTradSesMethodEnum {
    Electronic, // = "1"
    OpenOutcry, // = "2"
    TwoParty, // = "3"

    Undefined
}

impl FromStr for FieldTradSesMethodEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldTradSesMethodEnum::Electronic),
            "2" => Ok(FieldTradSesMethodEnum::OpenOutcry),
            "3" => Ok(FieldTradSesMethodEnum::TwoParty),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldTradSesMethodEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldTradSesMethodEnum::Electronic => {
                f.write_str( "1" )
            },
            &FieldTradSesMethodEnum::OpenOutcry => {
                f.write_str( "2" )
            },
            &FieldTradSesMethodEnum::TwoParty => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldTradSesMethodEnum {
    fn default() -> Self {
        FieldTradSesMethodEnum::Undefined
    }
}
const FIELD_TRADSESMODE : u32 = 339; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldTradSesModeEnum {
    Testing, // = "1"
    Simulated, // = "2"
    Production, // = "3"

    Undefined
}

impl FromStr for FieldTradSesModeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldTradSesModeEnum::Testing),
            "2" => Ok(FieldTradSesModeEnum::Simulated),
            "3" => Ok(FieldTradSesModeEnum::Production),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldTradSesModeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldTradSesModeEnum::Testing => {
                f.write_str( "1" )
            },
            &FieldTradSesModeEnum::Simulated => {
                f.write_str( "2" )
            },
            &FieldTradSesModeEnum::Production => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldTradSesModeEnum {
    fn default() -> Self {
        FieldTradSesModeEnum::Undefined
    }
}
const FIELD_TRADSESSTATUS : u32 = 340; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldTradSesStatusEnum {
    Halted, // = "1"
    Open, // = "2"
    Closed, // = "3"
    PreOpen, // = "4"
    PreClose, // = "5"

    Undefined
}

impl FromStr for FieldTradSesStatusEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldTradSesStatusEnum::Halted),
            "2" => Ok(FieldTradSesStatusEnum::Open),
            "3" => Ok(FieldTradSesStatusEnum::Closed),
            "4" => Ok(FieldTradSesStatusEnum::PreOpen),
            "5" => Ok(FieldTradSesStatusEnum::PreClose),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldTradSesStatusEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldTradSesStatusEnum::Halted => {
                f.write_str( "1" )
            },
            &FieldTradSesStatusEnum::Open => {
                f.write_str( "2" )
            },
            &FieldTradSesStatusEnum::Closed => {
                f.write_str( "3" )
            },
            &FieldTradSesStatusEnum::PreOpen => {
                f.write_str( "4" )
            },
            &FieldTradSesStatusEnum::PreClose => {
                f.write_str( "5" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldTradSesStatusEnum {
    fn default() -> Self {
        FieldTradSesStatusEnum::Undefined
    }
}
const FIELD_TRADSESSTARTTIME : u32 = 341; // UTCTIMESTAMP


const FIELD_TRADSESOPENTIME : u32 = 342; // UTCTIMESTAMP


const FIELD_TRADSESPRECLOSETIME : u32 = 343; // UTCTIMESTAMP


const FIELD_TRADSESCLOSETIME : u32 = 344; // UTCTIMESTAMP


const FIELD_TRADSESENDTIME : u32 = 345; // UTCTIMESTAMP


const FIELD_NUMBEROFORDERS : u32 = 346; // INT


const FIELD_MESSAGEENCODING : u32 = 347; // STRING


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldMessageEncodingEnum {
    EucJp, // = "EUC-JP"
    Iso2022Jp, // = "ISO-2022-JP"
    ShiftJis, // = "SHIFT_JIS"
    Utf8, // = "UTF-8"

    Undefined
}

impl FromStr for FieldMessageEncodingEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EUC-JP" => Ok(FieldMessageEncodingEnum::EucJp),
            "ISO-2022-JP" => Ok(FieldMessageEncodingEnum::Iso2022Jp),
            "SHIFT_JIS" => Ok(FieldMessageEncodingEnum::ShiftJis),
            "UTF-8" => Ok(FieldMessageEncodingEnum::Utf8),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldMessageEncodingEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldMessageEncodingEnum::EucJp => {
                f.write_str( "EUC-JP" )
            },
            &FieldMessageEncodingEnum::Iso2022Jp => {
                f.write_str( "ISO-2022-JP" )
            },
            &FieldMessageEncodingEnum::ShiftJis => {
                f.write_str( "SHIFT_JIS" )
            },
            &FieldMessageEncodingEnum::Utf8 => {
                f.write_str( "UTF-8" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldMessageEncodingEnum {
    fn default() -> Self {
        FieldMessageEncodingEnum::Undefined
    }
}
const FIELD_ENCODEDISSUERLEN : u32 = 348; // LENGTH


const FIELD_ENCODEDISSUER : u32 = 349; // DATA


const FIELD_ENCODEDSECURITYDESCLEN : u32 = 350; // LENGTH


const FIELD_ENCODEDSECURITYDESC : u32 = 351; // DATA


const FIELD_ENCODEDLISTEXECINSTLEN : u32 = 352; // LENGTH


const FIELD_ENCODEDLISTEXECINST : u32 = 353; // DATA


const FIELD_ENCODEDTEXTLEN : u32 = 354; // LENGTH


const FIELD_ENCODEDTEXT : u32 = 355; // DATA


const FIELD_ENCODEDSUBJECTLEN : u32 = 356; // LENGTH


const FIELD_ENCODEDSUBJECT : u32 = 357; // DATA


const FIELD_ENCODEDHEADLINELEN : u32 = 358; // LENGTH


const FIELD_ENCODEDHEADLINE : u32 = 359; // DATA


const FIELD_ENCODEDALLOCTEXTLEN : u32 = 360; // LENGTH


const FIELD_ENCODEDALLOCTEXT : u32 = 361; // DATA


const FIELD_ENCODEDUNDERLYINGISSUERLEN : u32 = 362; // LENGTH


const FIELD_ENCODEDUNDERLYINGISSUER : u32 = 363; // DATA


const FIELD_ENCODEDUNDERLYINGSECURITYDESCLEN : u32 = 364; // LENGTH


const FIELD_ENCODEDUNDERLYINGSECURITYDESC : u32 = 365; // DATA


const FIELD_ALLOCPRICE : u32 = 366; // PRICE


const FIELD_QUOTESETVALIDUNTILTIME : u32 = 367; // UTCTIMESTAMP


const FIELD_QUOTEENTRYREJECTREASON : u32 = 368; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldQuoteEntryRejectReasonEnum {
    UnknownSymbol, // = "1"
    Exchange, // = "2"
    QuoteExceedsLimit, // = "3"
    TooLateToEnter, // = "4"
    UnknownQuote, // = "5"
    DuplicateQuote, // = "6"
    InvalidBidAskSpread, // = "7"
    InvalidPrice, // = "8"
    NotAuthorizedToQuoteSecurity, // = "9"

    Undefined
}

impl FromStr for FieldQuoteEntryRejectReasonEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldQuoteEntryRejectReasonEnum::UnknownSymbol),
            "2" => Ok(FieldQuoteEntryRejectReasonEnum::Exchange),
            "3" => Ok(FieldQuoteEntryRejectReasonEnum::QuoteExceedsLimit),
            "4" => Ok(FieldQuoteEntryRejectReasonEnum::TooLateToEnter),
            "5" => Ok(FieldQuoteEntryRejectReasonEnum::UnknownQuote),
            "6" => Ok(FieldQuoteEntryRejectReasonEnum::DuplicateQuote),
            "7" => Ok(FieldQuoteEntryRejectReasonEnum::InvalidBidAskSpread),
            "8" => Ok(FieldQuoteEntryRejectReasonEnum::InvalidPrice),
            "9" => Ok(FieldQuoteEntryRejectReasonEnum::NotAuthorizedToQuoteSecurity),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldQuoteEntryRejectReasonEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldQuoteEntryRejectReasonEnum::UnknownSymbol => {
                f.write_str( "1" )
            },
            &FieldQuoteEntryRejectReasonEnum::Exchange => {
                f.write_str( "2" )
            },
            &FieldQuoteEntryRejectReasonEnum::QuoteExceedsLimit => {
                f.write_str( "3" )
            },
            &FieldQuoteEntryRejectReasonEnum::TooLateToEnter => {
                f.write_str( "4" )
            },
            &FieldQuoteEntryRejectReasonEnum::UnknownQuote => {
                f.write_str( "5" )
            },
            &FieldQuoteEntryRejectReasonEnum::DuplicateQuote => {
                f.write_str( "6" )
            },
            &FieldQuoteEntryRejectReasonEnum::InvalidBidAskSpread => {
                f.write_str( "7" )
            },
            &FieldQuoteEntryRejectReasonEnum::InvalidPrice => {
                f.write_str( "8" )
            },
            &FieldQuoteEntryRejectReasonEnum::NotAuthorizedToQuoteSecurity => {
                f.write_str( "9" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldQuoteEntryRejectReasonEnum {
    fn default() -> Self {
        FieldQuoteEntryRejectReasonEnum::Undefined
    }
}
const FIELD_LASTMSGSEQNUMPROCESSED : u32 = 369; // INT


const FIELD_ONBEHALFOFSENDINGTIME : u32 = 370; // UTCTIMESTAMP


const FIELD_REFTAGID : u32 = 371; // INT


const FIELD_REFMSGTYPE : u32 = 372; // STRING


const FIELD_SESSIONREJECTREASON : u32 = 373; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldSessionRejectReasonEnum {
    InvalidTagNumber, // = "0"
    RequiredTagMissing, // = "1"
    SendingtimeAccuracyProblem, // = "10"
    InvalidMsgtype, // = "11"
    TagNotDefinedForThisMessageType, // = "2"
    UndefinedTag, // = "3"
    TagSpecifiedWithoutAValue, // = "4"
    ValueIsIncorrect, // = "5"
    IncorrectDataFormatForValue, // = "6"
    DecryptionProblem, // = "7"
    SignatureProblem, // = "8"
    CompidProblem, // = "9"

    Undefined
}

impl FromStr for FieldSessionRejectReasonEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldSessionRejectReasonEnum::InvalidTagNumber),
            "1" => Ok(FieldSessionRejectReasonEnum::RequiredTagMissing),
            "10" => Ok(FieldSessionRejectReasonEnum::SendingtimeAccuracyProblem),
            "11" => Ok(FieldSessionRejectReasonEnum::InvalidMsgtype),
            "2" => Ok(FieldSessionRejectReasonEnum::TagNotDefinedForThisMessageType),
            "3" => Ok(FieldSessionRejectReasonEnum::UndefinedTag),
            "4" => Ok(FieldSessionRejectReasonEnum::TagSpecifiedWithoutAValue),
            "5" => Ok(FieldSessionRejectReasonEnum::ValueIsIncorrect),
            "6" => Ok(FieldSessionRejectReasonEnum::IncorrectDataFormatForValue),
            "7" => Ok(FieldSessionRejectReasonEnum::DecryptionProblem),
            "8" => Ok(FieldSessionRejectReasonEnum::SignatureProblem),
            "9" => Ok(FieldSessionRejectReasonEnum::CompidProblem),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldSessionRejectReasonEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldSessionRejectReasonEnum::InvalidTagNumber => {
                f.write_str( "0" )
            },
            &FieldSessionRejectReasonEnum::RequiredTagMissing => {
                f.write_str( "1" )
            },
            &FieldSessionRejectReasonEnum::SendingtimeAccuracyProblem => {
                f.write_str( "10" )
            },
            &FieldSessionRejectReasonEnum::InvalidMsgtype => {
                f.write_str( "11" )
            },
            &FieldSessionRejectReasonEnum::TagNotDefinedForThisMessageType => {
                f.write_str( "2" )
            },
            &FieldSessionRejectReasonEnum::UndefinedTag => {
                f.write_str( "3" )
            },
            &FieldSessionRejectReasonEnum::TagSpecifiedWithoutAValue => {
                f.write_str( "4" )
            },
            &FieldSessionRejectReasonEnum::ValueIsIncorrect => {
                f.write_str( "5" )
            },
            &FieldSessionRejectReasonEnum::IncorrectDataFormatForValue => {
                f.write_str( "6" )
            },
            &FieldSessionRejectReasonEnum::DecryptionProblem => {
                f.write_str( "7" )
            },
            &FieldSessionRejectReasonEnum::SignatureProblem => {
                f.write_str( "8" )
            },
            &FieldSessionRejectReasonEnum::CompidProblem => {
                f.write_str( "9" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldSessionRejectReasonEnum {
    fn default() -> Self {
        FieldSessionRejectReasonEnum::Undefined
    }
}
const FIELD_BIDREQUESTTRANSTYPE : u32 = 374; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldBidRequestTransTypeEnum {
    Cancel, // = "C"
    No, // = "N"

    Undefined
}

impl FromStr for FieldBidRequestTransTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(FieldBidRequestTransTypeEnum::Cancel),
            "N" => Ok(FieldBidRequestTransTypeEnum::No),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldBidRequestTransTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldBidRequestTransTypeEnum::Cancel => {
                f.write_str( "C" )
            },
            &FieldBidRequestTransTypeEnum::No => {
                f.write_str( "N" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldBidRequestTransTypeEnum {
    fn default() -> Self {
        FieldBidRequestTransTypeEnum::Undefined
    }
}
const FIELD_CONTRABROKER : u32 = 375; // STRING


const FIELD_COMPLIANCEID : u32 = 376; // STRING


const FIELD_SOLICITEDFLAG : u32 = 377; // BOOLEAN


const FIELD_EXECRESTATEMENTREASON : u32 = 378; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldExecRestatementReasonEnum {
    GtCorporateAction, // = "0"
    GtRenewal, // = "1"
    VerbalChange, // = "2"
    RepricingOfOrder, // = "3"
    BrokerOption, // = "4"
    PartialDeclineOfOrderqty, // = "5"

    Undefined
}

impl FromStr for FieldExecRestatementReasonEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldExecRestatementReasonEnum::GtCorporateAction),
            "1" => Ok(FieldExecRestatementReasonEnum::GtRenewal),
            "2" => Ok(FieldExecRestatementReasonEnum::VerbalChange),
            "3" => Ok(FieldExecRestatementReasonEnum::RepricingOfOrder),
            "4" => Ok(FieldExecRestatementReasonEnum::BrokerOption),
            "5" => Ok(FieldExecRestatementReasonEnum::PartialDeclineOfOrderqty),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldExecRestatementReasonEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldExecRestatementReasonEnum::GtCorporateAction => {
                f.write_str( "0" )
            },
            &FieldExecRestatementReasonEnum::GtRenewal => {
                f.write_str( "1" )
            },
            &FieldExecRestatementReasonEnum::VerbalChange => {
                f.write_str( "2" )
            },
            &FieldExecRestatementReasonEnum::RepricingOfOrder => {
                f.write_str( "3" )
            },
            &FieldExecRestatementReasonEnum::BrokerOption => {
                f.write_str( "4" )
            },
            &FieldExecRestatementReasonEnum::PartialDeclineOfOrderqty => {
                f.write_str( "5" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldExecRestatementReasonEnum {
    fn default() -> Self {
        FieldExecRestatementReasonEnum::Undefined
    }
}
const FIELD_BUSINESSREJECTREFID : u32 = 379; // STRING


const FIELD_BUSINESSREJECTREASON : u32 = 380; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldBusinessRejectReasonEnum {
    Other, // = "0"
    UnkownId, // = "1"
    UnknownSecurity, // = "2"
    UnsupportedMessageType, // = "3"
    ApplicationNotAvailable, // = "4"
    ConditionallyRequiredFieldMissing, // = "5"

    Undefined
}

impl FromStr for FieldBusinessRejectReasonEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldBusinessRejectReasonEnum::Other),
            "1" => Ok(FieldBusinessRejectReasonEnum::UnkownId),
            "2" => Ok(FieldBusinessRejectReasonEnum::UnknownSecurity),
            "3" => Ok(FieldBusinessRejectReasonEnum::UnsupportedMessageType),
            "4" => Ok(FieldBusinessRejectReasonEnum::ApplicationNotAvailable),
            "5" => Ok(FieldBusinessRejectReasonEnum::ConditionallyRequiredFieldMissing),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldBusinessRejectReasonEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldBusinessRejectReasonEnum::Other => {
                f.write_str( "0" )
            },
            &FieldBusinessRejectReasonEnum::UnkownId => {
                f.write_str( "1" )
            },
            &FieldBusinessRejectReasonEnum::UnknownSecurity => {
                f.write_str( "2" )
            },
            &FieldBusinessRejectReasonEnum::UnsupportedMessageType => {
                f.write_str( "3" )
            },
            &FieldBusinessRejectReasonEnum::ApplicationNotAvailable => {
                f.write_str( "4" )
            },
            &FieldBusinessRejectReasonEnum::ConditionallyRequiredFieldMissing => {
                f.write_str( "5" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldBusinessRejectReasonEnum {
    fn default() -> Self {
        FieldBusinessRejectReasonEnum::Undefined
    }
}
const FIELD_GROSSTRADEAMT : u32 = 381; // AMT


const FIELD_NOCONTRABROKERS : u32 = 382; // INT


const FIELD_MAXMESSAGESIZE : u32 = 383; // INT


const FIELD_NOMSGTYPES : u32 = 384; // INT


const FIELD_MSGDIRECTION : u32 = 385; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldMsgDirectionEnum {
    Receive, // = "R"
    Send, // = "S"

    Undefined
}

impl FromStr for FieldMsgDirectionEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(FieldMsgDirectionEnum::Receive),
            "S" => Ok(FieldMsgDirectionEnum::Send),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldMsgDirectionEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldMsgDirectionEnum::Receive => {
                f.write_str( "R" )
            },
            &FieldMsgDirectionEnum::Send => {
                f.write_str( "S" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldMsgDirectionEnum {
    fn default() -> Self {
        FieldMsgDirectionEnum::Undefined
    }
}
const FIELD_NOTRADINGSESSIONS : u32 = 386; // INT


const FIELD_TOTALVOLUMETRADED : u32 = 387; // QTY


const FIELD_DISCRETIONINST : u32 = 388; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldDiscretionInstEnum {
    RelatedToDisplayedPrice, // = "0"
    RelatedToMarketPrice, // = "1"
    RelatedToPrimaryPrice, // = "2"
    RelatedToLocalPrimaryPrice, // = "3"
    RelatedToMidpointPrice, // = "4"
    RelatedToLastTradePrice, // = "5"

    Undefined
}

impl FromStr for FieldDiscretionInstEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldDiscretionInstEnum::RelatedToDisplayedPrice),
            "1" => Ok(FieldDiscretionInstEnum::RelatedToMarketPrice),
            "2" => Ok(FieldDiscretionInstEnum::RelatedToPrimaryPrice),
            "3" => Ok(FieldDiscretionInstEnum::RelatedToLocalPrimaryPrice),
            "4" => Ok(FieldDiscretionInstEnum::RelatedToMidpointPrice),
            "5" => Ok(FieldDiscretionInstEnum::RelatedToLastTradePrice),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldDiscretionInstEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldDiscretionInstEnum::RelatedToDisplayedPrice => {
                f.write_str( "0" )
            },
            &FieldDiscretionInstEnum::RelatedToMarketPrice => {
                f.write_str( "1" )
            },
            &FieldDiscretionInstEnum::RelatedToPrimaryPrice => {
                f.write_str( "2" )
            },
            &FieldDiscretionInstEnum::RelatedToLocalPrimaryPrice => {
                f.write_str( "3" )
            },
            &FieldDiscretionInstEnum::RelatedToMidpointPrice => {
                f.write_str( "4" )
            },
            &FieldDiscretionInstEnum::RelatedToLastTradePrice => {
                f.write_str( "5" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldDiscretionInstEnum {
    fn default() -> Self {
        FieldDiscretionInstEnum::Undefined
    }
}
const FIELD_DISCRETIONOFFSET : u32 = 389; // PRICEOFFSET


const FIELD_BIDID : u32 = 390; // STRING


const FIELD_CLIENTBIDID : u32 = 391; // STRING


const FIELD_LISTNAME : u32 = 392; // STRING


const FIELD_TOTALNUMSECURITIES : u32 = 393; // INT


const FIELD_BIDTYPE : u32 = 394; // INT


const FIELD_NUMTICKETS : u32 = 395; // INT


const FIELD_SIDEVALUE1 : u32 = 396; // AMT


const FIELD_SIDEVALUE2 : u32 = 397; // AMT


const FIELD_NOBIDDESCRIPTORS : u32 = 398; // INT


const FIELD_BIDDESCRIPTORTYPE : u32 = 399; // INT


const FIELD_BIDDESCRIPTOR : u32 = 400; // STRING


const FIELD_SIDEVALUEIND : u32 = 401; // INT


const FIELD_LIQUIDITYPCTLOW : u32 = 402; // FLOAT


const FIELD_LIQUIDITYPCTHIGH : u32 = 403; // FLOAT


const FIELD_LIQUIDITYVALUE : u32 = 404; // AMT


const FIELD_EFPTRACKINGERROR : u32 = 405; // FLOAT


const FIELD_FAIRVALUE : u32 = 406; // AMT


const FIELD_OUTSIDEINDEXPCT : u32 = 407; // FLOAT


const FIELD_VALUEOFFUTURES : u32 = 408; // AMT


const FIELD_LIQUIDITYINDTYPE : u32 = 409; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldLiquidityIndTypeEnum {
    A5DayMovingAverage, // = "1"
    A20DayMovingAverage, // = "2"
    NormalMarketSize, // = "3"
    Other, // = "4"

    Undefined
}

impl FromStr for FieldLiquidityIndTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldLiquidityIndTypeEnum::A5DayMovingAverage),
            "2" => Ok(FieldLiquidityIndTypeEnum::A20DayMovingAverage),
            "3" => Ok(FieldLiquidityIndTypeEnum::NormalMarketSize),
            "4" => Ok(FieldLiquidityIndTypeEnum::Other),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldLiquidityIndTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldLiquidityIndTypeEnum::A5DayMovingAverage => {
                f.write_str( "1" )
            },
            &FieldLiquidityIndTypeEnum::A20DayMovingAverage => {
                f.write_str( "2" )
            },
            &FieldLiquidityIndTypeEnum::NormalMarketSize => {
                f.write_str( "3" )
            },
            &FieldLiquidityIndTypeEnum::Other => {
                f.write_str( "4" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldLiquidityIndTypeEnum {
    fn default() -> Self {
        FieldLiquidityIndTypeEnum::Undefined
    }
}
const FIELD_WTAVERAGELIQUIDITY : u32 = 410; // FLOAT


const FIELD_EXCHANGEFORPHYSICAL : u32 = 411; // BOOLEAN


const FIELD_OUTMAINCNTRYUINDEX : u32 = 412; // AMT


const FIELD_CROSSPERCENT : u32 = 413; // FLOAT


const FIELD_PROGRPTREQS : u32 = 414; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldProgRptReqsEnum {
    BuysideExplicitlyRequestsStatusUsingStatusrequest, // = "1"
    SellsidePeriodicallySendsStatusUsingListstatusPeriodOptionallySpecifiedInProgressperiod, // = "2"
    RealTimeExecutionReports, // = "3"

    Undefined
}

impl FromStr for FieldProgRptReqsEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldProgRptReqsEnum::BuysideExplicitlyRequestsStatusUsingStatusrequest),
            "2" => Ok(FieldProgRptReqsEnum::SellsidePeriodicallySendsStatusUsingListstatusPeriodOptionallySpecifiedInProgressperiod),
            "3" => Ok(FieldProgRptReqsEnum::RealTimeExecutionReports),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldProgRptReqsEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldProgRptReqsEnum::BuysideExplicitlyRequestsStatusUsingStatusrequest => {
                f.write_str( "1" )
            },
            &FieldProgRptReqsEnum::SellsidePeriodicallySendsStatusUsingListstatusPeriodOptionallySpecifiedInProgressperiod => {
                f.write_str( "2" )
            },
            &FieldProgRptReqsEnum::RealTimeExecutionReports => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldProgRptReqsEnum {
    fn default() -> Self {
        FieldProgRptReqsEnum::Undefined
    }
}
const FIELD_PROGPERIODINTERVAL : u32 = 415; // INT


const FIELD_INCTAXIND : u32 = 416; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldIncTaxIndEnum {
    Net, // = "1"
    Gross, // = "2"

    Undefined
}

impl FromStr for FieldIncTaxIndEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldIncTaxIndEnum::Net),
            "2" => Ok(FieldIncTaxIndEnum::Gross),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldIncTaxIndEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldIncTaxIndEnum::Net => {
                f.write_str( "1" )
            },
            &FieldIncTaxIndEnum::Gross => {
                f.write_str( "2" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldIncTaxIndEnum {
    fn default() -> Self {
        FieldIncTaxIndEnum::Undefined
    }
}
const FIELD_NUMBIDDERS : u32 = 417; // INT


const FIELD_TRADETYPE : u32 = 418; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldTradeTypeEnum {
    Agency, // = "A"
    VwapGuarantee, // = "G"
    GuaranteedClose, // = "J"
    RiskTrade, // = "R"

    Undefined
}

impl FromStr for FieldTradeTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(FieldTradeTypeEnum::Agency),
            "G" => Ok(FieldTradeTypeEnum::VwapGuarantee),
            "J" => Ok(FieldTradeTypeEnum::GuaranteedClose),
            "R" => Ok(FieldTradeTypeEnum::RiskTrade),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldTradeTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldTradeTypeEnum::Agency => {
                f.write_str( "A" )
            },
            &FieldTradeTypeEnum::VwapGuarantee => {
                f.write_str( "G" )
            },
            &FieldTradeTypeEnum::GuaranteedClose => {
                f.write_str( "J" )
            },
            &FieldTradeTypeEnum::RiskTrade => {
                f.write_str( "R" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldTradeTypeEnum {
    fn default() -> Self {
        FieldTradeTypeEnum::Undefined
    }
}
const FIELD_BASISPXTYPE : u32 = 419; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldBasisPxTypeEnum {
    ClosingPriceAtMorningSession, // = "2"
    ClosingPrice, // = "3"
    CurrentPrice, // = "4"
    Sq, // = "5"
    VwapThroughADay, // = "6"
    VwapThroughAMorningSession, // = "7"
    VwapThroughAnAfternoonSession, // = "8"
    VwapThroughADayExceptYori, // = "9"
    VwapThroughAMorningSessionExceptYori, // = "A"
    VwapThroughAnAfternoonSessionExceptYori, // = "B"
    Strike, // = "C"
    Open, // = "D"
    Others, // = "Z"

    Undefined
}

impl FromStr for FieldBasisPxTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(FieldBasisPxTypeEnum::ClosingPriceAtMorningSession),
            "3" => Ok(FieldBasisPxTypeEnum::ClosingPrice),
            "4" => Ok(FieldBasisPxTypeEnum::CurrentPrice),
            "5" => Ok(FieldBasisPxTypeEnum::Sq),
            "6" => Ok(FieldBasisPxTypeEnum::VwapThroughADay),
            "7" => Ok(FieldBasisPxTypeEnum::VwapThroughAMorningSession),
            "8" => Ok(FieldBasisPxTypeEnum::VwapThroughAnAfternoonSession),
            "9" => Ok(FieldBasisPxTypeEnum::VwapThroughADayExceptYori),
            "A" => Ok(FieldBasisPxTypeEnum::VwapThroughAMorningSessionExceptYori),
            "B" => Ok(FieldBasisPxTypeEnum::VwapThroughAnAfternoonSessionExceptYori),
            "C" => Ok(FieldBasisPxTypeEnum::Strike),
            "D" => Ok(FieldBasisPxTypeEnum::Open),
            "Z" => Ok(FieldBasisPxTypeEnum::Others),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldBasisPxTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldBasisPxTypeEnum::ClosingPriceAtMorningSession => {
                f.write_str( "2" )
            },
            &FieldBasisPxTypeEnum::ClosingPrice => {
                f.write_str( "3" )
            },
            &FieldBasisPxTypeEnum::CurrentPrice => {
                f.write_str( "4" )
            },
            &FieldBasisPxTypeEnum::Sq => {
                f.write_str( "5" )
            },
            &FieldBasisPxTypeEnum::VwapThroughADay => {
                f.write_str( "6" )
            },
            &FieldBasisPxTypeEnum::VwapThroughAMorningSession => {
                f.write_str( "7" )
            },
            &FieldBasisPxTypeEnum::VwapThroughAnAfternoonSession => {
                f.write_str( "8" )
            },
            &FieldBasisPxTypeEnum::VwapThroughADayExceptYori => {
                f.write_str( "9" )
            },
            &FieldBasisPxTypeEnum::VwapThroughAMorningSessionExceptYori => {
                f.write_str( "A" )
            },
            &FieldBasisPxTypeEnum::VwapThroughAnAfternoonSessionExceptYori => {
                f.write_str( "B" )
            },
            &FieldBasisPxTypeEnum::Strike => {
                f.write_str( "C" )
            },
            &FieldBasisPxTypeEnum::Open => {
                f.write_str( "D" )
            },
            &FieldBasisPxTypeEnum::Others => {
                f.write_str( "Z" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldBasisPxTypeEnum {
    fn default() -> Self {
        FieldBasisPxTypeEnum::Undefined
    }
}
const FIELD_NOBIDCOMPONENTS : u32 = 420; // INT


const FIELD_COUNTRY : u32 = 421; // STRING


const FIELD_TOTNOSTRIKES : u32 = 422; // INT


const FIELD_PRICETYPE : u32 = 423; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldPriceTypeEnum {
    Percentage, // = "1"
    PerShare, // = "2"
    FixedAmount, // = "3"

    Undefined
}

impl FromStr for FieldPriceTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldPriceTypeEnum::Percentage),
            "2" => Ok(FieldPriceTypeEnum::PerShare),
            "3" => Ok(FieldPriceTypeEnum::FixedAmount),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldPriceTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldPriceTypeEnum::Percentage => {
                f.write_str( "1" )
            },
            &FieldPriceTypeEnum::PerShare => {
                f.write_str( "2" )
            },
            &FieldPriceTypeEnum::FixedAmount => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldPriceTypeEnum {
    fn default() -> Self {
        FieldPriceTypeEnum::Undefined
    }
}
const FIELD_DAYORDERQTY : u32 = 424; // QTY


const FIELD_DAYCUMQTY : u32 = 425; // QTY


const FIELD_DAYAVGPX : u32 = 426; // PRICE


const FIELD_GTBOOKINGINST : u32 = 427; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldGTBookingInstEnum {
    BookOutAllTradesOnDayOfExecution, // = "0"
    AccumulateExecutionsUntilOrderIsFilledOrExpires, // = "1"
    AccumulateUntilVerballyNotifiedOtherwise, // = "2"

    Undefined
}

impl FromStr for FieldGTBookingInstEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(FieldGTBookingInstEnum::BookOutAllTradesOnDayOfExecution),
            "1" => Ok(FieldGTBookingInstEnum::AccumulateExecutionsUntilOrderIsFilledOrExpires),
            "2" => Ok(FieldGTBookingInstEnum::AccumulateUntilVerballyNotifiedOtherwise),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldGTBookingInstEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldGTBookingInstEnum::BookOutAllTradesOnDayOfExecution => {
                f.write_str( "0" )
            },
            &FieldGTBookingInstEnum::AccumulateExecutionsUntilOrderIsFilledOrExpires => {
                f.write_str( "1" )
            },
            &FieldGTBookingInstEnum::AccumulateUntilVerballyNotifiedOtherwise => {
                f.write_str( "2" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldGTBookingInstEnum {
    fn default() -> Self {
        FieldGTBookingInstEnum::Undefined
    }
}
const FIELD_NOSTRIKES : u32 = 428; // INT


const FIELD_LISTSTATUSTYPE : u32 = 429; // INT


const FIELD_NETGROSSIND : u32 = 430; // INT


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldNetGrossIndEnum {
    Net, // = "1"
    Gross, // = "2"

    Undefined
}

impl FromStr for FieldNetGrossIndEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldNetGrossIndEnum::Net),
            "2" => Ok(FieldNetGrossIndEnum::Gross),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldNetGrossIndEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldNetGrossIndEnum::Net => {
                f.write_str( "1" )
            },
            &FieldNetGrossIndEnum::Gross => {
                f.write_str( "2" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldNetGrossIndEnum {
    fn default() -> Self {
        FieldNetGrossIndEnum::Undefined
    }
}
const FIELD_LISTORDERSTATUS : u32 = 431; // INT


const FIELD_EXPIREDATE : u32 = 432; // LOCALMKTDATE


const FIELD_LISTEXECINSTTYPE : u32 = 433; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldListExecInstTypeEnum {
    Immediate, // = "1"
    WaitForExecuteInstruction, // = "2"

    Undefined
}

impl FromStr for FieldListExecInstTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldListExecInstTypeEnum::Immediate),
            "2" => Ok(FieldListExecInstTypeEnum::WaitForExecuteInstruction),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldListExecInstTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldListExecInstTypeEnum::Immediate => {
                f.write_str( "1" )
            },
            &FieldListExecInstTypeEnum::WaitForExecuteInstruction => {
                f.write_str( "2" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldListExecInstTypeEnum {
    fn default() -> Self {
        FieldListExecInstTypeEnum::Undefined
    }
}
const FIELD_CXLREJRESPONSETO : u32 = 434; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldCxlRejResponseToEnum {
    OrderCancelRequest, // = "1"
    OrderCancelReplaceRequest, // = "2"

    Undefined
}

impl FromStr for FieldCxlRejResponseToEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldCxlRejResponseToEnum::OrderCancelRequest),
            "2" => Ok(FieldCxlRejResponseToEnum::OrderCancelReplaceRequest),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldCxlRejResponseToEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldCxlRejResponseToEnum::OrderCancelRequest => {
                f.write_str( "1" )
            },
            &FieldCxlRejResponseToEnum::OrderCancelReplaceRequest => {
                f.write_str( "2" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldCxlRejResponseToEnum {
    fn default() -> Self {
        FieldCxlRejResponseToEnum::Undefined
    }
}
const FIELD_UNDERLYINGCOUPONRATE : u32 = 435; // FLOAT


const FIELD_UNDERLYINGCONTRACTMULTIPLIER : u32 = 436; // FLOAT


const FIELD_CONTRATRADEQTY : u32 = 437; // QTY


const FIELD_CONTRATRADETIME : u32 = 438; // UTCTIMESTAMP


const FIELD_CLEARINGFIRM : u32 = 439; // STRING


const FIELD_CLEARINGACCOUNT : u32 = 440; // STRING


const FIELD_LIQUIDITYNUMSECURITIES : u32 = 441; // INT


const FIELD_MULTILEGREPORTINGTYPE : u32 = 442; // CHAR


#[derive(PartialEq,Debug,Serialize,Deserialize,Clone,Copy)]
pub enum FieldMultiLegReportingTypeEnum {
    SingleSecurity, // = "1"
    IndividualLegOfAMultiLegSecurity, // = "2"
    MultiLegSecurity, // = "3"

    Undefined
}

impl FromStr for FieldMultiLegReportingTypeEnum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(FieldMultiLegReportingTypeEnum::SingleSecurity),
            "2" => Ok(FieldMultiLegReportingTypeEnum::IndividualLegOfAMultiLegSecurity),
            "3" => Ok(FieldMultiLegReportingTypeEnum::MultiLegSecurity),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for FieldMultiLegReportingTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &FieldMultiLegReportingTypeEnum::SingleSecurity => {
                f.write_str( "1" )
            },
            &FieldMultiLegReportingTypeEnum::IndividualLegOfAMultiLegSecurity => {
                f.write_str( "2" )
            },
            &FieldMultiLegReportingTypeEnum::MultiLegSecurity => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for FieldMultiLegReportingTypeEnum {
    fn default() -> Self {
        FieldMultiLegReportingTypeEnum::Undefined
    }
}
const FIELD_STRIKETIME : u32 = 443; // UTCTIMESTAMP


const FIELD_LISTSTATUSTEXT : u32 = 444; // STRING


const FIELD_ENCODEDLISTSTATUSTEXTLEN : u32 = 445; // LENGTH


const FIELD_ENCODEDLISTSTATUSTEXT : u32 = 446; // DATA




// TODO: If type=Mulstring needs to impl trait BitOr as well

#[derive(Serialize,Deserialize)]
pub struct UtcDateTime(DateTime<Utc>);

impl UtcDateTime {
    pub fn new(dt: DateTime<Utc>) -> UtcDateTime {
        UtcDateTime( dt )
    }
}

impl Default for UtcDateTime {
    fn default() -> Self {
        UtcDateTime(Utc::now())
    }
}
impl PartialEq for UtcDateTime {
    fn eq(&self, other: &Self) -> bool{
        self.0.eq( &other.0 )
    }
}
impl Debug for UtcDateTime {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}
impl FromStr for UtcDateTime {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: more robust impl
        // Fix data type YYYYMMDD-HH:MM:SS.sss
        Ok( UtcDateTime( Utc.datetime_from_str(s, "%Y%m%d-%H:%M:%S%.3f").unwrap() ))
    }
}
impl Display for UtcDateTime {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0.format("%Y%m%d-%H:%M:%S%.3f")) // prob not efficient

    }
}

#[derive(Serialize,Deserialize)]
pub struct UtcDate(DateTime<Utc>);

impl UtcDate {
    pub fn new(dt: DateTime<Utc>) -> UtcDate {
        UtcDate( dt )
    }
}

impl Default for UtcDate {
    fn default() -> Self {
        UtcDate(Utc::now())
    }
}
impl PartialEq for UtcDate {
    fn eq(&self, other: &Self) -> bool{
        self.0.date().eq( &other.0.date() )
    }
}
impl Debug for UtcDate {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0.date())
    }
}
impl FromStr for UtcDate {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: more robust impl
        // Fix data type: YYYYMMDD
        Ok( UtcDate( Utc.datetime_from_str(s, "%Y%m%d").unwrap() ) )
    }
}
impl Display for UtcDate {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0.format("%Y%m%d"))
    }
}

#[derive(Serialize,Deserialize)]
pub struct UtcTime(DateTime<Utc>);

impl Default for UtcTime {
    fn default() -> Self {
        UtcTime(Utc::now())
    }
}
impl PartialEq for UtcTime {
    fn eq(&self, other: &Self) -> bool{
        self.0.eq( &other.0 )
    }
}
impl Debug for UtcTime {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}
impl FromStr for UtcTime {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: more robust impl
        // Fix data type: either HH:MM:SS (whole seconds) or HH:MM:SS.sss (milliseconds) format
        Ok( UtcTime( Utc.datetime_from_str(s, "%H:%M:%S%.3f").unwrap() ) )
    }
}
impl Display for UtcTime {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0.format("%H:%M:%S%.3f"))
    }
}

fn boolconv(v: &str) -> bool {
    v == "Y"
}
fn to_boolconv(val: &bool) -> &'static str {
    if *val {
        "Y"
    } else {
        "N"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OurParserError {
    unregonized_val : String,
}


pub fn is_admin_message( m : &FixMessage ) -> bool {
    match m {
        &FixMessage::Heartbeat(_) => true,
        &FixMessage::TestRequest(_) => true,
        &FixMessage::ResendRequest(_) => true,
        &FixMessage::Reject(_) => true,
        &FixMessage::SequenceReset(_) => true,
        &FixMessage::Logout(_) => true,
        &FixMessage::Logon(_) => true,
        _ => false,
    }
}

pub fn build_fix_header( begin_str: &'static str, flds: &Vec<FieldVal> ) -> FixHeader {

    let filter = |f: &&FieldVal| {
        f.id == 35 ||f.id == 49 ||f.id == 56 ||f.id == 115 ||f.id == 128 ||f.id == 90 ||f.id == 91 ||f.id == 34 ||f.id == 50 ||f.id == 142 ||f.id == 57 ||f.id == 143 ||f.id == 116 ||f.id == 144 ||f.id == 129 ||f.id == 145 ||f.id == 43 ||f.id == 97 ||f.id == 52 ||f.id == 122 ||f.id == 212 ||f.id == 213 ||f.id == 347 ||f.id == 369 ||f.id == 370 ||
            f.id == 0
    };
    let mut msg_type : Option<FieldMsgTypeEnum> = None;
    let mut sender_comp_id : Option<String> = None;
    let mut target_comp_id : Option<String> = None;
    let mut on_behalf_of_comp_id : Option<String> = None;
    let mut deliver_to_comp_id : Option<String> = None;
    let mut secure_data_len : Option<usize> = None;
    let mut secure_data : Option<String> = None;
    let mut msg_seq_num : Option<i32> = None;
    let mut sender_sub_id : Option<String> = None;
    let mut sender_location_id : Option<String> = None;
    let mut target_sub_id : Option<String> = None;
    let mut target_location_id : Option<String> = None;
    let mut on_behalf_of_sub_id : Option<String> = None;
    let mut on_behalf_of_location_id : Option<String> = None;
    let mut deliver_to_sub_id : Option<String> = None;
    let mut deliver_to_location_id : Option<String> = None;
    let mut poss_dup_flag : Option<bool> = None;
    let mut poss_resend : Option<bool> = None;
    let mut sending_time : Option<UtcDateTime> = None;
    let mut orig_sending_time : Option<UtcDateTime> = None;
    let mut xml_data_len : Option<usize> = None;
    let mut xml_data : Option<String> = None;
    let mut message_encoding : Option<FieldMessageEncodingEnum> = None;
    let mut last_msg_seq_num_processed : Option<i32> = None;
    let mut on_behalf_of_sending_time : Option<UtcDateTime> = None;

    for fld in flds.iter().filter(filter) {
        match fld {
            &FieldVal { id: FIELD_MSGTYPE, val: v } => {
                msg_type = Some( FieldMsgTypeEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SENDERCOMPID, val: v } => {
                sender_comp_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_TARGETCOMPID, val: v } => {
                target_comp_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ONBEHALFOFCOMPID, val: v } => {
                on_behalf_of_comp_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_DELIVERTOCOMPID, val: v } => {
                deliver_to_comp_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SECUREDATALEN, val: v } => {
                secure_data_len = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECUREDATA, val: v } => {
                secure_data = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_MSGSEQNUM, val: v } => {
                msg_seq_num = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SENDERSUBID, val: v } => {
                sender_sub_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SENDERLOCATIONID, val: v } => {
                sender_location_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_TARGETSUBID, val: v } => {
                target_sub_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_TARGETLOCATIONID, val: v } => {
                target_location_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ONBEHALFOFSUBID, val: v } => {
                on_behalf_of_sub_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ONBEHALFOFLOCATIONID, val: v } => {
                on_behalf_of_location_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_DELIVERTOSUBID, val: v } => {
                deliver_to_sub_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_DELIVERTOLOCATIONID, val: v } => {
                deliver_to_location_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_POSSDUPFLAG, val: v } => {
                poss_dup_flag = Some( boolconv(v) );
            },
            &FieldVal { id: FIELD_POSSRESEND, val: v } => {
                poss_resend = Some( boolconv(v) );
            },
            &FieldVal { id: FIELD_SENDINGTIME, val: v } => {
                sending_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ORIGSENDINGTIME, val: v } => {
                orig_sending_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_XMLDATALEN, val: v } => {
                xml_data_len = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_XMLDATA, val: v } => {
                xml_data = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_MESSAGEENCODING, val: v } => {
                message_encoding = Some( FieldMessageEncodingEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_LASTMSGSEQNUMPROCESSED, val: v } => {
                last_msg_seq_num_processed = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ONBEHALFOFSENDINGTIME, val: v } => {
                on_behalf_of_sending_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            _ => {
                // return IResult::Error(error_code!(ErrorKind::Custom(42))); // TODO: better errors!
            }
        }
    }

    FixHeader  {
        begin_string: Cow::from(begin_str),
        msg_type: msg_type.unwrap() /* better error hdl? */ ,
        sender_comp_id: sender_comp_id.unwrap() /* better error hdl? */ ,
        target_comp_id: target_comp_id.unwrap() /* better error hdl? */ ,
        on_behalf_of_comp_id: on_behalf_of_comp_id,
        deliver_to_comp_id: deliver_to_comp_id,
        secure_data_len: secure_data_len,
        secure_data: secure_data,
        msg_seq_num: msg_seq_num.unwrap() /* better error hdl? */ ,
        sender_sub_id: sender_sub_id,
        sender_location_id: sender_location_id,
        target_sub_id: target_sub_id,
        target_location_id: target_location_id,
        on_behalf_of_sub_id: on_behalf_of_sub_id,
        on_behalf_of_location_id: on_behalf_of_location_id,
        deliver_to_sub_id: deliver_to_sub_id,
        deliver_to_location_id: deliver_to_location_id,
        poss_dup_flag: poss_dup_flag,
        poss_resend: poss_resend,
        sending_time: sending_time.unwrap() /* better error hdl? */ ,
        orig_sending_time: orig_sending_time,
        xml_data_len: xml_data_len,
        xml_data: xml_data,
        message_encoding: message_encoding,
        last_msg_seq_num_processed: last_msg_seq_num_processed,
        on_behalf_of_sending_time: on_behalf_of_sending_time,
    }
}

/// Main builder
pub fn build_fix_message( msg_type: &str, flds: &Vec<FieldVal> ) -> FixMessage {

    match msg_type {

        "0" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_heartbeat_fields( &mut consumer );
            FixMessage::Heartbeat(Box::new( fields ))
        },

        "1" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_test_request_fields( &mut consumer );
            FixMessage::TestRequest(Box::new( fields ))
        },

        "2" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_resend_request_fields( &mut consumer );
            FixMessage::ResendRequest(Box::new( fields ))
        },

        "3" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_reject_fields( &mut consumer );
            FixMessage::Reject(Box::new( fields ))
        },

        "4" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_sequence_reset_fields( &mut consumer );
            FixMessage::SequenceReset(Box::new( fields ))
        },

        "5" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_logout_fields( &mut consumer );
            FixMessage::Logout(Box::new( fields ))
        },

        "7" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_advertisement_fields( &mut consumer );
            FixMessage::Advertisement(Box::new( fields ))
        },

        "8" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_execution_report_fields( &mut consumer );
            FixMessage::ExecutionReport(Box::new( fields ))
        },

        "9" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_order_cancel_reject_fields( &mut consumer );
            FixMessage::OrderCancelReject(Box::new( fields ))
        },

        "A" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_logon_fields( &mut consumer );
            FixMessage::Logon(Box::new( fields ))
        },

        "D" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_new_order_single_fields( &mut consumer );
            FixMessage::NewOrderSingle(Box::new( fields ))
        },

        "E" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_new_order_list_fields( &mut consumer );
            FixMessage::NewOrderList(Box::new( fields ))
        },

        "F" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_order_cancel_request_fields( &mut consumer );
            FixMessage::OrderCancelRequest(Box::new( fields ))
        },

        "G" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_order_cancel_replace_request_fields( &mut consumer );
            FixMessage::OrderCancelReplaceRequest(Box::new( fields ))
        },

        "H" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_order_status_request_fields( &mut consumer );
            FixMessage::OrderStatusRequest(Box::new( fields ))
        },

        _ => {
            FixMessage::UndefinedMessage // replace by some error message?
        }
    }

}




fn parse_message_heartbeat_fields( consumer : &mut FixConsumer  ) -> HeartbeatFields {
    // fields:
    let mut test_req_id : Option<String> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_TESTREQID, val: v } => {

                test_req_id = Some( v.to_string() );
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    HeartbeatFields {
        test_req_id: test_req_id,
    }
}


fn parse_message_test_request_fields( consumer : &mut FixConsumer  ) -> TestRequestFields {
    // fields:
    let mut test_req_id : Option<String> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_TESTREQID, val: v } => {

                test_req_id = Some( v.to_string() );
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    TestRequestFields {
        test_req_id: test_req_id.unwrap() /* better error hdl? */ ,
    }
}


fn parse_message_resend_request_fields( consumer : &mut FixConsumer  ) -> ResendRequestFields {
    // fields:
    let mut begin_seq_no : Option<i32> = None;
    let mut end_seq_no : Option<i32> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_BEGINSEQNO, val: v } => {

                begin_seq_no = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ENDSEQNO, val: v } => {

                end_seq_no = Some( i32::from_str(v).unwrap() );
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    ResendRequestFields {
        begin_seq_no: begin_seq_no.unwrap() /* better error hdl? */ ,
        end_seq_no: end_seq_no.unwrap() /* better error hdl? */ ,
    }
}


fn parse_message_reject_fields( consumer : &mut FixConsumer  ) -> RejectFields {
    // fields:
    let mut ref_seq_num : Option<i32> = None;
    let mut ref_tag_id : Option<i32> = None;
    let mut ref_msg_type : Option<String> = None;
    let mut session_reject_reason : Option<FieldSessionRejectReasonEnum> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_REFSEQNUM, val: v } => {

                ref_seq_num = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_REFTAGID, val: v } => {

                ref_tag_id = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_REFMSGTYPE, val: v } => {

                ref_msg_type = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SESSIONREJECTREASON, val: v } => {

                session_reject_reason = Some( FieldSessionRejectReasonEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TEXT, val: v } => {

                text = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ENCODEDTEXTLEN, val: v } => {

                encoded_text_len = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ENCODEDTEXT, val: v } => {

                encoded_text = Some( v.to_string() );
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    RejectFields {
        ref_seq_num: ref_seq_num.unwrap() /* better error hdl? */ ,
        ref_tag_id: ref_tag_id,
        ref_msg_type: ref_msg_type,
        session_reject_reason: session_reject_reason,
        text: text,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
    }
}


fn parse_message_sequence_reset_fields( consumer : &mut FixConsumer  ) -> SequenceResetFields {
    // fields:
    let mut gap_fill_flag : Option<bool> = None;
    let mut new_seq_no : Option<i32> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_GAPFILLFLAG, val: v } => {

                gap_fill_flag = Some( boolconv(v) );
            },
            &FieldVal { id: FIELD_NEWSEQNO, val: v } => {

                new_seq_no = Some( i32::from_str(v).unwrap() );
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    SequenceResetFields {
        gap_fill_flag: gap_fill_flag,
        new_seq_no: new_seq_no.unwrap() /* better error hdl? */ ,
    }
}


fn parse_message_logout_fields( consumer : &mut FixConsumer  ) -> LogoutFields {
    // fields:
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_TEXT, val: v } => {

                text = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ENCODEDTEXTLEN, val: v } => {

                encoded_text_len = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ENCODEDTEXT, val: v } => {

                encoded_text = Some( v.to_string() );
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    LogoutFields {
        text: text,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
    }
}


fn parse_message_advertisement_fields( consumer : &mut FixConsumer  ) -> AdvertisementFields {
    // fields:
    let mut adv_id : Option<String> = None;
    let mut adv_trans_type : Option<FieldAdvTransTypeEnum> = None;
    let mut adv_ref_id : Option<String> = None;
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<FieldIDSourceEnum> = None;
    let mut security_type : Option<FieldSecurityTypeEnum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<FieldPutOrCallEnum> = None;
    let mut strike_price : Option<f32> = None;
    let mut opt_attribute : Option<char> = None;
    let mut contract_multiplier : Option<f32> = None;
    let mut coupon_rate : Option<f32> = None;
    let mut security_exchange : Option<String> = None;
    let mut issuer : Option<String> = None;
    let mut encoded_issuer_len : Option<usize> = None;
    let mut encoded_issuer : Option<String> = None;
    let mut security_desc : Option<String> = None;
    let mut encoded_security_desc_len : Option<usize> = None;
    let mut encoded_security_desc : Option<String> = None;
    let mut adv_side : Option<FieldAdvSideEnum> = None;
    let mut shares : Option<f32> = None;
    let mut price : Option<f32> = None;
    let mut currency : Option<f32> = None;
    let mut trade_date : Option<UtcDateTime> = None;
    let mut transact_time : Option<UtcDateTime> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;
    let mut urllink : Option<String> = None;
    let mut last_mkt : Option<String> = None;
    let mut trading_session_id : Option<String> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_ADVID, val: v } => {

                adv_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ADVTRANSTYPE, val: v } => {

                adv_trans_type = Some( FieldAdvTransTypeEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ADVREFID, val: v } => {

                adv_ref_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SYMBOL, val: v } => {

                symbol = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SYMBOLSFX, val: v } => {

                symbol_sfx = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SECURITYID, val: v } => {

                security_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_IDSOURCE, val: v } => {

                idsource = Some( FieldIDSourceEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                security_type = Some( FieldSecurityTypeEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYMONTHYEAR, val: v } => {

                maturity_month_year = Some( UtcDate::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYDAY, val: v } => {

                maturity_day = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PUTORCALL, val: v } => {

                put_or_call = Some( FieldPutOrCallEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_STRIKEPRICE, val: v } => {

                strike_price = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_OPTATTRIBUTE, val: v } => {

                opt_attribute = Some( char::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CONTRACTMULTIPLIER, val: v } => {

                contract_multiplier = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_COUPONRATE, val: v } => {

                coupon_rate = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYEXCHANGE, val: v } => {

                security_exchange = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ISSUER, val: v } => {

                issuer = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ENCODEDISSUERLEN, val: v } => {

                encoded_issuer_len = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ENCODEDISSUER, val: v } => {

                encoded_issuer = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SECURITYDESC, val: v } => {

                security_desc = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ENCODEDSECURITYDESCLEN, val: v } => {

                encoded_security_desc_len = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ENCODEDSECURITYDESC, val: v } => {

                encoded_security_desc = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ADVSIDE, val: v } => {

                adv_side = Some( FieldAdvSideEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SHARES, val: v } => {

                shares = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PRICE, val: v } => {

                price = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CURRENCY, val: v } => {

                currency = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRADEDATE, val: v } => {

                trade_date = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRANSACTTIME, val: v } => {

                transact_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TEXT, val: v } => {

                text = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ENCODEDTEXTLEN, val: v } => {

                encoded_text_len = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ENCODEDTEXT, val: v } => {

                encoded_text = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_URLLINK, val: v } => {

                urllink = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_LASTMKT, val: v } => {

                last_mkt = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_TRADINGSESSIONID, val: v } => {

                trading_session_id = Some( v.to_string() );
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    AdvertisementFields {
        adv_id: adv_id.unwrap() /* better error hdl? */ ,
        adv_trans_type: adv_trans_type.unwrap() /* better error hdl? */ ,
        adv_ref_id: adv_ref_id,
        symbol: symbol.unwrap() /* better error hdl? */ ,
        symbol_sfx: symbol_sfx,
        security_id: security_id,
        idsource: idsource,
        security_type: security_type,
        maturity_month_year: maturity_month_year,
        maturity_day: maturity_day,
        put_or_call: put_or_call,
        strike_price: strike_price,
        opt_attribute: opt_attribute,
        contract_multiplier: contract_multiplier,
        coupon_rate: coupon_rate,
        security_exchange: security_exchange,
        issuer: issuer,
        encoded_issuer_len: encoded_issuer_len,
        encoded_issuer: encoded_issuer,
        security_desc: security_desc,
        encoded_security_desc_len: encoded_security_desc_len,
        encoded_security_desc: encoded_security_desc,
        adv_side: adv_side.unwrap() /* better error hdl? */ ,
        shares: shares.unwrap() /* better error hdl? */ ,
        price: price,
        currency: currency,
        trade_date: trade_date,
        transact_time: transact_time,
        text: text,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
        urllink: urllink,
        last_mkt: last_mkt,
        trading_session_id: trading_session_id,
    }
}


fn parse_message_execution_report_fields( consumer : &mut FixConsumer  ) -> ExecutionReportFields {
    // fields:
    let mut order_id : Option<String> = None;
    let mut secondary_order_id : Option<String> = None;
    let mut cl_ord_id : Option<String> = None;
    let mut orig_cl_ord_id : Option<String> = None;
    let mut client_id : Option<String> = None;
    let mut exec_broker : Option<String> = None;
    let mut no_contra_brokers : Option<Vec<NoContraBrokers2Fields>> = None;
    let mut list_id : Option<String> = None;
    let mut exec_id : Option<String> = None;
    let mut exec_trans_type : Option<FieldExecTransTypeEnum> = None;
    let mut exec_ref_id : Option<String> = None;
    let mut exec_type : Option<FieldExecTypeEnum> = None;
    let mut ord_status : Option<FieldOrdStatusEnum> = None;
    let mut ord_rej_reason : Option<FieldOrdRejReasonEnum> = None;
    let mut exec_restatement_reason : Option<FieldExecRestatementReasonEnum> = None;
    let mut account : Option<String> = None;
    let mut settlmnt_typ : Option<FieldSettlmntTypEnum> = None;
    let mut fut_sett_date : Option<UtcDateTime> = None;
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<FieldIDSourceEnum> = None;
    let mut security_type : Option<FieldSecurityTypeEnum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<FieldPutOrCallEnum> = None;
    let mut strike_price : Option<f32> = None;
    let mut opt_attribute : Option<char> = None;
    let mut contract_multiplier : Option<f32> = None;
    let mut coupon_rate : Option<f32> = None;
    let mut security_exchange : Option<String> = None;
    let mut issuer : Option<String> = None;
    let mut encoded_issuer_len : Option<usize> = None;
    let mut encoded_issuer : Option<String> = None;
    let mut security_desc : Option<String> = None;
    let mut encoded_security_desc_len : Option<usize> = None;
    let mut encoded_security_desc : Option<String> = None;
    let mut side : Option<FieldSideEnum> = None;
    let mut order_qty : Option<f32> = None;
    let mut cash_order_qty : Option<f32> = None;
    let mut ord_type : Option<FieldOrdTypeEnum> = None;
    let mut price : Option<f32> = None;
    let mut stop_px : Option<f32> = None;
    let mut peg_difference : Option<f32> = None;
    let mut discretion_inst : Option<FieldDiscretionInstEnum> = None;
    let mut discretion_offset : Option<f32> = None;
    let mut currency : Option<f32> = None;
    let mut compliance_id : Option<String> = None;
    let mut solicited_flag : Option<bool> = None;
    let mut time_in_force : Option<FieldTimeInForceEnum> = None;
    let mut effective_time : Option<UtcDateTime> = None;
    let mut expire_date : Option<UtcDateTime> = None;
    let mut expire_time : Option<UtcDateTime> = None;
    let mut exec_inst : Option<FieldExecInstEnum> = None;
    let mut rule80_a : Option<FieldRule80AEnum> = None;
    let mut last_shares : Option<f32> = None;
    let mut last_px : Option<f32> = None;
    let mut last_spot_rate : Option<f32> = None;
    let mut last_forward_points : Option<f32> = None;
    let mut last_mkt : Option<String> = None;
    let mut trading_session_id : Option<String> = None;
    let mut last_capacity : Option<FieldLastCapacityEnum> = None;
    let mut leaves_qty : Option<f32> = None;
    let mut cum_qty : Option<f32> = None;
    let mut avg_px : Option<f32> = None;
    let mut day_order_qty : Option<f32> = None;
    let mut day_cum_qty : Option<f32> = None;
    let mut day_avg_px : Option<f32> = None;
    let mut gtbooking_inst : Option<FieldGTBookingInstEnum> = None;
    let mut trade_date : Option<UtcDateTime> = None;
    let mut transact_time : Option<UtcDateTime> = None;
    let mut report_to_exch : Option<bool> = None;
    let mut commission : Option<f32> = None;
    let mut comm_type : Option<FieldCommTypeEnum> = None;
    let mut gross_trade_amt : Option<f32> = None;
    let mut settl_curr_amt : Option<f32> = None;
    let mut settl_currency : Option<f32> = None;
    let mut settl_curr_fx_rate : Option<f32> = None;
    let mut settl_curr_fx_rate_calc : Option<FieldSettlCurrFxRateCalcEnum> = None;
    let mut handl_inst : Option<FieldHandlInstEnum> = None;
    let mut min_qty : Option<f32> = None;
    let mut max_floor : Option<f32> = None;
    let mut open_close : Option<FieldOpenCloseEnum> = None;
    let mut max_show : Option<f32> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;
    let mut fut_sett_date2 : Option<UtcDateTime> = None;
    let mut order_qty2 : Option<f32> = None;
    let mut clearing_firm : Option<String> = None;
    let mut clearing_account : Option<String> = None;
    let mut multi_leg_reporting_type : Option<FieldMultiLegReportingTypeEnum> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_ORDERID, val: v } => {

                order_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SECONDARYORDERID, val: v } => {

                secondary_order_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_CLORDID, val: v } => {

                cl_ord_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ORIGCLORDID, val: v } => {

                orig_cl_ord_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_CLIENTID, val: v } => {

                client_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_EXECBROKER, val: v } => {

                exec_broker = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_NOCONTRABROKERS, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_contra_brokers2_fields(consumer, size);
                no_contra_brokers = Some(subgroup);
            },
            &FieldVal { id: FIELD_LISTID, val: v } => {

                list_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_EXECID, val: v } => {

                exec_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_EXECTRANSTYPE, val: v } => {

                exec_trans_type = Some( FieldExecTransTypeEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_EXECREFID, val: v } => {

                exec_ref_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_EXECTYPE, val: v } => {

                exec_type = Some( FieldExecTypeEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ORDSTATUS, val: v } => {

                ord_status = Some( FieldOrdStatusEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ORDREJREASON, val: v } => {

                ord_rej_reason = Some( FieldOrdRejReasonEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_EXECRESTATEMENTREASON, val: v } => {

                exec_restatement_reason = Some( FieldExecRestatementReasonEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ACCOUNT, val: v } => {

                account = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SETTLMNTTYP, val: v } => {

                settlmnt_typ = Some( FieldSettlmntTypEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_FUTSETTDATE, val: v } => {

                fut_sett_date = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SYMBOL, val: v } => {

                symbol = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SYMBOLSFX, val: v } => {

                symbol_sfx = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SECURITYID, val: v } => {

                security_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_IDSOURCE, val: v } => {

                idsource = Some( FieldIDSourceEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                security_type = Some( FieldSecurityTypeEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYMONTHYEAR, val: v } => {

                maturity_month_year = Some( UtcDate::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYDAY, val: v } => {

                maturity_day = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PUTORCALL, val: v } => {

                put_or_call = Some( FieldPutOrCallEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_STRIKEPRICE, val: v } => {

                strike_price = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_OPTATTRIBUTE, val: v } => {

                opt_attribute = Some( char::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CONTRACTMULTIPLIER, val: v } => {

                contract_multiplier = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_COUPONRATE, val: v } => {

                coupon_rate = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYEXCHANGE, val: v } => {

                security_exchange = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ISSUER, val: v } => {

                issuer = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ENCODEDISSUERLEN, val: v } => {

                encoded_issuer_len = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ENCODEDISSUER, val: v } => {

                encoded_issuer = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SECURITYDESC, val: v } => {

                security_desc = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ENCODEDSECURITYDESCLEN, val: v } => {

                encoded_security_desc_len = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ENCODEDSECURITYDESC, val: v } => {

                encoded_security_desc = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SIDE, val: v } => {

                side = Some( FieldSideEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ORDERQTY, val: v } => {

                order_qty = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CASHORDERQTY, val: v } => {

                cash_order_qty = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ORDTYPE, val: v } => {

                ord_type = Some( FieldOrdTypeEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PRICE, val: v } => {

                price = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_STOPPX, val: v } => {

                stop_px = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PEGDIFFERENCE, val: v } => {

                peg_difference = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_DISCRETIONINST, val: v } => {

                discretion_inst = Some( FieldDiscretionInstEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_DISCRETIONOFFSET, val: v } => {

                discretion_offset = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CURRENCY, val: v } => {

                currency = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_COMPLIANCEID, val: v } => {

                compliance_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SOLICITEDFLAG, val: v } => {

                solicited_flag = Some( boolconv(v) );
            },
            &FieldVal { id: FIELD_TIMEINFORCE, val: v } => {

                time_in_force = Some( FieldTimeInForceEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_EFFECTIVETIME, val: v } => {

                effective_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_EXPIREDATE, val: v } => {

                expire_date = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_EXPIRETIME, val: v } => {

                expire_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_EXECINST, val: v } => {

                exec_inst = Some( FieldExecInstEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_RULE80A, val: v } => {

                rule80_a = Some( FieldRule80AEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_LASTSHARES, val: v } => {

                last_shares = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_LASTPX, val: v } => {

                last_px = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_LASTSPOTRATE, val: v } => {

                last_spot_rate = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_LASTFORWARDPOINTS, val: v } => {

                last_forward_points = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_LASTMKT, val: v } => {

                last_mkt = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_TRADINGSESSIONID, val: v } => {

                trading_session_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_LASTCAPACITY, val: v } => {

                last_capacity = Some( FieldLastCapacityEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_LEAVESQTY, val: v } => {

                leaves_qty = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CUMQTY, val: v } => {

                cum_qty = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_AVGPX, val: v } => {

                avg_px = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_DAYORDERQTY, val: v } => {

                day_order_qty = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_DAYCUMQTY, val: v } => {

                day_cum_qty = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_DAYAVGPX, val: v } => {

                day_avg_px = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_GTBOOKINGINST, val: v } => {

                gtbooking_inst = Some( FieldGTBookingInstEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRADEDATE, val: v } => {

                trade_date = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRANSACTTIME, val: v } => {

                transact_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_REPORTTOEXCH, val: v } => {

                report_to_exch = Some( boolconv(v) );
            },
            &FieldVal { id: FIELD_COMMISSION, val: v } => {

                commission = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_COMMTYPE, val: v } => {

                comm_type = Some( FieldCommTypeEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_GROSSTRADEAMT, val: v } => {

                gross_trade_amt = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SETTLCURRAMT, val: v } => {

                settl_curr_amt = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SETTLCURRENCY, val: v } => {

                settl_currency = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SETTLCURRFXRATE, val: v } => {

                settl_curr_fx_rate = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SETTLCURRFXRATECALC, val: v } => {

                settl_curr_fx_rate_calc = Some( FieldSettlCurrFxRateCalcEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_HANDLINST, val: v } => {

                handl_inst = Some( FieldHandlInstEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MINQTY, val: v } => {

                min_qty = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MAXFLOOR, val: v } => {

                max_floor = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_OPENCLOSE, val: v } => {

                open_close = Some( FieldOpenCloseEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MAXSHOW, val: v } => {

                max_show = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TEXT, val: v } => {

                text = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ENCODEDTEXTLEN, val: v } => {

                encoded_text_len = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ENCODEDTEXT, val: v } => {

                encoded_text = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_FUTSETTDATE2, val: v } => {

                fut_sett_date2 = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ORDERQTY2, val: v } => {

                order_qty2 = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CLEARINGFIRM, val: v } => {

                clearing_firm = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_CLEARINGACCOUNT, val: v } => {

                clearing_account = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_MULTILEGREPORTINGTYPE, val: v } => {

                multi_leg_reporting_type = Some( FieldMultiLegReportingTypeEnum::from_str(v).unwrap() );
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    ExecutionReportFields {
        order_id: order_id.unwrap() /* better error hdl? */ ,
        secondary_order_id: secondary_order_id,
        cl_ord_id: cl_ord_id,
        orig_cl_ord_id: orig_cl_ord_id,
        client_id: client_id,
        exec_broker: exec_broker,
        no_contra_brokers: no_contra_brokers,
        list_id: list_id,
        exec_id: exec_id.unwrap() /* better error hdl? */ ,
        exec_trans_type: exec_trans_type.unwrap() /* better error hdl? */ ,
        exec_ref_id: exec_ref_id,
        exec_type: exec_type.unwrap() /* better error hdl? */ ,
        ord_status: ord_status.unwrap() /* better error hdl? */ ,
        ord_rej_reason: ord_rej_reason,
        exec_restatement_reason: exec_restatement_reason,
        account: account,
        settlmnt_typ: settlmnt_typ,
        fut_sett_date: fut_sett_date,
        symbol: symbol.unwrap() /* better error hdl? */ ,
        symbol_sfx: symbol_sfx,
        security_id: security_id,
        idsource: idsource,
        security_type: security_type,
        maturity_month_year: maturity_month_year,
        maturity_day: maturity_day,
        put_or_call: put_or_call,
        strike_price: strike_price,
        opt_attribute: opt_attribute,
        contract_multiplier: contract_multiplier,
        coupon_rate: coupon_rate,
        security_exchange: security_exchange,
        issuer: issuer,
        encoded_issuer_len: encoded_issuer_len,
        encoded_issuer: encoded_issuer,
        security_desc: security_desc,
        encoded_security_desc_len: encoded_security_desc_len,
        encoded_security_desc: encoded_security_desc,
        side: side.unwrap() /* better error hdl? */ ,
        order_qty: order_qty,
        cash_order_qty: cash_order_qty,
        ord_type: ord_type,
        price: price,
        stop_px: stop_px,
        peg_difference: peg_difference,
        discretion_inst: discretion_inst,
        discretion_offset: discretion_offset,
        currency: currency,
        compliance_id: compliance_id,
        solicited_flag: solicited_flag,
        time_in_force: time_in_force,
        effective_time: effective_time,
        expire_date: expire_date,
        expire_time: expire_time,
        exec_inst: exec_inst,
        rule80_a: rule80_a,
        last_shares: last_shares,
        last_px: last_px,
        last_spot_rate: last_spot_rate,
        last_forward_points: last_forward_points,
        last_mkt: last_mkt,
        trading_session_id: trading_session_id,
        last_capacity: last_capacity,
        leaves_qty: leaves_qty.unwrap() /* better error hdl? */ ,
        cum_qty: cum_qty.unwrap() /* better error hdl? */ ,
        avg_px: avg_px.unwrap() /* better error hdl? */ ,
        day_order_qty: day_order_qty,
        day_cum_qty: day_cum_qty,
        day_avg_px: day_avg_px,
        gtbooking_inst: gtbooking_inst,
        trade_date: trade_date,
        transact_time: transact_time,
        report_to_exch: report_to_exch,
        commission: commission,
        comm_type: comm_type,
        gross_trade_amt: gross_trade_amt,
        settl_curr_amt: settl_curr_amt,
        settl_currency: settl_currency,
        settl_curr_fx_rate: settl_curr_fx_rate,
        settl_curr_fx_rate_calc: settl_curr_fx_rate_calc,
        handl_inst: handl_inst,
        min_qty: min_qty,
        max_floor: max_floor,
        open_close: open_close,
        max_show: max_show,
        text: text,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
        fut_sett_date2: fut_sett_date2,
        order_qty2: order_qty2,
        clearing_firm: clearing_firm,
        clearing_account: clearing_account,
        multi_leg_reporting_type: multi_leg_reporting_type,
    }
}


fn parse_message_order_cancel_reject_fields( consumer : &mut FixConsumer  ) -> OrderCancelRejectFields {
    // fields:
    let mut order_id : Option<String> = None;
    let mut secondary_order_id : Option<String> = None;
    let mut cl_ord_id : Option<String> = None;
    let mut orig_cl_ord_id : Option<String> = None;
    let mut ord_status : Option<FieldOrdStatusEnum> = None;
    let mut client_id : Option<String> = None;
    let mut exec_broker : Option<String> = None;
    let mut list_id : Option<String> = None;
    let mut account : Option<String> = None;
    let mut transact_time : Option<UtcDateTime> = None;
    let mut cxl_rej_response_to : Option<FieldCxlRejResponseToEnum> = None;
    let mut cxl_rej_reason : Option<FieldCxlRejReasonEnum> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_ORDERID, val: v } => {

                order_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SECONDARYORDERID, val: v } => {

                secondary_order_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_CLORDID, val: v } => {

                cl_ord_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ORIGCLORDID, val: v } => {

                orig_cl_ord_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ORDSTATUS, val: v } => {

                ord_status = Some( FieldOrdStatusEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CLIENTID, val: v } => {

                client_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_EXECBROKER, val: v } => {

                exec_broker = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_LISTID, val: v } => {

                list_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ACCOUNT, val: v } => {

                account = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_TRANSACTTIME, val: v } => {

                transact_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CXLREJRESPONSETO, val: v } => {

                cxl_rej_response_to = Some( FieldCxlRejResponseToEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CXLREJREASON, val: v } => {

                cxl_rej_reason = Some( FieldCxlRejReasonEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TEXT, val: v } => {

                text = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ENCODEDTEXTLEN, val: v } => {

                encoded_text_len = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ENCODEDTEXT, val: v } => {

                encoded_text = Some( v.to_string() );
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    OrderCancelRejectFields {
        order_id: order_id.unwrap() /* better error hdl? */ ,
        secondary_order_id: secondary_order_id,
        cl_ord_id: cl_ord_id.unwrap() /* better error hdl? */ ,
        orig_cl_ord_id: orig_cl_ord_id.unwrap() /* better error hdl? */ ,
        ord_status: ord_status.unwrap() /* better error hdl? */ ,
        client_id: client_id,
        exec_broker: exec_broker,
        list_id: list_id,
        account: account,
        transact_time: transact_time,
        cxl_rej_response_to: cxl_rej_response_to.unwrap() /* better error hdl? */ ,
        cxl_rej_reason: cxl_rej_reason,
        text: text,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
    }
}


fn parse_message_logon_fields( consumer : &mut FixConsumer  ) -> LogonFields {
    // fields:
    let mut encrypt_method : Option<FieldEncryptMethodEnum> = None;
    let mut heart_bt_int : Option<i32> = None;
    let mut raw_data_length : Option<usize> = None;
    let mut raw_data : Option<String> = None;
    let mut reset_seq_num_flag : Option<bool> = None;
    let mut max_message_size : Option<i32> = None;
    let mut no_msg_types : Option<Vec<NoMsgTypes3Fields>> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_ENCRYPTMETHOD, val: v } => {

                encrypt_method = Some( FieldEncryptMethodEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_HEARTBTINT, val: v } => {

                heart_bt_int = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_RAWDATALENGTH, val: v } => {

                raw_data_length = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_RAWDATA, val: v } => {

                raw_data = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_RESETSEQNUMFLAG, val: v } => {

                reset_seq_num_flag = Some( boolconv(v) );
            },
            &FieldVal { id: FIELD_MAXMESSAGESIZE, val: v } => {

                max_message_size = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_NOMSGTYPES, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_msg_types3_fields(consumer, size);
                no_msg_types = Some(subgroup);
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    LogonFields {
        encrypt_method: encrypt_method.unwrap() /* better error hdl? */ ,
        heart_bt_int: heart_bt_int.unwrap() /* better error hdl? */ ,
        raw_data_length: raw_data_length,
        raw_data: raw_data,
        reset_seq_num_flag: reset_seq_num_flag,
        max_message_size: max_message_size,
        no_msg_types: no_msg_types,
    }
}


fn parse_message_new_order_single_fields( consumer : &mut FixConsumer  ) -> NewOrderSingleFields {
    // fields:
    let mut cl_ord_id : Option<String> = None;
    let mut client_id : Option<String> = None;
    let mut exec_broker : Option<String> = None;
    let mut account : Option<String> = None;
    let mut no_allocs : Option<Vec<NoAllocs1Fields>> = None;
    let mut settlmnt_typ : Option<FieldSettlmntTypEnum> = None;
    let mut fut_sett_date : Option<UtcDateTime> = None;
    let mut handl_inst : Option<FieldHandlInstEnum> = None;
    let mut exec_inst : Option<FieldExecInstEnum> = None;
    let mut min_qty : Option<f32> = None;
    let mut max_floor : Option<f32> = None;
    let mut ex_destination : Option<String> = None;
    let mut no_trading_sessions : Option<Vec<NoTradingSessions5Fields>> = None;
    let mut process_code : Option<FieldProcessCodeEnum> = None;
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<FieldIDSourceEnum> = None;
    let mut security_type : Option<FieldSecurityTypeEnum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<FieldPutOrCallEnum> = None;
    let mut strike_price : Option<f32> = None;
    let mut opt_attribute : Option<char> = None;
    let mut contract_multiplier : Option<f32> = None;
    let mut coupon_rate : Option<f32> = None;
    let mut security_exchange : Option<String> = None;
    let mut issuer : Option<String> = None;
    let mut encoded_issuer_len : Option<usize> = None;
    let mut encoded_issuer : Option<String> = None;
    let mut security_desc : Option<String> = None;
    let mut encoded_security_desc_len : Option<usize> = None;
    let mut encoded_security_desc : Option<String> = None;
    let mut prev_close_px : Option<f32> = None;
    let mut side : Option<FieldSideEnum> = None;
    let mut locate_reqd : Option<bool> = None;
    let mut transact_time : Option<UtcDateTime> = None;
    let mut order_qty : Option<f32> = None;
    let mut cash_order_qty : Option<f32> = None;
    let mut ord_type : Option<FieldOrdTypeEnum> = None;
    let mut price : Option<f32> = None;
    let mut stop_px : Option<f32> = None;
    let mut currency : Option<f32> = None;
    let mut compliance_id : Option<String> = None;
    let mut solicited_flag : Option<bool> = None;
    let mut ioiid : Option<String> = None;
    let mut quote_id : Option<String> = None;
    let mut time_in_force : Option<FieldTimeInForceEnum> = None;
    let mut effective_time : Option<UtcDateTime> = None;
    let mut expire_date : Option<UtcDateTime> = None;
    let mut expire_time : Option<UtcDateTime> = None;
    let mut gtbooking_inst : Option<FieldGTBookingInstEnum> = None;
    let mut commission : Option<f32> = None;
    let mut comm_type : Option<FieldCommTypeEnum> = None;
    let mut rule80_a : Option<FieldRule80AEnum> = None;
    let mut forex_req : Option<bool> = None;
    let mut settl_currency : Option<f32> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;
    let mut fut_sett_date2 : Option<UtcDateTime> = None;
    let mut order_qty2 : Option<f32> = None;
    let mut open_close : Option<FieldOpenCloseEnum> = None;
    let mut covered_or_uncovered : Option<FieldCoveredOrUncoveredEnum> = None;
    let mut customer_or_firm : Option<FieldCustomerOrFirmEnum> = None;
    let mut max_show : Option<f32> = None;
    let mut peg_difference : Option<f32> = None;
    let mut discretion_inst : Option<FieldDiscretionInstEnum> = None;
    let mut discretion_offset : Option<f32> = None;
    let mut clearing_firm : Option<String> = None;
    let mut clearing_account : Option<String> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_CLORDID, val: v } => {

                cl_ord_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_CLIENTID, val: v } => {

                client_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_EXECBROKER, val: v } => {

                exec_broker = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ACCOUNT, val: v } => {

                account = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_NOALLOCS, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_allocs1_fields(consumer, size);
                no_allocs = Some(subgroup);
            },
            &FieldVal { id: FIELD_SETTLMNTTYP, val: v } => {

                settlmnt_typ = Some( FieldSettlmntTypEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_FUTSETTDATE, val: v } => {

                fut_sett_date = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_HANDLINST, val: v } => {

                handl_inst = Some( FieldHandlInstEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_EXECINST, val: v } => {

                exec_inst = Some( FieldExecInstEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MINQTY, val: v } => {

                min_qty = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MAXFLOOR, val: v } => {

                max_floor = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_EXDESTINATION, val: v } => {

                ex_destination = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_NOTRADINGSESSIONS, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_trading_sessions5_fields(consumer, size);
                no_trading_sessions = Some(subgroup);
            },
            &FieldVal { id: FIELD_PROCESSCODE, val: v } => {

                process_code = Some( FieldProcessCodeEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SYMBOL, val: v } => {

                symbol = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SYMBOLSFX, val: v } => {

                symbol_sfx = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SECURITYID, val: v } => {

                security_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_IDSOURCE, val: v } => {

                idsource = Some( FieldIDSourceEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                security_type = Some( FieldSecurityTypeEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYMONTHYEAR, val: v } => {

                maturity_month_year = Some( UtcDate::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYDAY, val: v } => {

                maturity_day = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PUTORCALL, val: v } => {

                put_or_call = Some( FieldPutOrCallEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_STRIKEPRICE, val: v } => {

                strike_price = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_OPTATTRIBUTE, val: v } => {

                opt_attribute = Some( char::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CONTRACTMULTIPLIER, val: v } => {

                contract_multiplier = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_COUPONRATE, val: v } => {

                coupon_rate = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYEXCHANGE, val: v } => {

                security_exchange = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ISSUER, val: v } => {

                issuer = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ENCODEDISSUERLEN, val: v } => {

                encoded_issuer_len = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ENCODEDISSUER, val: v } => {

                encoded_issuer = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SECURITYDESC, val: v } => {

                security_desc = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ENCODEDSECURITYDESCLEN, val: v } => {

                encoded_security_desc_len = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ENCODEDSECURITYDESC, val: v } => {

                encoded_security_desc = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_PREVCLOSEPX, val: v } => {

                prev_close_px = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SIDE, val: v } => {

                side = Some( FieldSideEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_LOCATEREQD, val: v } => {

                locate_reqd = Some( boolconv(v) );
            },
            &FieldVal { id: FIELD_TRANSACTTIME, val: v } => {

                transact_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ORDERQTY, val: v } => {

                order_qty = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CASHORDERQTY, val: v } => {

                cash_order_qty = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ORDTYPE, val: v } => {

                ord_type = Some( FieldOrdTypeEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PRICE, val: v } => {

                price = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_STOPPX, val: v } => {

                stop_px = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CURRENCY, val: v } => {

                currency = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_COMPLIANCEID, val: v } => {

                compliance_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SOLICITEDFLAG, val: v } => {

                solicited_flag = Some( boolconv(v) );
            },
            &FieldVal { id: FIELD_IOIID, val: v } => {

                ioiid = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_QUOTEID, val: v } => {

                quote_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_TIMEINFORCE, val: v } => {

                time_in_force = Some( FieldTimeInForceEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_EFFECTIVETIME, val: v } => {

                effective_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_EXPIREDATE, val: v } => {

                expire_date = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_EXPIRETIME, val: v } => {

                expire_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_GTBOOKINGINST, val: v } => {

                gtbooking_inst = Some( FieldGTBookingInstEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_COMMISSION, val: v } => {

                commission = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_COMMTYPE, val: v } => {

                comm_type = Some( FieldCommTypeEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_RULE80A, val: v } => {

                rule80_a = Some( FieldRule80AEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_FOREXREQ, val: v } => {

                forex_req = Some( boolconv(v) );
            },
            &FieldVal { id: FIELD_SETTLCURRENCY, val: v } => {

                settl_currency = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TEXT, val: v } => {

                text = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ENCODEDTEXTLEN, val: v } => {

                encoded_text_len = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ENCODEDTEXT, val: v } => {

                encoded_text = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_FUTSETTDATE2, val: v } => {

                fut_sett_date2 = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ORDERQTY2, val: v } => {

                order_qty2 = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_OPENCLOSE, val: v } => {

                open_close = Some( FieldOpenCloseEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_COVEREDORUNCOVERED, val: v } => {

                covered_or_uncovered = Some( FieldCoveredOrUncoveredEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CUSTOMERORFIRM, val: v } => {

                customer_or_firm = Some( FieldCustomerOrFirmEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MAXSHOW, val: v } => {

                max_show = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PEGDIFFERENCE, val: v } => {

                peg_difference = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_DISCRETIONINST, val: v } => {

                discretion_inst = Some( FieldDiscretionInstEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_DISCRETIONOFFSET, val: v } => {

                discretion_offset = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CLEARINGFIRM, val: v } => {

                clearing_firm = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_CLEARINGACCOUNT, val: v } => {

                clearing_account = Some( v.to_string() );
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    NewOrderSingleFields {
        cl_ord_id: cl_ord_id.unwrap() /* better error hdl? */ ,
        client_id: client_id,
        exec_broker: exec_broker,
        account: account,
        no_allocs: no_allocs,
        settlmnt_typ: settlmnt_typ,
        fut_sett_date: fut_sett_date,
        handl_inst: handl_inst.unwrap() /* better error hdl? */ ,
        exec_inst: exec_inst,
        min_qty: min_qty,
        max_floor: max_floor,
        ex_destination: ex_destination,
        no_trading_sessions: no_trading_sessions,
        process_code: process_code,
        symbol: symbol.unwrap() /* better error hdl? */ ,
        symbol_sfx: symbol_sfx,
        security_id: security_id,
        idsource: idsource,
        security_type: security_type,
        maturity_month_year: maturity_month_year,
        maturity_day: maturity_day,
        put_or_call: put_or_call,
        strike_price: strike_price,
        opt_attribute: opt_attribute,
        contract_multiplier: contract_multiplier,
        coupon_rate: coupon_rate,
        security_exchange: security_exchange,
        issuer: issuer,
        encoded_issuer_len: encoded_issuer_len,
        encoded_issuer: encoded_issuer,
        security_desc: security_desc,
        encoded_security_desc_len: encoded_security_desc_len,
        encoded_security_desc: encoded_security_desc,
        prev_close_px: prev_close_px,
        side: side.unwrap() /* better error hdl? */ ,
        locate_reqd: locate_reqd,
        transact_time: transact_time.unwrap() /* better error hdl? */ ,
        order_qty: order_qty,
        cash_order_qty: cash_order_qty,
        ord_type: ord_type.unwrap() /* better error hdl? */ ,
        price: price,
        stop_px: stop_px,
        currency: currency,
        compliance_id: compliance_id,
        solicited_flag: solicited_flag,
        ioiid: ioiid,
        quote_id: quote_id,
        time_in_force: time_in_force,
        effective_time: effective_time,
        expire_date: expire_date,
        expire_time: expire_time,
        gtbooking_inst: gtbooking_inst,
        commission: commission,
        comm_type: comm_type,
        rule80_a: rule80_a,
        forex_req: forex_req,
        settl_currency: settl_currency,
        text: text,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
        fut_sett_date2: fut_sett_date2,
        order_qty2: order_qty2,
        open_close: open_close,
        covered_or_uncovered: covered_or_uncovered,
        customer_or_firm: customer_or_firm,
        max_show: max_show,
        peg_difference: peg_difference,
        discretion_inst: discretion_inst,
        discretion_offset: discretion_offset,
        clearing_firm: clearing_firm,
        clearing_account: clearing_account,
    }
}


fn parse_message_new_order_list_fields( consumer : &mut FixConsumer  ) -> NewOrderListFields {
    // fields:
    let mut list_id : Option<String> = None;
    let mut bid_id : Option<String> = None;
    let mut client_bid_id : Option<String> = None;
    let mut prog_rpt_reqs : Option<FieldProgRptReqsEnum> = None;
    let mut bid_type : Option<i32> = None;
    let mut prog_period_interval : Option<i32> = None;
    let mut list_exec_inst_type : Option<FieldListExecInstTypeEnum> = None;
    let mut list_exec_inst : Option<String> = None;
    let mut encoded_list_exec_inst_len : Option<usize> = None;
    let mut encoded_list_exec_inst : Option<String> = None;
    let mut tot_no_orders : Option<i32> = None;
    let mut no_orders : Option<Vec<NoOrders4Fields>> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_LISTID, val: v } => {

                list_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_BIDID, val: v } => {

                bid_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_CLIENTBIDID, val: v } => {

                client_bid_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_PROGRPTREQS, val: v } => {

                prog_rpt_reqs = Some( FieldProgRptReqsEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_BIDTYPE, val: v } => {

                bid_type = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PROGPERIODINTERVAL, val: v } => {

                prog_period_interval = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_LISTEXECINSTTYPE, val: v } => {

                list_exec_inst_type = Some( FieldListExecInstTypeEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_LISTEXECINST, val: v } => {

                list_exec_inst = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ENCODEDLISTEXECINSTLEN, val: v } => {

                encoded_list_exec_inst_len = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ENCODEDLISTEXECINST, val: v } => {

                encoded_list_exec_inst = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_TOTNOORDERS, val: v } => {

                tot_no_orders = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_NOORDERS, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_orders4_fields(consumer, size);
                no_orders = Some(subgroup);
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    NewOrderListFields {
        list_id: list_id.unwrap() /* better error hdl? */ ,
        bid_id: bid_id,
        client_bid_id: client_bid_id,
        prog_rpt_reqs: prog_rpt_reqs,
        bid_type: bid_type.unwrap() /* better error hdl? */ ,
        prog_period_interval: prog_period_interval,
        list_exec_inst_type: list_exec_inst_type,
        list_exec_inst: list_exec_inst,
        encoded_list_exec_inst_len: encoded_list_exec_inst_len,
        encoded_list_exec_inst: encoded_list_exec_inst,
        tot_no_orders: tot_no_orders.unwrap() /* better error hdl? */ ,
        no_orders: no_orders.unwrap() /* better error hdl? */ ,
    }
}


fn parse_message_order_cancel_request_fields( consumer : &mut FixConsumer  ) -> OrderCancelRequestFields {
    // fields:
    let mut orig_cl_ord_id : Option<String> = None;
    let mut order_id : Option<String> = None;
    let mut cl_ord_id : Option<String> = None;
    let mut list_id : Option<String> = None;
    let mut account : Option<String> = None;
    let mut client_id : Option<String> = None;
    let mut exec_broker : Option<String> = None;
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<FieldIDSourceEnum> = None;
    let mut security_type : Option<FieldSecurityTypeEnum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<FieldPutOrCallEnum> = None;
    let mut strike_price : Option<f32> = None;
    let mut opt_attribute : Option<char> = None;
    let mut contract_multiplier : Option<f32> = None;
    let mut coupon_rate : Option<f32> = None;
    let mut security_exchange : Option<String> = None;
    let mut issuer : Option<String> = None;
    let mut encoded_issuer_len : Option<usize> = None;
    let mut encoded_issuer : Option<String> = None;
    let mut security_desc : Option<String> = None;
    let mut encoded_security_desc_len : Option<usize> = None;
    let mut encoded_security_desc : Option<String> = None;
    let mut side : Option<FieldSideEnum> = None;
    let mut transact_time : Option<UtcDateTime> = None;
    let mut order_qty : Option<f32> = None;
    let mut cash_order_qty : Option<f32> = None;
    let mut compliance_id : Option<String> = None;
    let mut solicited_flag : Option<bool> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_ORIGCLORDID, val: v } => {

                orig_cl_ord_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ORDERID, val: v } => {

                order_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_CLORDID, val: v } => {

                cl_ord_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_LISTID, val: v } => {

                list_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ACCOUNT, val: v } => {

                account = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_CLIENTID, val: v } => {

                client_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_EXECBROKER, val: v } => {

                exec_broker = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SYMBOL, val: v } => {

                symbol = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SYMBOLSFX, val: v } => {

                symbol_sfx = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SECURITYID, val: v } => {

                security_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_IDSOURCE, val: v } => {

                idsource = Some( FieldIDSourceEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                security_type = Some( FieldSecurityTypeEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYMONTHYEAR, val: v } => {

                maturity_month_year = Some( UtcDate::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYDAY, val: v } => {

                maturity_day = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PUTORCALL, val: v } => {

                put_or_call = Some( FieldPutOrCallEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_STRIKEPRICE, val: v } => {

                strike_price = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_OPTATTRIBUTE, val: v } => {

                opt_attribute = Some( char::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CONTRACTMULTIPLIER, val: v } => {

                contract_multiplier = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_COUPONRATE, val: v } => {

                coupon_rate = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYEXCHANGE, val: v } => {

                security_exchange = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ISSUER, val: v } => {

                issuer = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ENCODEDISSUERLEN, val: v } => {

                encoded_issuer_len = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ENCODEDISSUER, val: v } => {

                encoded_issuer = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SECURITYDESC, val: v } => {

                security_desc = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ENCODEDSECURITYDESCLEN, val: v } => {

                encoded_security_desc_len = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ENCODEDSECURITYDESC, val: v } => {

                encoded_security_desc = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SIDE, val: v } => {

                side = Some( FieldSideEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRANSACTTIME, val: v } => {

                transact_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ORDERQTY, val: v } => {

                order_qty = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CASHORDERQTY, val: v } => {

                cash_order_qty = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_COMPLIANCEID, val: v } => {

                compliance_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SOLICITEDFLAG, val: v } => {

                solicited_flag = Some( boolconv(v) );
            },
            &FieldVal { id: FIELD_TEXT, val: v } => {

                text = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ENCODEDTEXTLEN, val: v } => {

                encoded_text_len = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ENCODEDTEXT, val: v } => {

                encoded_text = Some( v.to_string() );
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    OrderCancelRequestFields {
        orig_cl_ord_id: orig_cl_ord_id.unwrap() /* better error hdl? */ ,
        order_id: order_id,
        cl_ord_id: cl_ord_id.unwrap() /* better error hdl? */ ,
        list_id: list_id,
        account: account,
        client_id: client_id,
        exec_broker: exec_broker,
        symbol: symbol.unwrap() /* better error hdl? */ ,
        symbol_sfx: symbol_sfx,
        security_id: security_id,
        idsource: idsource,
        security_type: security_type,
        maturity_month_year: maturity_month_year,
        maturity_day: maturity_day,
        put_or_call: put_or_call,
        strike_price: strike_price,
        opt_attribute: opt_attribute,
        contract_multiplier: contract_multiplier,
        coupon_rate: coupon_rate,
        security_exchange: security_exchange,
        issuer: issuer,
        encoded_issuer_len: encoded_issuer_len,
        encoded_issuer: encoded_issuer,
        security_desc: security_desc,
        encoded_security_desc_len: encoded_security_desc_len,
        encoded_security_desc: encoded_security_desc,
        side: side.unwrap() /* better error hdl? */ ,
        transact_time: transact_time.unwrap() /* better error hdl? */ ,
        order_qty: order_qty,
        cash_order_qty: cash_order_qty,
        compliance_id: compliance_id,
        solicited_flag: solicited_flag,
        text: text,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
    }
}


fn parse_message_order_cancel_replace_request_fields( consumer : &mut FixConsumer  ) -> OrderCancelReplaceRequestFields {
    // fields:
    let mut order_id : Option<String> = None;
    let mut client_id : Option<String> = None;
    let mut exec_broker : Option<String> = None;
    let mut orig_cl_ord_id : Option<String> = None;
    let mut cl_ord_id : Option<String> = None;
    let mut list_id : Option<String> = None;
    let mut account : Option<String> = None;
    let mut no_allocs : Option<Vec<NoAllocs1Fields>> = None;
    let mut settlmnt_typ : Option<FieldSettlmntTypEnum> = None;
    let mut fut_sett_date : Option<UtcDateTime> = None;
    let mut handl_inst : Option<FieldHandlInstEnum> = None;
    let mut exec_inst : Option<FieldExecInstEnum> = None;
    let mut min_qty : Option<f32> = None;
    let mut max_floor : Option<f32> = None;
    let mut ex_destination : Option<String> = None;
    let mut no_trading_sessions : Option<Vec<NoTradingSessions5Fields>> = None;
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<FieldIDSourceEnum> = None;
    let mut security_type : Option<FieldSecurityTypeEnum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<FieldPutOrCallEnum> = None;
    let mut strike_price : Option<f32> = None;
    let mut opt_attribute : Option<char> = None;
    let mut contract_multiplier : Option<f32> = None;
    let mut coupon_rate : Option<f32> = None;
    let mut security_exchange : Option<String> = None;
    let mut issuer : Option<String> = None;
    let mut encoded_issuer_len : Option<usize> = None;
    let mut encoded_issuer : Option<String> = None;
    let mut security_desc : Option<String> = None;
    let mut encoded_security_desc_len : Option<usize> = None;
    let mut encoded_security_desc : Option<String> = None;
    let mut side : Option<FieldSideEnum> = None;
    let mut transact_time : Option<UtcDateTime> = None;
    let mut order_qty : Option<f32> = None;
    let mut cash_order_qty : Option<f32> = None;
    let mut ord_type : Option<FieldOrdTypeEnum> = None;
    let mut price : Option<f32> = None;
    let mut stop_px : Option<f32> = None;
    let mut peg_difference : Option<f32> = None;
    let mut discretion_inst : Option<FieldDiscretionInstEnum> = None;
    let mut discretion_offset : Option<f32> = None;
    let mut compliance_id : Option<String> = None;
    let mut solicited_flag : Option<bool> = None;
    let mut currency : Option<f32> = None;
    let mut time_in_force : Option<FieldTimeInForceEnum> = None;
    let mut effective_time : Option<UtcDateTime> = None;
    let mut expire_date : Option<UtcDateTime> = None;
    let mut expire_time : Option<UtcDateTime> = None;
    let mut gtbooking_inst : Option<FieldGTBookingInstEnum> = None;
    let mut commission : Option<f32> = None;
    let mut comm_type : Option<FieldCommTypeEnum> = None;
    let mut rule80_a : Option<FieldRule80AEnum> = None;
    let mut forex_req : Option<bool> = None;
    let mut settl_currency : Option<f32> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;
    let mut fut_sett_date2 : Option<UtcDateTime> = None;
    let mut order_qty2 : Option<f32> = None;
    let mut open_close : Option<FieldOpenCloseEnum> = None;
    let mut covered_or_uncovered : Option<FieldCoveredOrUncoveredEnum> = None;
    let mut customer_or_firm : Option<FieldCustomerOrFirmEnum> = None;
    let mut max_show : Option<f32> = None;
    let mut locate_reqd : Option<bool> = None;
    let mut clearing_firm : Option<String> = None;
    let mut clearing_account : Option<String> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_ORDERID, val: v } => {

                order_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_CLIENTID, val: v } => {

                client_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_EXECBROKER, val: v } => {

                exec_broker = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ORIGCLORDID, val: v } => {

                orig_cl_ord_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_CLORDID, val: v } => {

                cl_ord_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_LISTID, val: v } => {

                list_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ACCOUNT, val: v } => {

                account = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_NOALLOCS, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_allocs1_fields(consumer, size);
                no_allocs = Some(subgroup);
            },
            &FieldVal { id: FIELD_SETTLMNTTYP, val: v } => {

                settlmnt_typ = Some( FieldSettlmntTypEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_FUTSETTDATE, val: v } => {

                fut_sett_date = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_HANDLINST, val: v } => {

                handl_inst = Some( FieldHandlInstEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_EXECINST, val: v } => {

                exec_inst = Some( FieldExecInstEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MINQTY, val: v } => {

                min_qty = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MAXFLOOR, val: v } => {

                max_floor = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_EXDESTINATION, val: v } => {

                ex_destination = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_NOTRADINGSESSIONS, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_trading_sessions5_fields(consumer, size);
                no_trading_sessions = Some(subgroup);
            },
            &FieldVal { id: FIELD_SYMBOL, val: v } => {

                symbol = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SYMBOLSFX, val: v } => {

                symbol_sfx = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SECURITYID, val: v } => {

                security_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_IDSOURCE, val: v } => {

                idsource = Some( FieldIDSourceEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                security_type = Some( FieldSecurityTypeEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYMONTHYEAR, val: v } => {

                maturity_month_year = Some( UtcDate::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYDAY, val: v } => {

                maturity_day = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PUTORCALL, val: v } => {

                put_or_call = Some( FieldPutOrCallEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_STRIKEPRICE, val: v } => {

                strike_price = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_OPTATTRIBUTE, val: v } => {

                opt_attribute = Some( char::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CONTRACTMULTIPLIER, val: v } => {

                contract_multiplier = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_COUPONRATE, val: v } => {

                coupon_rate = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYEXCHANGE, val: v } => {

                security_exchange = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ISSUER, val: v } => {

                issuer = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ENCODEDISSUERLEN, val: v } => {

                encoded_issuer_len = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ENCODEDISSUER, val: v } => {

                encoded_issuer = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SECURITYDESC, val: v } => {

                security_desc = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ENCODEDSECURITYDESCLEN, val: v } => {

                encoded_security_desc_len = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ENCODEDSECURITYDESC, val: v } => {

                encoded_security_desc = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SIDE, val: v } => {

                side = Some( FieldSideEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRANSACTTIME, val: v } => {

                transact_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ORDERQTY, val: v } => {

                order_qty = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CASHORDERQTY, val: v } => {

                cash_order_qty = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ORDTYPE, val: v } => {

                ord_type = Some( FieldOrdTypeEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PRICE, val: v } => {

                price = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_STOPPX, val: v } => {

                stop_px = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PEGDIFFERENCE, val: v } => {

                peg_difference = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_DISCRETIONINST, val: v } => {

                discretion_inst = Some( FieldDiscretionInstEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_DISCRETIONOFFSET, val: v } => {

                discretion_offset = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_COMPLIANCEID, val: v } => {

                compliance_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SOLICITEDFLAG, val: v } => {

                solicited_flag = Some( boolconv(v) );
            },
            &FieldVal { id: FIELD_CURRENCY, val: v } => {

                currency = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TIMEINFORCE, val: v } => {

                time_in_force = Some( FieldTimeInForceEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_EFFECTIVETIME, val: v } => {

                effective_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_EXPIREDATE, val: v } => {

                expire_date = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_EXPIRETIME, val: v } => {

                expire_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_GTBOOKINGINST, val: v } => {

                gtbooking_inst = Some( FieldGTBookingInstEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_COMMISSION, val: v } => {

                commission = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_COMMTYPE, val: v } => {

                comm_type = Some( FieldCommTypeEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_RULE80A, val: v } => {

                rule80_a = Some( FieldRule80AEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_FOREXREQ, val: v } => {

                forex_req = Some( boolconv(v) );
            },
            &FieldVal { id: FIELD_SETTLCURRENCY, val: v } => {

                settl_currency = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TEXT, val: v } => {

                text = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ENCODEDTEXTLEN, val: v } => {

                encoded_text_len = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ENCODEDTEXT, val: v } => {

                encoded_text = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_FUTSETTDATE2, val: v } => {

                fut_sett_date2 = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ORDERQTY2, val: v } => {

                order_qty2 = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_OPENCLOSE, val: v } => {

                open_close = Some( FieldOpenCloseEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_COVEREDORUNCOVERED, val: v } => {

                covered_or_uncovered = Some( FieldCoveredOrUncoveredEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CUSTOMERORFIRM, val: v } => {

                customer_or_firm = Some( FieldCustomerOrFirmEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MAXSHOW, val: v } => {

                max_show = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_LOCATEREQD, val: v } => {

                locate_reqd = Some( boolconv(v) );
            },
            &FieldVal { id: FIELD_CLEARINGFIRM, val: v } => {

                clearing_firm = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_CLEARINGACCOUNT, val: v } => {

                clearing_account = Some( v.to_string() );
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    OrderCancelReplaceRequestFields {
        order_id: order_id,
        client_id: client_id,
        exec_broker: exec_broker,
        orig_cl_ord_id: orig_cl_ord_id.unwrap() /* better error hdl? */ ,
        cl_ord_id: cl_ord_id.unwrap() /* better error hdl? */ ,
        list_id: list_id,
        account: account,
        no_allocs: no_allocs,
        settlmnt_typ: settlmnt_typ,
        fut_sett_date: fut_sett_date,
        handl_inst: handl_inst.unwrap() /* better error hdl? */ ,
        exec_inst: exec_inst,
        min_qty: min_qty,
        max_floor: max_floor,
        ex_destination: ex_destination,
        no_trading_sessions: no_trading_sessions,
        symbol: symbol.unwrap() /* better error hdl? */ ,
        symbol_sfx: symbol_sfx,
        security_id: security_id,
        idsource: idsource,
        security_type: security_type,
        maturity_month_year: maturity_month_year,
        maturity_day: maturity_day,
        put_or_call: put_or_call,
        strike_price: strike_price,
        opt_attribute: opt_attribute,
        contract_multiplier: contract_multiplier,
        coupon_rate: coupon_rate,
        security_exchange: security_exchange,
        issuer: issuer,
        encoded_issuer_len: encoded_issuer_len,
        encoded_issuer: encoded_issuer,
        security_desc: security_desc,
        encoded_security_desc_len: encoded_security_desc_len,
        encoded_security_desc: encoded_security_desc,
        side: side.unwrap() /* better error hdl? */ ,
        transact_time: transact_time.unwrap() /* better error hdl? */ ,
        order_qty: order_qty,
        cash_order_qty: cash_order_qty,
        ord_type: ord_type.unwrap() /* better error hdl? */ ,
        price: price,
        stop_px: stop_px,
        peg_difference: peg_difference,
        discretion_inst: discretion_inst,
        discretion_offset: discretion_offset,
        compliance_id: compliance_id,
        solicited_flag: solicited_flag,
        currency: currency,
        time_in_force: time_in_force,
        effective_time: effective_time,
        expire_date: expire_date,
        expire_time: expire_time,
        gtbooking_inst: gtbooking_inst,
        commission: commission,
        comm_type: comm_type,
        rule80_a: rule80_a,
        forex_req: forex_req,
        settl_currency: settl_currency,
        text: text,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
        fut_sett_date2: fut_sett_date2,
        order_qty2: order_qty2,
        open_close: open_close,
        covered_or_uncovered: covered_or_uncovered,
        customer_or_firm: customer_or_firm,
        max_show: max_show,
        locate_reqd: locate_reqd,
        clearing_firm: clearing_firm,
        clearing_account: clearing_account,
    }
}


fn parse_message_order_status_request_fields( consumer : &mut FixConsumer  ) -> OrderStatusRequestFields {
    // fields:
    let mut order_id : Option<String> = None;
    let mut cl_ord_id : Option<String> = None;
    let mut client_id : Option<String> = None;
    let mut account : Option<String> = None;
    let mut exec_broker : Option<String> = None;
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<FieldIDSourceEnum> = None;
    let mut security_type : Option<FieldSecurityTypeEnum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<FieldPutOrCallEnum> = None;
    let mut strike_price : Option<f32> = None;
    let mut opt_attribute : Option<char> = None;
    let mut contract_multiplier : Option<f32> = None;
    let mut coupon_rate : Option<f32> = None;
    let mut security_exchange : Option<String> = None;
    let mut issuer : Option<String> = None;
    let mut encoded_issuer_len : Option<usize> = None;
    let mut encoded_issuer : Option<String> = None;
    let mut security_desc : Option<String> = None;
    let mut encoded_security_desc_len : Option<usize> = None;
    let mut encoded_security_desc : Option<String> = None;
    let mut side : Option<FieldSideEnum> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_ORDERID, val: v } => {

                order_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_CLORDID, val: v } => {

                cl_ord_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_CLIENTID, val: v } => {

                client_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ACCOUNT, val: v } => {

                account = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_EXECBROKER, val: v } => {

                exec_broker = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SYMBOL, val: v } => {

                symbol = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SYMBOLSFX, val: v } => {

                symbol_sfx = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SECURITYID, val: v } => {

                security_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_IDSOURCE, val: v } => {

                idsource = Some( FieldIDSourceEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                security_type = Some( FieldSecurityTypeEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYMONTHYEAR, val: v } => {

                maturity_month_year = Some( UtcDate::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYDAY, val: v } => {

                maturity_day = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PUTORCALL, val: v } => {

                put_or_call = Some( FieldPutOrCallEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_STRIKEPRICE, val: v } => {

                strike_price = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_OPTATTRIBUTE, val: v } => {

                opt_attribute = Some( char::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CONTRACTMULTIPLIER, val: v } => {

                contract_multiplier = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_COUPONRATE, val: v } => {

                coupon_rate = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYEXCHANGE, val: v } => {

                security_exchange = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ISSUER, val: v } => {

                issuer = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ENCODEDISSUERLEN, val: v } => {

                encoded_issuer_len = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ENCODEDISSUER, val: v } => {

                encoded_issuer = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SECURITYDESC, val: v } => {

                security_desc = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ENCODEDSECURITYDESCLEN, val: v } => {

                encoded_security_desc_len = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ENCODEDSECURITYDESC, val: v } => {

                encoded_security_desc = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SIDE, val: v } => {

                side = Some( FieldSideEnum::from_str(v).unwrap() );
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    OrderStatusRequestFields {
        order_id: order_id,
        cl_ord_id: cl_ord_id.unwrap() /* better error hdl? */ ,
        client_id: client_id,
        account: account,
        exec_broker: exec_broker,
        symbol: symbol.unwrap() /* better error hdl? */ ,
        symbol_sfx: symbol_sfx,
        security_id: security_id,
        idsource: idsource,
        security_type: security_type,
        maturity_month_year: maturity_month_year,
        maturity_day: maturity_day,
        put_or_call: put_or_call,
        strike_price: strike_price,
        opt_attribute: opt_attribute,
        contract_multiplier: contract_multiplier,
        coupon_rate: coupon_rate,
        security_exchange: security_exchange,
        issuer: issuer,
        encoded_issuer_len: encoded_issuer_len,
        encoded_issuer: encoded_issuer,
        security_desc: security_desc,
        encoded_security_desc_len: encoded_security_desc_len,
        encoded_security_desc: encoded_security_desc,
        side: side.unwrap() /* better error hdl? */ ,
    }
}





fn build_group_no_contra_brokers2_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoContraBrokers2Fields> {
    let mut items : Vec<NoContraBrokers2Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_contra_brokers2_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_contra_brokers2_fields_line(consumer: &mut FixConsumer) -> NoContraBrokers2Fields {
    // fields
    let mut contra_broker : Option<String> = None;
    let mut contra_trader : Option<String> = None;
    let mut contra_trade_qty : Option<f32> = None;
    let mut contra_trade_time : Option<UtcDateTime> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_CONTRABROKER, val: v } => {

                if contra_broker.is_some() { break; }

                contra_broker = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_CONTRATRADER, val: v } => {

                if contra_trader.is_some() { break; }

                contra_trader = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_CONTRATRADEQTY, val: v } => {

                if contra_trade_qty.is_some() { break; }

                contra_trade_qty = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_CONTRATRADETIME, val: v } => {

                if contra_trade_time.is_some() { break; }

                contra_trade_time = Some( UtcDateTime::from_str(v).unwrap() );
            },


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoContraBrokers2Fields {
        contra_broker: contra_broker,
        contra_trader: contra_trader,
        contra_trade_qty: contra_trade_qty,
        contra_trade_time: contra_trade_time,
    }
}


fn build_group_no_allocs1_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoAllocs1Fields> {
    let mut items : Vec<NoAllocs1Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_allocs1_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_allocs1_fields_line(consumer: &mut FixConsumer) -> NoAllocs1Fields {
    // fields
    let mut alloc_account : Option<String> = None;
    let mut alloc_shares : Option<f32> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_ALLOCACCOUNT, val: v } => {

                if alloc_account.is_some() { break; }

                alloc_account = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_ALLOCSHARES, val: v } => {

                if alloc_shares.is_some() { break; }

                alloc_shares = Some( f32::from_str(v).unwrap() );
            },


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoAllocs1Fields {
        alloc_account: alloc_account,
        alloc_shares: alloc_shares,
    }
}


fn build_group_no_msg_types3_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoMsgTypes3Fields> {
    let mut items : Vec<NoMsgTypes3Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_msg_types3_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_msg_types3_fields_line(consumer: &mut FixConsumer) -> NoMsgTypes3Fields {
    // fields
    let mut ref_msg_type : Option<String> = None;
    let mut msg_direction : Option<FieldMsgDirectionEnum> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_REFMSGTYPE, val: v } => {

                if ref_msg_type.is_some() { break; }

                ref_msg_type = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_MSGDIRECTION, val: v } => {

                if msg_direction.is_some() { break; }

                msg_direction = Some( FieldMsgDirectionEnum::from_str(v).unwrap() );
            },


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoMsgTypes3Fields {
        ref_msg_type: ref_msg_type,
        msg_direction: msg_direction,
    }
}


fn build_group_no_trading_sessions5_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoTradingSessions5Fields> {
    let mut items : Vec<NoTradingSessions5Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_trading_sessions5_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_trading_sessions5_fields_line(consumer: &mut FixConsumer) -> NoTradingSessions5Fields {
    // fields
    let mut trading_session_id : Option<String> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_TRADINGSESSIONID, val: v } => {

                if trading_session_id.is_some() { break; }

                trading_session_id = Some( v.to_string() );
            },


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoTradingSessions5Fields {
        trading_session_id: trading_session_id,
    }
}


fn build_group_no_orders4_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoOrders4Fields> {
    let mut items : Vec<NoOrders4Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_orders4_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_orders4_fields_line(consumer: &mut FixConsumer) -> NoOrders4Fields {
    // fields
    let mut cl_ord_id : Option<String> = None;
    let mut list_seq_no : Option<i32> = None;
    let mut settl_inst_mode : Option<FieldSettlInstModeEnum> = None;
    let mut client_id : Option<String> = None;
    let mut exec_broker : Option<String> = None;
    let mut account : Option<String> = None;
    let mut no_allocs : Option<Vec<NoAllocs1Fields>> = None;
    let mut settlmnt_typ : Option<FieldSettlmntTypEnum> = None;
    let mut fut_sett_date : Option<UtcDateTime> = None;
    let mut handl_inst : Option<FieldHandlInstEnum> = None;
    let mut exec_inst : Option<FieldExecInstEnum> = None;
    let mut min_qty : Option<f32> = None;
    let mut max_floor : Option<f32> = None;
    let mut ex_destination : Option<String> = None;
    let mut no_trading_sessions : Option<Vec<NoTradingSessions5Fields>> = None;
    let mut process_code : Option<FieldProcessCodeEnum> = None;
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<FieldIDSourceEnum> = None;
    let mut security_type : Option<FieldSecurityTypeEnum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<FieldPutOrCallEnum> = None;
    let mut strike_price : Option<f32> = None;
    let mut opt_attribute : Option<char> = None;
    let mut contract_multiplier : Option<f32> = None;
    let mut coupon_rate : Option<f32> = None;
    let mut security_exchange : Option<String> = None;
    let mut issuer : Option<String> = None;
    let mut encoded_issuer_len : Option<usize> = None;
    let mut encoded_issuer : Option<String> = None;
    let mut security_desc : Option<String> = None;
    let mut encoded_security_desc_len : Option<usize> = None;
    let mut encoded_security_desc : Option<String> = None;
    let mut prev_close_px : Option<f32> = None;
    let mut side : Option<FieldSideEnum> = None;
    let mut side_value_ind : Option<i32> = None;
    let mut locate_reqd : Option<bool> = None;
    let mut transact_time : Option<UtcDateTime> = None;
    let mut order_qty : Option<f32> = None;
    let mut cash_order_qty : Option<f32> = None;
    let mut ord_type : Option<FieldOrdTypeEnum> = None;
    let mut price : Option<f32> = None;
    let mut stop_px : Option<f32> = None;
    let mut currency : Option<f32> = None;
    let mut compliance_id : Option<String> = None;
    let mut solicited_flag : Option<bool> = None;
    let mut ioiid : Option<String> = None;
    let mut quote_id : Option<String> = None;
    let mut time_in_force : Option<FieldTimeInForceEnum> = None;
    let mut effective_time : Option<UtcDateTime> = None;
    let mut expire_date : Option<UtcDateTime> = None;
    let mut expire_time : Option<UtcDateTime> = None;
    let mut gtbooking_inst : Option<FieldGTBookingInstEnum> = None;
    let mut commission : Option<f32> = None;
    let mut comm_type : Option<FieldCommTypeEnum> = None;
    let mut rule80_a : Option<FieldRule80AEnum> = None;
    let mut forex_req : Option<bool> = None;
    let mut settl_currency : Option<f32> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;
    let mut fut_sett_date2 : Option<UtcDateTime> = None;
    let mut order_qty2 : Option<f32> = None;
    let mut open_close : Option<FieldOpenCloseEnum> = None;
    let mut covered_or_uncovered : Option<FieldCoveredOrUncoveredEnum> = None;
    let mut customer_or_firm : Option<FieldCustomerOrFirmEnum> = None;
    let mut max_show : Option<f32> = None;
    let mut peg_difference : Option<f32> = None;
    let mut discretion_inst : Option<FieldDiscretionInstEnum> = None;
    let mut discretion_offset : Option<f32> = None;
    let mut clearing_firm : Option<String> = None;
    let mut clearing_account : Option<String> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_CLORDID, val: v } => {

                if cl_ord_id.is_some() { break; }

                cl_ord_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_LISTSEQNO, val: v } => {

                if list_seq_no.is_some() { break; }

                list_seq_no = Some( i32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SETTLINSTMODE, val: v } => {

                if settl_inst_mode.is_some() { break; }

                settl_inst_mode = Some( FieldSettlInstModeEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_CLIENTID, val: v } => {

                if client_id.is_some() { break; }

                client_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_EXECBROKER, val: v } => {

                if exec_broker.is_some() { break; }

                exec_broker = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_ACCOUNT, val: v } => {

                if account.is_some() { break; }

                account = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_NOALLOCS, val: v } => {

                if no_allocs.is_some() { break; }

                let size = usize::from_str(v).unwrap();
                let items = build_group_no_allocs1_fields(consumer, size);
                no_allocs = Some(items);
                continue;
            },

            &FieldVal { id: FIELD_SETTLMNTTYP, val: v } => {

                if settlmnt_typ.is_some() { break; }

                settlmnt_typ = Some( FieldSettlmntTypEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_FUTSETTDATE, val: v } => {

                if fut_sett_date.is_some() { break; }

                fut_sett_date = Some( UtcDateTime::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_HANDLINST, val: v } => {

                if handl_inst.is_some() { break; }

                handl_inst = Some( FieldHandlInstEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_EXECINST, val: v } => {

                if exec_inst.is_some() { break; }

                exec_inst = Some( FieldExecInstEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_MINQTY, val: v } => {

                if min_qty.is_some() { break; }

                min_qty = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_MAXFLOOR, val: v } => {

                if max_floor.is_some() { break; }

                max_floor = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_EXDESTINATION, val: v } => {

                if ex_destination.is_some() { break; }

                ex_destination = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_NOTRADINGSESSIONS, val: v } => {

                if no_trading_sessions.is_some() { break; }

                let size = usize::from_str(v).unwrap();
                let items = build_group_no_trading_sessions5_fields(consumer, size);
                no_trading_sessions = Some(items);
                continue;
            },

            &FieldVal { id: FIELD_PROCESSCODE, val: v } => {

                if process_code.is_some() { break; }

                process_code = Some( FieldProcessCodeEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SYMBOL, val: v } => {

                if symbol.is_some() { break; }

                symbol = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_SYMBOLSFX, val: v } => {

                if symbol_sfx.is_some() { break; }

                symbol_sfx = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_SECURITYID, val: v } => {

                if security_id.is_some() { break; }

                security_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_IDSOURCE, val: v } => {

                if idsource.is_some() { break; }

                idsource = Some( FieldIDSourceEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                if security_type.is_some() { break; }

                security_type = Some( FieldSecurityTypeEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_MATURITYMONTHYEAR, val: v } => {

                if maturity_month_year.is_some() { break; }

                maturity_month_year = Some( UtcDate::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_MATURITYDAY, val: v } => {

                if maturity_day.is_some() { break; }

                maturity_day = Some( i32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_PUTORCALL, val: v } => {

                if put_or_call.is_some() { break; }

                put_or_call = Some( FieldPutOrCallEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_STRIKEPRICE, val: v } => {

                if strike_price.is_some() { break; }

                strike_price = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_OPTATTRIBUTE, val: v } => {

                if opt_attribute.is_some() { break; }

                opt_attribute = Some( char::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_CONTRACTMULTIPLIER, val: v } => {

                if contract_multiplier.is_some() { break; }

                contract_multiplier = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_COUPONRATE, val: v } => {

                if coupon_rate.is_some() { break; }

                coupon_rate = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SECURITYEXCHANGE, val: v } => {

                if security_exchange.is_some() { break; }

                security_exchange = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_ISSUER, val: v } => {

                if issuer.is_some() { break; }

                issuer = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_ENCODEDISSUERLEN, val: v } => {

                if encoded_issuer_len.is_some() { break; }

                encoded_issuer_len = Some( usize::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ENCODEDISSUER, val: v } => {

                if encoded_issuer.is_some() { break; }

                encoded_issuer = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_SECURITYDESC, val: v } => {

                if security_desc.is_some() { break; }

                security_desc = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_ENCODEDSECURITYDESCLEN, val: v } => {

                if encoded_security_desc_len.is_some() { break; }

                encoded_security_desc_len = Some( usize::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ENCODEDSECURITYDESC, val: v } => {

                if encoded_security_desc.is_some() { break; }

                encoded_security_desc = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_PREVCLOSEPX, val: v } => {

                if prev_close_px.is_some() { break; }

                prev_close_px = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SIDE, val: v } => {

                if side.is_some() { break; }

                side = Some( FieldSideEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SIDEVALUEIND, val: v } => {

                if side_value_ind.is_some() { break; }

                side_value_ind = Some( i32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_LOCATEREQD, val: v } => {

                if locate_reqd.is_some() { break; }

                locate_reqd = Some( boolconv(v) );
            },

            &FieldVal { id: FIELD_TRANSACTTIME, val: v } => {

                if transact_time.is_some() { break; }

                transact_time = Some( UtcDateTime::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ORDERQTY, val: v } => {

                if order_qty.is_some() { break; }

                order_qty = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_CASHORDERQTY, val: v } => {

                if cash_order_qty.is_some() { break; }

                cash_order_qty = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ORDTYPE, val: v } => {

                if ord_type.is_some() { break; }

                ord_type = Some( FieldOrdTypeEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_PRICE, val: v } => {

                if price.is_some() { break; }

                price = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_STOPPX, val: v } => {

                if stop_px.is_some() { break; }

                stop_px = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_CURRENCY, val: v } => {

                if currency.is_some() { break; }

                currency = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_COMPLIANCEID, val: v } => {

                if compliance_id.is_some() { break; }

                compliance_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_SOLICITEDFLAG, val: v } => {

                if solicited_flag.is_some() { break; }

                solicited_flag = Some( boolconv(v) );
            },

            &FieldVal { id: FIELD_IOIID, val: v } => {

                if ioiid.is_some() { break; }

                ioiid = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_QUOTEID, val: v } => {

                if quote_id.is_some() { break; }

                quote_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_TIMEINFORCE, val: v } => {

                if time_in_force.is_some() { break; }

                time_in_force = Some( FieldTimeInForceEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_EFFECTIVETIME, val: v } => {

                if effective_time.is_some() { break; }

                effective_time = Some( UtcDateTime::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_EXPIREDATE, val: v } => {

                if expire_date.is_some() { break; }

                expire_date = Some( UtcDateTime::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_EXPIRETIME, val: v } => {

                if expire_time.is_some() { break; }

                expire_time = Some( UtcDateTime::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_GTBOOKINGINST, val: v } => {

                if gtbooking_inst.is_some() { break; }

                gtbooking_inst = Some( FieldGTBookingInstEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_COMMISSION, val: v } => {

                if commission.is_some() { break; }

                commission = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_COMMTYPE, val: v } => {

                if comm_type.is_some() { break; }

                comm_type = Some( FieldCommTypeEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_RULE80A, val: v } => {

                if rule80_a.is_some() { break; }

                rule80_a = Some( FieldRule80AEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_FOREXREQ, val: v } => {

                if forex_req.is_some() { break; }

                forex_req = Some( boolconv(v) );
            },

            &FieldVal { id: FIELD_SETTLCURRENCY, val: v } => {

                if settl_currency.is_some() { break; }

                settl_currency = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_TEXT, val: v } => {

                if text.is_some() { break; }

                text = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_ENCODEDTEXTLEN, val: v } => {

                if encoded_text_len.is_some() { break; }

                encoded_text_len = Some( usize::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ENCODEDTEXT, val: v } => {

                if encoded_text.is_some() { break; }

                encoded_text = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_FUTSETTDATE2, val: v } => {

                if fut_sett_date2.is_some() { break; }

                fut_sett_date2 = Some( UtcDateTime::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ORDERQTY2, val: v } => {

                if order_qty2.is_some() { break; }

                order_qty2 = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_OPENCLOSE, val: v } => {

                if open_close.is_some() { break; }

                open_close = Some( FieldOpenCloseEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_COVEREDORUNCOVERED, val: v } => {

                if covered_or_uncovered.is_some() { break; }

                covered_or_uncovered = Some( FieldCoveredOrUncoveredEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_CUSTOMERORFIRM, val: v } => {

                if customer_or_firm.is_some() { break; }

                customer_or_firm = Some( FieldCustomerOrFirmEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_MAXSHOW, val: v } => {

                if max_show.is_some() { break; }

                max_show = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_PEGDIFFERENCE, val: v } => {

                if peg_difference.is_some() { break; }

                peg_difference = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_DISCRETIONINST, val: v } => {

                if discretion_inst.is_some() { break; }

                discretion_inst = Some( FieldDiscretionInstEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_DISCRETIONOFFSET, val: v } => {

                if discretion_offset.is_some() { break; }

                discretion_offset = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_CLEARINGFIRM, val: v } => {

                if clearing_firm.is_some() { break; }

                clearing_firm = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_CLEARINGACCOUNT, val: v } => {

                if clearing_account.is_some() { break; }

                clearing_account = Some( v.to_string() );
            },


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoOrders4Fields {
        cl_ord_id: cl_ord_id.unwrap() ,
        list_seq_no: list_seq_no.unwrap() ,
        settl_inst_mode: settl_inst_mode,
        client_id: client_id,
        exec_broker: exec_broker,
        account: account,
        no_allocs: no_allocs,
        settlmnt_typ: settlmnt_typ,
        fut_sett_date: fut_sett_date,
        handl_inst: handl_inst,
        exec_inst: exec_inst,
        min_qty: min_qty,
        max_floor: max_floor,
        ex_destination: ex_destination,
        no_trading_sessions: no_trading_sessions,
        process_code: process_code,
        symbol: symbol.unwrap() ,
        symbol_sfx: symbol_sfx,
        security_id: security_id,
        idsource: idsource,
        security_type: security_type,
        maturity_month_year: maturity_month_year,
        maturity_day: maturity_day,
        put_or_call: put_or_call,
        strike_price: strike_price,
        opt_attribute: opt_attribute,
        contract_multiplier: contract_multiplier,
        coupon_rate: coupon_rate,
        security_exchange: security_exchange,
        issuer: issuer,
        encoded_issuer_len: encoded_issuer_len,
        encoded_issuer: encoded_issuer,
        security_desc: security_desc,
        encoded_security_desc_len: encoded_security_desc_len,
        encoded_security_desc: encoded_security_desc,
        prev_close_px: prev_close_px,
        side: side.unwrap() ,
        side_value_ind: side_value_ind,
        locate_reqd: locate_reqd,
        transact_time: transact_time,
        order_qty: order_qty,
        cash_order_qty: cash_order_qty,
        ord_type: ord_type,
        price: price,
        stop_px: stop_px,
        currency: currency,
        compliance_id: compliance_id,
        solicited_flag: solicited_flag,
        ioiid: ioiid,
        quote_id: quote_id,
        time_in_force: time_in_force,
        effective_time: effective_time,
        expire_date: expire_date,
        expire_time: expire_time,
        gtbooking_inst: gtbooking_inst,
        commission: commission,
        comm_type: comm_type,
        rule80_a: rule80_a,
        forex_req: forex_req,
        settl_currency: settl_currency,
        text: text,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
        fut_sett_date2: fut_sett_date2,
        order_qty2: order_qty2,
        open_close: open_close,
        covered_or_uncovered: covered_or_uncovered,
        customer_or_firm: customer_or_firm,
        max_show: max_show,
        peg_difference: peg_difference,
        discretion_inst: discretion_inst,
        discretion_offset: discretion_offset,
        clearing_firm: clearing_firm,
        clearing_account: clearing_account,
    }
}



struct WriteWrapper<'a> {
    buf: &'a mut BytesMut
}

pub fn write_fix_header( header: &FixHeader, buf: &mut BytesMut ) -> io::Result<()> {
    let mut output = WriteWrapper { buf };

    // required fields and also in a specific controlled order:
    write!(output, "35={msgtype}\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
           msgtype= header.msg_type,
           ts= header.sending_time,
           seq= header.msg_seq_num,
           sender= header.sender_comp_id,
           target= header.target_comp_id )?;

    // no-required ones
    if header.on_behalf_of_comp_id.is_some() {
        let val = header.on_behalf_of_comp_id.as_ref().unwrap();
        write!(output, "115={}\u{01}", val )?; // FIELD_ONBEHALFOFCOMPID
    }
    if header.deliver_to_comp_id.is_some() {
        let val = header.deliver_to_comp_id.as_ref().unwrap();
        write!(output, "128={}\u{01}", val )?; // FIELD_DELIVERTOCOMPID
    }
    if header.secure_data_len.is_some() {
        let val = header.secure_data_len.as_ref().unwrap();
        write!(output, "90={}\u{01}", val )?; // FIELD_SECUREDATALEN
    }
    if header.secure_data.is_some() {
        let val = header.secure_data.as_ref().unwrap();
        write!(output, "91={}\u{01}", val )?; // FIELD_SECUREDATA
    }
    if header.sender_sub_id.is_some() {
        let val = header.sender_sub_id.as_ref().unwrap();
        write!(output, "50={}\u{01}", val )?; // FIELD_SENDERSUBID
    }
    if header.sender_location_id.is_some() {
        let val = header.sender_location_id.as_ref().unwrap();
        write!(output, "142={}\u{01}", val )?; // FIELD_SENDERLOCATIONID
    }
    if header.target_sub_id.is_some() {
        let val = header.target_sub_id.as_ref().unwrap();
        write!(output, "57={}\u{01}", val )?; // FIELD_TARGETSUBID
    }
    if header.target_location_id.is_some() {
        let val = header.target_location_id.as_ref().unwrap();
        write!(output, "143={}\u{01}", val )?; // FIELD_TARGETLOCATIONID
    }
    if header.on_behalf_of_sub_id.is_some() {
        let val = header.on_behalf_of_sub_id.as_ref().unwrap();
        write!(output, "116={}\u{01}", val )?; // FIELD_ONBEHALFOFSUBID
    }
    if header.on_behalf_of_location_id.is_some() {
        let val = header.on_behalf_of_location_id.as_ref().unwrap();
        write!(output, "144={}\u{01}", val )?; // FIELD_ONBEHALFOFLOCATIONID
    }
    if header.deliver_to_sub_id.is_some() {
        let val = header.deliver_to_sub_id.as_ref().unwrap();
        write!(output, "129={}\u{01}", val )?; // FIELD_DELIVERTOSUBID
    }
    if header.deliver_to_location_id.is_some() {
        let val = header.deliver_to_location_id.as_ref().unwrap();
        write!(output, "145={}\u{01}", val )?; // FIELD_DELIVERTOLOCATIONID
    }
    if header.poss_dup_flag.is_some() {
        let val = header.poss_dup_flag.as_ref().unwrap();
        write!(output, "43={}\u{01}", to_boolconv(val) )?; // FIELD_POSSDUPFLAG
    }
    if header.poss_resend.is_some() {
        let val = header.poss_resend.as_ref().unwrap();
        write!(output, "97={}\u{01}", to_boolconv(val) )?; // FIELD_POSSRESEND
    }
    if header.orig_sending_time.is_some() {
        let val = header.orig_sending_time.as_ref().unwrap();
        write!(output, "122={}\u{01}", val )?; // FIELD_ORIGSENDINGTIME
    }
    if header.xml_data_len.is_some() {
        let val = header.xml_data_len.as_ref().unwrap();
        write!(output, "212={}\u{01}", val )?; // FIELD_XMLDATALEN
    }
    if header.xml_data.is_some() {
        let val = header.xml_data.as_ref().unwrap();
        write!(output, "213={}\u{01}", val )?; // FIELD_XMLDATA
    }
    if header.message_encoding.is_some() {
        let val = header.message_encoding.as_ref().unwrap();
        write!(output, "347={}\u{01}", val )?; // FIELD_MESSAGEENCODING
    }
    if header.last_msg_seq_num_processed.is_some() {
        let val = header.last_msg_seq_num_processed.as_ref().unwrap();
        write!(output, "369={}\u{01}", val )?; // FIELD_LASTMSGSEQNUMPROCESSED
    }
    if header.on_behalf_of_sending_time.is_some() {
        let val = header.on_behalf_of_sending_time.as_ref().unwrap();
        write!(output, "370={}\u{01}", val )?; // FIELD_ONBEHALFOFSENDINGTIME
    }

    Ok( () )
}

// impl of the Write trait so we can use format! and/or write!
// directly without string allocs
impl <'a> io::Write for WriteWrapper<'a> {

    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        let len = buf.len();
        self.buf.reserve( len );
        self.buf.extend( buf );

        Ok( len )
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        // nothing to do
        Ok( () )
    }
}



pub fn write_fix_message(msg: &FixMessage, buf: &mut BytesMut) -> Result<(), io::Error> {
    let mut writer_wrapper = WriteWrapper { buf };

    match msg {

        // type: 0
        &FixMessage::Heartbeat(ref box_flds) => {
            write_heartbeat_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: 1
        &FixMessage::TestRequest(ref box_flds) => {
            write_test_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: 2
        &FixMessage::ResendRequest(ref box_flds) => {
            write_resend_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: 3
        &FixMessage::Reject(ref box_flds) => {
            write_reject_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: 4
        &FixMessage::SequenceReset(ref box_flds) => {
            write_sequence_reset_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: 5
        &FixMessage::Logout(ref box_flds) => {
            write_logout_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: 7
        &FixMessage::Advertisement(ref box_flds) => {
            write_advertisement_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: 8
        &FixMessage::ExecutionReport(ref box_flds) => {
            write_execution_report_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: 9
        &FixMessage::OrderCancelReject(ref box_flds) => {
            write_order_cancel_reject_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: A
        &FixMessage::Logon(ref box_flds) => {
            write_logon_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: D
        &FixMessage::NewOrderSingle(ref box_flds) => {
            write_new_order_single_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: E
        &FixMessage::NewOrderList(ref box_flds) => {
            write_new_order_list_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: F
        &FixMessage::OrderCancelRequest(ref box_flds) => {
            write_order_cancel_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: G
        &FixMessage::OrderCancelReplaceRequest(ref box_flds) => {
            write_order_cancel_replace_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: H
        &FixMessage::OrderStatusRequest(ref box_flds) => {
            write_order_status_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        _ => {
            // say what?
            Err(io::Error::new(io::ErrorKind::Other, "should never happen"))
        }
    }
}





fn write_heartbeat_fields(flds: &HeartbeatFields, output: &mut Write) -> Result<(), io::Error> {

    if flds.test_req_id.is_some() {
        let val = flds.test_req_id.as_ref().unwrap();

        write!(output, "112={}\u{01}", val )?; // FIELD_TESTREQID
    }
    Ok( () )
}


fn write_test_request_fields(flds: &TestRequestFields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.test_req_id;

        write!(output, "112={}\u{01}", val )?; // FIELD_TESTREQID
    }
    Ok( () )
}


fn write_resend_request_fields(flds: &ResendRequestFields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.begin_seq_no;

        write!(output, "7={}\u{01}", val )?; // FIELD_BEGINSEQNO
    }
    {
        let val = &flds.end_seq_no;

        write!(output, "16={}\u{01}", val )?; // FIELD_ENDSEQNO
    }
    Ok( () )
}


fn write_reject_fields(flds: &RejectFields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.ref_seq_num;

        write!(output, "45={}\u{01}", val )?; // FIELD_REFSEQNUM
    }
    if flds.ref_tag_id.is_some() {
        let val = flds.ref_tag_id.as_ref().unwrap();

        write!(output, "371={}\u{01}", val )?; // FIELD_REFTAGID
    }
    if flds.ref_msg_type.is_some() {
        let val = flds.ref_msg_type.as_ref().unwrap();

        write!(output, "372={}\u{01}", val )?; // FIELD_REFMSGTYPE
    }
    if flds.session_reject_reason.is_some() {
        let val = flds.session_reject_reason.as_ref().unwrap();

        write!(output, "373={}\u{01}", val )?; // FIELD_SESSIONREJECTREASON
    }
    if flds.text.is_some() {
        let val = flds.text.as_ref().unwrap();

        write!(output, "58={}\u{01}", val )?; // FIELD_TEXT
    }
    if flds.encoded_text_len.is_some() {
        let val = flds.encoded_text_len.as_ref().unwrap();

        write!(output, "354={}\u{01}", val )?; // FIELD_ENCODEDTEXTLEN
    }
    if flds.encoded_text.is_some() {
        let val = flds.encoded_text.as_ref().unwrap();

        write!(output, "355={}\u{01}", val )?; // FIELD_ENCODEDTEXT
    }
    Ok( () )
}


fn write_sequence_reset_fields(flds: &SequenceResetFields, output: &mut Write) -> Result<(), io::Error> {

    if flds.gap_fill_flag.is_some() {
        let val = flds.gap_fill_flag.as_ref().unwrap();

        write!(output, "123={}\u{01}", to_boolconv(val) )?; // FIELD_GAPFILLFLAG
    }
    {
        let val = &flds.new_seq_no;

        write!(output, "36={}\u{01}", val )?; // FIELD_NEWSEQNO
    }
    Ok( () )
}


fn write_logout_fields(flds: &LogoutFields, output: &mut Write) -> Result<(), io::Error> {

    if flds.text.is_some() {
        let val = flds.text.as_ref().unwrap();

        write!(output, "58={}\u{01}", val )?; // FIELD_TEXT
    }
    if flds.encoded_text_len.is_some() {
        let val = flds.encoded_text_len.as_ref().unwrap();

        write!(output, "354={}\u{01}", val )?; // FIELD_ENCODEDTEXTLEN
    }
    if flds.encoded_text.is_some() {
        let val = flds.encoded_text.as_ref().unwrap();

        write!(output, "355={}\u{01}", val )?; // FIELD_ENCODEDTEXT
    }
    Ok( () )
}


fn write_advertisement_fields(flds: &AdvertisementFields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.adv_id;

        write!(output, "2={}\u{01}", val )?; // FIELD_ADVID
    }
    {
        let val = &flds.adv_trans_type;

        write!(output, "5={}\u{01}", val )?; // FIELD_ADVTRANSTYPE
    }
    if flds.adv_ref_id.is_some() {
        let val = flds.adv_ref_id.as_ref().unwrap();

        write!(output, "3={}\u{01}", val )?; // FIELD_ADVREFID
    }
    {
        let val = &flds.symbol;

        write!(output, "55={}\u{01}", val )?; // FIELD_SYMBOL
    }
    if flds.symbol_sfx.is_some() {
        let val = flds.symbol_sfx.as_ref().unwrap();

        write!(output, "65={}\u{01}", val )?; // FIELD_SYMBOLSFX
    }
    if flds.security_id.is_some() {
        let val = flds.security_id.as_ref().unwrap();

        write!(output, "48={}\u{01}", val )?; // FIELD_SECURITYID
    }
    if flds.idsource.is_some() {
        let val = flds.idsource.as_ref().unwrap();

        write!(output, "22={}\u{01}", val )?; // FIELD_IDSOURCE
    }
    if flds.security_type.is_some() {
        let val = flds.security_type.as_ref().unwrap();

        write!(output, "167={}\u{01}", val )?; // FIELD_SECURITYTYPE
    }
    if flds.maturity_month_year.is_some() {
        let val = flds.maturity_month_year.as_ref().unwrap();

        write!(output, "200={}\u{01}", val )?; // FIELD_MATURITYMONTHYEAR
    }
    if flds.maturity_day.is_some() {
        let val = flds.maturity_day.as_ref().unwrap();

        write!(output, "205={}\u{01}", val )?; // FIELD_MATURITYDAY
    }
    if flds.put_or_call.is_some() {
        let val = flds.put_or_call.as_ref().unwrap();

        write!(output, "201={}\u{01}", val )?; // FIELD_PUTORCALL
    }
    if flds.strike_price.is_some() {
        let val = flds.strike_price.as_ref().unwrap();

        write!(output, "202={}\u{01}", val )?; // FIELD_STRIKEPRICE
    }
    if flds.opt_attribute.is_some() {
        let val = flds.opt_attribute.as_ref().unwrap();

        write!(output, "206={}\u{01}", val )?; // FIELD_OPTATTRIBUTE
    }
    if flds.contract_multiplier.is_some() {
        let val = flds.contract_multiplier.as_ref().unwrap();

        write!(output, "231={}\u{01}", val )?; // FIELD_CONTRACTMULTIPLIER
    }
    if flds.coupon_rate.is_some() {
        let val = flds.coupon_rate.as_ref().unwrap();

        write!(output, "223={}\u{01}", val )?; // FIELD_COUPONRATE
    }
    if flds.security_exchange.is_some() {
        let val = flds.security_exchange.as_ref().unwrap();

        write!(output, "207={}\u{01}", val )?; // FIELD_SECURITYEXCHANGE
    }
    if flds.issuer.is_some() {
        let val = flds.issuer.as_ref().unwrap();

        write!(output, "106={}\u{01}", val )?; // FIELD_ISSUER
    }
    if flds.encoded_issuer_len.is_some() {
        let val = flds.encoded_issuer_len.as_ref().unwrap();

        write!(output, "348={}\u{01}", val )?; // FIELD_ENCODEDISSUERLEN
    }
    if flds.encoded_issuer.is_some() {
        let val = flds.encoded_issuer.as_ref().unwrap();

        write!(output, "349={}\u{01}", val )?; // FIELD_ENCODEDISSUER
    }
    if flds.security_desc.is_some() {
        let val = flds.security_desc.as_ref().unwrap();

        write!(output, "107={}\u{01}", val )?; // FIELD_SECURITYDESC
    }
    if flds.encoded_security_desc_len.is_some() {
        let val = flds.encoded_security_desc_len.as_ref().unwrap();

        write!(output, "350={}\u{01}", val )?; // FIELD_ENCODEDSECURITYDESCLEN
    }
    if flds.encoded_security_desc.is_some() {
        let val = flds.encoded_security_desc.as_ref().unwrap();

        write!(output, "351={}\u{01}", val )?; // FIELD_ENCODEDSECURITYDESC
    }
    {
        let val = &flds.adv_side;

        write!(output, "4={}\u{01}", val )?; // FIELD_ADVSIDE
    }
    {
        let val = &flds.shares;

        write!(output, "53={}\u{01}", val )?; // FIELD_SHARES
    }
    if flds.price.is_some() {
        let val = flds.price.as_ref().unwrap();

        write!(output, "44={}\u{01}", val )?; // FIELD_PRICE
    }
    if flds.currency.is_some() {
        let val = flds.currency.as_ref().unwrap();

        write!(output, "15={}\u{01}", val )?; // FIELD_CURRENCY
    }
    if flds.trade_date.is_some() {
        let val = flds.trade_date.as_ref().unwrap();

        write!(output, "75={}\u{01}", val )?; // FIELD_TRADEDATE
    }
    if flds.transact_time.is_some() {
        let val = flds.transact_time.as_ref().unwrap();

        write!(output, "60={}\u{01}", val )?; // FIELD_TRANSACTTIME
    }
    if flds.text.is_some() {
        let val = flds.text.as_ref().unwrap();

        write!(output, "58={}\u{01}", val )?; // FIELD_TEXT
    }
    if flds.encoded_text_len.is_some() {
        let val = flds.encoded_text_len.as_ref().unwrap();

        write!(output, "354={}\u{01}", val )?; // FIELD_ENCODEDTEXTLEN
    }
    if flds.encoded_text.is_some() {
        let val = flds.encoded_text.as_ref().unwrap();

        write!(output, "355={}\u{01}", val )?; // FIELD_ENCODEDTEXT
    }
    if flds.urllink.is_some() {
        let val = flds.urllink.as_ref().unwrap();

        write!(output, "149={}\u{01}", val )?; // FIELD_URLLINK
    }
    if flds.last_mkt.is_some() {
        let val = flds.last_mkt.as_ref().unwrap();

        write!(output, "30={}\u{01}", val )?; // FIELD_LASTMKT
    }
    if flds.trading_session_id.is_some() {
        let val = flds.trading_session_id.as_ref().unwrap();

        write!(output, "336={}\u{01}", val )?; // FIELD_TRADINGSESSIONID
    }
    Ok( () )
}


fn write_execution_report_fields(flds: &ExecutionReportFields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.order_id;

        write!(output, "37={}\u{01}", val )?; // FIELD_ORDERID
    }
    if flds.secondary_order_id.is_some() {
        let val = flds.secondary_order_id.as_ref().unwrap();

        write!(output, "198={}\u{01}", val )?; // FIELD_SECONDARYORDERID
    }
    if flds.cl_ord_id.is_some() {
        let val = flds.cl_ord_id.as_ref().unwrap();

        write!(output, "11={}\u{01}", val )?; // FIELD_CLORDID
    }
    if flds.orig_cl_ord_id.is_some() {
        let val = flds.orig_cl_ord_id.as_ref().unwrap();

        write!(output, "41={}\u{01}", val )?; // FIELD_ORIGCLORDID
    }
    if flds.client_id.is_some() {
        let val = flds.client_id.as_ref().unwrap();

        write!(output, "109={}\u{01}", val )?; // FIELD_CLIENTID
    }
    if flds.exec_broker.is_some() {
        let val = flds.exec_broker.as_ref().unwrap();

        write!(output, "76={}\u{01}", val )?; // FIELD_EXECBROKER
    }
    if flds.no_contra_brokers.is_some() {
        let val = flds.no_contra_brokers.as_ref().unwrap();

        write_group_no_contra_brokers2_fields( val, output )?;
    }
    if flds.list_id.is_some() {
        let val = flds.list_id.as_ref().unwrap();

        write!(output, "66={}\u{01}", val )?; // FIELD_LISTID
    }
    {
        let val = &flds.exec_id;

        write!(output, "17={}\u{01}", val )?; // FIELD_EXECID
    }
    {
        let val = &flds.exec_trans_type;

        write!(output, "20={}\u{01}", val )?; // FIELD_EXECTRANSTYPE
    }
    if flds.exec_ref_id.is_some() {
        let val = flds.exec_ref_id.as_ref().unwrap();

        write!(output, "19={}\u{01}", val )?; // FIELD_EXECREFID
    }
    {
        let val = &flds.exec_type;

        write!(output, "150={}\u{01}", val )?; // FIELD_EXECTYPE
    }
    {
        let val = &flds.ord_status;

        write!(output, "39={}\u{01}", val )?; // FIELD_ORDSTATUS
    }
    if flds.ord_rej_reason.is_some() {
        let val = flds.ord_rej_reason.as_ref().unwrap();

        write!(output, "103={}\u{01}", val )?; // FIELD_ORDREJREASON
    }
    if flds.exec_restatement_reason.is_some() {
        let val = flds.exec_restatement_reason.as_ref().unwrap();

        write!(output, "378={}\u{01}", val )?; // FIELD_EXECRESTATEMENTREASON
    }
    if flds.account.is_some() {
        let val = flds.account.as_ref().unwrap();

        write!(output, "1={}\u{01}", val )?; // FIELD_ACCOUNT
    }
    if flds.settlmnt_typ.is_some() {
        let val = flds.settlmnt_typ.as_ref().unwrap();

        write!(output, "63={}\u{01}", val )?; // FIELD_SETTLMNTTYP
    }
    if flds.fut_sett_date.is_some() {
        let val = flds.fut_sett_date.as_ref().unwrap();

        write!(output, "64={}\u{01}", val )?; // FIELD_FUTSETTDATE
    }
    {
        let val = &flds.symbol;

        write!(output, "55={}\u{01}", val )?; // FIELD_SYMBOL
    }
    if flds.symbol_sfx.is_some() {
        let val = flds.symbol_sfx.as_ref().unwrap();

        write!(output, "65={}\u{01}", val )?; // FIELD_SYMBOLSFX
    }
    if flds.security_id.is_some() {
        let val = flds.security_id.as_ref().unwrap();

        write!(output, "48={}\u{01}", val )?; // FIELD_SECURITYID
    }
    if flds.idsource.is_some() {
        let val = flds.idsource.as_ref().unwrap();

        write!(output, "22={}\u{01}", val )?; // FIELD_IDSOURCE
    }
    if flds.security_type.is_some() {
        let val = flds.security_type.as_ref().unwrap();

        write!(output, "167={}\u{01}", val )?; // FIELD_SECURITYTYPE
    }
    if flds.maturity_month_year.is_some() {
        let val = flds.maturity_month_year.as_ref().unwrap();

        write!(output, "200={}\u{01}", val )?; // FIELD_MATURITYMONTHYEAR
    }
    if flds.maturity_day.is_some() {
        let val = flds.maturity_day.as_ref().unwrap();

        write!(output, "205={}\u{01}", val )?; // FIELD_MATURITYDAY
    }
    if flds.put_or_call.is_some() {
        let val = flds.put_or_call.as_ref().unwrap();

        write!(output, "201={}\u{01}", val )?; // FIELD_PUTORCALL
    }
    if flds.strike_price.is_some() {
        let val = flds.strike_price.as_ref().unwrap();

        write!(output, "202={}\u{01}", val )?; // FIELD_STRIKEPRICE
    }
    if flds.opt_attribute.is_some() {
        let val = flds.opt_attribute.as_ref().unwrap();

        write!(output, "206={}\u{01}", val )?; // FIELD_OPTATTRIBUTE
    }
    if flds.contract_multiplier.is_some() {
        let val = flds.contract_multiplier.as_ref().unwrap();

        write!(output, "231={}\u{01}", val )?; // FIELD_CONTRACTMULTIPLIER
    }
    if flds.coupon_rate.is_some() {
        let val = flds.coupon_rate.as_ref().unwrap();

        write!(output, "223={}\u{01}", val )?; // FIELD_COUPONRATE
    }
    if flds.security_exchange.is_some() {
        let val = flds.security_exchange.as_ref().unwrap();

        write!(output, "207={}\u{01}", val )?; // FIELD_SECURITYEXCHANGE
    }
    if flds.issuer.is_some() {
        let val = flds.issuer.as_ref().unwrap();

        write!(output, "106={}\u{01}", val )?; // FIELD_ISSUER
    }
    if flds.encoded_issuer_len.is_some() {
        let val = flds.encoded_issuer_len.as_ref().unwrap();

        write!(output, "348={}\u{01}", val )?; // FIELD_ENCODEDISSUERLEN
    }
    if flds.encoded_issuer.is_some() {
        let val = flds.encoded_issuer.as_ref().unwrap();

        write!(output, "349={}\u{01}", val )?; // FIELD_ENCODEDISSUER
    }
    if flds.security_desc.is_some() {
        let val = flds.security_desc.as_ref().unwrap();

        write!(output, "107={}\u{01}", val )?; // FIELD_SECURITYDESC
    }
    if flds.encoded_security_desc_len.is_some() {
        let val = flds.encoded_security_desc_len.as_ref().unwrap();

        write!(output, "350={}\u{01}", val )?; // FIELD_ENCODEDSECURITYDESCLEN
    }
    if flds.encoded_security_desc.is_some() {
        let val = flds.encoded_security_desc.as_ref().unwrap();

        write!(output, "351={}\u{01}", val )?; // FIELD_ENCODEDSECURITYDESC
    }
    {
        let val = &flds.side;

        write!(output, "54={}\u{01}", val )?; // FIELD_SIDE
    }
    if flds.order_qty.is_some() {
        let val = flds.order_qty.as_ref().unwrap();

        write!(output, "38={}\u{01}", val )?; // FIELD_ORDERQTY
    }
    if flds.cash_order_qty.is_some() {
        let val = flds.cash_order_qty.as_ref().unwrap();

        write!(output, "152={}\u{01}", val )?; // FIELD_CASHORDERQTY
    }
    if flds.ord_type.is_some() {
        let val = flds.ord_type.as_ref().unwrap();

        write!(output, "40={}\u{01}", val )?; // FIELD_ORDTYPE
    }
    if flds.price.is_some() {
        let val = flds.price.as_ref().unwrap();

        write!(output, "44={}\u{01}", val )?; // FIELD_PRICE
    }
    if flds.stop_px.is_some() {
        let val = flds.stop_px.as_ref().unwrap();

        write!(output, "99={}\u{01}", val )?; // FIELD_STOPPX
    }
    if flds.peg_difference.is_some() {
        let val = flds.peg_difference.as_ref().unwrap();

        write!(output, "211={}\u{01}", val )?; // FIELD_PEGDIFFERENCE
    }
    if flds.discretion_inst.is_some() {
        let val = flds.discretion_inst.as_ref().unwrap();

        write!(output, "388={}\u{01}", val )?; // FIELD_DISCRETIONINST
    }
    if flds.discretion_offset.is_some() {
        let val = flds.discretion_offset.as_ref().unwrap();

        write!(output, "389={}\u{01}", val )?; // FIELD_DISCRETIONOFFSET
    }
    if flds.currency.is_some() {
        let val = flds.currency.as_ref().unwrap();

        write!(output, "15={}\u{01}", val )?; // FIELD_CURRENCY
    }
    if flds.compliance_id.is_some() {
        let val = flds.compliance_id.as_ref().unwrap();

        write!(output, "376={}\u{01}", val )?; // FIELD_COMPLIANCEID
    }
    if flds.solicited_flag.is_some() {
        let val = flds.solicited_flag.as_ref().unwrap();

        write!(output, "377={}\u{01}", to_boolconv(val) )?; // FIELD_SOLICITEDFLAG
    }
    if flds.time_in_force.is_some() {
        let val = flds.time_in_force.as_ref().unwrap();

        write!(output, "59={}\u{01}", val )?; // FIELD_TIMEINFORCE
    }
    if flds.effective_time.is_some() {
        let val = flds.effective_time.as_ref().unwrap();

        write!(output, "168={}\u{01}", val )?; // FIELD_EFFECTIVETIME
    }
    if flds.expire_date.is_some() {
        let val = flds.expire_date.as_ref().unwrap();

        write!(output, "432={}\u{01}", val )?; // FIELD_EXPIREDATE
    }
    if flds.expire_time.is_some() {
        let val = flds.expire_time.as_ref().unwrap();

        write!(output, "126={}\u{01}", val )?; // FIELD_EXPIRETIME
    }
    if flds.exec_inst.is_some() {
        let val = flds.exec_inst.as_ref().unwrap();

        write!(output, "18={}\u{01}", val )?; // FIELD_EXECINST
    }
    if flds.rule80_a.is_some() {
        let val = flds.rule80_a.as_ref().unwrap();

        write!(output, "47={}\u{01}", val )?; // FIELD_RULE80A
    }
    if flds.last_shares.is_some() {
        let val = flds.last_shares.as_ref().unwrap();

        write!(output, "32={}\u{01}", val )?; // FIELD_LASTSHARES
    }
    if flds.last_px.is_some() {
        let val = flds.last_px.as_ref().unwrap();

        write!(output, "31={}\u{01}", val )?; // FIELD_LASTPX
    }
    if flds.last_spot_rate.is_some() {
        let val = flds.last_spot_rate.as_ref().unwrap();

        write!(output, "194={}\u{01}", val )?; // FIELD_LASTSPOTRATE
    }
    if flds.last_forward_points.is_some() {
        let val = flds.last_forward_points.as_ref().unwrap();

        write!(output, "195={}\u{01}", val )?; // FIELD_LASTFORWARDPOINTS
    }
    if flds.last_mkt.is_some() {
        let val = flds.last_mkt.as_ref().unwrap();

        write!(output, "30={}\u{01}", val )?; // FIELD_LASTMKT
    }
    if flds.trading_session_id.is_some() {
        let val = flds.trading_session_id.as_ref().unwrap();

        write!(output, "336={}\u{01}", val )?; // FIELD_TRADINGSESSIONID
    }
    if flds.last_capacity.is_some() {
        let val = flds.last_capacity.as_ref().unwrap();

        write!(output, "29={}\u{01}", val )?; // FIELD_LASTCAPACITY
    }
    {
        let val = &flds.leaves_qty;

        write!(output, "151={}\u{01}", val )?; // FIELD_LEAVESQTY
    }
    {
        let val = &flds.cum_qty;

        write!(output, "14={}\u{01}", val )?; // FIELD_CUMQTY
    }
    {
        let val = &flds.avg_px;

        write!(output, "6={}\u{01}", val )?; // FIELD_AVGPX
    }
    if flds.day_order_qty.is_some() {
        let val = flds.day_order_qty.as_ref().unwrap();

        write!(output, "424={}\u{01}", val )?; // FIELD_DAYORDERQTY
    }
    if flds.day_cum_qty.is_some() {
        let val = flds.day_cum_qty.as_ref().unwrap();

        write!(output, "425={}\u{01}", val )?; // FIELD_DAYCUMQTY
    }
    if flds.day_avg_px.is_some() {
        let val = flds.day_avg_px.as_ref().unwrap();

        write!(output, "426={}\u{01}", val )?; // FIELD_DAYAVGPX
    }
    if flds.gtbooking_inst.is_some() {
        let val = flds.gtbooking_inst.as_ref().unwrap();

        write!(output, "427={}\u{01}", val )?; // FIELD_GTBOOKINGINST
    }
    if flds.trade_date.is_some() {
        let val = flds.trade_date.as_ref().unwrap();

        write!(output, "75={}\u{01}", val )?; // FIELD_TRADEDATE
    }
    if flds.transact_time.is_some() {
        let val = flds.transact_time.as_ref().unwrap();

        write!(output, "60={}\u{01}", val )?; // FIELD_TRANSACTTIME
    }
    if flds.report_to_exch.is_some() {
        let val = flds.report_to_exch.as_ref().unwrap();

        write!(output, "113={}\u{01}", to_boolconv(val) )?; // FIELD_REPORTTOEXCH
    }
    if flds.commission.is_some() {
        let val = flds.commission.as_ref().unwrap();

        write!(output, "12={}\u{01}", val )?; // FIELD_COMMISSION
    }
    if flds.comm_type.is_some() {
        let val = flds.comm_type.as_ref().unwrap();

        write!(output, "13={}\u{01}", val )?; // FIELD_COMMTYPE
    }
    if flds.gross_trade_amt.is_some() {
        let val = flds.gross_trade_amt.as_ref().unwrap();

        write!(output, "381={}\u{01}", val )?; // FIELD_GROSSTRADEAMT
    }
    if flds.settl_curr_amt.is_some() {
        let val = flds.settl_curr_amt.as_ref().unwrap();

        write!(output, "119={}\u{01}", val )?; // FIELD_SETTLCURRAMT
    }
    if flds.settl_currency.is_some() {
        let val = flds.settl_currency.as_ref().unwrap();

        write!(output, "120={}\u{01}", val )?; // FIELD_SETTLCURRENCY
    }
    if flds.settl_curr_fx_rate.is_some() {
        let val = flds.settl_curr_fx_rate.as_ref().unwrap();

        write!(output, "155={}\u{01}", val )?; // FIELD_SETTLCURRFXRATE
    }
    if flds.settl_curr_fx_rate_calc.is_some() {
        let val = flds.settl_curr_fx_rate_calc.as_ref().unwrap();

        write!(output, "156={}\u{01}", val )?; // FIELD_SETTLCURRFXRATECALC
    }
    if flds.handl_inst.is_some() {
        let val = flds.handl_inst.as_ref().unwrap();

        write!(output, "21={}\u{01}", val )?; // FIELD_HANDLINST
    }
    if flds.min_qty.is_some() {
        let val = flds.min_qty.as_ref().unwrap();

        write!(output, "110={}\u{01}", val )?; // FIELD_MINQTY
    }
    if flds.max_floor.is_some() {
        let val = flds.max_floor.as_ref().unwrap();

        write!(output, "111={}\u{01}", val )?; // FIELD_MAXFLOOR
    }
    if flds.open_close.is_some() {
        let val = flds.open_close.as_ref().unwrap();

        write!(output, "77={}\u{01}", val )?; // FIELD_OPENCLOSE
    }
    if flds.max_show.is_some() {
        let val = flds.max_show.as_ref().unwrap();

        write!(output, "210={}\u{01}", val )?; // FIELD_MAXSHOW
    }
    if flds.text.is_some() {
        let val = flds.text.as_ref().unwrap();

        write!(output, "58={}\u{01}", val )?; // FIELD_TEXT
    }
    if flds.encoded_text_len.is_some() {
        let val = flds.encoded_text_len.as_ref().unwrap();

        write!(output, "354={}\u{01}", val )?; // FIELD_ENCODEDTEXTLEN
    }
    if flds.encoded_text.is_some() {
        let val = flds.encoded_text.as_ref().unwrap();

        write!(output, "355={}\u{01}", val )?; // FIELD_ENCODEDTEXT
    }
    if flds.fut_sett_date2.is_some() {
        let val = flds.fut_sett_date2.as_ref().unwrap();

        write!(output, "193={}\u{01}", val )?; // FIELD_FUTSETTDATE2
    }
    if flds.order_qty2.is_some() {
        let val = flds.order_qty2.as_ref().unwrap();

        write!(output, "192={}\u{01}", val )?; // FIELD_ORDERQTY2
    }
    if flds.clearing_firm.is_some() {
        let val = flds.clearing_firm.as_ref().unwrap();

        write!(output, "439={}\u{01}", val )?; // FIELD_CLEARINGFIRM
    }
    if flds.clearing_account.is_some() {
        let val = flds.clearing_account.as_ref().unwrap();

        write!(output, "440={}\u{01}", val )?; // FIELD_CLEARINGACCOUNT
    }
    if flds.multi_leg_reporting_type.is_some() {
        let val = flds.multi_leg_reporting_type.as_ref().unwrap();

        write!(output, "442={}\u{01}", val )?; // FIELD_MULTILEGREPORTINGTYPE
    }
    Ok( () )
}


fn write_order_cancel_reject_fields(flds: &OrderCancelRejectFields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.order_id;

        write!(output, "37={}\u{01}", val )?; // FIELD_ORDERID
    }
    if flds.secondary_order_id.is_some() {
        let val = flds.secondary_order_id.as_ref().unwrap();

        write!(output, "198={}\u{01}", val )?; // FIELD_SECONDARYORDERID
    }
    {
        let val = &flds.cl_ord_id;

        write!(output, "11={}\u{01}", val )?; // FIELD_CLORDID
    }
    {
        let val = &flds.orig_cl_ord_id;

        write!(output, "41={}\u{01}", val )?; // FIELD_ORIGCLORDID
    }
    {
        let val = &flds.ord_status;

        write!(output, "39={}\u{01}", val )?; // FIELD_ORDSTATUS
    }
    if flds.client_id.is_some() {
        let val = flds.client_id.as_ref().unwrap();

        write!(output, "109={}\u{01}", val )?; // FIELD_CLIENTID
    }
    if flds.exec_broker.is_some() {
        let val = flds.exec_broker.as_ref().unwrap();

        write!(output, "76={}\u{01}", val )?; // FIELD_EXECBROKER
    }
    if flds.list_id.is_some() {
        let val = flds.list_id.as_ref().unwrap();

        write!(output, "66={}\u{01}", val )?; // FIELD_LISTID
    }
    if flds.account.is_some() {
        let val = flds.account.as_ref().unwrap();

        write!(output, "1={}\u{01}", val )?; // FIELD_ACCOUNT
    }
    if flds.transact_time.is_some() {
        let val = flds.transact_time.as_ref().unwrap();

        write!(output, "60={}\u{01}", val )?; // FIELD_TRANSACTTIME
    }
    {
        let val = &flds.cxl_rej_response_to;

        write!(output, "434={}\u{01}", val )?; // FIELD_CXLREJRESPONSETO
    }
    if flds.cxl_rej_reason.is_some() {
        let val = flds.cxl_rej_reason.as_ref().unwrap();

        write!(output, "102={}\u{01}", val )?; // FIELD_CXLREJREASON
    }
    if flds.text.is_some() {
        let val = flds.text.as_ref().unwrap();

        write!(output, "58={}\u{01}", val )?; // FIELD_TEXT
    }
    if flds.encoded_text_len.is_some() {
        let val = flds.encoded_text_len.as_ref().unwrap();

        write!(output, "354={}\u{01}", val )?; // FIELD_ENCODEDTEXTLEN
    }
    if flds.encoded_text.is_some() {
        let val = flds.encoded_text.as_ref().unwrap();

        write!(output, "355={}\u{01}", val )?; // FIELD_ENCODEDTEXT
    }
    Ok( () )
}


fn write_logon_fields(flds: &LogonFields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.encrypt_method;

        write!(output, "98={}\u{01}", val )?; // FIELD_ENCRYPTMETHOD
    }
    {
        let val = &flds.heart_bt_int;

        write!(output, "108={}\u{01}", val )?; // FIELD_HEARTBTINT
    }
    if flds.raw_data_length.is_some() {
        let val = flds.raw_data_length.as_ref().unwrap();

        write!(output, "95={}\u{01}", val )?; // FIELD_RAWDATALENGTH
    }
    if flds.raw_data.is_some() {
        let val = flds.raw_data.as_ref().unwrap();

        write!(output, "96={}\u{01}", val )?; // FIELD_RAWDATA
    }
    if flds.reset_seq_num_flag.is_some() {
        let val = flds.reset_seq_num_flag.as_ref().unwrap();

        write!(output, "141={}\u{01}", to_boolconv(val) )?; // FIELD_RESETSEQNUMFLAG
    }
    if flds.max_message_size.is_some() {
        let val = flds.max_message_size.as_ref().unwrap();

        write!(output, "383={}\u{01}", val )?; // FIELD_MAXMESSAGESIZE
    }
    if flds.no_msg_types.is_some() {
        let val = flds.no_msg_types.as_ref().unwrap();

        write_group_no_msg_types3_fields( val, output )?;
    }
    Ok( () )
}


fn write_new_order_single_fields(flds: &NewOrderSingleFields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.cl_ord_id;

        write!(output, "11={}\u{01}", val )?; // FIELD_CLORDID
    }
    if flds.client_id.is_some() {
        let val = flds.client_id.as_ref().unwrap();

        write!(output, "109={}\u{01}", val )?; // FIELD_CLIENTID
    }
    if flds.exec_broker.is_some() {
        let val = flds.exec_broker.as_ref().unwrap();

        write!(output, "76={}\u{01}", val )?; // FIELD_EXECBROKER
    }
    if flds.account.is_some() {
        let val = flds.account.as_ref().unwrap();

        write!(output, "1={}\u{01}", val )?; // FIELD_ACCOUNT
    }
    if flds.no_allocs.is_some() {
        let val = flds.no_allocs.as_ref().unwrap();

        write_group_no_allocs1_fields( val, output )?;
    }
    if flds.settlmnt_typ.is_some() {
        let val = flds.settlmnt_typ.as_ref().unwrap();

        write!(output, "63={}\u{01}", val )?; // FIELD_SETTLMNTTYP
    }
    if flds.fut_sett_date.is_some() {
        let val = flds.fut_sett_date.as_ref().unwrap();

        write!(output, "64={}\u{01}", val )?; // FIELD_FUTSETTDATE
    }
    {
        let val = &flds.handl_inst;

        write!(output, "21={}\u{01}", val )?; // FIELD_HANDLINST
    }
    if flds.exec_inst.is_some() {
        let val = flds.exec_inst.as_ref().unwrap();

        write!(output, "18={}\u{01}", val )?; // FIELD_EXECINST
    }
    if flds.min_qty.is_some() {
        let val = flds.min_qty.as_ref().unwrap();

        write!(output, "110={}\u{01}", val )?; // FIELD_MINQTY
    }
    if flds.max_floor.is_some() {
        let val = flds.max_floor.as_ref().unwrap();

        write!(output, "111={}\u{01}", val )?; // FIELD_MAXFLOOR
    }
    if flds.ex_destination.is_some() {
        let val = flds.ex_destination.as_ref().unwrap();

        write!(output, "100={}\u{01}", val )?; // FIELD_EXDESTINATION
    }
    if flds.no_trading_sessions.is_some() {
        let val = flds.no_trading_sessions.as_ref().unwrap();

        write_group_no_trading_sessions5_fields( val, output )?;
    }
    if flds.process_code.is_some() {
        let val = flds.process_code.as_ref().unwrap();

        write!(output, "81={}\u{01}", val )?; // FIELD_PROCESSCODE
    }
    {
        let val = &flds.symbol;

        write!(output, "55={}\u{01}", val )?; // FIELD_SYMBOL
    }
    if flds.symbol_sfx.is_some() {
        let val = flds.symbol_sfx.as_ref().unwrap();

        write!(output, "65={}\u{01}", val )?; // FIELD_SYMBOLSFX
    }
    if flds.security_id.is_some() {
        let val = flds.security_id.as_ref().unwrap();

        write!(output, "48={}\u{01}", val )?; // FIELD_SECURITYID
    }
    if flds.idsource.is_some() {
        let val = flds.idsource.as_ref().unwrap();

        write!(output, "22={}\u{01}", val )?; // FIELD_IDSOURCE
    }
    if flds.security_type.is_some() {
        let val = flds.security_type.as_ref().unwrap();

        write!(output, "167={}\u{01}", val )?; // FIELD_SECURITYTYPE
    }
    if flds.maturity_month_year.is_some() {
        let val = flds.maturity_month_year.as_ref().unwrap();

        write!(output, "200={}\u{01}", val )?; // FIELD_MATURITYMONTHYEAR
    }
    if flds.maturity_day.is_some() {
        let val = flds.maturity_day.as_ref().unwrap();

        write!(output, "205={}\u{01}", val )?; // FIELD_MATURITYDAY
    }
    if flds.put_or_call.is_some() {
        let val = flds.put_or_call.as_ref().unwrap();

        write!(output, "201={}\u{01}", val )?; // FIELD_PUTORCALL
    }
    if flds.strike_price.is_some() {
        let val = flds.strike_price.as_ref().unwrap();

        write!(output, "202={}\u{01}", val )?; // FIELD_STRIKEPRICE
    }
    if flds.opt_attribute.is_some() {
        let val = flds.opt_attribute.as_ref().unwrap();

        write!(output, "206={}\u{01}", val )?; // FIELD_OPTATTRIBUTE
    }
    if flds.contract_multiplier.is_some() {
        let val = flds.contract_multiplier.as_ref().unwrap();

        write!(output, "231={}\u{01}", val )?; // FIELD_CONTRACTMULTIPLIER
    }
    if flds.coupon_rate.is_some() {
        let val = flds.coupon_rate.as_ref().unwrap();

        write!(output, "223={}\u{01}", val )?; // FIELD_COUPONRATE
    }
    if flds.security_exchange.is_some() {
        let val = flds.security_exchange.as_ref().unwrap();

        write!(output, "207={}\u{01}", val )?; // FIELD_SECURITYEXCHANGE
    }
    if flds.issuer.is_some() {
        let val = flds.issuer.as_ref().unwrap();

        write!(output, "106={}\u{01}", val )?; // FIELD_ISSUER
    }
    if flds.encoded_issuer_len.is_some() {
        let val = flds.encoded_issuer_len.as_ref().unwrap();

        write!(output, "348={}\u{01}", val )?; // FIELD_ENCODEDISSUERLEN
    }
    if flds.encoded_issuer.is_some() {
        let val = flds.encoded_issuer.as_ref().unwrap();

        write!(output, "349={}\u{01}", val )?; // FIELD_ENCODEDISSUER
    }
    if flds.security_desc.is_some() {
        let val = flds.security_desc.as_ref().unwrap();

        write!(output, "107={}\u{01}", val )?; // FIELD_SECURITYDESC
    }
    if flds.encoded_security_desc_len.is_some() {
        let val = flds.encoded_security_desc_len.as_ref().unwrap();

        write!(output, "350={}\u{01}", val )?; // FIELD_ENCODEDSECURITYDESCLEN
    }
    if flds.encoded_security_desc.is_some() {
        let val = flds.encoded_security_desc.as_ref().unwrap();

        write!(output, "351={}\u{01}", val )?; // FIELD_ENCODEDSECURITYDESC
    }
    if flds.prev_close_px.is_some() {
        let val = flds.prev_close_px.as_ref().unwrap();

        write!(output, "140={}\u{01}", val )?; // FIELD_PREVCLOSEPX
    }
    {
        let val = &flds.side;

        write!(output, "54={}\u{01}", val )?; // FIELD_SIDE
    }
    if flds.locate_reqd.is_some() {
        let val = flds.locate_reqd.as_ref().unwrap();

        write!(output, "114={}\u{01}", to_boolconv(val) )?; // FIELD_LOCATEREQD
    }
    {
        let val = &flds.transact_time;

        write!(output, "60={}\u{01}", val )?; // FIELD_TRANSACTTIME
    }
    if flds.order_qty.is_some() {
        let val = flds.order_qty.as_ref().unwrap();

        write!(output, "38={}\u{01}", val )?; // FIELD_ORDERQTY
    }
    if flds.cash_order_qty.is_some() {
        let val = flds.cash_order_qty.as_ref().unwrap();

        write!(output, "152={}\u{01}", val )?; // FIELD_CASHORDERQTY
    }
    {
        let val = &flds.ord_type;

        write!(output, "40={}\u{01}", val )?; // FIELD_ORDTYPE
    }
    if flds.price.is_some() {
        let val = flds.price.as_ref().unwrap();

        write!(output, "44={}\u{01}", val )?; // FIELD_PRICE
    }
    if flds.stop_px.is_some() {
        let val = flds.stop_px.as_ref().unwrap();

        write!(output, "99={}\u{01}", val )?; // FIELD_STOPPX
    }
    if flds.currency.is_some() {
        let val = flds.currency.as_ref().unwrap();

        write!(output, "15={}\u{01}", val )?; // FIELD_CURRENCY
    }
    if flds.compliance_id.is_some() {
        let val = flds.compliance_id.as_ref().unwrap();

        write!(output, "376={}\u{01}", val )?; // FIELD_COMPLIANCEID
    }
    if flds.solicited_flag.is_some() {
        let val = flds.solicited_flag.as_ref().unwrap();

        write!(output, "377={}\u{01}", to_boolconv(val) )?; // FIELD_SOLICITEDFLAG
    }
    if flds.ioiid.is_some() {
        let val = flds.ioiid.as_ref().unwrap();

        write!(output, "23={}\u{01}", val )?; // FIELD_IOIID
    }
    if flds.quote_id.is_some() {
        let val = flds.quote_id.as_ref().unwrap();

        write!(output, "117={}\u{01}", val )?; // FIELD_QUOTEID
    }
    if flds.time_in_force.is_some() {
        let val = flds.time_in_force.as_ref().unwrap();

        write!(output, "59={}\u{01}", val )?; // FIELD_TIMEINFORCE
    }
    if flds.effective_time.is_some() {
        let val = flds.effective_time.as_ref().unwrap();

        write!(output, "168={}\u{01}", val )?; // FIELD_EFFECTIVETIME
    }
    if flds.expire_date.is_some() {
        let val = flds.expire_date.as_ref().unwrap();

        write!(output, "432={}\u{01}", val )?; // FIELD_EXPIREDATE
    }
    if flds.expire_time.is_some() {
        let val = flds.expire_time.as_ref().unwrap();

        write!(output, "126={}\u{01}", val )?; // FIELD_EXPIRETIME
    }
    if flds.gtbooking_inst.is_some() {
        let val = flds.gtbooking_inst.as_ref().unwrap();

        write!(output, "427={}\u{01}", val )?; // FIELD_GTBOOKINGINST
    }
    if flds.commission.is_some() {
        let val = flds.commission.as_ref().unwrap();

        write!(output, "12={}\u{01}", val )?; // FIELD_COMMISSION
    }
    if flds.comm_type.is_some() {
        let val = flds.comm_type.as_ref().unwrap();

        write!(output, "13={}\u{01}", val )?; // FIELD_COMMTYPE
    }
    if flds.rule80_a.is_some() {
        let val = flds.rule80_a.as_ref().unwrap();

        write!(output, "47={}\u{01}", val )?; // FIELD_RULE80A
    }
    if flds.forex_req.is_some() {
        let val = flds.forex_req.as_ref().unwrap();

        write!(output, "121={}\u{01}", to_boolconv(val) )?; // FIELD_FOREXREQ
    }
    if flds.settl_currency.is_some() {
        let val = flds.settl_currency.as_ref().unwrap();

        write!(output, "120={}\u{01}", val )?; // FIELD_SETTLCURRENCY
    }
    if flds.text.is_some() {
        let val = flds.text.as_ref().unwrap();

        write!(output, "58={}\u{01}", val )?; // FIELD_TEXT
    }
    if flds.encoded_text_len.is_some() {
        let val = flds.encoded_text_len.as_ref().unwrap();

        write!(output, "354={}\u{01}", val )?; // FIELD_ENCODEDTEXTLEN
    }
    if flds.encoded_text.is_some() {
        let val = flds.encoded_text.as_ref().unwrap();

        write!(output, "355={}\u{01}", val )?; // FIELD_ENCODEDTEXT
    }
    if flds.fut_sett_date2.is_some() {
        let val = flds.fut_sett_date2.as_ref().unwrap();

        write!(output, "193={}\u{01}", val )?; // FIELD_FUTSETTDATE2
    }
    if flds.order_qty2.is_some() {
        let val = flds.order_qty2.as_ref().unwrap();

        write!(output, "192={}\u{01}", val )?; // FIELD_ORDERQTY2
    }
    if flds.open_close.is_some() {
        let val = flds.open_close.as_ref().unwrap();

        write!(output, "77={}\u{01}", val )?; // FIELD_OPENCLOSE
    }
    if flds.covered_or_uncovered.is_some() {
        let val = flds.covered_or_uncovered.as_ref().unwrap();

        write!(output, "203={}\u{01}", val )?; // FIELD_COVEREDORUNCOVERED
    }
    if flds.customer_or_firm.is_some() {
        let val = flds.customer_or_firm.as_ref().unwrap();

        write!(output, "204={}\u{01}", val )?; // FIELD_CUSTOMERORFIRM
    }
    if flds.max_show.is_some() {
        let val = flds.max_show.as_ref().unwrap();

        write!(output, "210={}\u{01}", val )?; // FIELD_MAXSHOW
    }
    if flds.peg_difference.is_some() {
        let val = flds.peg_difference.as_ref().unwrap();

        write!(output, "211={}\u{01}", val )?; // FIELD_PEGDIFFERENCE
    }
    if flds.discretion_inst.is_some() {
        let val = flds.discretion_inst.as_ref().unwrap();

        write!(output, "388={}\u{01}", val )?; // FIELD_DISCRETIONINST
    }
    if flds.discretion_offset.is_some() {
        let val = flds.discretion_offset.as_ref().unwrap();

        write!(output, "389={}\u{01}", val )?; // FIELD_DISCRETIONOFFSET
    }
    if flds.clearing_firm.is_some() {
        let val = flds.clearing_firm.as_ref().unwrap();

        write!(output, "439={}\u{01}", val )?; // FIELD_CLEARINGFIRM
    }
    if flds.clearing_account.is_some() {
        let val = flds.clearing_account.as_ref().unwrap();

        write!(output, "440={}\u{01}", val )?; // FIELD_CLEARINGACCOUNT
    }
    Ok( () )
}


fn write_new_order_list_fields(flds: &NewOrderListFields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.list_id;

        write!(output, "66={}\u{01}", val )?; // FIELD_LISTID
    }
    if flds.bid_id.is_some() {
        let val = flds.bid_id.as_ref().unwrap();

        write!(output, "390={}\u{01}", val )?; // FIELD_BIDID
    }
    if flds.client_bid_id.is_some() {
        let val = flds.client_bid_id.as_ref().unwrap();

        write!(output, "391={}\u{01}", val )?; // FIELD_CLIENTBIDID
    }
    if flds.prog_rpt_reqs.is_some() {
        let val = flds.prog_rpt_reqs.as_ref().unwrap();

        write!(output, "414={}\u{01}", val )?; // FIELD_PROGRPTREQS
    }
    {
        let val = &flds.bid_type;

        write!(output, "394={}\u{01}", val )?; // FIELD_BIDTYPE
    }
    if flds.prog_period_interval.is_some() {
        let val = flds.prog_period_interval.as_ref().unwrap();

        write!(output, "415={}\u{01}", val )?; // FIELD_PROGPERIODINTERVAL
    }
    if flds.list_exec_inst_type.is_some() {
        let val = flds.list_exec_inst_type.as_ref().unwrap();

        write!(output, "433={}\u{01}", val )?; // FIELD_LISTEXECINSTTYPE
    }
    if flds.list_exec_inst.is_some() {
        let val = flds.list_exec_inst.as_ref().unwrap();

        write!(output, "69={}\u{01}", val )?; // FIELD_LISTEXECINST
    }
    if flds.encoded_list_exec_inst_len.is_some() {
        let val = flds.encoded_list_exec_inst_len.as_ref().unwrap();

        write!(output, "352={}\u{01}", val )?; // FIELD_ENCODEDLISTEXECINSTLEN
    }
    if flds.encoded_list_exec_inst.is_some() {
        let val = flds.encoded_list_exec_inst.as_ref().unwrap();

        write!(output, "353={}\u{01}", val )?; // FIELD_ENCODEDLISTEXECINST
    }
    {
        let val = &flds.tot_no_orders;

        write!(output, "68={}\u{01}", val )?; // FIELD_TOTNOORDERS
    }
    {
        let val = &flds.no_orders;

        write_group_no_orders4_fields( val, output )?;
    }
    Ok( () )
}


fn write_order_cancel_request_fields(flds: &OrderCancelRequestFields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.orig_cl_ord_id;

        write!(output, "41={}\u{01}", val )?; // FIELD_ORIGCLORDID
    }
    if flds.order_id.is_some() {
        let val = flds.order_id.as_ref().unwrap();

        write!(output, "37={}\u{01}", val )?; // FIELD_ORDERID
    }
    {
        let val = &flds.cl_ord_id;

        write!(output, "11={}\u{01}", val )?; // FIELD_CLORDID
    }
    if flds.list_id.is_some() {
        let val = flds.list_id.as_ref().unwrap();

        write!(output, "66={}\u{01}", val )?; // FIELD_LISTID
    }
    if flds.account.is_some() {
        let val = flds.account.as_ref().unwrap();

        write!(output, "1={}\u{01}", val )?; // FIELD_ACCOUNT
    }
    if flds.client_id.is_some() {
        let val = flds.client_id.as_ref().unwrap();

        write!(output, "109={}\u{01}", val )?; // FIELD_CLIENTID
    }
    if flds.exec_broker.is_some() {
        let val = flds.exec_broker.as_ref().unwrap();

        write!(output, "76={}\u{01}", val )?; // FIELD_EXECBROKER
    }
    {
        let val = &flds.symbol;

        write!(output, "55={}\u{01}", val )?; // FIELD_SYMBOL
    }
    if flds.symbol_sfx.is_some() {
        let val = flds.symbol_sfx.as_ref().unwrap();

        write!(output, "65={}\u{01}", val )?; // FIELD_SYMBOLSFX
    }
    if flds.security_id.is_some() {
        let val = flds.security_id.as_ref().unwrap();

        write!(output, "48={}\u{01}", val )?; // FIELD_SECURITYID
    }
    if flds.idsource.is_some() {
        let val = flds.idsource.as_ref().unwrap();

        write!(output, "22={}\u{01}", val )?; // FIELD_IDSOURCE
    }
    if flds.security_type.is_some() {
        let val = flds.security_type.as_ref().unwrap();

        write!(output, "167={}\u{01}", val )?; // FIELD_SECURITYTYPE
    }
    if flds.maturity_month_year.is_some() {
        let val = flds.maturity_month_year.as_ref().unwrap();

        write!(output, "200={}\u{01}", val )?; // FIELD_MATURITYMONTHYEAR
    }
    if flds.maturity_day.is_some() {
        let val = flds.maturity_day.as_ref().unwrap();

        write!(output, "205={}\u{01}", val )?; // FIELD_MATURITYDAY
    }
    if flds.put_or_call.is_some() {
        let val = flds.put_or_call.as_ref().unwrap();

        write!(output, "201={}\u{01}", val )?; // FIELD_PUTORCALL
    }
    if flds.strike_price.is_some() {
        let val = flds.strike_price.as_ref().unwrap();

        write!(output, "202={}\u{01}", val )?; // FIELD_STRIKEPRICE
    }
    if flds.opt_attribute.is_some() {
        let val = flds.opt_attribute.as_ref().unwrap();

        write!(output, "206={}\u{01}", val )?; // FIELD_OPTATTRIBUTE
    }
    if flds.contract_multiplier.is_some() {
        let val = flds.contract_multiplier.as_ref().unwrap();

        write!(output, "231={}\u{01}", val )?; // FIELD_CONTRACTMULTIPLIER
    }
    if flds.coupon_rate.is_some() {
        let val = flds.coupon_rate.as_ref().unwrap();

        write!(output, "223={}\u{01}", val )?; // FIELD_COUPONRATE
    }
    if flds.security_exchange.is_some() {
        let val = flds.security_exchange.as_ref().unwrap();

        write!(output, "207={}\u{01}", val )?; // FIELD_SECURITYEXCHANGE
    }
    if flds.issuer.is_some() {
        let val = flds.issuer.as_ref().unwrap();

        write!(output, "106={}\u{01}", val )?; // FIELD_ISSUER
    }
    if flds.encoded_issuer_len.is_some() {
        let val = flds.encoded_issuer_len.as_ref().unwrap();

        write!(output, "348={}\u{01}", val )?; // FIELD_ENCODEDISSUERLEN
    }
    if flds.encoded_issuer.is_some() {
        let val = flds.encoded_issuer.as_ref().unwrap();

        write!(output, "349={}\u{01}", val )?; // FIELD_ENCODEDISSUER
    }
    if flds.security_desc.is_some() {
        let val = flds.security_desc.as_ref().unwrap();

        write!(output, "107={}\u{01}", val )?; // FIELD_SECURITYDESC
    }
    if flds.encoded_security_desc_len.is_some() {
        let val = flds.encoded_security_desc_len.as_ref().unwrap();

        write!(output, "350={}\u{01}", val )?; // FIELD_ENCODEDSECURITYDESCLEN
    }
    if flds.encoded_security_desc.is_some() {
        let val = flds.encoded_security_desc.as_ref().unwrap();

        write!(output, "351={}\u{01}", val )?; // FIELD_ENCODEDSECURITYDESC
    }
    {
        let val = &flds.side;

        write!(output, "54={}\u{01}", val )?; // FIELD_SIDE
    }
    {
        let val = &flds.transact_time;

        write!(output, "60={}\u{01}", val )?; // FIELD_TRANSACTTIME
    }
    if flds.order_qty.is_some() {
        let val = flds.order_qty.as_ref().unwrap();

        write!(output, "38={}\u{01}", val )?; // FIELD_ORDERQTY
    }
    if flds.cash_order_qty.is_some() {
        let val = flds.cash_order_qty.as_ref().unwrap();

        write!(output, "152={}\u{01}", val )?; // FIELD_CASHORDERQTY
    }
    if flds.compliance_id.is_some() {
        let val = flds.compliance_id.as_ref().unwrap();

        write!(output, "376={}\u{01}", val )?; // FIELD_COMPLIANCEID
    }
    if flds.solicited_flag.is_some() {
        let val = flds.solicited_flag.as_ref().unwrap();

        write!(output, "377={}\u{01}", to_boolconv(val) )?; // FIELD_SOLICITEDFLAG
    }
    if flds.text.is_some() {
        let val = flds.text.as_ref().unwrap();

        write!(output, "58={}\u{01}", val )?; // FIELD_TEXT
    }
    if flds.encoded_text_len.is_some() {
        let val = flds.encoded_text_len.as_ref().unwrap();

        write!(output, "354={}\u{01}", val )?; // FIELD_ENCODEDTEXTLEN
    }
    if flds.encoded_text.is_some() {
        let val = flds.encoded_text.as_ref().unwrap();

        write!(output, "355={}\u{01}", val )?; // FIELD_ENCODEDTEXT
    }
    Ok( () )
}


fn write_order_cancel_replace_request_fields(flds: &OrderCancelReplaceRequestFields, output: &mut Write) -> Result<(), io::Error> {

    if flds.order_id.is_some() {
        let val = flds.order_id.as_ref().unwrap();

        write!(output, "37={}\u{01}", val )?; // FIELD_ORDERID
    }
    if flds.client_id.is_some() {
        let val = flds.client_id.as_ref().unwrap();

        write!(output, "109={}\u{01}", val )?; // FIELD_CLIENTID
    }
    if flds.exec_broker.is_some() {
        let val = flds.exec_broker.as_ref().unwrap();

        write!(output, "76={}\u{01}", val )?; // FIELD_EXECBROKER
    }
    {
        let val = &flds.orig_cl_ord_id;

        write!(output, "41={}\u{01}", val )?; // FIELD_ORIGCLORDID
    }
    {
        let val = &flds.cl_ord_id;

        write!(output, "11={}\u{01}", val )?; // FIELD_CLORDID
    }
    if flds.list_id.is_some() {
        let val = flds.list_id.as_ref().unwrap();

        write!(output, "66={}\u{01}", val )?; // FIELD_LISTID
    }
    if flds.account.is_some() {
        let val = flds.account.as_ref().unwrap();

        write!(output, "1={}\u{01}", val )?; // FIELD_ACCOUNT
    }
    if flds.no_allocs.is_some() {
        let val = flds.no_allocs.as_ref().unwrap();

        write_group_no_allocs1_fields( val, output )?;
    }
    if flds.settlmnt_typ.is_some() {
        let val = flds.settlmnt_typ.as_ref().unwrap();

        write!(output, "63={}\u{01}", val )?; // FIELD_SETTLMNTTYP
    }
    if flds.fut_sett_date.is_some() {
        let val = flds.fut_sett_date.as_ref().unwrap();

        write!(output, "64={}\u{01}", val )?; // FIELD_FUTSETTDATE
    }
    {
        let val = &flds.handl_inst;

        write!(output, "21={}\u{01}", val )?; // FIELD_HANDLINST
    }
    if flds.exec_inst.is_some() {
        let val = flds.exec_inst.as_ref().unwrap();

        write!(output, "18={}\u{01}", val )?; // FIELD_EXECINST
    }
    if flds.min_qty.is_some() {
        let val = flds.min_qty.as_ref().unwrap();

        write!(output, "110={}\u{01}", val )?; // FIELD_MINQTY
    }
    if flds.max_floor.is_some() {
        let val = flds.max_floor.as_ref().unwrap();

        write!(output, "111={}\u{01}", val )?; // FIELD_MAXFLOOR
    }
    if flds.ex_destination.is_some() {
        let val = flds.ex_destination.as_ref().unwrap();

        write!(output, "100={}\u{01}", val )?; // FIELD_EXDESTINATION
    }
    if flds.no_trading_sessions.is_some() {
        let val = flds.no_trading_sessions.as_ref().unwrap();

        write_group_no_trading_sessions5_fields( val, output )?;
    }
    {
        let val = &flds.symbol;

        write!(output, "55={}\u{01}", val )?; // FIELD_SYMBOL
    }
    if flds.symbol_sfx.is_some() {
        let val = flds.symbol_sfx.as_ref().unwrap();

        write!(output, "65={}\u{01}", val )?; // FIELD_SYMBOLSFX
    }
    if flds.security_id.is_some() {
        let val = flds.security_id.as_ref().unwrap();

        write!(output, "48={}\u{01}", val )?; // FIELD_SECURITYID
    }
    if flds.idsource.is_some() {
        let val = flds.idsource.as_ref().unwrap();

        write!(output, "22={}\u{01}", val )?; // FIELD_IDSOURCE
    }
    if flds.security_type.is_some() {
        let val = flds.security_type.as_ref().unwrap();

        write!(output, "167={}\u{01}", val )?; // FIELD_SECURITYTYPE
    }
    if flds.maturity_month_year.is_some() {
        let val = flds.maturity_month_year.as_ref().unwrap();

        write!(output, "200={}\u{01}", val )?; // FIELD_MATURITYMONTHYEAR
    }
    if flds.maturity_day.is_some() {
        let val = flds.maturity_day.as_ref().unwrap();

        write!(output, "205={}\u{01}", val )?; // FIELD_MATURITYDAY
    }
    if flds.put_or_call.is_some() {
        let val = flds.put_or_call.as_ref().unwrap();

        write!(output, "201={}\u{01}", val )?; // FIELD_PUTORCALL
    }
    if flds.strike_price.is_some() {
        let val = flds.strike_price.as_ref().unwrap();

        write!(output, "202={}\u{01}", val )?; // FIELD_STRIKEPRICE
    }
    if flds.opt_attribute.is_some() {
        let val = flds.opt_attribute.as_ref().unwrap();

        write!(output, "206={}\u{01}", val )?; // FIELD_OPTATTRIBUTE
    }
    if flds.contract_multiplier.is_some() {
        let val = flds.contract_multiplier.as_ref().unwrap();

        write!(output, "231={}\u{01}", val )?; // FIELD_CONTRACTMULTIPLIER
    }
    if flds.coupon_rate.is_some() {
        let val = flds.coupon_rate.as_ref().unwrap();

        write!(output, "223={}\u{01}", val )?; // FIELD_COUPONRATE
    }
    if flds.security_exchange.is_some() {
        let val = flds.security_exchange.as_ref().unwrap();

        write!(output, "207={}\u{01}", val )?; // FIELD_SECURITYEXCHANGE
    }
    if flds.issuer.is_some() {
        let val = flds.issuer.as_ref().unwrap();

        write!(output, "106={}\u{01}", val )?; // FIELD_ISSUER
    }
    if flds.encoded_issuer_len.is_some() {
        let val = flds.encoded_issuer_len.as_ref().unwrap();

        write!(output, "348={}\u{01}", val )?; // FIELD_ENCODEDISSUERLEN
    }
    if flds.encoded_issuer.is_some() {
        let val = flds.encoded_issuer.as_ref().unwrap();

        write!(output, "349={}\u{01}", val )?; // FIELD_ENCODEDISSUER
    }
    if flds.security_desc.is_some() {
        let val = flds.security_desc.as_ref().unwrap();

        write!(output, "107={}\u{01}", val )?; // FIELD_SECURITYDESC
    }
    if flds.encoded_security_desc_len.is_some() {
        let val = flds.encoded_security_desc_len.as_ref().unwrap();

        write!(output, "350={}\u{01}", val )?; // FIELD_ENCODEDSECURITYDESCLEN
    }
    if flds.encoded_security_desc.is_some() {
        let val = flds.encoded_security_desc.as_ref().unwrap();

        write!(output, "351={}\u{01}", val )?; // FIELD_ENCODEDSECURITYDESC
    }
    {
        let val = &flds.side;

        write!(output, "54={}\u{01}", val )?; // FIELD_SIDE
    }
    {
        let val = &flds.transact_time;

        write!(output, "60={}\u{01}", val )?; // FIELD_TRANSACTTIME
    }
    if flds.order_qty.is_some() {
        let val = flds.order_qty.as_ref().unwrap();

        write!(output, "38={}\u{01}", val )?; // FIELD_ORDERQTY
    }
    if flds.cash_order_qty.is_some() {
        let val = flds.cash_order_qty.as_ref().unwrap();

        write!(output, "152={}\u{01}", val )?; // FIELD_CASHORDERQTY
    }
    {
        let val = &flds.ord_type;

        write!(output, "40={}\u{01}", val )?; // FIELD_ORDTYPE
    }
    if flds.price.is_some() {
        let val = flds.price.as_ref().unwrap();

        write!(output, "44={}\u{01}", val )?; // FIELD_PRICE
    }
    if flds.stop_px.is_some() {
        let val = flds.stop_px.as_ref().unwrap();

        write!(output, "99={}\u{01}", val )?; // FIELD_STOPPX
    }
    if flds.peg_difference.is_some() {
        let val = flds.peg_difference.as_ref().unwrap();

        write!(output, "211={}\u{01}", val )?; // FIELD_PEGDIFFERENCE
    }
    if flds.discretion_inst.is_some() {
        let val = flds.discretion_inst.as_ref().unwrap();

        write!(output, "388={}\u{01}", val )?; // FIELD_DISCRETIONINST
    }
    if flds.discretion_offset.is_some() {
        let val = flds.discretion_offset.as_ref().unwrap();

        write!(output, "389={}\u{01}", val )?; // FIELD_DISCRETIONOFFSET
    }
    if flds.compliance_id.is_some() {
        let val = flds.compliance_id.as_ref().unwrap();

        write!(output, "376={}\u{01}", val )?; // FIELD_COMPLIANCEID
    }
    if flds.solicited_flag.is_some() {
        let val = flds.solicited_flag.as_ref().unwrap();

        write!(output, "377={}\u{01}", to_boolconv(val) )?; // FIELD_SOLICITEDFLAG
    }
    if flds.currency.is_some() {
        let val = flds.currency.as_ref().unwrap();

        write!(output, "15={}\u{01}", val )?; // FIELD_CURRENCY
    }
    if flds.time_in_force.is_some() {
        let val = flds.time_in_force.as_ref().unwrap();

        write!(output, "59={}\u{01}", val )?; // FIELD_TIMEINFORCE
    }
    if flds.effective_time.is_some() {
        let val = flds.effective_time.as_ref().unwrap();

        write!(output, "168={}\u{01}", val )?; // FIELD_EFFECTIVETIME
    }
    if flds.expire_date.is_some() {
        let val = flds.expire_date.as_ref().unwrap();

        write!(output, "432={}\u{01}", val )?; // FIELD_EXPIREDATE
    }
    if flds.expire_time.is_some() {
        let val = flds.expire_time.as_ref().unwrap();

        write!(output, "126={}\u{01}", val )?; // FIELD_EXPIRETIME
    }
    if flds.gtbooking_inst.is_some() {
        let val = flds.gtbooking_inst.as_ref().unwrap();

        write!(output, "427={}\u{01}", val )?; // FIELD_GTBOOKINGINST
    }
    if flds.commission.is_some() {
        let val = flds.commission.as_ref().unwrap();

        write!(output, "12={}\u{01}", val )?; // FIELD_COMMISSION
    }
    if flds.comm_type.is_some() {
        let val = flds.comm_type.as_ref().unwrap();

        write!(output, "13={}\u{01}", val )?; // FIELD_COMMTYPE
    }
    if flds.rule80_a.is_some() {
        let val = flds.rule80_a.as_ref().unwrap();

        write!(output, "47={}\u{01}", val )?; // FIELD_RULE80A
    }
    if flds.forex_req.is_some() {
        let val = flds.forex_req.as_ref().unwrap();

        write!(output, "121={}\u{01}", to_boolconv(val) )?; // FIELD_FOREXREQ
    }
    if flds.settl_currency.is_some() {
        let val = flds.settl_currency.as_ref().unwrap();

        write!(output, "120={}\u{01}", val )?; // FIELD_SETTLCURRENCY
    }
    if flds.text.is_some() {
        let val = flds.text.as_ref().unwrap();

        write!(output, "58={}\u{01}", val )?; // FIELD_TEXT
    }
    if flds.encoded_text_len.is_some() {
        let val = flds.encoded_text_len.as_ref().unwrap();

        write!(output, "354={}\u{01}", val )?; // FIELD_ENCODEDTEXTLEN
    }
    if flds.encoded_text.is_some() {
        let val = flds.encoded_text.as_ref().unwrap();

        write!(output, "355={}\u{01}", val )?; // FIELD_ENCODEDTEXT
    }
    if flds.fut_sett_date2.is_some() {
        let val = flds.fut_sett_date2.as_ref().unwrap();

        write!(output, "193={}\u{01}", val )?; // FIELD_FUTSETTDATE2
    }
    if flds.order_qty2.is_some() {
        let val = flds.order_qty2.as_ref().unwrap();

        write!(output, "192={}\u{01}", val )?; // FIELD_ORDERQTY2
    }
    if flds.open_close.is_some() {
        let val = flds.open_close.as_ref().unwrap();

        write!(output, "77={}\u{01}", val )?; // FIELD_OPENCLOSE
    }
    if flds.covered_or_uncovered.is_some() {
        let val = flds.covered_or_uncovered.as_ref().unwrap();

        write!(output, "203={}\u{01}", val )?; // FIELD_COVEREDORUNCOVERED
    }
    if flds.customer_or_firm.is_some() {
        let val = flds.customer_or_firm.as_ref().unwrap();

        write!(output, "204={}\u{01}", val )?; // FIELD_CUSTOMERORFIRM
    }
    if flds.max_show.is_some() {
        let val = flds.max_show.as_ref().unwrap();

        write!(output, "210={}\u{01}", val )?; // FIELD_MAXSHOW
    }
    if flds.locate_reqd.is_some() {
        let val = flds.locate_reqd.as_ref().unwrap();

        write!(output, "114={}\u{01}", to_boolconv(val) )?; // FIELD_LOCATEREQD
    }
    if flds.clearing_firm.is_some() {
        let val = flds.clearing_firm.as_ref().unwrap();

        write!(output, "439={}\u{01}", val )?; // FIELD_CLEARINGFIRM
    }
    if flds.clearing_account.is_some() {
        let val = flds.clearing_account.as_ref().unwrap();

        write!(output, "440={}\u{01}", val )?; // FIELD_CLEARINGACCOUNT
    }
    Ok( () )
}


fn write_order_status_request_fields(flds: &OrderStatusRequestFields, output: &mut Write) -> Result<(), io::Error> {

    if flds.order_id.is_some() {
        let val = flds.order_id.as_ref().unwrap();

        write!(output, "37={}\u{01}", val )?; // FIELD_ORDERID
    }
    {
        let val = &flds.cl_ord_id;

        write!(output, "11={}\u{01}", val )?; // FIELD_CLORDID
    }
    if flds.client_id.is_some() {
        let val = flds.client_id.as_ref().unwrap();

        write!(output, "109={}\u{01}", val )?; // FIELD_CLIENTID
    }
    if flds.account.is_some() {
        let val = flds.account.as_ref().unwrap();

        write!(output, "1={}\u{01}", val )?; // FIELD_ACCOUNT
    }
    if flds.exec_broker.is_some() {
        let val = flds.exec_broker.as_ref().unwrap();

        write!(output, "76={}\u{01}", val )?; // FIELD_EXECBROKER
    }
    {
        let val = &flds.symbol;

        write!(output, "55={}\u{01}", val )?; // FIELD_SYMBOL
    }
    if flds.symbol_sfx.is_some() {
        let val = flds.symbol_sfx.as_ref().unwrap();

        write!(output, "65={}\u{01}", val )?; // FIELD_SYMBOLSFX
    }
    if flds.security_id.is_some() {
        let val = flds.security_id.as_ref().unwrap();

        write!(output, "48={}\u{01}", val )?; // FIELD_SECURITYID
    }
    if flds.idsource.is_some() {
        let val = flds.idsource.as_ref().unwrap();

        write!(output, "22={}\u{01}", val )?; // FIELD_IDSOURCE
    }
    if flds.security_type.is_some() {
        let val = flds.security_type.as_ref().unwrap();

        write!(output, "167={}\u{01}", val )?; // FIELD_SECURITYTYPE
    }
    if flds.maturity_month_year.is_some() {
        let val = flds.maturity_month_year.as_ref().unwrap();

        write!(output, "200={}\u{01}", val )?; // FIELD_MATURITYMONTHYEAR
    }
    if flds.maturity_day.is_some() {
        let val = flds.maturity_day.as_ref().unwrap();

        write!(output, "205={}\u{01}", val )?; // FIELD_MATURITYDAY
    }
    if flds.put_or_call.is_some() {
        let val = flds.put_or_call.as_ref().unwrap();

        write!(output, "201={}\u{01}", val )?; // FIELD_PUTORCALL
    }
    if flds.strike_price.is_some() {
        let val = flds.strike_price.as_ref().unwrap();

        write!(output, "202={}\u{01}", val )?; // FIELD_STRIKEPRICE
    }
    if flds.opt_attribute.is_some() {
        let val = flds.opt_attribute.as_ref().unwrap();

        write!(output, "206={}\u{01}", val )?; // FIELD_OPTATTRIBUTE
    }
    if flds.contract_multiplier.is_some() {
        let val = flds.contract_multiplier.as_ref().unwrap();

        write!(output, "231={}\u{01}", val )?; // FIELD_CONTRACTMULTIPLIER
    }
    if flds.coupon_rate.is_some() {
        let val = flds.coupon_rate.as_ref().unwrap();

        write!(output, "223={}\u{01}", val )?; // FIELD_COUPONRATE
    }
    if flds.security_exchange.is_some() {
        let val = flds.security_exchange.as_ref().unwrap();

        write!(output, "207={}\u{01}", val )?; // FIELD_SECURITYEXCHANGE
    }
    if flds.issuer.is_some() {
        let val = flds.issuer.as_ref().unwrap();

        write!(output, "106={}\u{01}", val )?; // FIELD_ISSUER
    }
    if flds.encoded_issuer_len.is_some() {
        let val = flds.encoded_issuer_len.as_ref().unwrap();

        write!(output, "348={}\u{01}", val )?; // FIELD_ENCODEDISSUERLEN
    }
    if flds.encoded_issuer.is_some() {
        let val = flds.encoded_issuer.as_ref().unwrap();

        write!(output, "349={}\u{01}", val )?; // FIELD_ENCODEDISSUER
    }
    if flds.security_desc.is_some() {
        let val = flds.security_desc.as_ref().unwrap();

        write!(output, "107={}\u{01}", val )?; // FIELD_SECURITYDESC
    }
    if flds.encoded_security_desc_len.is_some() {
        let val = flds.encoded_security_desc_len.as_ref().unwrap();

        write!(output, "350={}\u{01}", val )?; // FIELD_ENCODEDSECURITYDESCLEN
    }
    if flds.encoded_security_desc.is_some() {
        let val = flds.encoded_security_desc.as_ref().unwrap();

        write!(output, "351={}\u{01}", val )?; // FIELD_ENCODEDSECURITYDESC
    }
    {
        let val = &flds.side;

        write!(output, "54={}\u{01}", val )?; // FIELD_SIDE
    }
    Ok( () )
}








fn write_group_no_contra_brokers2_fields( group: &Vec<NoContraBrokers2Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOCONTRABROKERS, len )?;

    for g in group {
        write_group_no_contra_brokers2_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_contra_brokers2_fields_line( flds: &NoContraBrokers2Fields, output: &mut Write) -> Result<(), io::Error> {

    if flds.contra_broker.is_some() {
        let val = flds.contra_broker.as_ref().unwrap();

        write!(output, "375={}\u{01}", val )?; // FIELD_CONTRABROKER
    }
    if flds.contra_trader.is_some() {
        let val = flds.contra_trader.as_ref().unwrap();

        write!(output, "337={}\u{01}", val )?; // FIELD_CONTRATRADER
    }
    if flds.contra_trade_qty.is_some() {
        let val = flds.contra_trade_qty.as_ref().unwrap();

        write!(output, "437={}\u{01}", val )?; // FIELD_CONTRATRADEQTY
    }
    if flds.contra_trade_time.is_some() {
        let val = flds.contra_trade_time.as_ref().unwrap();

        write!(output, "438={}\u{01}", val )?; // FIELD_CONTRATRADETIME
    }

    Ok( () )
}



fn write_group_no_allocs1_fields( group: &Vec<NoAllocs1Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOALLOCS, len )?;

    for g in group {
        write_group_no_allocs1_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_allocs1_fields_line( flds: &NoAllocs1Fields, output: &mut Write) -> Result<(), io::Error> {

    if flds.alloc_account.is_some() {
        let val = flds.alloc_account.as_ref().unwrap();

        write!(output, "79={}\u{01}", val )?; // FIELD_ALLOCACCOUNT
    }
    if flds.alloc_shares.is_some() {
        let val = flds.alloc_shares.as_ref().unwrap();

        write!(output, "80={}\u{01}", val )?; // FIELD_ALLOCSHARES
    }

    Ok( () )
}



fn write_group_no_msg_types3_fields( group: &Vec<NoMsgTypes3Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOMSGTYPES, len )?;

    for g in group {
        write_group_no_msg_types3_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_msg_types3_fields_line( flds: &NoMsgTypes3Fields, output: &mut Write) -> Result<(), io::Error> {

    if flds.ref_msg_type.is_some() {
        let val = flds.ref_msg_type.as_ref().unwrap();

        write!(output, "372={}\u{01}", val )?; // FIELD_REFMSGTYPE
    }
    if flds.msg_direction.is_some() {
        let val = flds.msg_direction.as_ref().unwrap();

        write!(output, "385={}\u{01}", val )?; // FIELD_MSGDIRECTION
    }

    Ok( () )
}



fn write_group_no_trading_sessions5_fields( group: &Vec<NoTradingSessions5Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOTRADINGSESSIONS, len )?;

    for g in group {
        write_group_no_trading_sessions5_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_trading_sessions5_fields_line( flds: &NoTradingSessions5Fields, output: &mut Write) -> Result<(), io::Error> {

    if flds.trading_session_id.is_some() {
        let val = flds.trading_session_id.as_ref().unwrap();

        write!(output, "336={}\u{01}", val )?; // FIELD_TRADINGSESSIONID
    }

    Ok( () )
}



fn write_group_no_orders4_fields( group: &Vec<NoOrders4Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOORDERS, len )?;

    for g in group {
        write_group_no_orders4_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_orders4_fields_line( flds: &NoOrders4Fields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.cl_ord_id;

        write!(output, "11={}\u{01}", val )?; // FIELD_CLORDID
    }
    {
        let val = &flds.list_seq_no;

        write!(output, "67={}\u{01}", val )?; // FIELD_LISTSEQNO
    }
    if flds.settl_inst_mode.is_some() {
        let val = flds.settl_inst_mode.as_ref().unwrap();

        write!(output, "160={}\u{01}", val )?; // FIELD_SETTLINSTMODE
    }
    if flds.client_id.is_some() {
        let val = flds.client_id.as_ref().unwrap();

        write!(output, "109={}\u{01}", val )?; // FIELD_CLIENTID
    }
    if flds.exec_broker.is_some() {
        let val = flds.exec_broker.as_ref().unwrap();

        write!(output, "76={}\u{01}", val )?; // FIELD_EXECBROKER
    }
    if flds.account.is_some() {
        let val = flds.account.as_ref().unwrap();

        write!(output, "1={}\u{01}", val )?; // FIELD_ACCOUNT
    }
    if flds.no_allocs.is_some() {
        let val = flds.no_allocs.as_ref().unwrap();

        write_group_no_allocs1_fields( val, output )?;
    }
    if flds.settlmnt_typ.is_some() {
        let val = flds.settlmnt_typ.as_ref().unwrap();

        write!(output, "63={}\u{01}", val )?; // FIELD_SETTLMNTTYP
    }
    if flds.fut_sett_date.is_some() {
        let val = flds.fut_sett_date.as_ref().unwrap();

        write!(output, "64={}\u{01}", val )?; // FIELD_FUTSETTDATE
    }
    if flds.handl_inst.is_some() {
        let val = flds.handl_inst.as_ref().unwrap();

        write!(output, "21={}\u{01}", val )?; // FIELD_HANDLINST
    }
    if flds.exec_inst.is_some() {
        let val = flds.exec_inst.as_ref().unwrap();

        write!(output, "18={}\u{01}", val )?; // FIELD_EXECINST
    }
    if flds.min_qty.is_some() {
        let val = flds.min_qty.as_ref().unwrap();

        write!(output, "110={}\u{01}", val )?; // FIELD_MINQTY
    }
    if flds.max_floor.is_some() {
        let val = flds.max_floor.as_ref().unwrap();

        write!(output, "111={}\u{01}", val )?; // FIELD_MAXFLOOR
    }
    if flds.ex_destination.is_some() {
        let val = flds.ex_destination.as_ref().unwrap();

        write!(output, "100={}\u{01}", val )?; // FIELD_EXDESTINATION
    }
    if flds.no_trading_sessions.is_some() {
        let val = flds.no_trading_sessions.as_ref().unwrap();

        write_group_no_trading_sessions5_fields( val, output )?;
    }
    if flds.process_code.is_some() {
        let val = flds.process_code.as_ref().unwrap();

        write!(output, "81={}\u{01}", val )?; // FIELD_PROCESSCODE
    }
    {
        let val = &flds.symbol;

        write!(output, "55={}\u{01}", val )?; // FIELD_SYMBOL
    }
    if flds.symbol_sfx.is_some() {
        let val = flds.symbol_sfx.as_ref().unwrap();

        write!(output, "65={}\u{01}", val )?; // FIELD_SYMBOLSFX
    }
    if flds.security_id.is_some() {
        let val = flds.security_id.as_ref().unwrap();

        write!(output, "48={}\u{01}", val )?; // FIELD_SECURITYID
    }
    if flds.idsource.is_some() {
        let val = flds.idsource.as_ref().unwrap();

        write!(output, "22={}\u{01}", val )?; // FIELD_IDSOURCE
    }
    if flds.security_type.is_some() {
        let val = flds.security_type.as_ref().unwrap();

        write!(output, "167={}\u{01}", val )?; // FIELD_SECURITYTYPE
    }
    if flds.maturity_month_year.is_some() {
        let val = flds.maturity_month_year.as_ref().unwrap();

        write!(output, "200={}\u{01}", val )?; // FIELD_MATURITYMONTHYEAR
    }
    if flds.maturity_day.is_some() {
        let val = flds.maturity_day.as_ref().unwrap();

        write!(output, "205={}\u{01}", val )?; // FIELD_MATURITYDAY
    }
    if flds.put_or_call.is_some() {
        let val = flds.put_or_call.as_ref().unwrap();

        write!(output, "201={}\u{01}", val )?; // FIELD_PUTORCALL
    }
    if flds.strike_price.is_some() {
        let val = flds.strike_price.as_ref().unwrap();

        write!(output, "202={}\u{01}", val )?; // FIELD_STRIKEPRICE
    }
    if flds.opt_attribute.is_some() {
        let val = flds.opt_attribute.as_ref().unwrap();

        write!(output, "206={}\u{01}", val )?; // FIELD_OPTATTRIBUTE
    }
    if flds.contract_multiplier.is_some() {
        let val = flds.contract_multiplier.as_ref().unwrap();

        write!(output, "231={}\u{01}", val )?; // FIELD_CONTRACTMULTIPLIER
    }
    if flds.coupon_rate.is_some() {
        let val = flds.coupon_rate.as_ref().unwrap();

        write!(output, "223={}\u{01}", val )?; // FIELD_COUPONRATE
    }
    if flds.security_exchange.is_some() {
        let val = flds.security_exchange.as_ref().unwrap();

        write!(output, "207={}\u{01}", val )?; // FIELD_SECURITYEXCHANGE
    }
    if flds.issuer.is_some() {
        let val = flds.issuer.as_ref().unwrap();

        write!(output, "106={}\u{01}", val )?; // FIELD_ISSUER
    }
    if flds.encoded_issuer_len.is_some() {
        let val = flds.encoded_issuer_len.as_ref().unwrap();

        write!(output, "348={}\u{01}", val )?; // FIELD_ENCODEDISSUERLEN
    }
    if flds.encoded_issuer.is_some() {
        let val = flds.encoded_issuer.as_ref().unwrap();

        write!(output, "349={}\u{01}", val )?; // FIELD_ENCODEDISSUER
    }
    if flds.security_desc.is_some() {
        let val = flds.security_desc.as_ref().unwrap();

        write!(output, "107={}\u{01}", val )?; // FIELD_SECURITYDESC
    }
    if flds.encoded_security_desc_len.is_some() {
        let val = flds.encoded_security_desc_len.as_ref().unwrap();

        write!(output, "350={}\u{01}", val )?; // FIELD_ENCODEDSECURITYDESCLEN
    }
    if flds.encoded_security_desc.is_some() {
        let val = flds.encoded_security_desc.as_ref().unwrap();

        write!(output, "351={}\u{01}", val )?; // FIELD_ENCODEDSECURITYDESC
    }
    if flds.prev_close_px.is_some() {
        let val = flds.prev_close_px.as_ref().unwrap();

        write!(output, "140={}\u{01}", val )?; // FIELD_PREVCLOSEPX
    }
    {
        let val = &flds.side;

        write!(output, "54={}\u{01}", val )?; // FIELD_SIDE
    }
    if flds.side_value_ind.is_some() {
        let val = flds.side_value_ind.as_ref().unwrap();

        write!(output, "401={}\u{01}", val )?; // FIELD_SIDEVALUEIND
    }
    if flds.locate_reqd.is_some() {
        let val = flds.locate_reqd.as_ref().unwrap();

        write!(output, "114={}\u{01}", to_boolconv(val) )?; // FIELD_LOCATEREQD
    }
    if flds.transact_time.is_some() {
        let val = flds.transact_time.as_ref().unwrap();

        write!(output, "60={}\u{01}", val )?; // FIELD_TRANSACTTIME
    }
    if flds.order_qty.is_some() {
        let val = flds.order_qty.as_ref().unwrap();

        write!(output, "38={}\u{01}", val )?; // FIELD_ORDERQTY
    }
    if flds.cash_order_qty.is_some() {
        let val = flds.cash_order_qty.as_ref().unwrap();

        write!(output, "152={}\u{01}", val )?; // FIELD_CASHORDERQTY
    }
    if flds.ord_type.is_some() {
        let val = flds.ord_type.as_ref().unwrap();

        write!(output, "40={}\u{01}", val )?; // FIELD_ORDTYPE
    }
    if flds.price.is_some() {
        let val = flds.price.as_ref().unwrap();

        write!(output, "44={}\u{01}", val )?; // FIELD_PRICE
    }
    if flds.stop_px.is_some() {
        let val = flds.stop_px.as_ref().unwrap();

        write!(output, "99={}\u{01}", val )?; // FIELD_STOPPX
    }
    if flds.currency.is_some() {
        let val = flds.currency.as_ref().unwrap();

        write!(output, "15={}\u{01}", val )?; // FIELD_CURRENCY
    }
    if flds.compliance_id.is_some() {
        let val = flds.compliance_id.as_ref().unwrap();

        write!(output, "376={}\u{01}", val )?; // FIELD_COMPLIANCEID
    }
    if flds.solicited_flag.is_some() {
        let val = flds.solicited_flag.as_ref().unwrap();

        write!(output, "377={}\u{01}", to_boolconv(val) )?; // FIELD_SOLICITEDFLAG
    }
    if flds.ioiid.is_some() {
        let val = flds.ioiid.as_ref().unwrap();

        write!(output, "23={}\u{01}", val )?; // FIELD_IOIID
    }
    if flds.quote_id.is_some() {
        let val = flds.quote_id.as_ref().unwrap();

        write!(output, "117={}\u{01}", val )?; // FIELD_QUOTEID
    }
    if flds.time_in_force.is_some() {
        let val = flds.time_in_force.as_ref().unwrap();

        write!(output, "59={}\u{01}", val )?; // FIELD_TIMEINFORCE
    }
    if flds.effective_time.is_some() {
        let val = flds.effective_time.as_ref().unwrap();

        write!(output, "168={}\u{01}", val )?; // FIELD_EFFECTIVETIME
    }
    if flds.expire_date.is_some() {
        let val = flds.expire_date.as_ref().unwrap();

        write!(output, "432={}\u{01}", val )?; // FIELD_EXPIREDATE
    }
    if flds.expire_time.is_some() {
        let val = flds.expire_time.as_ref().unwrap();

        write!(output, "126={}\u{01}", val )?; // FIELD_EXPIRETIME
    }
    if flds.gtbooking_inst.is_some() {
        let val = flds.gtbooking_inst.as_ref().unwrap();

        write!(output, "427={}\u{01}", val )?; // FIELD_GTBOOKINGINST
    }
    if flds.commission.is_some() {
        let val = flds.commission.as_ref().unwrap();

        write!(output, "12={}\u{01}", val )?; // FIELD_COMMISSION
    }
    if flds.comm_type.is_some() {
        let val = flds.comm_type.as_ref().unwrap();

        write!(output, "13={}\u{01}", val )?; // FIELD_COMMTYPE
    }
    if flds.rule80_a.is_some() {
        let val = flds.rule80_a.as_ref().unwrap();

        write!(output, "47={}\u{01}", val )?; // FIELD_RULE80A
    }
    if flds.forex_req.is_some() {
        let val = flds.forex_req.as_ref().unwrap();

        write!(output, "121={}\u{01}", to_boolconv(val) )?; // FIELD_FOREXREQ
    }
    if flds.settl_currency.is_some() {
        let val = flds.settl_currency.as_ref().unwrap();

        write!(output, "120={}\u{01}", val )?; // FIELD_SETTLCURRENCY
    }
    if flds.text.is_some() {
        let val = flds.text.as_ref().unwrap();

        write!(output, "58={}\u{01}", val )?; // FIELD_TEXT
    }
    if flds.encoded_text_len.is_some() {
        let val = flds.encoded_text_len.as_ref().unwrap();

        write!(output, "354={}\u{01}", val )?; // FIELD_ENCODEDTEXTLEN
    }
    if flds.encoded_text.is_some() {
        let val = flds.encoded_text.as_ref().unwrap();

        write!(output, "355={}\u{01}", val )?; // FIELD_ENCODEDTEXT
    }
    if flds.fut_sett_date2.is_some() {
        let val = flds.fut_sett_date2.as_ref().unwrap();

        write!(output, "193={}\u{01}", val )?; // FIELD_FUTSETTDATE2
    }
    if flds.order_qty2.is_some() {
        let val = flds.order_qty2.as_ref().unwrap();

        write!(output, "192={}\u{01}", val )?; // FIELD_ORDERQTY2
    }
    if flds.open_close.is_some() {
        let val = flds.open_close.as_ref().unwrap();

        write!(output, "77={}\u{01}", val )?; // FIELD_OPENCLOSE
    }
    if flds.covered_or_uncovered.is_some() {
        let val = flds.covered_or_uncovered.as_ref().unwrap();

        write!(output, "203={}\u{01}", val )?; // FIELD_COVEREDORUNCOVERED
    }
    if flds.customer_or_firm.is_some() {
        let val = flds.customer_or_firm.as_ref().unwrap();

        write!(output, "204={}\u{01}", val )?; // FIELD_CUSTOMERORFIRM
    }
    if flds.max_show.is_some() {
        let val = flds.max_show.as_ref().unwrap();

        write!(output, "210={}\u{01}", val )?; // FIELD_MAXSHOW
    }
    if flds.peg_difference.is_some() {
        let val = flds.peg_difference.as_ref().unwrap();

        write!(output, "211={}\u{01}", val )?; // FIELD_PEGDIFFERENCE
    }
    if flds.discretion_inst.is_some() {
        let val = flds.discretion_inst.as_ref().unwrap();

        write!(output, "388={}\u{01}", val )?; // FIELD_DISCRETIONINST
    }
    if flds.discretion_offset.is_some() {
        let val = flds.discretion_offset.as_ref().unwrap();

        write!(output, "389={}\u{01}", val )?; // FIELD_DISCRETIONOFFSET
    }
    if flds.clearing_firm.is_some() {
        let val = flds.clearing_firm.as_ref().unwrap();

        write!(output, "439={}\u{01}", val )?; // FIELD_CLEARINGFIRM
    }
    if flds.clearing_account.is_some() {
        let val = flds.clearing_account.as_ref().unwrap();

        write!(output, "440={}\u{01}", val )?; // FIELD_CLEARINGACCOUNT
    }

    Ok( () )
}




