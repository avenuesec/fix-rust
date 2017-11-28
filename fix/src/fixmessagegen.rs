// Generated file.
// Structs, enums to support parsing/generation of fix messages

#![ignore(unused_imports)]
#![ignore(dead_code)]

use std::str::{FromStr};
use std::{io, str, char, i32};
use std::default::{Default};

use chrono::prelude::*;  // DateTime
use serde::{Serialize,Deserialize,Deserializer,Serializer};
use serde::de::{self, Visitor};


use frame::{FieldVal};
use fixmessage::*;


use std::fmt::{Debug, Formatter, Display, Error};

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
    // type: 6
    IOI(Box<IOIFields>),
    // type: 7
    Advertisement(Box<AdvertisementFields>),
    // type: 8
    ExecutionReport(Box<ExecutionReportFields>),
    // type: 9
    OrderCancelReject(Box<OrderCancelRejectFields>),
    // type: A
    Logon(Box<LogonFields>),
    // type: B
    News(Box<NewsFields>),
    // type: C
    Email(Box<EmailFields>),
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
    // type: J
    Allocation(Box<AllocationFields>),
    // type: K
    ListCancelRequest(Box<ListCancelRequestFields>),
    // type: L
    ListExecute(Box<ListExecuteFields>),
    // type: M
    ListStatusRequest(Box<ListStatusRequestFields>),
    // type: N
    ListStatus(Box<ListStatusFields>),
    // type: P
    AllocationInstructionAck(Box<AllocationInstructionAckFields>),
    // type: Q
    DontKnowTrade(Box<DontKnowTradeFields>),
    // type: R
    QuoteRequest(Box<QuoteRequestFields>),
    // type: S
    Quote(Box<QuoteFields>),
    // type: T
    SettlementInstructions(Box<SettlementInstructionsFields>),
    // type: V
    MarketDataRequest(Box<MarketDataRequestFields>),
    // type: W
    MarketDataSnapshotFullRefresh(Box<MarketDataSnapshotFullRefreshFields>),
    // type: X
    MarketDataIncrementalRefresh(Box<MarketDataIncrementalRefreshFields>),
    // type: Y
    MarketDataRequestReject(Box<MarketDataRequestRejectFields>),
    // type: Z
    QuoteCancel(Box<QuoteCancelFields>),
    // type: a
    QuoteStatusRequest(Box<QuoteStatusRequestFields>),
    // type: b
    QuoteAcknowledgement(Box<QuoteAcknowledgementFields>),
    // type: c
    SecurityDefinitionRequest(Box<SecurityDefinitionRequestFields>),
    // type: d
    SecurityDefinition(Box<SecurityDefinitionFields>),
    // type: e
    SecurityStatusRequest(Box<SecurityStatusRequestFields>),
    // type: f
    SecurityStatus(Box<SecurityStatusFields>),
    // type: g
    TradingSessionStatusRequest(Box<TradingSessionStatusRequestFields>),
    // type: h
    TradingSessionStatus(Box<TradingSessionStatusFields>),
    // type: i
    MassQuote(Box<MassQuoteFields>),
    // type: j
    BusinessMessageReject(Box<BusinessMessageRejectFields>),
    // type: k
    BidRequest(Box<BidRequestFields>),
    // type: l
    BidResponse(Box<BidResponseFields>),
    // type: m
    ListStrikePrice(Box<ListStrikePriceFields>),
    // fixed, for when the parser can't figure out the message type
    UndefinedMessage,
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
    pub session_reject_reason : Option<Field_SessionRejectReason_Enum>, // FIELD_SESSIONREJECTREASON 373
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
pub struct IOIFields {
    pub ioiid : String, // FIELD_IOIID 23
    pub ioitrans_type : Field_IOITransType_Enum, // FIELD_IOITRANSTYPE 28
    pub ioiref_id : Option<String>, // FIELD_IOIREFID 26
    pub symbol : String, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<Field_IDSource_Enum>, // FIELD_IDSOURCE 22
    pub security_type : Option<Field_SecurityType_Enum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<Field_PutOrCall_Enum>, // FIELD_PUTORCALL 201
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
    pub side : Field_Side_Enum, // FIELD_SIDE 54
    pub ioishares : Field_IOIShares_Enum, // FIELD_IOISHARES 27
    pub price : Option<f32>, // FIELD_PRICE 44
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub valid_until_time : Option<UtcDateTime>, // FIELD_VALIDUNTILTIME 62
    pub ioiqlty_ind : Option<Field_IOIQltyInd_Enum>, // FIELD_IOIQLTYIND 25
    pub ioinatural_flag : Option<bool>, // FIELD_IOINATURALFLAG 130
    pub no_ioiqualifiers : Option<Vec<NoIOIQualifiers9Fields>>, // FIELD_NOIOIQUALIFIERS 0
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355
    pub transact_time : Option<UtcDateTime>, // FIELD_TRANSACTTIME 60
    pub urllink : Option<String>, // FIELD_URLLINK 149
    pub no_routing_ids : Option<Vec<NoRoutingIDs27Fields>>, // FIELD_NOROUTINGIDS 0
    pub spread_to_benchmark : Option<f32>, // FIELD_SPREADTOBENCHMARK 218
    pub benchmark : Option<Field_Benchmark_Enum>, // FIELD_BENCHMARK 219

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct AdvertisementFields {
    pub adv_id : String, // FIELD_ADVID 2
    pub adv_trans_type : Field_AdvTransType_Enum, // FIELD_ADVTRANSTYPE 5
    pub adv_ref_id : Option<String>, // FIELD_ADVREFID 3
    pub symbol : String, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<Field_IDSource_Enum>, // FIELD_IDSOURCE 22
    pub security_type : Option<Field_SecurityType_Enum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<Field_PutOrCall_Enum>, // FIELD_PUTORCALL 201
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
    pub adv_side : Field_AdvSide_Enum, // FIELD_ADVSIDE 4
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
    pub no_contra_brokers : Option<Vec<NoContraBrokers7Fields>>, // FIELD_NOCONTRABROKERS 0
    pub list_id : Option<String>, // FIELD_LISTID 66
    pub exec_id : String, // FIELD_EXECID 17
    pub exec_trans_type : Field_ExecTransType_Enum, // FIELD_EXECTRANSTYPE 20
    pub exec_ref_id : Option<String>, // FIELD_EXECREFID 19
    pub exec_type : Field_ExecType_Enum, // FIELD_EXECTYPE 150
    pub ord_status : Field_OrdStatus_Enum, // FIELD_ORDSTATUS 39
    pub ord_rej_reason : Option<Field_OrdRejReason_Enum>, // FIELD_ORDREJREASON 103
    pub exec_restatement_reason : Option<Field_ExecRestatementReason_Enum>, // FIELD_EXECRESTATEMENTREASON 378
    pub account : Option<String>, // FIELD_ACCOUNT 1
    pub settlmnt_typ : Option<Field_SettlmntTyp_Enum>, // FIELD_SETTLMNTTYP 63
    pub fut_sett_date : Option<UtcDateTime>, // FIELD_FUTSETTDATE 64
    pub symbol : String, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<Field_IDSource_Enum>, // FIELD_IDSOURCE 22
    pub security_type : Option<Field_SecurityType_Enum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<Field_PutOrCall_Enum>, // FIELD_PUTORCALL 201
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
    pub side : Field_Side_Enum, // FIELD_SIDE 54
    pub order_qty : Option<f32>, // FIELD_ORDERQTY 38
    pub cash_order_qty : Option<f32>, // FIELD_CASHORDERQTY 152
    pub ord_type : Option<Field_OrdType_Enum>, // FIELD_ORDTYPE 40
    pub price : Option<f32>, // FIELD_PRICE 44
    pub stop_px : Option<f32>, // FIELD_STOPPX 99
    pub peg_difference : Option<f32>, // FIELD_PEGDIFFERENCE 211
    pub discretion_inst : Option<Field_DiscretionInst_Enum>, // FIELD_DISCRETIONINST 388
    pub discretion_offset : Option<f32>, // FIELD_DISCRETIONOFFSET 389
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub compliance_id : Option<String>, // FIELD_COMPLIANCEID 376
    pub solicited_flag : Option<bool>, // FIELD_SOLICITEDFLAG 377
    pub time_in_force : Option<Field_TimeInForce_Enum>, // FIELD_TIMEINFORCE 59
    pub effective_time : Option<UtcDateTime>, // FIELD_EFFECTIVETIME 168
    pub expire_date : Option<UtcDateTime>, // FIELD_EXPIREDATE 432
    pub expire_time : Option<UtcDateTime>, // FIELD_EXPIRETIME 126
    pub exec_inst : Option<Field_ExecInst_Enum>, // FIELD_EXECINST 18
    pub rule80_a : Option<Field_Rule80A_Enum>, // FIELD_RULE80A 47
    pub last_shares : Option<f32>, // FIELD_LASTSHARES 32
    pub last_px : Option<f32>, // FIELD_LASTPX 31
    pub last_spot_rate : Option<f32>, // FIELD_LASTSPOTRATE 194
    pub last_forward_points : Option<f32>, // FIELD_LASTFORWARDPOINTS 195
    pub last_mkt : Option<String>, // FIELD_LASTMKT 30
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub last_capacity : Option<Field_LastCapacity_Enum>, // FIELD_LASTCAPACITY 29
    pub leaves_qty : f32, // FIELD_LEAVESQTY 151
    pub cum_qty : f32, // FIELD_CUMQTY 14
    pub avg_px : f32, // FIELD_AVGPX 6
    pub day_order_qty : Option<f32>, // FIELD_DAYORDERQTY 424
    pub day_cum_qty : Option<f32>, // FIELD_DAYCUMQTY 425
    pub day_avg_px : Option<f32>, // FIELD_DAYAVGPX 426
    pub gtbooking_inst : Option<Field_GTBookingInst_Enum>, // FIELD_GTBOOKINGINST 427
    pub trade_date : Option<UtcDateTime>, // FIELD_TRADEDATE 75
    pub transact_time : Option<UtcDateTime>, // FIELD_TRANSACTTIME 60
    pub report_to_exch : Option<bool>, // FIELD_REPORTTOEXCH 113
    pub commission : Option<f32>, // FIELD_COMMISSION 12
    pub comm_type : Option<Field_CommType_Enum>, // FIELD_COMMTYPE 13
    pub gross_trade_amt : Option<f32>, // FIELD_GROSSTRADEAMT 381
    pub settl_curr_amt : Option<f32>, // FIELD_SETTLCURRAMT 119
    pub settl_currency : Option<f32>, // FIELD_SETTLCURRENCY 120
    pub settl_curr_fx_rate : Option<f32>, // FIELD_SETTLCURRFXRATE 155
    pub settl_curr_fx_rate_calc : Option<Field_SettlCurrFxRateCalc_Enum>, // FIELD_SETTLCURRFXRATECALC 156
    pub handl_inst : Option<Field_HandlInst_Enum>, // FIELD_HANDLINST 21
    pub min_qty : Option<f32>, // FIELD_MINQTY 110
    pub max_floor : Option<f32>, // FIELD_MAXFLOOR 111
    pub open_close : Option<Field_OpenClose_Enum>, // FIELD_OPENCLOSE 77
    pub max_show : Option<f32>, // FIELD_MAXSHOW 210
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355
    pub fut_sett_date2 : Option<UtcDateTime>, // FIELD_FUTSETTDATE2 193
    pub order_qty2 : Option<f32>, // FIELD_ORDERQTY2 192
    pub clearing_firm : Option<String>, // FIELD_CLEARINGFIRM 439
    pub clearing_account : Option<String>, // FIELD_CLEARINGACCOUNT 440
    pub multi_leg_reporting_type : Option<Field_MultiLegReportingType_Enum>, // FIELD_MULTILEGREPORTINGTYPE 442

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct OrderCancelRejectFields {
    pub order_id : String, // FIELD_ORDERID 37
    pub secondary_order_id : Option<String>, // FIELD_SECONDARYORDERID 198
    pub cl_ord_id : String, // FIELD_CLORDID 11
    pub orig_cl_ord_id : String, // FIELD_ORIGCLORDID 41
    pub ord_status : Field_OrdStatus_Enum, // FIELD_ORDSTATUS 39
    pub client_id : Option<String>, // FIELD_CLIENTID 109
    pub exec_broker : Option<String>, // FIELD_EXECBROKER 76
    pub list_id : Option<String>, // FIELD_LISTID 66
    pub account : Option<String>, // FIELD_ACCOUNT 1
    pub transact_time : Option<UtcDateTime>, // FIELD_TRANSACTTIME 60
    pub cxl_rej_response_to : Field_CxlRejResponseTo_Enum, // FIELD_CXLREJRESPONSETO 434
    pub cxl_rej_reason : Option<Field_CxlRejReason_Enum>, // FIELD_CXLREJREASON 102
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct LogonFields {
    pub encrypt_method : Field_EncryptMethod_Enum, // FIELD_ENCRYPTMETHOD 98
    pub heart_bt_int : i32, // FIELD_HEARTBTINT 108
    pub raw_data_length : Option<usize>, // FIELD_RAWDATALENGTH 95
    pub raw_data : Option<String>, // FIELD_RAWDATA 96
    pub reset_seq_num_flag : Option<bool>, // FIELD_RESETSEQNUMFLAG 141
    pub max_message_size : Option<i32>, // FIELD_MAXMESSAGESIZE 383
    pub no_msg_types : Option<Vec<NoMsgTypes14Fields>>, // FIELD_NOMSGTYPES 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NewsFields {
    pub orig_time : Option<UtcDateTime>, // FIELD_ORIGTIME 42
    pub urgency : Option<Field_Urgency_Enum>, // FIELD_URGENCY 61
    pub headline : String, // FIELD_HEADLINE 148
    pub encoded_headline_len : Option<usize>, // FIELD_ENCODEDHEADLINELEN 358
    pub encoded_headline : Option<String>, // FIELD_ENCODEDHEADLINE 359
    pub no_routing_ids : Option<Vec<NoRoutingIDs27Fields>>, // FIELD_NOROUTINGIDS 0
    pub no_related_sym : Option<Vec<NoRelatedSym26Fields>>, // FIELD_NORELATEDSYM 0
    pub lines_of_text : Vec<LinesOfText1Fields>, // FIELD_LINESOFTEXT 0
    pub urllink : Option<String>, // FIELD_URLLINK 149
    pub raw_data_length : Option<usize>, // FIELD_RAWDATALENGTH 95
    pub raw_data : Option<String>, // FIELD_RAWDATA 96

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct EmailFields {
    pub email_thread_id : String, // FIELD_EMAILTHREADID 164
    pub email_type : Field_EmailType_Enum, // FIELD_EMAILTYPE 94
    pub orig_time : Option<UtcDateTime>, // FIELD_ORIGTIME 42
    pub subject : String, // FIELD_SUBJECT 147
    pub encoded_subject_len : Option<usize>, // FIELD_ENCODEDSUBJECTLEN 356
    pub encoded_subject : Option<String>, // FIELD_ENCODEDSUBJECT 357
    pub no_routing_ids : Option<Vec<NoRoutingIDs27Fields>>, // FIELD_NOROUTINGIDS 0
    pub no_related_sym : Option<Vec<NoRelatedSym26Fields>>, // FIELD_NORELATEDSYM 0
    pub order_id : Option<String>, // FIELD_ORDERID 37
    pub cl_ord_id : Option<String>, // FIELD_CLORDID 11
    pub lines_of_text : Vec<LinesOfText1Fields>, // FIELD_LINESOFTEXT 0
    pub raw_data_length : Option<usize>, // FIELD_RAWDATALENGTH 95
    pub raw_data : Option<String>, // FIELD_RAWDATA 96

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NewOrderSingleFields {
    pub cl_ord_id : String, // FIELD_CLORDID 11
    pub client_id : Option<String>, // FIELD_CLIENTID 109
    pub exec_broker : Option<String>, // FIELD_EXECBROKER 76
    pub account : Option<String>, // FIELD_ACCOUNT 1
    pub no_allocs : Option<Vec<NoAllocs3Fields>>, // FIELD_NOALLOCS 0
    pub settlmnt_typ : Option<Field_SettlmntTyp_Enum>, // FIELD_SETTLMNTTYP 63
    pub fut_sett_date : Option<UtcDateTime>, // FIELD_FUTSETTDATE 64
    pub handl_inst : Field_HandlInst_Enum, // FIELD_HANDLINST 21
    pub exec_inst : Option<Field_ExecInst_Enum>, // FIELD_EXECINST 18
    pub min_qty : Option<f32>, // FIELD_MINQTY 110
    pub max_floor : Option<f32>, // FIELD_MAXFLOOR 111
    pub ex_destination : Option<String>, // FIELD_EXDESTINATION 100
    pub no_trading_sessions : Option<Vec<NoTradingSessions29Fields>>, // FIELD_NOTRADINGSESSIONS 0
    pub process_code : Option<Field_ProcessCode_Enum>, // FIELD_PROCESSCODE 81
    pub symbol : String, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<Field_IDSource_Enum>, // FIELD_IDSOURCE 22
    pub security_type : Option<Field_SecurityType_Enum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<Field_PutOrCall_Enum>, // FIELD_PUTORCALL 201
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
    pub side : Field_Side_Enum, // FIELD_SIDE 54
    pub locate_reqd : Option<bool>, // FIELD_LOCATEREQD 114
    pub transact_time : UtcDateTime, // FIELD_TRANSACTTIME 60
    pub order_qty : Option<f32>, // FIELD_ORDERQTY 38
    pub cash_order_qty : Option<f32>, // FIELD_CASHORDERQTY 152
    pub ord_type : Field_OrdType_Enum, // FIELD_ORDTYPE 40
    pub price : Option<f32>, // FIELD_PRICE 44
    pub stop_px : Option<f32>, // FIELD_STOPPX 99
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub compliance_id : Option<String>, // FIELD_COMPLIANCEID 376
    pub solicited_flag : Option<bool>, // FIELD_SOLICITEDFLAG 377
    pub ioiid : Option<String>, // FIELD_IOIID 23
    pub quote_id : Option<String>, // FIELD_QUOTEID 117
    pub time_in_force : Option<Field_TimeInForce_Enum>, // FIELD_TIMEINFORCE 59
    pub effective_time : Option<UtcDateTime>, // FIELD_EFFECTIVETIME 168
    pub expire_date : Option<UtcDateTime>, // FIELD_EXPIREDATE 432
    pub expire_time : Option<UtcDateTime>, // FIELD_EXPIRETIME 126
    pub gtbooking_inst : Option<Field_GTBookingInst_Enum>, // FIELD_GTBOOKINGINST 427
    pub commission : Option<f32>, // FIELD_COMMISSION 12
    pub comm_type : Option<Field_CommType_Enum>, // FIELD_COMMTYPE 13
    pub rule80_a : Option<Field_Rule80A_Enum>, // FIELD_RULE80A 47
    pub forex_req : Option<bool>, // FIELD_FOREXREQ 121
    pub settl_currency : Option<f32>, // FIELD_SETTLCURRENCY 120
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355
    pub fut_sett_date2 : Option<UtcDateTime>, // FIELD_FUTSETTDATE2 193
    pub order_qty2 : Option<f32>, // FIELD_ORDERQTY2 192
    pub open_close : Option<Field_OpenClose_Enum>, // FIELD_OPENCLOSE 77
    pub covered_or_uncovered : Option<Field_CoveredOrUncovered_Enum>, // FIELD_COVEREDORUNCOVERED 203
    pub customer_or_firm : Option<Field_CustomerOrFirm_Enum>, // FIELD_CUSTOMERORFIRM 204
    pub max_show : Option<f32>, // FIELD_MAXSHOW 210
    pub peg_difference : Option<f32>, // FIELD_PEGDIFFERENCE 211
    pub discretion_inst : Option<Field_DiscretionInst_Enum>, // FIELD_DISCRETIONINST 388
    pub discretion_offset : Option<f32>, // FIELD_DISCRETIONOFFSET 389
    pub clearing_firm : Option<String>, // FIELD_CLEARINGFIRM 439
    pub clearing_account : Option<String>, // FIELD_CLEARINGACCOUNT 440

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NewOrderListFields {
    pub list_id : String, // FIELD_LISTID 66
    pub bid_id : Option<String>, // FIELD_BIDID 390
    pub client_bid_id : Option<String>, // FIELD_CLIENTBIDID 391
    pub prog_rpt_reqs : Option<Field_ProgRptReqs_Enum>, // FIELD_PROGRPTREQS 414
    pub bid_type : i32, // FIELD_BIDTYPE 394
    pub prog_period_interval : Option<i32>, // FIELD_PROGPERIODINTERVAL 415
    pub list_exec_inst_type : Option<Field_ListExecInstType_Enum>, // FIELD_LISTEXECINSTTYPE 433
    pub list_exec_inst : Option<String>, // FIELD_LISTEXECINST 69
    pub encoded_list_exec_inst_len : Option<usize>, // FIELD_ENCODEDLISTEXECINSTLEN 352
    pub encoded_list_exec_inst : Option<String>, // FIELD_ENCODEDLISTEXECINST 353
    pub tot_no_orders : i32, // FIELD_TOTNOORDERS 68
    pub no_orders : Vec<NoOrders15Fields>, // FIELD_NOORDERS 0

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
    pub idsource : Option<Field_IDSource_Enum>, // FIELD_IDSOURCE 22
    pub security_type : Option<Field_SecurityType_Enum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<Field_PutOrCall_Enum>, // FIELD_PUTORCALL 201
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
    pub side : Field_Side_Enum, // FIELD_SIDE 54
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
    pub no_allocs : Option<Vec<NoAllocs3Fields>>, // FIELD_NOALLOCS 0
    pub settlmnt_typ : Option<Field_SettlmntTyp_Enum>, // FIELD_SETTLMNTTYP 63
    pub fut_sett_date : Option<UtcDateTime>, // FIELD_FUTSETTDATE 64
    pub handl_inst : Field_HandlInst_Enum, // FIELD_HANDLINST 21
    pub exec_inst : Option<Field_ExecInst_Enum>, // FIELD_EXECINST 18
    pub min_qty : Option<f32>, // FIELD_MINQTY 110
    pub max_floor : Option<f32>, // FIELD_MAXFLOOR 111
    pub ex_destination : Option<String>, // FIELD_EXDESTINATION 100
    pub no_trading_sessions : Option<Vec<NoTradingSessions29Fields>>, // FIELD_NOTRADINGSESSIONS 0
    pub symbol : String, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<Field_IDSource_Enum>, // FIELD_IDSOURCE 22
    pub security_type : Option<Field_SecurityType_Enum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<Field_PutOrCall_Enum>, // FIELD_PUTORCALL 201
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
    pub side : Field_Side_Enum, // FIELD_SIDE 54
    pub transact_time : UtcDateTime, // FIELD_TRANSACTTIME 60
    pub order_qty : Option<f32>, // FIELD_ORDERQTY 38
    pub cash_order_qty : Option<f32>, // FIELD_CASHORDERQTY 152
    pub ord_type : Field_OrdType_Enum, // FIELD_ORDTYPE 40
    pub price : Option<f32>, // FIELD_PRICE 44
    pub stop_px : Option<f32>, // FIELD_STOPPX 99
    pub peg_difference : Option<f32>, // FIELD_PEGDIFFERENCE 211
    pub discretion_inst : Option<Field_DiscretionInst_Enum>, // FIELD_DISCRETIONINST 388
    pub discretion_offset : Option<f32>, // FIELD_DISCRETIONOFFSET 389
    pub compliance_id : Option<String>, // FIELD_COMPLIANCEID 376
    pub solicited_flag : Option<bool>, // FIELD_SOLICITEDFLAG 377
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub time_in_force : Option<Field_TimeInForce_Enum>, // FIELD_TIMEINFORCE 59
    pub effective_time : Option<UtcDateTime>, // FIELD_EFFECTIVETIME 168
    pub expire_date : Option<UtcDateTime>, // FIELD_EXPIREDATE 432
    pub expire_time : Option<UtcDateTime>, // FIELD_EXPIRETIME 126
    pub gtbooking_inst : Option<Field_GTBookingInst_Enum>, // FIELD_GTBOOKINGINST 427
    pub commission : Option<f32>, // FIELD_COMMISSION 12
    pub comm_type : Option<Field_CommType_Enum>, // FIELD_COMMTYPE 13
    pub rule80_a : Option<Field_Rule80A_Enum>, // FIELD_RULE80A 47
    pub forex_req : Option<bool>, // FIELD_FOREXREQ 121
    pub settl_currency : Option<f32>, // FIELD_SETTLCURRENCY 120
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355
    pub fut_sett_date2 : Option<UtcDateTime>, // FIELD_FUTSETTDATE2 193
    pub order_qty2 : Option<f32>, // FIELD_ORDERQTY2 192
    pub open_close : Option<Field_OpenClose_Enum>, // FIELD_OPENCLOSE 77
    pub covered_or_uncovered : Option<Field_CoveredOrUncovered_Enum>, // FIELD_COVEREDORUNCOVERED 203
    pub customer_or_firm : Option<Field_CustomerOrFirm_Enum>, // FIELD_CUSTOMERORFIRM 204
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
    pub idsource : Option<Field_IDSource_Enum>, // FIELD_IDSOURCE 22
    pub security_type : Option<Field_SecurityType_Enum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<Field_PutOrCall_Enum>, // FIELD_PUTORCALL 201
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
    pub side : Field_Side_Enum, // FIELD_SIDE 54

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct AllocationFields {
    pub alloc_id : String, // FIELD_ALLOCID 70
    pub alloc_trans_type : Field_AllocTransType_Enum, // FIELD_ALLOCTRANSTYPE 71
    pub ref_alloc_id : Option<String>, // FIELD_REFALLOCID 72
    pub alloc_link_id : Option<String>, // FIELD_ALLOCLINKID 196
    pub alloc_link_type : Option<Field_AllocLinkType_Enum>, // FIELD_ALLOCLINKTYPE 197
    pub no_orders : Option<Vec<NoOrders16Fields>>, // FIELD_NOORDERS 0
    pub no_execs : Option<Vec<NoExecs8Fields>>, // FIELD_NOEXECS 0
    pub side : Field_Side_Enum, // FIELD_SIDE 54
    pub symbol : String, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<Field_IDSource_Enum>, // FIELD_IDSOURCE 22
    pub security_type : Option<Field_SecurityType_Enum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<Field_PutOrCall_Enum>, // FIELD_PUTORCALL 201
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
    pub shares : f32, // FIELD_SHARES 53
    pub last_mkt : Option<String>, // FIELD_LASTMKT 30
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub avg_px : f32, // FIELD_AVGPX 6
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub avg_prx_precision : Option<i32>, // FIELD_AVGPRXPRECISION 74
    pub trade_date : UtcDateTime, // FIELD_TRADEDATE 75
    pub transact_time : Option<UtcDateTime>, // FIELD_TRANSACTTIME 60
    pub settlmnt_typ : Option<Field_SettlmntTyp_Enum>, // FIELD_SETTLMNTTYP 63
    pub fut_sett_date : Option<UtcDateTime>, // FIELD_FUTSETTDATE 64
    pub gross_trade_amt : Option<f32>, // FIELD_GROSSTRADEAMT 381
    pub net_money : Option<f32>, // FIELD_NETMONEY 118
    pub open_close : Option<Field_OpenClose_Enum>, // FIELD_OPENCLOSE 77
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355
    pub num_days_interest : Option<i32>, // FIELD_NUMDAYSINTEREST 157
    pub accrued_interest_rate : Option<f32>, // FIELD_ACCRUEDINTERESTRATE 158
    pub no_allocs : Option<Vec<NoAllocs2Fields>>, // FIELD_NOALLOCS 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct ListCancelRequestFields {
    pub list_id : String, // FIELD_LISTID 66
    pub transact_time : UtcDateTime, // FIELD_TRANSACTTIME 60
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct ListExecuteFields {
    pub list_id : String, // FIELD_LISTID 66
    pub client_bid_id : Option<String>, // FIELD_CLIENTBIDID 391
    pub bid_id : Option<String>, // FIELD_BIDID 390
    pub transact_time : UtcDateTime, // FIELD_TRANSACTTIME 60
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct ListStatusRequestFields {
    pub list_id : String, // FIELD_LISTID 66
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct ListStatusFields {
    pub list_id : String, // FIELD_LISTID 66
    pub list_status_type : i32, // FIELD_LISTSTATUSTYPE 429
    pub no_rpts : i32, // FIELD_NORPTS 82
    pub list_order_status : i32, // FIELD_LISTORDERSTATUS 431
    pub rpt_seq : i32, // FIELD_RPTSEQ 83
    pub list_status_text : Option<String>, // FIELD_LISTSTATUSTEXT 444
    pub encoded_list_status_text_len : Option<usize>, // FIELD_ENCODEDLISTSTATUSTEXTLEN 445
    pub encoded_list_status_text : Option<String>, // FIELD_ENCODEDLISTSTATUSTEXT 446
    pub transact_time : Option<UtcDateTime>, // FIELD_TRANSACTTIME 60
    pub tot_no_orders : i32, // FIELD_TOTNOORDERS 68
    pub no_orders : Vec<NoOrders17Fields>, // FIELD_NOORDERS 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct AllocationInstructionAckFields {
    pub client_id : Option<String>, // FIELD_CLIENTID 109
    pub exec_broker : Option<String>, // FIELD_EXECBROKER 76
    pub alloc_id : String, // FIELD_ALLOCID 70
    pub trade_date : UtcDateTime, // FIELD_TRADEDATE 75
    pub transact_time : Option<UtcDateTime>, // FIELD_TRANSACTTIME 60
    pub alloc_status : Field_AllocStatus_Enum, // FIELD_ALLOCSTATUS 87
    pub alloc_rej_code : Option<Field_AllocRejCode_Enum>, // FIELD_ALLOCREJCODE 88
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct DontKnowTradeFields {
    pub order_id : String, // FIELD_ORDERID 37
    pub exec_id : String, // FIELD_EXECID 17
    pub dkreason : Field_DKReason_Enum, // FIELD_DKREASON 127
    pub symbol : String, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<Field_IDSource_Enum>, // FIELD_IDSOURCE 22
    pub security_type : Option<Field_SecurityType_Enum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<Field_PutOrCall_Enum>, // FIELD_PUTORCALL 201
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
    pub side : Field_Side_Enum, // FIELD_SIDE 54
    pub order_qty : Option<f32>, // FIELD_ORDERQTY 38
    pub cash_order_qty : Option<f32>, // FIELD_CASHORDERQTY 152
    pub last_shares : Option<f32>, // FIELD_LASTSHARES 32
    pub last_px : Option<f32>, // FIELD_LASTPX 31
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct QuoteRequestFields {
    pub quote_req_id : String, // FIELD_QUOTEREQID 131
    pub no_related_sym : Vec<NoRelatedSym25Fields>, // FIELD_NORELATEDSYM 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct QuoteFields {
    pub quote_req_id : Option<String>, // FIELD_QUOTEREQID 131
    pub quote_id : String, // FIELD_QUOTEID 117
    pub quote_response_level : Option<Field_QuoteResponseLevel_Enum>, // FIELD_QUOTERESPONSELEVEL 301
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub symbol : String, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<Field_IDSource_Enum>, // FIELD_IDSOURCE 22
    pub security_type : Option<Field_SecurityType_Enum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<Field_PutOrCall_Enum>, // FIELD_PUTORCALL 201
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
    pub bid_px : Option<f32>, // FIELD_BIDPX 132
    pub offer_px : Option<f32>, // FIELD_OFFERPX 133
    pub bid_size : Option<f32>, // FIELD_BIDSIZE 134
    pub offer_size : Option<f32>, // FIELD_OFFERSIZE 135
    pub valid_until_time : Option<UtcDateTime>, // FIELD_VALIDUNTILTIME 62
    pub bid_spot_rate : Option<f32>, // FIELD_BIDSPOTRATE 188
    pub offer_spot_rate : Option<f32>, // FIELD_OFFERSPOTRATE 190
    pub bid_forward_points : Option<f32>, // FIELD_BIDFORWARDPOINTS 189
    pub offer_forward_points : Option<f32>, // FIELD_OFFERFORWARDPOINTS 191
    pub transact_time : Option<UtcDateTime>, // FIELD_TRANSACTTIME 60
    pub fut_sett_date : Option<UtcDateTime>, // FIELD_FUTSETTDATE 64
    pub ord_type : Option<Field_OrdType_Enum>, // FIELD_ORDTYPE 40
    pub fut_sett_date2 : Option<UtcDateTime>, // FIELD_FUTSETTDATE2 193
    pub order_qty2 : Option<f32>, // FIELD_ORDERQTY2 192
    pub currency : Option<f32>, // FIELD_CURRENCY 15

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct SettlementInstructionsFields {
    pub settl_inst_id : String, // FIELD_SETTLINSTID 162
    pub settl_inst_trans_type : Field_SettlInstTransType_Enum, // FIELD_SETTLINSTTRANSTYPE 163
    pub settl_inst_ref_id : String, // FIELD_SETTLINSTREFID 214
    pub settl_inst_mode : Field_SettlInstMode_Enum, // FIELD_SETTLINSTMODE 160
    pub settl_inst_source : Field_SettlInstSource_Enum, // FIELD_SETTLINSTSOURCE 165
    pub alloc_account : String, // FIELD_ALLOCACCOUNT 79
    pub settl_location : Option<Field_SettlLocation_Enum>, // FIELD_SETTLLOCATION 166
    pub trade_date : Option<UtcDateTime>, // FIELD_TRADEDATE 75
    pub alloc_id : Option<String>, // FIELD_ALLOCID 70
    pub last_mkt : Option<String>, // FIELD_LASTMKT 30
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub side : Option<Field_Side_Enum>, // FIELD_SIDE 54
    pub security_type : Option<Field_SecurityType_Enum>, // FIELD_SECURITYTYPE 167
    pub effective_time : Option<UtcDateTime>, // FIELD_EFFECTIVETIME 168
    pub transact_time : UtcDateTime, // FIELD_TRANSACTTIME 60
    pub client_id : Option<String>, // FIELD_CLIENTID 109
    pub exec_broker : Option<String>, // FIELD_EXECBROKER 76
    pub stand_inst_db_type : Option<Field_StandInstDbType_Enum>, // FIELD_STANDINSTDBTYPE 169
    pub stand_inst_db_name : Option<String>, // FIELD_STANDINSTDBNAME 170
    pub stand_inst_db_id : Option<String>, // FIELD_STANDINSTDBID 171
    pub settl_delivery_type : Option<i32>, // FIELD_SETTLDELIVERYTYPE 172
    pub settl_depository_code : Option<String>, // FIELD_SETTLDEPOSITORYCODE 173
    pub settl_brkr_code : Option<String>, // FIELD_SETTLBRKRCODE 174
    pub settl_inst_code : Option<String>, // FIELD_SETTLINSTCODE 175
    pub security_settl_agent_name : Option<String>, // FIELD_SECURITYSETTLAGENTNAME 176
    pub security_settl_agent_code : Option<String>, // FIELD_SECURITYSETTLAGENTCODE 177
    pub security_settl_agent_acct_num : Option<String>, // FIELD_SECURITYSETTLAGENTACCTNUM 178
    pub security_settl_agent_acct_name : Option<String>, // FIELD_SECURITYSETTLAGENTACCTNAME 179
    pub security_settl_agent_contact_name : Option<String>, // FIELD_SECURITYSETTLAGENTCONTACTNAME 180
    pub security_settl_agent_contact_phone : Option<String>, // FIELD_SECURITYSETTLAGENTCONTACTPHONE 181
    pub cash_settl_agent_name : Option<String>, // FIELD_CASHSETTLAGENTNAME 182
    pub cash_settl_agent_code : Option<String>, // FIELD_CASHSETTLAGENTCODE 183
    pub cash_settl_agent_acct_num : Option<String>, // FIELD_CASHSETTLAGENTACCTNUM 184
    pub cash_settl_agent_acct_name : Option<String>, // FIELD_CASHSETTLAGENTACCTNAME 185
    pub cash_settl_agent_contact_name : Option<String>, // FIELD_CASHSETTLAGENTCONTACTNAME 186
    pub cash_settl_agent_contact_phone : Option<String>, // FIELD_CASHSETTLAGENTCONTACTPHONE 187

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct MarketDataRequestFields {
    pub mdreq_id : String, // FIELD_MDREQID 262
    pub subscription_request_type : Field_SubscriptionRequestType_Enum, // FIELD_SUBSCRIPTIONREQUESTTYPE 263
    pub market_depth : i32, // FIELD_MARKETDEPTH 264
    pub mdupdate_type : Option<Field_MDUpdateType_Enum>, // FIELD_MDUPDATETYPE 265
    pub aggregated_book : Option<bool>, // FIELD_AGGREGATEDBOOK 266
    pub no_mdentry_types : Vec<NoMDEntryTypes12Fields>, // FIELD_NOMDENTRYTYPES 0
    pub no_related_sym : Vec<NoRelatedSym24Fields>, // FIELD_NORELATEDSYM 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct MarketDataSnapshotFullRefreshFields {
    pub mdreq_id : Option<String>, // FIELD_MDREQID 262
    pub symbol : String, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<Field_IDSource_Enum>, // FIELD_IDSOURCE 22
    pub security_type : Option<Field_SecurityType_Enum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<Field_PutOrCall_Enum>, // FIELD_PUTORCALL 201
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
    pub financial_status : Option<Field_FinancialStatus_Enum>, // FIELD_FINANCIALSTATUS 291
    pub corporate_action : Option<Field_CorporateAction_Enum>, // FIELD_CORPORATEACTION 292
    pub total_volume_traded : Option<f32>, // FIELD_TOTALVOLUMETRADED 387
    pub no_mdentries : Vec<NoMDEntries10Fields>, // FIELD_NOMDENTRIES 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct MarketDataIncrementalRefreshFields {
    pub mdreq_id : Option<String>, // FIELD_MDREQID 262
    pub no_mdentries : Vec<NoMDEntries11Fields>, // FIELD_NOMDENTRIES 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct MarketDataRequestRejectFields {
    pub mdreq_id : String, // FIELD_MDREQID 262
    pub mdreq_rej_reason : Option<Field_MDReqRejReason_Enum>, // FIELD_MDREQREJREASON 281
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct QuoteCancelFields {
    pub quote_req_id : Option<String>, // FIELD_QUOTEREQID 131
    pub quote_id : String, // FIELD_QUOTEID 117
    pub quote_cancel_type : Field_QuoteCancelType_Enum, // FIELD_QUOTECANCELTYPE 298
    pub quote_response_level : Option<Field_QuoteResponseLevel_Enum>, // FIELD_QUOTERESPONSELEVEL 301
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub no_quote_entries : Vec<NoQuoteEntries18Fields>, // FIELD_NOQUOTEENTRIES 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct QuoteStatusRequestFields {
    pub quote_id : Option<String>, // FIELD_QUOTEID 117
    pub symbol : String, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<Field_IDSource_Enum>, // FIELD_IDSOURCE 22
    pub security_type : Option<Field_SecurityType_Enum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<Field_PutOrCall_Enum>, // FIELD_PUTORCALL 201
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
    pub side : Option<Field_Side_Enum>, // FIELD_SIDE 54
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct QuoteAcknowledgementFields {
    pub quote_req_id : Option<String>, // FIELD_QUOTEREQID 131
    pub quote_id : Option<String>, // FIELD_QUOTEID 117
    pub quote_ack_status : Field_QuoteAckStatus_Enum, // FIELD_QUOTEACKSTATUS 297
    pub quote_reject_reason : Option<Field_QuoteRejectReason_Enum>, // FIELD_QUOTEREJECTREASON 300
    pub quote_response_level : Option<Field_QuoteResponseLevel_Enum>, // FIELD_QUOTERESPONSELEVEL 301
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub text : Option<String>, // FIELD_TEXT 58
    pub no_quote_sets : Option<Vec<NoQuoteSets21Fields>>, // FIELD_NOQUOTESETS 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct SecurityDefinitionRequestFields {
    pub security_req_id : String, // FIELD_SECURITYREQID 320
    pub security_request_type : Field_SecurityRequestType_Enum, // FIELD_SECURITYREQUESTTYPE 321
    pub symbol : Option<String>, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<Field_IDSource_Enum>, // FIELD_IDSOURCE 22
    pub security_type : Option<Field_SecurityType_Enum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<Field_PutOrCall_Enum>, // FIELD_PUTORCALL 201
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
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub no_related_sym : Option<Vec<NoRelatedSym23Fields>>, // FIELD_NORELATEDSYM 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct SecurityDefinitionFields {
    pub security_req_id : String, // FIELD_SECURITYREQID 320
    pub security_response_id : String, // FIELD_SECURITYRESPONSEID 322
    pub security_response_type : Option<Field_SecurityResponseType_Enum>, // FIELD_SECURITYRESPONSETYPE 323
    pub total_num_securities : i32, // FIELD_TOTALNUMSECURITIES 393
    pub symbol : Option<String>, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<Field_IDSource_Enum>, // FIELD_IDSOURCE 22
    pub security_type : Option<Field_SecurityType_Enum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<Field_PutOrCall_Enum>, // FIELD_PUTORCALL 201
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
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355
    pub no_related_sym : Option<Vec<NoRelatedSym23Fields>>, // FIELD_NORELATEDSYM 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct SecurityStatusRequestFields {
    pub security_status_req_id : String, // FIELD_SECURITYSTATUSREQID 324
    pub symbol : String, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<Field_IDSource_Enum>, // FIELD_IDSOURCE 22
    pub security_type : Option<Field_SecurityType_Enum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<Field_PutOrCall_Enum>, // FIELD_PUTORCALL 201
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
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub subscription_request_type : Field_SubscriptionRequestType_Enum, // FIELD_SUBSCRIPTIONREQUESTTYPE 263
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct SecurityStatusFields {
    pub security_status_req_id : Option<String>, // FIELD_SECURITYSTATUSREQID 324
    pub symbol : String, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<Field_IDSource_Enum>, // FIELD_IDSOURCE 22
    pub security_type : Option<Field_SecurityType_Enum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<Field_PutOrCall_Enum>, // FIELD_PUTORCALL 201
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
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub unsolicited_indicator : Option<bool>, // FIELD_UNSOLICITEDINDICATOR 325
    pub security_trading_status : Option<Field_SecurityTradingStatus_Enum>, // FIELD_SECURITYTRADINGSTATUS 326
    pub financial_status : Option<Field_FinancialStatus_Enum>, // FIELD_FINANCIALSTATUS 291
    pub corporate_action : Option<Field_CorporateAction_Enum>, // FIELD_CORPORATEACTION 292
    pub halt_reason_char : Option<Field_HaltReasonChar_Enum>, // FIELD_HALTREASONCHAR 327
    pub in_view_of_common : Option<bool>, // FIELD_INVIEWOFCOMMON 328
    pub due_to_related : Option<bool>, // FIELD_DUETORELATED 329
    pub buy_volume : Option<f32>, // FIELD_BUYVOLUME 330
    pub sell_volume : Option<f32>, // FIELD_SELLVOLUME 331
    pub high_px : Option<f32>, // FIELD_HIGHPX 332
    pub low_px : Option<f32>, // FIELD_LOWPX 333
    pub last_px : Option<f32>, // FIELD_LASTPX 31
    pub transact_time : Option<UtcDateTime>, // FIELD_TRANSACTTIME 60
    pub adjustment : Option<Field_Adjustment_Enum>, // FIELD_ADJUSTMENT 334

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct TradingSessionStatusRequestFields {
    pub trad_ses_req_id : String, // FIELD_TRADSESREQID 335
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub trad_ses_method : Option<Field_TradSesMethod_Enum>, // FIELD_TRADSESMETHOD 338
    pub trad_ses_mode : Option<Field_TradSesMode_Enum>, // FIELD_TRADSESMODE 339
    pub subscription_request_type : Field_SubscriptionRequestType_Enum, // FIELD_SUBSCRIPTIONREQUESTTYPE 263

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct TradingSessionStatusFields {
    pub trad_ses_req_id : Option<String>, // FIELD_TRADSESREQID 335
    pub trading_session_id : String, // FIELD_TRADINGSESSIONID 336
    pub trad_ses_method : Option<Field_TradSesMethod_Enum>, // FIELD_TRADSESMETHOD 338
    pub trad_ses_mode : Option<Field_TradSesMode_Enum>, // FIELD_TRADSESMODE 339
    pub unsolicited_indicator : Option<bool>, // FIELD_UNSOLICITEDINDICATOR 325
    pub trad_ses_status : Field_TradSesStatus_Enum, // FIELD_TRADSESSTATUS 340
    pub trad_ses_start_time : Option<UtcDateTime>, // FIELD_TRADSESSTARTTIME 341
    pub trad_ses_open_time : Option<UtcDateTime>, // FIELD_TRADSESOPENTIME 342
    pub trad_ses_pre_close_time : Option<UtcDateTime>, // FIELD_TRADSESPRECLOSETIME 343
    pub trad_ses_close_time : Option<UtcDateTime>, // FIELD_TRADSESCLOSETIME 344
    pub trad_ses_end_time : Option<UtcDateTime>, // FIELD_TRADSESENDTIME 345
    pub total_volume_traded : Option<f32>, // FIELD_TOTALVOLUMETRADED 387
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct MassQuoteFields {
    pub quote_req_id : Option<String>, // FIELD_QUOTEREQID 131
    pub quote_id : String, // FIELD_QUOTEID 117
    pub quote_response_level : Option<Field_QuoteResponseLevel_Enum>, // FIELD_QUOTERESPONSELEVEL 301
    pub def_bid_size : Option<f32>, // FIELD_DEFBIDSIZE 293
    pub def_offer_size : Option<f32>, // FIELD_DEFOFFERSIZE 294
    pub no_quote_sets : Vec<NoQuoteSets22Fields>, // FIELD_NOQUOTESETS 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct BusinessMessageRejectFields {
    pub ref_seq_num : Option<i32>, // FIELD_REFSEQNUM 45
    pub ref_msg_type : String, // FIELD_REFMSGTYPE 372
    pub business_reject_ref_id : Option<String>, // FIELD_BUSINESSREJECTREFID 379
    pub business_reject_reason : Field_BusinessRejectReason_Enum, // FIELD_BUSINESSREJECTREASON 380
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct BidRequestFields {
    pub bid_id : Option<String>, // FIELD_BIDID 390
    pub client_bid_id : String, // FIELD_CLIENTBIDID 391
    pub bid_request_trans_type : Field_BidRequestTransType_Enum, // FIELD_BIDREQUESTTRANSTYPE 374
    pub list_name : Option<String>, // FIELD_LISTNAME 392
    pub total_num_securities : i32, // FIELD_TOTALNUMSECURITIES 393
    pub bid_type : i32, // FIELD_BIDTYPE 394
    pub num_tickets : Option<i32>, // FIELD_NUMTICKETS 395
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub side_value1 : Option<f32>, // FIELD_SIDEVALUE1 396
    pub side_value2 : Option<f32>, // FIELD_SIDEVALUE2 397
    pub no_bid_descriptors : Option<Vec<NoBidDescriptors6Fields>>, // FIELD_NOBIDDESCRIPTORS 0
    pub no_bid_components : Option<Vec<NoBidComponents4Fields>>, // FIELD_NOBIDCOMPONENTS 0
    pub liquidity_ind_type : Option<Field_LiquidityIndType_Enum>, // FIELD_LIQUIDITYINDTYPE 409
    pub wt_average_liquidity : Option<f32>, // FIELD_WTAVERAGELIQUIDITY 410
    pub exchange_for_physical : Option<bool>, // FIELD_EXCHANGEFORPHYSICAL 411
    pub out_main_cntry_uindex : Option<f32>, // FIELD_OUTMAINCNTRYUINDEX 412
    pub cross_percent : Option<f32>, // FIELD_CROSSPERCENT 413
    pub prog_rpt_reqs : Option<Field_ProgRptReqs_Enum>, // FIELD_PROGRPTREQS 414
    pub prog_period_interval : Option<i32>, // FIELD_PROGPERIODINTERVAL 415
    pub inc_tax_ind : Option<Field_IncTaxInd_Enum>, // FIELD_INCTAXIND 416
    pub forex_req : Option<bool>, // FIELD_FOREXREQ 121
    pub num_bidders : Option<i32>, // FIELD_NUMBIDDERS 417
    pub trade_date : Option<UtcDateTime>, // FIELD_TRADEDATE 75
    pub trade_type : Field_TradeType_Enum, // FIELD_TRADETYPE 418
    pub basis_px_type : Field_BasisPxType_Enum, // FIELD_BASISPXTYPE 419
    pub strike_time : Option<UtcDateTime>, // FIELD_STRIKETIME 443
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct BidResponseFields {
    pub bid_id : Option<String>, // FIELD_BIDID 390
    pub client_bid_id : Option<String>, // FIELD_CLIENTBIDID 391
    pub no_bid_components : Vec<NoBidComponents5Fields>, // FIELD_NOBIDCOMPONENTS 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct ListStrikePriceFields {
    pub list_id : String, // FIELD_LISTID 66
    pub tot_no_strikes : i32, // FIELD_TOTNOSTRIKES 422
    pub no_strikes : Vec<NoStrikes28Fields>, // FIELD_NOSTRIKES 0

}




// components struct Fields





// groups struct Fields



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoAllocs3Fields {
    pub alloc_account : Option<String>, // FIELD_ALLOCACCOUNT 79
    pub alloc_shares : Option<f32>, // FIELD_ALLOCSHARES 80

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoMDEntries10Fields {
    pub mdentry_type : Field_MDEntryType_Enum, // FIELD_MDENTRYTYPE 269
    pub mdentry_px : f32, // FIELD_MDENTRYPX 270
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub mdentry_size : Option<f32>, // FIELD_MDENTRYSIZE 271
    pub mdentry_date : Option<UtcDate>, // FIELD_MDENTRYDATE 272
    pub mdentry_time : Option<UtcTime>, // FIELD_MDENTRYTIME 273
    pub tick_direction : Option<Field_TickDirection_Enum>, // FIELD_TICKDIRECTION 274
    pub mdmkt : Option<String>, // FIELD_MDMKT 275
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub quote_condition : Option<Field_QuoteCondition_Enum>, // FIELD_QUOTECONDITION 276
    pub trade_condition : Option<Field_TradeCondition_Enum>, // FIELD_TRADECONDITION 277
    pub mdentry_originator : Option<String>, // FIELD_MDENTRYORIGINATOR 282
    pub location_id : Option<String>, // FIELD_LOCATIONID 283
    pub desk_id : Option<String>, // FIELD_DESKID 284
    pub open_close_settle_flag : Option<Field_OpenCloseSettleFlag_Enum>, // FIELD_OPENCLOSESETTLEFLAG 286
    pub time_in_force : Option<Field_TimeInForce_Enum>, // FIELD_TIMEINFORCE 59
    pub expire_date : Option<UtcDateTime>, // FIELD_EXPIREDATE 432
    pub expire_time : Option<UtcDateTime>, // FIELD_EXPIRETIME 126
    pub min_qty : Option<f32>, // FIELD_MINQTY 110
    pub exec_inst : Option<Field_ExecInst_Enum>, // FIELD_EXECINST 18
    pub seller_days : Option<i32>, // FIELD_SELLERDAYS 287
    pub order_id : Option<String>, // FIELD_ORDERID 37
    pub quote_entry_id : Option<String>, // FIELD_QUOTEENTRYID 299
    pub mdentry_buyer : Option<String>, // FIELD_MDENTRYBUYER 288
    pub mdentry_seller : Option<String>, // FIELD_MDENTRYSELLER 289
    pub number_of_orders : Option<i32>, // FIELD_NUMBEROFORDERS 346
    pub mdentry_position_no : Option<i32>, // FIELD_MDENTRYPOSITIONNO 290
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoRelatedSym23Fields {
    pub underlying_symbol : Option<String>, // FIELD_UNDERLYINGSYMBOL 311
    pub underlying_symbol_sfx : Option<String>, // FIELD_UNDERLYINGSYMBOLSFX 312
    pub underlying_security_id : Option<String>, // FIELD_UNDERLYINGSECURITYID 309
    pub underlying_idsource : Option<String>, // FIELD_UNDERLYINGIDSOURCE 305
    pub underlying_security_type : Option<String>, // FIELD_UNDERLYINGSECURITYTYPE 310
    pub underlying_maturity_month_year : Option<UtcDate>, // FIELD_UNDERLYINGMATURITYMONTHYEAR 313
    pub underlying_maturity_day : Option<i32>, // FIELD_UNDERLYINGMATURITYDAY 314
    pub underlying_put_or_call : Option<i32>, // FIELD_UNDERLYINGPUTORCALL 315
    pub underlying_strike_price : Option<f32>, // FIELD_UNDERLYINGSTRIKEPRICE 316
    pub underlying_opt_attribute : Option<char>, // FIELD_UNDERLYINGOPTATTRIBUTE 317
    pub underlying_contract_multiplier : Option<f32>, // FIELD_UNDERLYINGCONTRACTMULTIPLIER 436
    pub underlying_coupon_rate : Option<f32>, // FIELD_UNDERLYINGCOUPONRATE 435
    pub underlying_security_exchange : Option<String>, // FIELD_UNDERLYINGSECURITYEXCHANGE 308
    pub underlying_issuer : Option<String>, // FIELD_UNDERLYINGISSUER 306
    pub encoded_underlying_issuer_len : Option<usize>, // FIELD_ENCODEDUNDERLYINGISSUERLEN 362
    pub encoded_underlying_issuer : Option<String>, // FIELD_ENCODEDUNDERLYINGISSUER 363
    pub underlying_security_desc : Option<String>, // FIELD_UNDERLYINGSECURITYDESC 307
    pub encoded_underlying_security_desc_len : Option<usize>, // FIELD_ENCODEDUNDERLYINGSECURITYDESCLEN 364
    pub encoded_underlying_security_desc : Option<String>, // FIELD_ENCODEDUNDERLYINGSECURITYDESC 365
    pub ratio_qty : Option<f32>, // FIELD_RATIOQTY 319
    pub side : Option<Field_Side_Enum>, // FIELD_SIDE 54
    pub underlying_currency : Option<f32>, // FIELD_UNDERLYINGCURRENCY 318

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoMDEntryTypes12Fields {
    pub mdentry_type : Field_MDEntryType_Enum, // FIELD_MDENTRYTYPE 269

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoQuoteEntries18Fields {
    pub symbol : String, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<Field_IDSource_Enum>, // FIELD_IDSOURCE 22
    pub security_type : Option<Field_SecurityType_Enum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<Field_PutOrCall_Enum>, // FIELD_PUTORCALL 201
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
    pub underlying_symbol : Option<String>, // FIELD_UNDERLYINGSYMBOL 311

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoQuoteEntries20Fields {
    pub quote_entry_id : String, // FIELD_QUOTEENTRYID 299
    pub symbol : Option<String>, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<Field_IDSource_Enum>, // FIELD_IDSOURCE 22
    pub security_type : Option<Field_SecurityType_Enum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<Field_PutOrCall_Enum>, // FIELD_PUTORCALL 201
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
    pub bid_px : Option<f32>, // FIELD_BIDPX 132
    pub offer_px : Option<f32>, // FIELD_OFFERPX 133
    pub bid_size : Option<f32>, // FIELD_BIDSIZE 134
    pub offer_size : Option<f32>, // FIELD_OFFERSIZE 135
    pub valid_until_time : Option<UtcDateTime>, // FIELD_VALIDUNTILTIME 62
    pub bid_spot_rate : Option<f32>, // FIELD_BIDSPOTRATE 188
    pub offer_spot_rate : Option<f32>, // FIELD_OFFERSPOTRATE 190
    pub bid_forward_points : Option<f32>, // FIELD_BIDFORWARDPOINTS 189
    pub offer_forward_points : Option<f32>, // FIELD_OFFERFORWARDPOINTS 191
    pub transact_time : Option<UtcDateTime>, // FIELD_TRANSACTTIME 60
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub fut_sett_date : Option<UtcDateTime>, // FIELD_FUTSETTDATE 64
    pub ord_type : Option<Field_OrdType_Enum>, // FIELD_ORDTYPE 40
    pub fut_sett_date2 : Option<UtcDateTime>, // FIELD_FUTSETTDATE2 193
    pub order_qty2 : Option<f32>, // FIELD_ORDERQTY2 192
    pub currency : Option<f32>, // FIELD_CURRENCY 15

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoMDEntries11Fields {
    pub mdupdate_action : Field_MDUpdateAction_Enum, // FIELD_MDUPDATEACTION 279
    pub delete_reason : Option<Field_DeleteReason_Enum>, // FIELD_DELETEREASON 285
    pub mdentry_type : Option<Field_MDEntryType_Enum>, // FIELD_MDENTRYTYPE 269
    pub mdentry_id : Option<String>, // FIELD_MDENTRYID 278
    pub mdentry_ref_id : Option<String>, // FIELD_MDENTRYREFID 280
    pub symbol : Option<String>, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<Field_IDSource_Enum>, // FIELD_IDSOURCE 22
    pub security_type : Option<Field_SecurityType_Enum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<Field_PutOrCall_Enum>, // FIELD_PUTORCALL 201
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
    pub financial_status : Option<Field_FinancialStatus_Enum>, // FIELD_FINANCIALSTATUS 291
    pub corporate_action : Option<Field_CorporateAction_Enum>, // FIELD_CORPORATEACTION 292
    pub mdentry_px : Option<f32>, // FIELD_MDENTRYPX 270
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub mdentry_size : Option<f32>, // FIELD_MDENTRYSIZE 271
    pub mdentry_date : Option<UtcDate>, // FIELD_MDENTRYDATE 272
    pub mdentry_time : Option<UtcTime>, // FIELD_MDENTRYTIME 273
    pub tick_direction : Option<Field_TickDirection_Enum>, // FIELD_TICKDIRECTION 274
    pub mdmkt : Option<String>, // FIELD_MDMKT 275
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub quote_condition : Option<Field_QuoteCondition_Enum>, // FIELD_QUOTECONDITION 276
    pub trade_condition : Option<Field_TradeCondition_Enum>, // FIELD_TRADECONDITION 277
    pub mdentry_originator : Option<String>, // FIELD_MDENTRYORIGINATOR 282
    pub location_id : Option<String>, // FIELD_LOCATIONID 283
    pub desk_id : Option<String>, // FIELD_DESKID 284
    pub open_close_settle_flag : Option<Field_OpenCloseSettleFlag_Enum>, // FIELD_OPENCLOSESETTLEFLAG 286
    pub time_in_force : Option<Field_TimeInForce_Enum>, // FIELD_TIMEINFORCE 59
    pub expire_date : Option<UtcDateTime>, // FIELD_EXPIREDATE 432
    pub expire_time : Option<UtcDateTime>, // FIELD_EXPIRETIME 126
    pub min_qty : Option<f32>, // FIELD_MINQTY 110
    pub exec_inst : Option<Field_ExecInst_Enum>, // FIELD_EXECINST 18
    pub seller_days : Option<i32>, // FIELD_SELLERDAYS 287
    pub order_id : Option<String>, // FIELD_ORDERID 37
    pub quote_entry_id : Option<String>, // FIELD_QUOTEENTRYID 299
    pub mdentry_buyer : Option<String>, // FIELD_MDENTRYBUYER 288
    pub mdentry_seller : Option<String>, // FIELD_MDENTRYSELLER 289
    pub number_of_orders : Option<i32>, // FIELD_NUMBEROFORDERS 346
    pub mdentry_position_no : Option<i32>, // FIELD_MDENTRYPOSITIONNO 290
    pub total_volume_traded : Option<f32>, // FIELD_TOTALVOLUMETRADED 387
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoBidComponents4Fields {
    pub list_id : Option<String>, // FIELD_LISTID 66
    pub side : Option<Field_Side_Enum>, // FIELD_SIDE 54
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub net_gross_ind : Option<Field_NetGrossInd_Enum>, // FIELD_NETGROSSIND 430
    pub settlmnt_typ : Option<Field_SettlmntTyp_Enum>, // FIELD_SETTLMNTTYP 63
    pub fut_sett_date : Option<UtcDateTime>, // FIELD_FUTSETTDATE 64
    pub account : Option<String>, // FIELD_ACCOUNT 1

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoExecs8Fields {
    pub last_shares : Option<f32>, // FIELD_LASTSHARES 32
    pub exec_id : Option<String>, // FIELD_EXECID 17
    pub last_px : Option<f32>, // FIELD_LASTPX 31
    pub last_capacity : Option<Field_LastCapacity_Enum>, // FIELD_LASTCAPACITY 29

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoContraBrokers7Fields {
    pub contra_broker : Option<String>, // FIELD_CONTRABROKER 375
    pub contra_trader : Option<String>, // FIELD_CONTRATRADER 337
    pub contra_trade_qty : Option<f32>, // FIELD_CONTRATRADEQTY 437
    pub contra_trade_time : Option<UtcDateTime>, // FIELD_CONTRATRADETIME 438

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoQuoteEntries19Fields {
    pub quote_entry_id : Option<String>, // FIELD_QUOTEENTRYID 299
    pub symbol : Option<String>, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<Field_IDSource_Enum>, // FIELD_IDSOURCE 22
    pub security_type : Option<Field_SecurityType_Enum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<Field_PutOrCall_Enum>, // FIELD_PUTORCALL 201
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
    pub quote_entry_reject_reason : Option<Field_QuoteEntryRejectReason_Enum>, // FIELD_QUOTEENTRYREJECTREASON 368

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoTradingSessions29Fields {
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoIOIQualifiers9Fields {
    pub ioiqualifier : Option<Field_IOIQualifier_Enum>, // FIELD_IOIQUALIFIER 104

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoMsgTypes14Fields {
    pub ref_msg_type : Option<String>, // FIELD_REFMSGTYPE 372
    pub msg_direction : Option<Field_MsgDirection_Enum>, // FIELD_MSGDIRECTION 385

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct LinesOfText1Fields {
    pub text : String, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoAllocs2Fields {
    pub alloc_account : Option<String>, // FIELD_ALLOCACCOUNT 79
    pub alloc_price : Option<f32>, // FIELD_ALLOCPRICE 366
    pub alloc_shares : f32, // FIELD_ALLOCSHARES 80
    pub process_code : Option<Field_ProcessCode_Enum>, // FIELD_PROCESSCODE 81
    pub broker_of_credit : Option<String>, // FIELD_BROKEROFCREDIT 92
    pub notify_broker_of_credit : Option<bool>, // FIELD_NOTIFYBROKEROFCREDIT 208
    pub alloc_handl_inst : Option<Field_AllocHandlInst_Enum>, // FIELD_ALLOCHANDLINST 209
    pub alloc_text : Option<String>, // FIELD_ALLOCTEXT 161
    pub encoded_alloc_text_len : Option<usize>, // FIELD_ENCODEDALLOCTEXTLEN 360
    pub encoded_alloc_text : Option<String>, // FIELD_ENCODEDALLOCTEXT 361
    pub exec_broker : Option<String>, // FIELD_EXECBROKER 76
    pub client_id : Option<String>, // FIELD_CLIENTID 109
    pub commission : Option<f32>, // FIELD_COMMISSION 12
    pub comm_type : Option<Field_CommType_Enum>, // FIELD_COMMTYPE 13
    pub alloc_avg_px : Option<f32>, // FIELD_ALLOCAVGPX 153
    pub alloc_net_money : Option<f32>, // FIELD_ALLOCNETMONEY 154
    pub settl_curr_amt : Option<f32>, // FIELD_SETTLCURRAMT 119
    pub settl_currency : Option<f32>, // FIELD_SETTLCURRENCY 120
    pub settl_curr_fx_rate : Option<f32>, // FIELD_SETTLCURRFXRATE 155
    pub settl_curr_fx_rate_calc : Option<Field_SettlCurrFxRateCalc_Enum>, // FIELD_SETTLCURRFXRATECALC 156
    pub accrued_interest_amt : Option<f32>, // FIELD_ACCRUEDINTERESTAMT 159
    pub settl_inst_mode : Option<Field_SettlInstMode_Enum>, // FIELD_SETTLINSTMODE 160
    pub no_misc_fees : Option<Vec<NoMiscFees13Fields>>, // FIELD_NOMISCFEES 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoOrders17Fields {
    pub cl_ord_id : String, // FIELD_CLORDID 11
    pub cum_qty : f32, // FIELD_CUMQTY 14
    pub ord_status : Field_OrdStatus_Enum, // FIELD_ORDSTATUS 39
    pub leaves_qty : f32, // FIELD_LEAVESQTY 151
    pub cxl_qty : f32, // FIELD_CXLQTY 84
    pub avg_px : f32, // FIELD_AVGPX 6
    pub ord_rej_reason : Option<Field_OrdRejReason_Enum>, // FIELD_ORDREJREASON 103
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoStrikes28Fields {
    pub symbol : String, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<Field_IDSource_Enum>, // FIELD_IDSOURCE 22
    pub security_type : Option<Field_SecurityType_Enum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<Field_PutOrCall_Enum>, // FIELD_PUTORCALL 201
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
    pub cl_ord_id : Option<String>, // FIELD_CLORDID 11
    pub side : Option<Field_Side_Enum>, // FIELD_SIDE 54
    pub price : f32, // FIELD_PRICE 44
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoOrders16Fields {
    pub cl_ord_id : Option<String>, // FIELD_CLORDID 11
    pub order_id : Option<String>, // FIELD_ORDERID 37
    pub secondary_order_id : Option<String>, // FIELD_SECONDARYORDERID 198
    pub list_id : Option<String>, // FIELD_LISTID 66
    pub wave_no : Option<String>, // FIELD_WAVENO 105

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoRelatedSym25Fields {
    pub symbol : String, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<Field_IDSource_Enum>, // FIELD_IDSOURCE 22
    pub security_type : Option<Field_SecurityType_Enum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<Field_PutOrCall_Enum>, // FIELD_PUTORCALL 201
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
    pub quote_request_type : Option<Field_QuoteRequestType_Enum>, // FIELD_QUOTEREQUESTTYPE 303
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub side : Option<Field_Side_Enum>, // FIELD_SIDE 54
    pub order_qty : Option<f32>, // FIELD_ORDERQTY 38
    pub fut_sett_date : Option<UtcDateTime>, // FIELD_FUTSETTDATE 64
    pub ord_type : Option<Field_OrdType_Enum>, // FIELD_ORDTYPE 40
    pub fut_sett_date2 : Option<UtcDateTime>, // FIELD_FUTSETTDATE2 193
    pub order_qty2 : Option<f32>, // FIELD_ORDERQTY2 192
    pub expire_time : Option<UtcDateTime>, // FIELD_EXPIRETIME 126
    pub transact_time : Option<UtcDateTime>, // FIELD_TRANSACTTIME 60
    pub currency : Option<f32>, // FIELD_CURRENCY 15

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoQuoteSets22Fields {
    pub quote_set_id : String, // FIELD_QUOTESETID 302
    pub underlying_symbol : String, // FIELD_UNDERLYINGSYMBOL 311
    pub underlying_symbol_sfx : Option<String>, // FIELD_UNDERLYINGSYMBOLSFX 312
    pub underlying_security_id : Option<String>, // FIELD_UNDERLYINGSECURITYID 309
    pub underlying_idsource : Option<String>, // FIELD_UNDERLYINGIDSOURCE 305
    pub underlying_security_type : Option<String>, // FIELD_UNDERLYINGSECURITYTYPE 310
    pub underlying_maturity_month_year : Option<UtcDate>, // FIELD_UNDERLYINGMATURITYMONTHYEAR 313
    pub underlying_maturity_day : Option<i32>, // FIELD_UNDERLYINGMATURITYDAY 314
    pub underlying_put_or_call : Option<i32>, // FIELD_UNDERLYINGPUTORCALL 315
    pub underlying_strike_price : Option<f32>, // FIELD_UNDERLYINGSTRIKEPRICE 316
    pub underlying_opt_attribute : Option<char>, // FIELD_UNDERLYINGOPTATTRIBUTE 317
    pub underlying_contract_multiplier : Option<f32>, // FIELD_UNDERLYINGCONTRACTMULTIPLIER 436
    pub underlying_coupon_rate : Option<f32>, // FIELD_UNDERLYINGCOUPONRATE 435
    pub underlying_security_exchange : Option<String>, // FIELD_UNDERLYINGSECURITYEXCHANGE 308
    pub underlying_issuer : Option<String>, // FIELD_UNDERLYINGISSUER 306
    pub encoded_underlying_issuer_len : Option<usize>, // FIELD_ENCODEDUNDERLYINGISSUERLEN 362
    pub encoded_underlying_issuer : Option<String>, // FIELD_ENCODEDUNDERLYINGISSUER 363
    pub underlying_security_desc : Option<String>, // FIELD_UNDERLYINGSECURITYDESC 307
    pub encoded_underlying_security_desc_len : Option<usize>, // FIELD_ENCODEDUNDERLYINGSECURITYDESCLEN 364
    pub encoded_underlying_security_desc : Option<String>, // FIELD_ENCODEDUNDERLYINGSECURITYDESC 365
    pub quote_set_valid_until_time : Option<UtcDateTime>, // FIELD_QUOTESETVALIDUNTILTIME 367
    pub tot_quote_entries : i32, // FIELD_TOTQUOTEENTRIES 304
    pub no_quote_entries : Vec<NoQuoteEntries20Fields>, // FIELD_NOQUOTEENTRIES 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoRelatedSym24Fields {
    pub symbol : String, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<Field_IDSource_Enum>, // FIELD_IDSOURCE 22
    pub security_type : Option<Field_SecurityType_Enum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<Field_PutOrCall_Enum>, // FIELD_PUTORCALL 201
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
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoBidComponents5Fields {
    pub commission : f32, // FIELD_COMMISSION 12
    pub comm_type : Field_CommType_Enum, // FIELD_COMMTYPE 13
    pub list_id : Option<String>, // FIELD_LISTID 66
    pub country : Option<String>, // FIELD_COUNTRY 421
    pub side : Option<Field_Side_Enum>, // FIELD_SIDE 54
    pub price : Option<f32>, // FIELD_PRICE 44
    pub price_type : Option<Field_PriceType_Enum>, // FIELD_PRICETYPE 423
    pub fair_value : Option<f32>, // FIELD_FAIRVALUE 406
    pub net_gross_ind : Option<Field_NetGrossInd_Enum>, // FIELD_NETGROSSIND 430
    pub settlmnt_typ : Option<Field_SettlmntTyp_Enum>, // FIELD_SETTLMNTTYP 63
    pub fut_sett_date : Option<UtcDateTime>, // FIELD_FUTSETTDATE 64
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoOrders15Fields {
    pub cl_ord_id : String, // FIELD_CLORDID 11
    pub list_seq_no : i32, // FIELD_LISTSEQNO 67
    pub settl_inst_mode : Option<Field_SettlInstMode_Enum>, // FIELD_SETTLINSTMODE 160
    pub client_id : Option<String>, // FIELD_CLIENTID 109
    pub exec_broker : Option<String>, // FIELD_EXECBROKER 76
    pub account : Option<String>, // FIELD_ACCOUNT 1
    pub no_allocs : Option<Vec<NoAllocs3Fields>>, // FIELD_NOALLOCS 0
    pub settlmnt_typ : Option<Field_SettlmntTyp_Enum>, // FIELD_SETTLMNTTYP 63
    pub fut_sett_date : Option<UtcDateTime>, // FIELD_FUTSETTDATE 64
    pub handl_inst : Option<Field_HandlInst_Enum>, // FIELD_HANDLINST 21
    pub exec_inst : Option<Field_ExecInst_Enum>, // FIELD_EXECINST 18
    pub min_qty : Option<f32>, // FIELD_MINQTY 110
    pub max_floor : Option<f32>, // FIELD_MAXFLOOR 111
    pub ex_destination : Option<String>, // FIELD_EXDESTINATION 100
    pub no_trading_sessions : Option<Vec<NoTradingSessions29Fields>>, // FIELD_NOTRADINGSESSIONS 0
    pub process_code : Option<Field_ProcessCode_Enum>, // FIELD_PROCESSCODE 81
    pub symbol : String, // FIELD_SYMBOL 55
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<Field_IDSource_Enum>, // FIELD_IDSOURCE 22
    pub security_type : Option<Field_SecurityType_Enum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<Field_PutOrCall_Enum>, // FIELD_PUTORCALL 201
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
    pub side : Field_Side_Enum, // FIELD_SIDE 54
    pub side_value_ind : Option<i32>, // FIELD_SIDEVALUEIND 401
    pub locate_reqd : Option<bool>, // FIELD_LOCATEREQD 114
    pub transact_time : Option<UtcDateTime>, // FIELD_TRANSACTTIME 60
    pub order_qty : Option<f32>, // FIELD_ORDERQTY 38
    pub cash_order_qty : Option<f32>, // FIELD_CASHORDERQTY 152
    pub ord_type : Option<Field_OrdType_Enum>, // FIELD_ORDTYPE 40
    pub price : Option<f32>, // FIELD_PRICE 44
    pub stop_px : Option<f32>, // FIELD_STOPPX 99
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub compliance_id : Option<String>, // FIELD_COMPLIANCEID 376
    pub solicited_flag : Option<bool>, // FIELD_SOLICITEDFLAG 377
    pub ioiid : Option<String>, // FIELD_IOIID 23
    pub quote_id : Option<String>, // FIELD_QUOTEID 117
    pub time_in_force : Option<Field_TimeInForce_Enum>, // FIELD_TIMEINFORCE 59
    pub effective_time : Option<UtcDateTime>, // FIELD_EFFECTIVETIME 168
    pub expire_date : Option<UtcDateTime>, // FIELD_EXPIREDATE 432
    pub expire_time : Option<UtcDateTime>, // FIELD_EXPIRETIME 126
    pub gtbooking_inst : Option<Field_GTBookingInst_Enum>, // FIELD_GTBOOKINGINST 427
    pub commission : Option<f32>, // FIELD_COMMISSION 12
    pub comm_type : Option<Field_CommType_Enum>, // FIELD_COMMTYPE 13
    pub rule80_a : Option<Field_Rule80A_Enum>, // FIELD_RULE80A 47
    pub forex_req : Option<bool>, // FIELD_FOREXREQ 121
    pub settl_currency : Option<f32>, // FIELD_SETTLCURRENCY 120
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355
    pub fut_sett_date2 : Option<UtcDateTime>, // FIELD_FUTSETTDATE2 193
    pub order_qty2 : Option<f32>, // FIELD_ORDERQTY2 192
    pub open_close : Option<Field_OpenClose_Enum>, // FIELD_OPENCLOSE 77
    pub covered_or_uncovered : Option<Field_CoveredOrUncovered_Enum>, // FIELD_COVEREDORUNCOVERED 203
    pub customer_or_firm : Option<Field_CustomerOrFirm_Enum>, // FIELD_CUSTOMERORFIRM 204
    pub max_show : Option<f32>, // FIELD_MAXSHOW 210
    pub peg_difference : Option<f32>, // FIELD_PEGDIFFERENCE 211
    pub discretion_inst : Option<Field_DiscretionInst_Enum>, // FIELD_DISCRETIONINST 388
    pub discretion_offset : Option<f32>, // FIELD_DISCRETIONOFFSET 389
    pub clearing_firm : Option<String>, // FIELD_CLEARINGFIRM 439
    pub clearing_account : Option<String>, // FIELD_CLEARINGACCOUNT 440

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoRelatedSym26Fields {
    pub relatd_sym : Option<String>, // FIELD_RELATDSYM 46
    pub symbol_sfx : Option<String>, // FIELD_SYMBOLSFX 65
    pub security_id : Option<String>, // FIELD_SECURITYID 48
    pub idsource : Option<Field_IDSource_Enum>, // FIELD_IDSOURCE 22
    pub security_type : Option<Field_SecurityType_Enum>, // FIELD_SECURITYTYPE 167
    pub maturity_month_year : Option<UtcDate>, // FIELD_MATURITYMONTHYEAR 200
    pub maturity_day : Option<i32>, // FIELD_MATURITYDAY 205
    pub put_or_call : Option<Field_PutOrCall_Enum>, // FIELD_PUTORCALL 201
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

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoBidDescriptors6Fields {
    pub bid_descriptor_type : Option<i32>, // FIELD_BIDDESCRIPTORTYPE 399
    pub bid_descriptor : Option<String>, // FIELD_BIDDESCRIPTOR 400
    pub side_value_ind : Option<i32>, // FIELD_SIDEVALUEIND 401
    pub liquidity_value : Option<f32>, // FIELD_LIQUIDITYVALUE 404
    pub liquidity_num_securities : Option<i32>, // FIELD_LIQUIDITYNUMSECURITIES 441
    pub liquidity_pct_low : Option<f32>, // FIELD_LIQUIDITYPCTLOW 402
    pub liquidity_pct_high : Option<f32>, // FIELD_LIQUIDITYPCTHIGH 403
    pub efptracking_error : Option<f32>, // FIELD_EFPTRACKINGERROR 405
    pub fair_value : Option<f32>, // FIELD_FAIRVALUE 406
    pub outside_index_pct : Option<f32>, // FIELD_OUTSIDEINDEXPCT 407
    pub value_of_futures : Option<f32>, // FIELD_VALUEOFFUTURES 408

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoQuoteSets21Fields {
    pub quote_set_id : Option<String>, // FIELD_QUOTESETID 302
    pub underlying_symbol : Option<String>, // FIELD_UNDERLYINGSYMBOL 311
    pub underlying_symbol_sfx : Option<String>, // FIELD_UNDERLYINGSYMBOLSFX 312
    pub underlying_security_id : Option<String>, // FIELD_UNDERLYINGSECURITYID 309
    pub underlying_idsource : Option<String>, // FIELD_UNDERLYINGIDSOURCE 305
    pub underlying_security_type : Option<String>, // FIELD_UNDERLYINGSECURITYTYPE 310
    pub underlying_maturity_month_year : Option<UtcDate>, // FIELD_UNDERLYINGMATURITYMONTHYEAR 313
    pub underlying_maturity_day : Option<i32>, // FIELD_UNDERLYINGMATURITYDAY 314
    pub underlying_put_or_call : Option<i32>, // FIELD_UNDERLYINGPUTORCALL 315
    pub underlying_strike_price : Option<f32>, // FIELD_UNDERLYINGSTRIKEPRICE 316
    pub underlying_opt_attribute : Option<char>, // FIELD_UNDERLYINGOPTATTRIBUTE 317
    pub underlying_contract_multiplier : Option<f32>, // FIELD_UNDERLYINGCONTRACTMULTIPLIER 436
    pub underlying_coupon_rate : Option<f32>, // FIELD_UNDERLYINGCOUPONRATE 435
    pub underlying_security_exchange : Option<String>, // FIELD_UNDERLYINGSECURITYEXCHANGE 308
    pub underlying_issuer : Option<String>, // FIELD_UNDERLYINGISSUER 306
    pub encoded_underlying_issuer_len : Option<usize>, // FIELD_ENCODEDUNDERLYINGISSUERLEN 362
    pub encoded_underlying_issuer : Option<String>, // FIELD_ENCODEDUNDERLYINGISSUER 363
    pub underlying_security_desc : Option<String>, // FIELD_UNDERLYINGSECURITYDESC 307
    pub encoded_underlying_security_desc_len : Option<usize>, // FIELD_ENCODEDUNDERLYINGSECURITYDESCLEN 364
    pub encoded_underlying_security_desc : Option<String>, // FIELD_ENCODEDUNDERLYINGSECURITYDESC 365
    pub tot_quote_entries : Option<i32>, // FIELD_TOTQUOTEENTRIES 304
    pub no_quote_entries : Option<Vec<NoQuoteEntries19Fields>>, // FIELD_NOQUOTEENTRIES 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoRoutingIDs27Fields {
    pub routing_type : Option<Field_RoutingType_Enum>, // FIELD_ROUTINGTYPE 216
    pub routing_id : Option<String>, // FIELD_ROUTINGID 217

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoMiscFees13Fields {
    pub misc_fee_amt : Option<f32>, // FIELD_MISCFEEAMT 137
    pub misc_fee_curr : Option<f32>, // FIELD_MISCFEECURR 138
    pub misc_fee_type : Option<Field_MiscFeeType_Enum>, // FIELD_MISCFEETYPE 139

}








// Fields Constants / enums

const FIELD_ACCOUNT : u32 = 1; // STRING

const FIELD_ADVID : u32 = 2; // STRING

const FIELD_ADVREFID : u32 = 3; // STRING

const FIELD_ADVSIDE : u32 = 4; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_AdvSide_Enum {
    BUY, // = "B"
    SELL, // = "S"
    TRADE, // = "T"
    CROSS, // = "X"

    Undefined
}

impl FromStr for Field_AdvSide_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "B" => Ok(Field_AdvSide_Enum::BUY),
            "S" => Ok(Field_AdvSide_Enum::SELL),
            "T" => Ok(Field_AdvSide_Enum::TRADE),
            "X" => Ok(Field_AdvSide_Enum::CROSS),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_AdvSide_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_AdvSide_Enum::BUY => {
                f.write_str( "B" )
            },
            &Field_AdvSide_Enum::SELL => {
                f.write_str( "S" )
            },
            &Field_AdvSide_Enum::TRADE => {
                f.write_str( "T" )
            },
            &Field_AdvSide_Enum::CROSS => {
                f.write_str( "X" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_AdvSide_Enum {
    fn default() -> Self {
        Field_AdvSide_Enum::Undefined
    }
}
const FIELD_ADVTRANSTYPE : u32 = 5; // STRING
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_AdvTransType_Enum {
    CANCEL, // = "C"
    NEW, // = "N"
    REPLACE, // = "R"

    Undefined
}

impl FromStr for Field_AdvTransType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(Field_AdvTransType_Enum::CANCEL),
            "N" => Ok(Field_AdvTransType_Enum::NEW),
            "R" => Ok(Field_AdvTransType_Enum::REPLACE),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_AdvTransType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_AdvTransType_Enum::CANCEL => {
                f.write_str( "C" )
            },
            &Field_AdvTransType_Enum::NEW => {
                f.write_str( "N" )
            },
            &Field_AdvTransType_Enum::REPLACE => {
                f.write_str( "R" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_AdvTransType_Enum {
    fn default() -> Self {
        Field_AdvTransType_Enum::Undefined
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_CommType_Enum {
    PER_SHARE, // = "1"
    PERCENTAGE, // = "2"
    ABSOLUTE, // = "3"

    Undefined
}

impl FromStr for Field_CommType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_CommType_Enum::PER_SHARE),
            "2" => Ok(Field_CommType_Enum::PERCENTAGE),
            "3" => Ok(Field_CommType_Enum::ABSOLUTE),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_CommType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_CommType_Enum::PER_SHARE => {
                f.write_str( "1" )
            },
            &Field_CommType_Enum::PERCENTAGE => {
                f.write_str( "2" )
            },
            &Field_CommType_Enum::ABSOLUTE => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_CommType_Enum {
    fn default() -> Self {
        Field_CommType_Enum::Undefined
    }
}
const FIELD_CUMQTY : u32 = 14; // QTY

const FIELD_CURRENCY : u32 = 15; // CURRENCY

const FIELD_ENDSEQNO : u32 = 16; // INT

const FIELD_EXECID : u32 = 17; // STRING

const FIELD_EXECINST : u32 = 18; // MULTIPLEVALUESTRING
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_ExecInst_Enum {
    STAY_ON_OFFERSIDE, // = "0"
    NOT_HELD, // = "1"
    WORK, // = "2"
    GO_ALONG, // = "3"
    OVER_THE_DAY, // = "4"
    HELD, // = "5"
    PARTICIPATE_DONT_INITIATE, // = "6"
    STRICT_SCALE, // = "7"
    TRY_TO_SCALE, // = "8"
    STAY_ON_BIDSIDE, // = "9"
    NO_CROSS, // = "A"
    OK_TO_CROSS, // = "B"
    CALL_FIRST, // = "C"
    PERCENT_OF_VOLUME, // = "D"
    DO_NOT_INCREASE, // = "E"
    DO_NOT_REDUCE, // = "F"
    ALL_OR_NONE, // = "G"
    INSTITUTIONS_ONLY, // = "I"
    LAST_PEG, // = "L"
    MID_PRICE_PEG, // = "M"
    NON_NEGOTIABLE, // = "N"
    OPENING_PEG, // = "O"
    MARKET_PEG, // = "P"
    PRIMARY_PEG, // = "R"
    SUSPEND, // = "S"
    FIXED_PEG_TO_LOCAL_BEST_BID_OR_OFFER_AT_TIME_OF_ORDER, // = "T"
    CUSTOMER_DISPLAY_INSTRUCTION, // = "U"
    NETTING, // = "V"
    PEG_TO_VWAP, // = "W"

    Undefined
}

impl FromStr for Field_ExecInst_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_ExecInst_Enum::STAY_ON_OFFERSIDE),
            "1" => Ok(Field_ExecInst_Enum::NOT_HELD),
            "2" => Ok(Field_ExecInst_Enum::WORK),
            "3" => Ok(Field_ExecInst_Enum::GO_ALONG),
            "4" => Ok(Field_ExecInst_Enum::OVER_THE_DAY),
            "5" => Ok(Field_ExecInst_Enum::HELD),
            "6" => Ok(Field_ExecInst_Enum::PARTICIPATE_DONT_INITIATE),
            "7" => Ok(Field_ExecInst_Enum::STRICT_SCALE),
            "8" => Ok(Field_ExecInst_Enum::TRY_TO_SCALE),
            "9" => Ok(Field_ExecInst_Enum::STAY_ON_BIDSIDE),
            "A" => Ok(Field_ExecInst_Enum::NO_CROSS),
            "B" => Ok(Field_ExecInst_Enum::OK_TO_CROSS),
            "C" => Ok(Field_ExecInst_Enum::CALL_FIRST),
            "D" => Ok(Field_ExecInst_Enum::PERCENT_OF_VOLUME),
            "E" => Ok(Field_ExecInst_Enum::DO_NOT_INCREASE),
            "F" => Ok(Field_ExecInst_Enum::DO_NOT_REDUCE),
            "G" => Ok(Field_ExecInst_Enum::ALL_OR_NONE),
            "I" => Ok(Field_ExecInst_Enum::INSTITUTIONS_ONLY),
            "L" => Ok(Field_ExecInst_Enum::LAST_PEG),
            "M" => Ok(Field_ExecInst_Enum::MID_PRICE_PEG),
            "N" => Ok(Field_ExecInst_Enum::NON_NEGOTIABLE),
            "O" => Ok(Field_ExecInst_Enum::OPENING_PEG),
            "P" => Ok(Field_ExecInst_Enum::MARKET_PEG),
            "R" => Ok(Field_ExecInst_Enum::PRIMARY_PEG),
            "S" => Ok(Field_ExecInst_Enum::SUSPEND),
            "T" => Ok(Field_ExecInst_Enum::FIXED_PEG_TO_LOCAL_BEST_BID_OR_OFFER_AT_TIME_OF_ORDER),
            "U" => Ok(Field_ExecInst_Enum::CUSTOMER_DISPLAY_INSTRUCTION),
            "V" => Ok(Field_ExecInst_Enum::NETTING),
            "W" => Ok(Field_ExecInst_Enum::PEG_TO_VWAP),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_ExecInst_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_ExecInst_Enum::STAY_ON_OFFERSIDE => {
                f.write_str( "0" )
            },
            &Field_ExecInst_Enum::NOT_HELD => {
                f.write_str( "1" )
            },
            &Field_ExecInst_Enum::WORK => {
                f.write_str( "2" )
            },
            &Field_ExecInst_Enum::GO_ALONG => {
                f.write_str( "3" )
            },
            &Field_ExecInst_Enum::OVER_THE_DAY => {
                f.write_str( "4" )
            },
            &Field_ExecInst_Enum::HELD => {
                f.write_str( "5" )
            },
            &Field_ExecInst_Enum::PARTICIPATE_DONT_INITIATE => {
                f.write_str( "6" )
            },
            &Field_ExecInst_Enum::STRICT_SCALE => {
                f.write_str( "7" )
            },
            &Field_ExecInst_Enum::TRY_TO_SCALE => {
                f.write_str( "8" )
            },
            &Field_ExecInst_Enum::STAY_ON_BIDSIDE => {
                f.write_str( "9" )
            },
            &Field_ExecInst_Enum::NO_CROSS => {
                f.write_str( "A" )
            },
            &Field_ExecInst_Enum::OK_TO_CROSS => {
                f.write_str( "B" )
            },
            &Field_ExecInst_Enum::CALL_FIRST => {
                f.write_str( "C" )
            },
            &Field_ExecInst_Enum::PERCENT_OF_VOLUME => {
                f.write_str( "D" )
            },
            &Field_ExecInst_Enum::DO_NOT_INCREASE => {
                f.write_str( "E" )
            },
            &Field_ExecInst_Enum::DO_NOT_REDUCE => {
                f.write_str( "F" )
            },
            &Field_ExecInst_Enum::ALL_OR_NONE => {
                f.write_str( "G" )
            },
            &Field_ExecInst_Enum::INSTITUTIONS_ONLY => {
                f.write_str( "I" )
            },
            &Field_ExecInst_Enum::LAST_PEG => {
                f.write_str( "L" )
            },
            &Field_ExecInst_Enum::MID_PRICE_PEG => {
                f.write_str( "M" )
            },
            &Field_ExecInst_Enum::NON_NEGOTIABLE => {
                f.write_str( "N" )
            },
            &Field_ExecInst_Enum::OPENING_PEG => {
                f.write_str( "O" )
            },
            &Field_ExecInst_Enum::MARKET_PEG => {
                f.write_str( "P" )
            },
            &Field_ExecInst_Enum::PRIMARY_PEG => {
                f.write_str( "R" )
            },
            &Field_ExecInst_Enum::SUSPEND => {
                f.write_str( "S" )
            },
            &Field_ExecInst_Enum::FIXED_PEG_TO_LOCAL_BEST_BID_OR_OFFER_AT_TIME_OF_ORDER => {
                f.write_str( "T" )
            },
            &Field_ExecInst_Enum::CUSTOMER_DISPLAY_INSTRUCTION => {
                f.write_str( "U" )
            },
            &Field_ExecInst_Enum::NETTING => {
                f.write_str( "V" )
            },
            &Field_ExecInst_Enum::PEG_TO_VWAP => {
                f.write_str( "W" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_ExecInst_Enum {
    fn default() -> Self {
        Field_ExecInst_Enum::Undefined
    }
}
const FIELD_EXECREFID : u32 = 19; // STRING

const FIELD_EXECTRANSTYPE : u32 = 20; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_ExecTransType_Enum {
    NEW, // = "0"
    CANCEL, // = "1"
    CORRECT, // = "2"
    STATUS, // = "3"

    Undefined
}

impl FromStr for Field_ExecTransType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_ExecTransType_Enum::NEW),
            "1" => Ok(Field_ExecTransType_Enum::CANCEL),
            "2" => Ok(Field_ExecTransType_Enum::CORRECT),
            "3" => Ok(Field_ExecTransType_Enum::STATUS),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_ExecTransType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_ExecTransType_Enum::NEW => {
                f.write_str( "0" )
            },
            &Field_ExecTransType_Enum::CANCEL => {
                f.write_str( "1" )
            },
            &Field_ExecTransType_Enum::CORRECT => {
                f.write_str( "2" )
            },
            &Field_ExecTransType_Enum::STATUS => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_ExecTransType_Enum {
    fn default() -> Self {
        Field_ExecTransType_Enum::Undefined
    }
}
const FIELD_HANDLINST : u32 = 21; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_HandlInst_Enum {
    AUTOMATED_EXECUTION_ORDER_PRIVATE_NO_BROKER_INTERVENTION, // = "1"
    AUTOMATED_EXECUTION_ORDER_PUBLIC_BROKER_INTERVENTION_OK, // = "2"
    MANUAL_ORDER_BEST_EXECUTION, // = "3"

    Undefined
}

impl FromStr for Field_HandlInst_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_HandlInst_Enum::AUTOMATED_EXECUTION_ORDER_PRIVATE_NO_BROKER_INTERVENTION),
            "2" => Ok(Field_HandlInst_Enum::AUTOMATED_EXECUTION_ORDER_PUBLIC_BROKER_INTERVENTION_OK),
            "3" => Ok(Field_HandlInst_Enum::MANUAL_ORDER_BEST_EXECUTION),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_HandlInst_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_HandlInst_Enum::AUTOMATED_EXECUTION_ORDER_PRIVATE_NO_BROKER_INTERVENTION => {
                f.write_str( "1" )
            },
            &Field_HandlInst_Enum::AUTOMATED_EXECUTION_ORDER_PUBLIC_BROKER_INTERVENTION_OK => {
                f.write_str( "2" )
            },
            &Field_HandlInst_Enum::MANUAL_ORDER_BEST_EXECUTION => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_HandlInst_Enum {
    fn default() -> Self {
        Field_HandlInst_Enum::Undefined
    }
}
const FIELD_IDSOURCE : u32 = 22; // STRING
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_IDSource_Enum {
    CUSIP, // = "1"
    SEDOL, // = "2"
    QUIK, // = "3"
    ISIN_NUMBER, // = "4"
    RIC_CODE, // = "5"
    ISO_CURRENCY_CODE, // = "6"
    ISO_COUNTRY_CODE, // = "7"
    EXCHANGE_SYMBOL, // = "8"
    CONSOLIDATED_TAPE_ASSOCIATION, // = "9"

    Undefined
}

impl FromStr for Field_IDSource_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_IDSource_Enum::CUSIP),
            "2" => Ok(Field_IDSource_Enum::SEDOL),
            "3" => Ok(Field_IDSource_Enum::QUIK),
            "4" => Ok(Field_IDSource_Enum::ISIN_NUMBER),
            "5" => Ok(Field_IDSource_Enum::RIC_CODE),
            "6" => Ok(Field_IDSource_Enum::ISO_CURRENCY_CODE),
            "7" => Ok(Field_IDSource_Enum::ISO_COUNTRY_CODE),
            "8" => Ok(Field_IDSource_Enum::EXCHANGE_SYMBOL),
            "9" => Ok(Field_IDSource_Enum::CONSOLIDATED_TAPE_ASSOCIATION),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_IDSource_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_IDSource_Enum::CUSIP => {
                f.write_str( "1" )
            },
            &Field_IDSource_Enum::SEDOL => {
                f.write_str( "2" )
            },
            &Field_IDSource_Enum::QUIK => {
                f.write_str( "3" )
            },
            &Field_IDSource_Enum::ISIN_NUMBER => {
                f.write_str( "4" )
            },
            &Field_IDSource_Enum::RIC_CODE => {
                f.write_str( "5" )
            },
            &Field_IDSource_Enum::ISO_CURRENCY_CODE => {
                f.write_str( "6" )
            },
            &Field_IDSource_Enum::ISO_COUNTRY_CODE => {
                f.write_str( "7" )
            },
            &Field_IDSource_Enum::EXCHANGE_SYMBOL => {
                f.write_str( "8" )
            },
            &Field_IDSource_Enum::CONSOLIDATED_TAPE_ASSOCIATION => {
                f.write_str( "9" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_IDSource_Enum {
    fn default() -> Self {
        Field_IDSource_Enum::Undefined
    }
}
const FIELD_IOIID : u32 = 23; // STRING

const FIELD_IOIOTHSVC : u32 = 24; // CHAR

const FIELD_IOIQLTYIND : u32 = 25; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_IOIQltyInd_Enum {
    HIGH, // = "H"
    LOW, // = "L"
    MEDIUM, // = "M"

    Undefined
}

impl FromStr for Field_IOIQltyInd_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "H" => Ok(Field_IOIQltyInd_Enum::HIGH),
            "L" => Ok(Field_IOIQltyInd_Enum::LOW),
            "M" => Ok(Field_IOIQltyInd_Enum::MEDIUM),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_IOIQltyInd_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_IOIQltyInd_Enum::HIGH => {
                f.write_str( "H" )
            },
            &Field_IOIQltyInd_Enum::LOW => {
                f.write_str( "L" )
            },
            &Field_IOIQltyInd_Enum::MEDIUM => {
                f.write_str( "M" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_IOIQltyInd_Enum {
    fn default() -> Self {
        Field_IOIQltyInd_Enum::Undefined
    }
}
const FIELD_IOIREFID : u32 = 26; // STRING

const FIELD_IOISHARES : u32 = 27; // STRING
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_IOIShares_Enum {
    LARGE, // = "L"
    MEDIUM, // = "M"
    SMALL, // = "S"

    Undefined
}

impl FromStr for Field_IOIShares_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Field_IOIShares_Enum::LARGE),
            "M" => Ok(Field_IOIShares_Enum::MEDIUM),
            "S" => Ok(Field_IOIShares_Enum::SMALL),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_IOIShares_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_IOIShares_Enum::LARGE => {
                f.write_str( "L" )
            },
            &Field_IOIShares_Enum::MEDIUM => {
                f.write_str( "M" )
            },
            &Field_IOIShares_Enum::SMALL => {
                f.write_str( "S" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_IOIShares_Enum {
    fn default() -> Self {
        Field_IOIShares_Enum::Undefined
    }
}
const FIELD_IOITRANSTYPE : u32 = 28; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_IOITransType_Enum {
    CANCEL, // = "C"
    NEW, // = "N"
    REPLACE, // = "R"

    Undefined
}

impl FromStr for Field_IOITransType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(Field_IOITransType_Enum::CANCEL),
            "N" => Ok(Field_IOITransType_Enum::NEW),
            "R" => Ok(Field_IOITransType_Enum::REPLACE),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_IOITransType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_IOITransType_Enum::CANCEL => {
                f.write_str( "C" )
            },
            &Field_IOITransType_Enum::NEW => {
                f.write_str( "N" )
            },
            &Field_IOITransType_Enum::REPLACE => {
                f.write_str( "R" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_IOITransType_Enum {
    fn default() -> Self {
        Field_IOITransType_Enum::Undefined
    }
}
const FIELD_LASTCAPACITY : u32 = 29; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_LastCapacity_Enum {
    AGENT, // = "1"
    CROSS_AS_AGENT, // = "2"
    CROSS_AS_PRINCIPAL, // = "3"
    PRINCIPAL, // = "4"

    Undefined
}

impl FromStr for Field_LastCapacity_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_LastCapacity_Enum::AGENT),
            "2" => Ok(Field_LastCapacity_Enum::CROSS_AS_AGENT),
            "3" => Ok(Field_LastCapacity_Enum::CROSS_AS_PRINCIPAL),
            "4" => Ok(Field_LastCapacity_Enum::PRINCIPAL),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_LastCapacity_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_LastCapacity_Enum::AGENT => {
                f.write_str( "1" )
            },
            &Field_LastCapacity_Enum::CROSS_AS_AGENT => {
                f.write_str( "2" )
            },
            &Field_LastCapacity_Enum::CROSS_AS_PRINCIPAL => {
                f.write_str( "3" )
            },
            &Field_LastCapacity_Enum::PRINCIPAL => {
                f.write_str( "4" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_LastCapacity_Enum {
    fn default() -> Self {
        Field_LastCapacity_Enum::Undefined
    }
}
const FIELD_LASTMKT : u32 = 30; // EXCHANGE

const FIELD_LASTPX : u32 = 31; // PRICE

const FIELD_LASTSHARES : u32 = 32; // QTY

const FIELD_LINESOFTEXT : u32 = 33; // INT

const FIELD_MSGSEQNUM : u32 = 34; // INT

const FIELD_MSGTYPE : u32 = 35; // STRING
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_MsgType_Enum {
    HEARTBEAT, // = "0"
    TEST_REQUEST, // = "1"
    RESEND_REQUEST, // = "2"
    REJECT, // = "3"
    SEQUENCE_RESET, // = "4"
    LOGOUT, // = "5"
    INDICATION_OF_INTEREST, // = "6"
    ADVERTISEMENT, // = "7"
    EXECUTION_REPORT, // = "8"
    ORDER_CANCEL_REJECT, // = "9"
    QUOTE_STATUS_REQUEST, // = "a"
    LOGON, // = "A"
    NEWS, // = "B"
    QUOTE_ACKNOWLEDGEMENT, // = "b"
    EMAIL, // = "C"
    SECURITY_DEFINITION_REQUEST, // = "c"
    ORDER_SINGLE, // = "D"
    SECURITY_DEFINITION, // = "d"
    ORDER_LIST, // = "E"
    SECURITY_STATUS_REQUEST, // = "e"
    SECURITY_STATUS, // = "f"
    ORDER_CANCEL_REQUEST, // = "F"
    ORDER_CANCEL_REPLACE_REQUEST, // = "G"
    TRADING_SESSION_STATUS_REQUEST, // = "g"
    ORDER_STATUS_REQUEST, // = "H"
    TRADING_SESSION_STATUS, // = "h"
    MASS_QUOTE, // = "i"
    BUSINESS_MESSAGE_REJECT, // = "j"
    ALLOCATION, // = "J"
    LIST_CANCEL_REQUEST, // = "K"
    BID_REQUEST, // = "k"
    BID_RESPONSE, // = "l"
    LIST_EXECUTE, // = "L"
    LIST_STRIKE_PRICE, // = "m"
    LIST_STATUS_REQUEST, // = "M"
    LIST_STATUS, // = "N"
    ALLOCATION_ACK, // = "P"
    DONT_KNOW_TRADE, // = "Q"
    QUOTE_REQUEST, // = "R"
    QUOTE, // = "S"
    SETTLEMENT_INSTRUCTIONS, // = "T"
    MARKET_DATA_REQUEST, // = "V"
    MARKET_DATA_SNAPSHOT_FULL_REFRESH, // = "W"
    MARKET_DATA_INCREMENTAL_REFRESH, // = "X"
    MARKET_DATA_REQUEST_REJECT, // = "Y"
    QUOTE_CANCEL, // = "Z"

    Undefined
}

impl FromStr for Field_MsgType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_MsgType_Enum::HEARTBEAT),
            "1" => Ok(Field_MsgType_Enum::TEST_REQUEST),
            "2" => Ok(Field_MsgType_Enum::RESEND_REQUEST),
            "3" => Ok(Field_MsgType_Enum::REJECT),
            "4" => Ok(Field_MsgType_Enum::SEQUENCE_RESET),
            "5" => Ok(Field_MsgType_Enum::LOGOUT),
            "6" => Ok(Field_MsgType_Enum::INDICATION_OF_INTEREST),
            "7" => Ok(Field_MsgType_Enum::ADVERTISEMENT),
            "8" => Ok(Field_MsgType_Enum::EXECUTION_REPORT),
            "9" => Ok(Field_MsgType_Enum::ORDER_CANCEL_REJECT),
            "a" => Ok(Field_MsgType_Enum::QUOTE_STATUS_REQUEST),
            "A" => Ok(Field_MsgType_Enum::LOGON),
            "B" => Ok(Field_MsgType_Enum::NEWS),
            "b" => Ok(Field_MsgType_Enum::QUOTE_ACKNOWLEDGEMENT),
            "C" => Ok(Field_MsgType_Enum::EMAIL),
            "c" => Ok(Field_MsgType_Enum::SECURITY_DEFINITION_REQUEST),
            "D" => Ok(Field_MsgType_Enum::ORDER_SINGLE),
            "d" => Ok(Field_MsgType_Enum::SECURITY_DEFINITION),
            "E" => Ok(Field_MsgType_Enum::ORDER_LIST),
            "e" => Ok(Field_MsgType_Enum::SECURITY_STATUS_REQUEST),
            "f" => Ok(Field_MsgType_Enum::SECURITY_STATUS),
            "F" => Ok(Field_MsgType_Enum::ORDER_CANCEL_REQUEST),
            "G" => Ok(Field_MsgType_Enum::ORDER_CANCEL_REPLACE_REQUEST),
            "g" => Ok(Field_MsgType_Enum::TRADING_SESSION_STATUS_REQUEST),
            "H" => Ok(Field_MsgType_Enum::ORDER_STATUS_REQUEST),
            "h" => Ok(Field_MsgType_Enum::TRADING_SESSION_STATUS),
            "i" => Ok(Field_MsgType_Enum::MASS_QUOTE),
            "j" => Ok(Field_MsgType_Enum::BUSINESS_MESSAGE_REJECT),
            "J" => Ok(Field_MsgType_Enum::ALLOCATION),
            "K" => Ok(Field_MsgType_Enum::LIST_CANCEL_REQUEST),
            "k" => Ok(Field_MsgType_Enum::BID_REQUEST),
            "l" => Ok(Field_MsgType_Enum::BID_RESPONSE),
            "L" => Ok(Field_MsgType_Enum::LIST_EXECUTE),
            "m" => Ok(Field_MsgType_Enum::LIST_STRIKE_PRICE),
            "M" => Ok(Field_MsgType_Enum::LIST_STATUS_REQUEST),
            "N" => Ok(Field_MsgType_Enum::LIST_STATUS),
            "P" => Ok(Field_MsgType_Enum::ALLOCATION_ACK),
            "Q" => Ok(Field_MsgType_Enum::DONT_KNOW_TRADE),
            "R" => Ok(Field_MsgType_Enum::QUOTE_REQUEST),
            "S" => Ok(Field_MsgType_Enum::QUOTE),
            "T" => Ok(Field_MsgType_Enum::SETTLEMENT_INSTRUCTIONS),
            "V" => Ok(Field_MsgType_Enum::MARKET_DATA_REQUEST),
            "W" => Ok(Field_MsgType_Enum::MARKET_DATA_SNAPSHOT_FULL_REFRESH),
            "X" => Ok(Field_MsgType_Enum::MARKET_DATA_INCREMENTAL_REFRESH),
            "Y" => Ok(Field_MsgType_Enum::MARKET_DATA_REQUEST_REJECT),
            "Z" => Ok(Field_MsgType_Enum::QUOTE_CANCEL),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_MsgType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_MsgType_Enum::HEARTBEAT => {
                f.write_str( "0" )
            },
            &Field_MsgType_Enum::TEST_REQUEST => {
                f.write_str( "1" )
            },
            &Field_MsgType_Enum::RESEND_REQUEST => {
                f.write_str( "2" )
            },
            &Field_MsgType_Enum::REJECT => {
                f.write_str( "3" )
            },
            &Field_MsgType_Enum::SEQUENCE_RESET => {
                f.write_str( "4" )
            },
            &Field_MsgType_Enum::LOGOUT => {
                f.write_str( "5" )
            },
            &Field_MsgType_Enum::INDICATION_OF_INTEREST => {
                f.write_str( "6" )
            },
            &Field_MsgType_Enum::ADVERTISEMENT => {
                f.write_str( "7" )
            },
            &Field_MsgType_Enum::EXECUTION_REPORT => {
                f.write_str( "8" )
            },
            &Field_MsgType_Enum::ORDER_CANCEL_REJECT => {
                f.write_str( "9" )
            },
            &Field_MsgType_Enum::QUOTE_STATUS_REQUEST => {
                f.write_str( "a" )
            },
            &Field_MsgType_Enum::LOGON => {
                f.write_str( "A" )
            },
            &Field_MsgType_Enum::NEWS => {
                f.write_str( "B" )
            },
            &Field_MsgType_Enum::QUOTE_ACKNOWLEDGEMENT => {
                f.write_str( "b" )
            },
            &Field_MsgType_Enum::EMAIL => {
                f.write_str( "C" )
            },
            &Field_MsgType_Enum::SECURITY_DEFINITION_REQUEST => {
                f.write_str( "c" )
            },
            &Field_MsgType_Enum::ORDER_SINGLE => {
                f.write_str( "D" )
            },
            &Field_MsgType_Enum::SECURITY_DEFINITION => {
                f.write_str( "d" )
            },
            &Field_MsgType_Enum::ORDER_LIST => {
                f.write_str( "E" )
            },
            &Field_MsgType_Enum::SECURITY_STATUS_REQUEST => {
                f.write_str( "e" )
            },
            &Field_MsgType_Enum::SECURITY_STATUS => {
                f.write_str( "f" )
            },
            &Field_MsgType_Enum::ORDER_CANCEL_REQUEST => {
                f.write_str( "F" )
            },
            &Field_MsgType_Enum::ORDER_CANCEL_REPLACE_REQUEST => {
                f.write_str( "G" )
            },
            &Field_MsgType_Enum::TRADING_SESSION_STATUS_REQUEST => {
                f.write_str( "g" )
            },
            &Field_MsgType_Enum::ORDER_STATUS_REQUEST => {
                f.write_str( "H" )
            },
            &Field_MsgType_Enum::TRADING_SESSION_STATUS => {
                f.write_str( "h" )
            },
            &Field_MsgType_Enum::MASS_QUOTE => {
                f.write_str( "i" )
            },
            &Field_MsgType_Enum::BUSINESS_MESSAGE_REJECT => {
                f.write_str( "j" )
            },
            &Field_MsgType_Enum::ALLOCATION => {
                f.write_str( "J" )
            },
            &Field_MsgType_Enum::LIST_CANCEL_REQUEST => {
                f.write_str( "K" )
            },
            &Field_MsgType_Enum::BID_REQUEST => {
                f.write_str( "k" )
            },
            &Field_MsgType_Enum::BID_RESPONSE => {
                f.write_str( "l" )
            },
            &Field_MsgType_Enum::LIST_EXECUTE => {
                f.write_str( "L" )
            },
            &Field_MsgType_Enum::LIST_STRIKE_PRICE => {
                f.write_str( "m" )
            },
            &Field_MsgType_Enum::LIST_STATUS_REQUEST => {
                f.write_str( "M" )
            },
            &Field_MsgType_Enum::LIST_STATUS => {
                f.write_str( "N" )
            },
            &Field_MsgType_Enum::ALLOCATION_ACK => {
                f.write_str( "P" )
            },
            &Field_MsgType_Enum::DONT_KNOW_TRADE => {
                f.write_str( "Q" )
            },
            &Field_MsgType_Enum::QUOTE_REQUEST => {
                f.write_str( "R" )
            },
            &Field_MsgType_Enum::QUOTE => {
                f.write_str( "S" )
            },
            &Field_MsgType_Enum::SETTLEMENT_INSTRUCTIONS => {
                f.write_str( "T" )
            },
            &Field_MsgType_Enum::MARKET_DATA_REQUEST => {
                f.write_str( "V" )
            },
            &Field_MsgType_Enum::MARKET_DATA_SNAPSHOT_FULL_REFRESH => {
                f.write_str( "W" )
            },
            &Field_MsgType_Enum::MARKET_DATA_INCREMENTAL_REFRESH => {
                f.write_str( "X" )
            },
            &Field_MsgType_Enum::MARKET_DATA_REQUEST_REJECT => {
                f.write_str( "Y" )
            },
            &Field_MsgType_Enum::QUOTE_CANCEL => {
                f.write_str( "Z" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_MsgType_Enum {
    fn default() -> Self {
        Field_MsgType_Enum::Undefined
    }
}
const FIELD_NEWSEQNO : u32 = 36; // INT

const FIELD_ORDERID : u32 = 37; // STRING

const FIELD_ORDERQTY : u32 = 38; // QTY

const FIELD_ORDSTATUS : u32 = 39; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_OrdStatus_Enum {
    NEW, // = "0"
    PARTIALLY_FILLED, // = "1"
    FILLED, // = "2"
    DONE_FOR_DAY, // = "3"
    CANCELED, // = "4"
    REPLACED, // = "5"
    PENDING_CANCEL, // = "6"
    STOPPED, // = "7"
    REJECTED, // = "8"
    SUSPENDED, // = "9"
    PENDING_NEW, // = "A"
    CALCULATED, // = "B"
    EXPIRED, // = "C"
    ACCEPTED_FOR_BIDDING, // = "D"
    PENDING_REPLACE, // = "E"

    Undefined
}

impl FromStr for Field_OrdStatus_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_OrdStatus_Enum::NEW),
            "1" => Ok(Field_OrdStatus_Enum::PARTIALLY_FILLED),
            "2" => Ok(Field_OrdStatus_Enum::FILLED),
            "3" => Ok(Field_OrdStatus_Enum::DONE_FOR_DAY),
            "4" => Ok(Field_OrdStatus_Enum::CANCELED),
            "5" => Ok(Field_OrdStatus_Enum::REPLACED),
            "6" => Ok(Field_OrdStatus_Enum::PENDING_CANCEL),
            "7" => Ok(Field_OrdStatus_Enum::STOPPED),
            "8" => Ok(Field_OrdStatus_Enum::REJECTED),
            "9" => Ok(Field_OrdStatus_Enum::SUSPENDED),
            "A" => Ok(Field_OrdStatus_Enum::PENDING_NEW),
            "B" => Ok(Field_OrdStatus_Enum::CALCULATED),
            "C" => Ok(Field_OrdStatus_Enum::EXPIRED),
            "D" => Ok(Field_OrdStatus_Enum::ACCEPTED_FOR_BIDDING),
            "E" => Ok(Field_OrdStatus_Enum::PENDING_REPLACE),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_OrdStatus_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_OrdStatus_Enum::NEW => {
                f.write_str( "0" )
            },
            &Field_OrdStatus_Enum::PARTIALLY_FILLED => {
                f.write_str( "1" )
            },
            &Field_OrdStatus_Enum::FILLED => {
                f.write_str( "2" )
            },
            &Field_OrdStatus_Enum::DONE_FOR_DAY => {
                f.write_str( "3" )
            },
            &Field_OrdStatus_Enum::CANCELED => {
                f.write_str( "4" )
            },
            &Field_OrdStatus_Enum::REPLACED => {
                f.write_str( "5" )
            },
            &Field_OrdStatus_Enum::PENDING_CANCEL => {
                f.write_str( "6" )
            },
            &Field_OrdStatus_Enum::STOPPED => {
                f.write_str( "7" )
            },
            &Field_OrdStatus_Enum::REJECTED => {
                f.write_str( "8" )
            },
            &Field_OrdStatus_Enum::SUSPENDED => {
                f.write_str( "9" )
            },
            &Field_OrdStatus_Enum::PENDING_NEW => {
                f.write_str( "A" )
            },
            &Field_OrdStatus_Enum::CALCULATED => {
                f.write_str( "B" )
            },
            &Field_OrdStatus_Enum::EXPIRED => {
                f.write_str( "C" )
            },
            &Field_OrdStatus_Enum::ACCEPTED_FOR_BIDDING => {
                f.write_str( "D" )
            },
            &Field_OrdStatus_Enum::PENDING_REPLACE => {
                f.write_str( "E" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_OrdStatus_Enum {
    fn default() -> Self {
        Field_OrdStatus_Enum::Undefined
    }
}
const FIELD_ORDTYPE : u32 = 40; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_OrdType_Enum {
    MARKET, // = "1"
    LIMIT, // = "2"
    STOP, // = "3"
    STOP_LIMIT, // = "4"
    MARKET_ON_CLOSE, // = "5"
    WITH_OR_WITHOUT, // = "6"
    LIMIT_OR_BETTER, // = "7"
    LIMIT_WITH_OR_WITHOUT, // = "8"
    ON_BASIS, // = "9"
    ON_CLOSE, // = "A"
    LIMIT_ON_CLOSE, // = "B"
    FOREX_C, // = "C"
    PREVIOUSLY_QUOTED, // = "D"
    PREVIOUSLY_INDICATED, // = "E"
    FOREX_F, // = "F"
    FOREX_G, // = "G"
    FOREX_H, // = "H"
    FUNARI, // = "I"
    PEGGED, // = "P"

    Undefined
}

impl FromStr for Field_OrdType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_OrdType_Enum::MARKET),
            "2" => Ok(Field_OrdType_Enum::LIMIT),
            "3" => Ok(Field_OrdType_Enum::STOP),
            "4" => Ok(Field_OrdType_Enum::STOP_LIMIT),
            "5" => Ok(Field_OrdType_Enum::MARKET_ON_CLOSE),
            "6" => Ok(Field_OrdType_Enum::WITH_OR_WITHOUT),
            "7" => Ok(Field_OrdType_Enum::LIMIT_OR_BETTER),
            "8" => Ok(Field_OrdType_Enum::LIMIT_WITH_OR_WITHOUT),
            "9" => Ok(Field_OrdType_Enum::ON_BASIS),
            "A" => Ok(Field_OrdType_Enum::ON_CLOSE),
            "B" => Ok(Field_OrdType_Enum::LIMIT_ON_CLOSE),
            "C" => Ok(Field_OrdType_Enum::FOREX_C),
            "D" => Ok(Field_OrdType_Enum::PREVIOUSLY_QUOTED),
            "E" => Ok(Field_OrdType_Enum::PREVIOUSLY_INDICATED),
            "F" => Ok(Field_OrdType_Enum::FOREX_F),
            "G" => Ok(Field_OrdType_Enum::FOREX_G),
            "H" => Ok(Field_OrdType_Enum::FOREX_H),
            "I" => Ok(Field_OrdType_Enum::FUNARI),
            "P" => Ok(Field_OrdType_Enum::PEGGED),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_OrdType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_OrdType_Enum::MARKET => {
                f.write_str( "1" )
            },
            &Field_OrdType_Enum::LIMIT => {
                f.write_str( "2" )
            },
            &Field_OrdType_Enum::STOP => {
                f.write_str( "3" )
            },
            &Field_OrdType_Enum::STOP_LIMIT => {
                f.write_str( "4" )
            },
            &Field_OrdType_Enum::MARKET_ON_CLOSE => {
                f.write_str( "5" )
            },
            &Field_OrdType_Enum::WITH_OR_WITHOUT => {
                f.write_str( "6" )
            },
            &Field_OrdType_Enum::LIMIT_OR_BETTER => {
                f.write_str( "7" )
            },
            &Field_OrdType_Enum::LIMIT_WITH_OR_WITHOUT => {
                f.write_str( "8" )
            },
            &Field_OrdType_Enum::ON_BASIS => {
                f.write_str( "9" )
            },
            &Field_OrdType_Enum::ON_CLOSE => {
                f.write_str( "A" )
            },
            &Field_OrdType_Enum::LIMIT_ON_CLOSE => {
                f.write_str( "B" )
            },
            &Field_OrdType_Enum::FOREX_C => {
                f.write_str( "C" )
            },
            &Field_OrdType_Enum::PREVIOUSLY_QUOTED => {
                f.write_str( "D" )
            },
            &Field_OrdType_Enum::PREVIOUSLY_INDICATED => {
                f.write_str( "E" )
            },
            &Field_OrdType_Enum::FOREX_F => {
                f.write_str( "F" )
            },
            &Field_OrdType_Enum::FOREX_G => {
                f.write_str( "G" )
            },
            &Field_OrdType_Enum::FOREX_H => {
                f.write_str( "H" )
            },
            &Field_OrdType_Enum::FUNARI => {
                f.write_str( "I" )
            },
            &Field_OrdType_Enum::PEGGED => {
                f.write_str( "P" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_OrdType_Enum {
    fn default() -> Self {
        Field_OrdType_Enum::Undefined
    }
}
const FIELD_ORIGCLORDID : u32 = 41; // STRING

const FIELD_ORIGTIME : u32 = 42; // UTCTIMESTAMP

const FIELD_POSSDUPFLAG : u32 = 43; // BOOLEAN

const FIELD_PRICE : u32 = 44; // PRICE

const FIELD_REFSEQNUM : u32 = 45; // INT

const FIELD_RELATDSYM : u32 = 46; // STRING

const FIELD_RULE80A : u32 = 47; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_Rule80A_Enum {
    AGENCY_SINGLE_ORDER, // = "A"
    SHORT_EXEMPT_TRANSACTION_B, // = "B"
    PROGRAM_ORDER_NON_INDEX_ARB_FOR_MEMBER_FIRM_ORG, // = "C"
    PROGRAM_ORDER_INDEX_ARB_FOR_MEMBER_FIRM_ORG, // = "D"
    REGISTERED_EQUITY_MARKET_MAKER_TRADES, // = "E"
    SHORT_EXEMPT_TRANSACTION_F, // = "F"
    SHORT_EXEMPT_TRANSACTION_H, // = "H"
    INDIVIDUAL_INVESTOR_SINGLE_ORDER, // = "I"
    PROGRAM_ORDER_INDEX_ARB_FOR_INDIVIDUAL_CUSTOMER, // = "J"
    PROGRAM_ORDER_NON_INDEX_ARB_FOR_INDIVIDUAL_CUSTOMER, // = "K"
    SHORT_EXEMPT_TRANSACTION_FOR_MEMBER_COMPETING_MARKET_MAKER_AFFILIATED_WITH_THE_FIRM_CLEARING_THE_TRADE, // = "L"
    PROGRAM_ORDER_INDEX_ARB_FOR_OTHER_MEMBER, // = "M"
    PROGRAM_ORDER_NON_INDEX_ARB_FOR_OTHER_MEMBER, // = "N"
    COMPETING_DEALER_TRADES_O, // = "O"
    PRINCIPAL, // = "P"
    COMPETING_DEALER_TRADES_R, // = "R"
    SPECIALIST_TRADES, // = "S"
    COMPETING_DEALER_TRADES_T, // = "T"
    PROGRAM_ORDER_INDEX_ARB_FOR_OTHER_AGENCY, // = "U"
    ALL_OTHER_ORDERS_AS_AGENT_FOR_OTHER_MEMBER, // = "W"
    SHORT_EXEMPT_TRANSACTION_FOR_MEMBER_COMPETING_MARKET_MAKER_NOT_AFFILIATED_WITH_THE_FIRM_CLEARING_THE_TRADE, // = "X"
    PROGRAM_ORDER_NON_INDEX_ARB_FOR_OTHER_AGENCY, // = "Y"
    SHORT_EXEMPT_TRANSACTION_FOR_NON_MEMBER_COMPETING_MARKET_MAKER, // = "Z"

    Undefined
}

impl FromStr for Field_Rule80A_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Field_Rule80A_Enum::AGENCY_SINGLE_ORDER),
            "B" => Ok(Field_Rule80A_Enum::SHORT_EXEMPT_TRANSACTION_B),
            "C" => Ok(Field_Rule80A_Enum::PROGRAM_ORDER_NON_INDEX_ARB_FOR_MEMBER_FIRM_ORG),
            "D" => Ok(Field_Rule80A_Enum::PROGRAM_ORDER_INDEX_ARB_FOR_MEMBER_FIRM_ORG),
            "E" => Ok(Field_Rule80A_Enum::REGISTERED_EQUITY_MARKET_MAKER_TRADES),
            "F" => Ok(Field_Rule80A_Enum::SHORT_EXEMPT_TRANSACTION_F),
            "H" => Ok(Field_Rule80A_Enum::SHORT_EXEMPT_TRANSACTION_H),
            "I" => Ok(Field_Rule80A_Enum::INDIVIDUAL_INVESTOR_SINGLE_ORDER),
            "J" => Ok(Field_Rule80A_Enum::PROGRAM_ORDER_INDEX_ARB_FOR_INDIVIDUAL_CUSTOMER),
            "K" => Ok(Field_Rule80A_Enum::PROGRAM_ORDER_NON_INDEX_ARB_FOR_INDIVIDUAL_CUSTOMER),
            "L" => Ok(Field_Rule80A_Enum::SHORT_EXEMPT_TRANSACTION_FOR_MEMBER_COMPETING_MARKET_MAKER_AFFILIATED_WITH_THE_FIRM_CLEARING_THE_TRADE),
            "M" => Ok(Field_Rule80A_Enum::PROGRAM_ORDER_INDEX_ARB_FOR_OTHER_MEMBER),
            "N" => Ok(Field_Rule80A_Enum::PROGRAM_ORDER_NON_INDEX_ARB_FOR_OTHER_MEMBER),
            "O" => Ok(Field_Rule80A_Enum::COMPETING_DEALER_TRADES_O),
            "P" => Ok(Field_Rule80A_Enum::PRINCIPAL),
            "R" => Ok(Field_Rule80A_Enum::COMPETING_DEALER_TRADES_R),
            "S" => Ok(Field_Rule80A_Enum::SPECIALIST_TRADES),
            "T" => Ok(Field_Rule80A_Enum::COMPETING_DEALER_TRADES_T),
            "U" => Ok(Field_Rule80A_Enum::PROGRAM_ORDER_INDEX_ARB_FOR_OTHER_AGENCY),
            "W" => Ok(Field_Rule80A_Enum::ALL_OTHER_ORDERS_AS_AGENT_FOR_OTHER_MEMBER),
            "X" => Ok(Field_Rule80A_Enum::SHORT_EXEMPT_TRANSACTION_FOR_MEMBER_COMPETING_MARKET_MAKER_NOT_AFFILIATED_WITH_THE_FIRM_CLEARING_THE_TRADE),
            "Y" => Ok(Field_Rule80A_Enum::PROGRAM_ORDER_NON_INDEX_ARB_FOR_OTHER_AGENCY),
            "Z" => Ok(Field_Rule80A_Enum::SHORT_EXEMPT_TRANSACTION_FOR_NON_MEMBER_COMPETING_MARKET_MAKER),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_Rule80A_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_Rule80A_Enum::AGENCY_SINGLE_ORDER => {
                f.write_str( "A" )
            },
            &Field_Rule80A_Enum::SHORT_EXEMPT_TRANSACTION_B => {
                f.write_str( "B" )
            },
            &Field_Rule80A_Enum::PROGRAM_ORDER_NON_INDEX_ARB_FOR_MEMBER_FIRM_ORG => {
                f.write_str( "C" )
            },
            &Field_Rule80A_Enum::PROGRAM_ORDER_INDEX_ARB_FOR_MEMBER_FIRM_ORG => {
                f.write_str( "D" )
            },
            &Field_Rule80A_Enum::REGISTERED_EQUITY_MARKET_MAKER_TRADES => {
                f.write_str( "E" )
            },
            &Field_Rule80A_Enum::SHORT_EXEMPT_TRANSACTION_F => {
                f.write_str( "F" )
            },
            &Field_Rule80A_Enum::SHORT_EXEMPT_TRANSACTION_H => {
                f.write_str( "H" )
            },
            &Field_Rule80A_Enum::INDIVIDUAL_INVESTOR_SINGLE_ORDER => {
                f.write_str( "I" )
            },
            &Field_Rule80A_Enum::PROGRAM_ORDER_INDEX_ARB_FOR_INDIVIDUAL_CUSTOMER => {
                f.write_str( "J" )
            },
            &Field_Rule80A_Enum::PROGRAM_ORDER_NON_INDEX_ARB_FOR_INDIVIDUAL_CUSTOMER => {
                f.write_str( "K" )
            },
            &Field_Rule80A_Enum::SHORT_EXEMPT_TRANSACTION_FOR_MEMBER_COMPETING_MARKET_MAKER_AFFILIATED_WITH_THE_FIRM_CLEARING_THE_TRADE => {
                f.write_str( "L" )
            },
            &Field_Rule80A_Enum::PROGRAM_ORDER_INDEX_ARB_FOR_OTHER_MEMBER => {
                f.write_str( "M" )
            },
            &Field_Rule80A_Enum::PROGRAM_ORDER_NON_INDEX_ARB_FOR_OTHER_MEMBER => {
                f.write_str( "N" )
            },
            &Field_Rule80A_Enum::COMPETING_DEALER_TRADES_O => {
                f.write_str( "O" )
            },
            &Field_Rule80A_Enum::PRINCIPAL => {
                f.write_str( "P" )
            },
            &Field_Rule80A_Enum::COMPETING_DEALER_TRADES_R => {
                f.write_str( "R" )
            },
            &Field_Rule80A_Enum::SPECIALIST_TRADES => {
                f.write_str( "S" )
            },
            &Field_Rule80A_Enum::COMPETING_DEALER_TRADES_T => {
                f.write_str( "T" )
            },
            &Field_Rule80A_Enum::PROGRAM_ORDER_INDEX_ARB_FOR_OTHER_AGENCY => {
                f.write_str( "U" )
            },
            &Field_Rule80A_Enum::ALL_OTHER_ORDERS_AS_AGENT_FOR_OTHER_MEMBER => {
                f.write_str( "W" )
            },
            &Field_Rule80A_Enum::SHORT_EXEMPT_TRANSACTION_FOR_MEMBER_COMPETING_MARKET_MAKER_NOT_AFFILIATED_WITH_THE_FIRM_CLEARING_THE_TRADE => {
                f.write_str( "X" )
            },
            &Field_Rule80A_Enum::PROGRAM_ORDER_NON_INDEX_ARB_FOR_OTHER_AGENCY => {
                f.write_str( "Y" )
            },
            &Field_Rule80A_Enum::SHORT_EXEMPT_TRANSACTION_FOR_NON_MEMBER_COMPETING_MARKET_MAKER => {
                f.write_str( "Z" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_Rule80A_Enum {
    fn default() -> Self {
        Field_Rule80A_Enum::Undefined
    }
}
const FIELD_SECURITYID : u32 = 48; // STRING

const FIELD_SENDERCOMPID : u32 = 49; // STRING

const FIELD_SENDERSUBID : u32 = 50; // STRING

const FIELD_SENDINGDATE : u32 = 51; // LOCALMKTDATE

const FIELD_SENDINGTIME : u32 = 52; // UTCTIMESTAMP

const FIELD_SHARES : u32 = 53; // QTY

const FIELD_SIDE : u32 = 54; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_Side_Enum {
    BUY, // = "1"
    SELL, // = "2"
    BUY_MINUS, // = "3"
    SELL_PLUS, // = "4"
    SELL_SHORT, // = "5"
    SELL_SHORT_EXEMPT, // = "6"
    UNDISCLOSED, // = "7"
    CROSS, // = "8"
    CROSS_SHORT, // = "9"

    Undefined
}

impl FromStr for Field_Side_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_Side_Enum::BUY),
            "2" => Ok(Field_Side_Enum::SELL),
            "3" => Ok(Field_Side_Enum::BUY_MINUS),
            "4" => Ok(Field_Side_Enum::SELL_PLUS),
            "5" => Ok(Field_Side_Enum::SELL_SHORT),
            "6" => Ok(Field_Side_Enum::SELL_SHORT_EXEMPT),
            "7" => Ok(Field_Side_Enum::UNDISCLOSED),
            "8" => Ok(Field_Side_Enum::CROSS),
            "9" => Ok(Field_Side_Enum::CROSS_SHORT),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_Side_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_Side_Enum::BUY => {
                f.write_str( "1" )
            },
            &Field_Side_Enum::SELL => {
                f.write_str( "2" )
            },
            &Field_Side_Enum::BUY_MINUS => {
                f.write_str( "3" )
            },
            &Field_Side_Enum::SELL_PLUS => {
                f.write_str( "4" )
            },
            &Field_Side_Enum::SELL_SHORT => {
                f.write_str( "5" )
            },
            &Field_Side_Enum::SELL_SHORT_EXEMPT => {
                f.write_str( "6" )
            },
            &Field_Side_Enum::UNDISCLOSED => {
                f.write_str( "7" )
            },
            &Field_Side_Enum::CROSS => {
                f.write_str( "8" )
            },
            &Field_Side_Enum::CROSS_SHORT => {
                f.write_str( "9" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_Side_Enum {
    fn default() -> Self {
        Field_Side_Enum::Undefined
    }
}
const FIELD_SYMBOL : u32 = 55; // STRING

const FIELD_TARGETCOMPID : u32 = 56; // STRING

const FIELD_TARGETSUBID : u32 = 57; // STRING

const FIELD_TEXT : u32 = 58; // STRING

const FIELD_TIMEINFORCE : u32 = 59; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_TimeInForce_Enum {
    DAY, // = "0"
    GOOD_TILL_CANCEL, // = "1"
    AT_THE_OPENING, // = "2"
    IMMEDIATE_OR_CANCEL, // = "3"
    FILL_OR_KILL, // = "4"
    GOOD_TILL_CROSSING, // = "5"
    GOOD_TILL_DATE, // = "6"

    Undefined
}

impl FromStr for Field_TimeInForce_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_TimeInForce_Enum::DAY),
            "1" => Ok(Field_TimeInForce_Enum::GOOD_TILL_CANCEL),
            "2" => Ok(Field_TimeInForce_Enum::AT_THE_OPENING),
            "3" => Ok(Field_TimeInForce_Enum::IMMEDIATE_OR_CANCEL),
            "4" => Ok(Field_TimeInForce_Enum::FILL_OR_KILL),
            "5" => Ok(Field_TimeInForce_Enum::GOOD_TILL_CROSSING),
            "6" => Ok(Field_TimeInForce_Enum::GOOD_TILL_DATE),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_TimeInForce_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_TimeInForce_Enum::DAY => {
                f.write_str( "0" )
            },
            &Field_TimeInForce_Enum::GOOD_TILL_CANCEL => {
                f.write_str( "1" )
            },
            &Field_TimeInForce_Enum::AT_THE_OPENING => {
                f.write_str( "2" )
            },
            &Field_TimeInForce_Enum::IMMEDIATE_OR_CANCEL => {
                f.write_str( "3" )
            },
            &Field_TimeInForce_Enum::FILL_OR_KILL => {
                f.write_str( "4" )
            },
            &Field_TimeInForce_Enum::GOOD_TILL_CROSSING => {
                f.write_str( "5" )
            },
            &Field_TimeInForce_Enum::GOOD_TILL_DATE => {
                f.write_str( "6" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_TimeInForce_Enum {
    fn default() -> Self {
        Field_TimeInForce_Enum::Undefined
    }
}
const FIELD_TRANSACTTIME : u32 = 60; // UTCTIMESTAMP

const FIELD_URGENCY : u32 = 61; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_Urgency_Enum {
    NORMAL, // = "0"
    FLASH, // = "1"
    BACKGROUND, // = "2"

    Undefined
}

impl FromStr for Field_Urgency_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_Urgency_Enum::NORMAL),
            "1" => Ok(Field_Urgency_Enum::FLASH),
            "2" => Ok(Field_Urgency_Enum::BACKGROUND),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_Urgency_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_Urgency_Enum::NORMAL => {
                f.write_str( "0" )
            },
            &Field_Urgency_Enum::FLASH => {
                f.write_str( "1" )
            },
            &Field_Urgency_Enum::BACKGROUND => {
                f.write_str( "2" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_Urgency_Enum {
    fn default() -> Self {
        Field_Urgency_Enum::Undefined
    }
}
const FIELD_VALIDUNTILTIME : u32 = 62; // UTCTIMESTAMP

const FIELD_SETTLMNTTYP : u32 = 63; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_SettlmntTyp_Enum {
    REGULAR, // = "0"
    CASH, // = "1"
    NEXT_DAY, // = "2"
    T_PLUS_2, // = "3"
    T_PLUS_3, // = "4"
    T_PLUS_4, // = "5"
    FUTURE, // = "6"
    WHEN_ISSUED, // = "7"
    SELLERS_OPTION, // = "8"
    T_PLUS_5, // = "9"

    Undefined
}

impl FromStr for Field_SettlmntTyp_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_SettlmntTyp_Enum::REGULAR),
            "1" => Ok(Field_SettlmntTyp_Enum::CASH),
            "2" => Ok(Field_SettlmntTyp_Enum::NEXT_DAY),
            "3" => Ok(Field_SettlmntTyp_Enum::T_PLUS_2),
            "4" => Ok(Field_SettlmntTyp_Enum::T_PLUS_3),
            "5" => Ok(Field_SettlmntTyp_Enum::T_PLUS_4),
            "6" => Ok(Field_SettlmntTyp_Enum::FUTURE),
            "7" => Ok(Field_SettlmntTyp_Enum::WHEN_ISSUED),
            "8" => Ok(Field_SettlmntTyp_Enum::SELLERS_OPTION),
            "9" => Ok(Field_SettlmntTyp_Enum::T_PLUS_5),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_SettlmntTyp_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_SettlmntTyp_Enum::REGULAR => {
                f.write_str( "0" )
            },
            &Field_SettlmntTyp_Enum::CASH => {
                f.write_str( "1" )
            },
            &Field_SettlmntTyp_Enum::NEXT_DAY => {
                f.write_str( "2" )
            },
            &Field_SettlmntTyp_Enum::T_PLUS_2 => {
                f.write_str( "3" )
            },
            &Field_SettlmntTyp_Enum::T_PLUS_3 => {
                f.write_str( "4" )
            },
            &Field_SettlmntTyp_Enum::T_PLUS_4 => {
                f.write_str( "5" )
            },
            &Field_SettlmntTyp_Enum::FUTURE => {
                f.write_str( "6" )
            },
            &Field_SettlmntTyp_Enum::WHEN_ISSUED => {
                f.write_str( "7" )
            },
            &Field_SettlmntTyp_Enum::SELLERS_OPTION => {
                f.write_str( "8" )
            },
            &Field_SettlmntTyp_Enum::T_PLUS_5 => {
                f.write_str( "9" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_SettlmntTyp_Enum {
    fn default() -> Self {
        Field_SettlmntTyp_Enum::Undefined
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_AllocTransType_Enum {
    NEW, // = "0"
    REPLACE, // = "1"
    CANCEL, // = "2"
    PRELIMINARY, // = "3"
    CALCULATED, // = "4"
    CALCULATED_WITHOUT_PRELIMINARY, // = "5"

    Undefined
}

impl FromStr for Field_AllocTransType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_AllocTransType_Enum::NEW),
            "1" => Ok(Field_AllocTransType_Enum::REPLACE),
            "2" => Ok(Field_AllocTransType_Enum::CANCEL),
            "3" => Ok(Field_AllocTransType_Enum::PRELIMINARY),
            "4" => Ok(Field_AllocTransType_Enum::CALCULATED),
            "5" => Ok(Field_AllocTransType_Enum::CALCULATED_WITHOUT_PRELIMINARY),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_AllocTransType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_AllocTransType_Enum::NEW => {
                f.write_str( "0" )
            },
            &Field_AllocTransType_Enum::REPLACE => {
                f.write_str( "1" )
            },
            &Field_AllocTransType_Enum::CANCEL => {
                f.write_str( "2" )
            },
            &Field_AllocTransType_Enum::PRELIMINARY => {
                f.write_str( "3" )
            },
            &Field_AllocTransType_Enum::CALCULATED => {
                f.write_str( "4" )
            },
            &Field_AllocTransType_Enum::CALCULATED_WITHOUT_PRELIMINARY => {
                f.write_str( "5" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_AllocTransType_Enum {
    fn default() -> Self {
        Field_AllocTransType_Enum::Undefined
    }
}
const FIELD_REFALLOCID : u32 = 72; // STRING

const FIELD_NOORDERS : u32 = 73; // INT

const FIELD_AVGPRXPRECISION : u32 = 74; // INT

const FIELD_TRADEDATE : u32 = 75; // LOCALMKTDATE

const FIELD_EXECBROKER : u32 = 76; // STRING

const FIELD_OPENCLOSE : u32 = 77; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_OpenClose_Enum {
    CLOSE, // = "C"
    OPEN, // = "O"

    Undefined
}

impl FromStr for Field_OpenClose_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(Field_OpenClose_Enum::CLOSE),
            "O" => Ok(Field_OpenClose_Enum::OPEN),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_OpenClose_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_OpenClose_Enum::CLOSE => {
                f.write_str( "C" )
            },
            &Field_OpenClose_Enum::OPEN => {
                f.write_str( "O" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_OpenClose_Enum {
    fn default() -> Self {
        Field_OpenClose_Enum::Undefined
    }
}
const FIELD_NOALLOCS : u32 = 78; // INT

const FIELD_ALLOCACCOUNT : u32 = 79; // STRING

const FIELD_ALLOCSHARES : u32 = 80; // QTY

const FIELD_PROCESSCODE : u32 = 81; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_ProcessCode_Enum {
    REGULAR, // = "0"
    SOFT_DOLLAR, // = "1"
    STEP_IN, // = "2"
    STEP_OUT, // = "3"
    SOFT_DOLLAR_STEP_IN, // = "4"
    SOFT_DOLLAR_STEP_OUT, // = "5"
    PLAN_SPONSOR, // = "6"

    Undefined
}

impl FromStr for Field_ProcessCode_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_ProcessCode_Enum::REGULAR),
            "1" => Ok(Field_ProcessCode_Enum::SOFT_DOLLAR),
            "2" => Ok(Field_ProcessCode_Enum::STEP_IN),
            "3" => Ok(Field_ProcessCode_Enum::STEP_OUT),
            "4" => Ok(Field_ProcessCode_Enum::SOFT_DOLLAR_STEP_IN),
            "5" => Ok(Field_ProcessCode_Enum::SOFT_DOLLAR_STEP_OUT),
            "6" => Ok(Field_ProcessCode_Enum::PLAN_SPONSOR),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_ProcessCode_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_ProcessCode_Enum::REGULAR => {
                f.write_str( "0" )
            },
            &Field_ProcessCode_Enum::SOFT_DOLLAR => {
                f.write_str( "1" )
            },
            &Field_ProcessCode_Enum::STEP_IN => {
                f.write_str( "2" )
            },
            &Field_ProcessCode_Enum::STEP_OUT => {
                f.write_str( "3" )
            },
            &Field_ProcessCode_Enum::SOFT_DOLLAR_STEP_IN => {
                f.write_str( "4" )
            },
            &Field_ProcessCode_Enum::SOFT_DOLLAR_STEP_OUT => {
                f.write_str( "5" )
            },
            &Field_ProcessCode_Enum::PLAN_SPONSOR => {
                f.write_str( "6" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_ProcessCode_Enum {
    fn default() -> Self {
        Field_ProcessCode_Enum::Undefined
    }
}
const FIELD_NORPTS : u32 = 82; // INT

const FIELD_RPTSEQ : u32 = 83; // INT

const FIELD_CXLQTY : u32 = 84; // QTY

const FIELD_NODLVYINST : u32 = 85; // INT

const FIELD_DLVYINST : u32 = 86; // STRING

const FIELD_ALLOCSTATUS : u32 = 87; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_AllocStatus_Enum {
    ACCEPTED, // = "0"
    REJECTED, // = "1"
    PARTIAL_ACCEPT, // = "2"
    RECEIVED, // = "3"

    Undefined
}

impl FromStr for Field_AllocStatus_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_AllocStatus_Enum::ACCEPTED),
            "1" => Ok(Field_AllocStatus_Enum::REJECTED),
            "2" => Ok(Field_AllocStatus_Enum::PARTIAL_ACCEPT),
            "3" => Ok(Field_AllocStatus_Enum::RECEIVED),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_AllocStatus_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_AllocStatus_Enum::ACCEPTED => {
                f.write_str( "0" )
            },
            &Field_AllocStatus_Enum::REJECTED => {
                f.write_str( "1" )
            },
            &Field_AllocStatus_Enum::PARTIAL_ACCEPT => {
                f.write_str( "2" )
            },
            &Field_AllocStatus_Enum::RECEIVED => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_AllocStatus_Enum {
    fn default() -> Self {
        Field_AllocStatus_Enum::Undefined
    }
}
const FIELD_ALLOCREJCODE : u32 = 88; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_AllocRejCode_Enum {
    UNKNOWN_ACCOUNT, // = "0"
    INCORRECT_QUANTITY, // = "1"
    INCORRECT_AVERAGE_PRICE, // = "2"
    UNKNOWN_EXECUTING_BROKER_MNEMONIC, // = "3"
    COMMISSION_DIFFERENCE, // = "4"
    UNKNOWN_ORDERID, // = "5"
    UNKNOWN_LISTID, // = "6"
    OTHER, // = "7"

    Undefined
}

impl FromStr for Field_AllocRejCode_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_AllocRejCode_Enum::UNKNOWN_ACCOUNT),
            "1" => Ok(Field_AllocRejCode_Enum::INCORRECT_QUANTITY),
            "2" => Ok(Field_AllocRejCode_Enum::INCORRECT_AVERAGE_PRICE),
            "3" => Ok(Field_AllocRejCode_Enum::UNKNOWN_EXECUTING_BROKER_MNEMONIC),
            "4" => Ok(Field_AllocRejCode_Enum::COMMISSION_DIFFERENCE),
            "5" => Ok(Field_AllocRejCode_Enum::UNKNOWN_ORDERID),
            "6" => Ok(Field_AllocRejCode_Enum::UNKNOWN_LISTID),
            "7" => Ok(Field_AllocRejCode_Enum::OTHER),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_AllocRejCode_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_AllocRejCode_Enum::UNKNOWN_ACCOUNT => {
                f.write_str( "0" )
            },
            &Field_AllocRejCode_Enum::INCORRECT_QUANTITY => {
                f.write_str( "1" )
            },
            &Field_AllocRejCode_Enum::INCORRECT_AVERAGE_PRICE => {
                f.write_str( "2" )
            },
            &Field_AllocRejCode_Enum::UNKNOWN_EXECUTING_BROKER_MNEMONIC => {
                f.write_str( "3" )
            },
            &Field_AllocRejCode_Enum::COMMISSION_DIFFERENCE => {
                f.write_str( "4" )
            },
            &Field_AllocRejCode_Enum::UNKNOWN_ORDERID => {
                f.write_str( "5" )
            },
            &Field_AllocRejCode_Enum::UNKNOWN_LISTID => {
                f.write_str( "6" )
            },
            &Field_AllocRejCode_Enum::OTHER => {
                f.write_str( "7" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_AllocRejCode_Enum {
    fn default() -> Self {
        Field_AllocRejCode_Enum::Undefined
    }
}
const FIELD_SIGNATURE : u32 = 89; // DATA

const FIELD_SECUREDATALEN : u32 = 90; // LENGTH

const FIELD_SECUREDATA : u32 = 91; // DATA

const FIELD_BROKEROFCREDIT : u32 = 92; // STRING

const FIELD_SIGNATURELENGTH : u32 = 93; // LENGTH

const FIELD_EMAILTYPE : u32 = 94; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_EmailType_Enum {
    NEW, // = "0"
    REPLY, // = "1"
    ADMIN_REPLY, // = "2"

    Undefined
}

impl FromStr for Field_EmailType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_EmailType_Enum::NEW),
            "1" => Ok(Field_EmailType_Enum::REPLY),
            "2" => Ok(Field_EmailType_Enum::ADMIN_REPLY),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_EmailType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_EmailType_Enum::NEW => {
                f.write_str( "0" )
            },
            &Field_EmailType_Enum::REPLY => {
                f.write_str( "1" )
            },
            &Field_EmailType_Enum::ADMIN_REPLY => {
                f.write_str( "2" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_EmailType_Enum {
    fn default() -> Self {
        Field_EmailType_Enum::Undefined
    }
}
const FIELD_RAWDATALENGTH : u32 = 95; // LENGTH

const FIELD_RAWDATA : u32 = 96; // DATA

const FIELD_POSSRESEND : u32 = 97; // BOOLEAN

const FIELD_ENCRYPTMETHOD : u32 = 98; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_EncryptMethod_Enum {
    NONE, // = "0"
    PKCS, // = "1"
    DES, // = "2"
    PKCS_DES, // = "3"
    PGP_DES, // = "4"
    PGP_DES_MD5, // = "5"
    PEM_DES_MD5, // = "6"

    Undefined
}

impl FromStr for Field_EncryptMethod_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_EncryptMethod_Enum::NONE),
            "1" => Ok(Field_EncryptMethod_Enum::PKCS),
            "2" => Ok(Field_EncryptMethod_Enum::DES),
            "3" => Ok(Field_EncryptMethod_Enum::PKCS_DES),
            "4" => Ok(Field_EncryptMethod_Enum::PGP_DES),
            "5" => Ok(Field_EncryptMethod_Enum::PGP_DES_MD5),
            "6" => Ok(Field_EncryptMethod_Enum::PEM_DES_MD5),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_EncryptMethod_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_EncryptMethod_Enum::NONE => {
                f.write_str( "0" )
            },
            &Field_EncryptMethod_Enum::PKCS => {
                f.write_str( "1" )
            },
            &Field_EncryptMethod_Enum::DES => {
                f.write_str( "2" )
            },
            &Field_EncryptMethod_Enum::PKCS_DES => {
                f.write_str( "3" )
            },
            &Field_EncryptMethod_Enum::PGP_DES => {
                f.write_str( "4" )
            },
            &Field_EncryptMethod_Enum::PGP_DES_MD5 => {
                f.write_str( "5" )
            },
            &Field_EncryptMethod_Enum::PEM_DES_MD5 => {
                f.write_str( "6" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_EncryptMethod_Enum {
    fn default() -> Self {
        Field_EncryptMethod_Enum::Undefined
    }
}
const FIELD_STOPPX : u32 = 99; // PRICE

const FIELD_EXDESTINATION : u32 = 100; // EXCHANGE

const FIELD_CXLREJREASON : u32 = 102; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_CxlRejReason_Enum {
    TOO_LATE_TO_CANCEL, // = "0"
    UNKNOWN_ORDER, // = "1"
    BROKER_OPTION, // = "2"
    ORDER_ALREADY_IN_PENDING_CANCEL_OR_PENDING_REPLACE_STATUS, // = "3"

    Undefined
}

impl FromStr for Field_CxlRejReason_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_CxlRejReason_Enum::TOO_LATE_TO_CANCEL),
            "1" => Ok(Field_CxlRejReason_Enum::UNKNOWN_ORDER),
            "2" => Ok(Field_CxlRejReason_Enum::BROKER_OPTION),
            "3" => Ok(Field_CxlRejReason_Enum::ORDER_ALREADY_IN_PENDING_CANCEL_OR_PENDING_REPLACE_STATUS),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_CxlRejReason_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_CxlRejReason_Enum::TOO_LATE_TO_CANCEL => {
                f.write_str( "0" )
            },
            &Field_CxlRejReason_Enum::UNKNOWN_ORDER => {
                f.write_str( "1" )
            },
            &Field_CxlRejReason_Enum::BROKER_OPTION => {
                f.write_str( "2" )
            },
            &Field_CxlRejReason_Enum::ORDER_ALREADY_IN_PENDING_CANCEL_OR_PENDING_REPLACE_STATUS => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_CxlRejReason_Enum {
    fn default() -> Self {
        Field_CxlRejReason_Enum::Undefined
    }
}
const FIELD_ORDREJREASON : u32 = 103; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_OrdRejReason_Enum {
    BROKER_OPTION, // = "0"
    UNKNOWN_SYMBOL, // = "1"
    EXCHANGE_CLOSED, // = "2"
    ORDER_EXCEEDS_LIMIT, // = "3"
    TOO_LATE_TO_ENTER, // = "4"
    UNKNOWN_ORDER, // = "5"
    DUPLICATE_ORDER, // = "6"
    DUPLICATE_OF_A_VERBALLY_COMMUNICATED_ORDER, // = "7"
    STALE_ORDER, // = "8"

    Undefined
}

impl FromStr for Field_OrdRejReason_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_OrdRejReason_Enum::BROKER_OPTION),
            "1" => Ok(Field_OrdRejReason_Enum::UNKNOWN_SYMBOL),
            "2" => Ok(Field_OrdRejReason_Enum::EXCHANGE_CLOSED),
            "3" => Ok(Field_OrdRejReason_Enum::ORDER_EXCEEDS_LIMIT),
            "4" => Ok(Field_OrdRejReason_Enum::TOO_LATE_TO_ENTER),
            "5" => Ok(Field_OrdRejReason_Enum::UNKNOWN_ORDER),
            "6" => Ok(Field_OrdRejReason_Enum::DUPLICATE_ORDER),
            "7" => Ok(Field_OrdRejReason_Enum::DUPLICATE_OF_A_VERBALLY_COMMUNICATED_ORDER),
            "8" => Ok(Field_OrdRejReason_Enum::STALE_ORDER),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_OrdRejReason_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_OrdRejReason_Enum::BROKER_OPTION => {
                f.write_str( "0" )
            },
            &Field_OrdRejReason_Enum::UNKNOWN_SYMBOL => {
                f.write_str( "1" )
            },
            &Field_OrdRejReason_Enum::EXCHANGE_CLOSED => {
                f.write_str( "2" )
            },
            &Field_OrdRejReason_Enum::ORDER_EXCEEDS_LIMIT => {
                f.write_str( "3" )
            },
            &Field_OrdRejReason_Enum::TOO_LATE_TO_ENTER => {
                f.write_str( "4" )
            },
            &Field_OrdRejReason_Enum::UNKNOWN_ORDER => {
                f.write_str( "5" )
            },
            &Field_OrdRejReason_Enum::DUPLICATE_ORDER => {
                f.write_str( "6" )
            },
            &Field_OrdRejReason_Enum::DUPLICATE_OF_A_VERBALLY_COMMUNICATED_ORDER => {
                f.write_str( "7" )
            },
            &Field_OrdRejReason_Enum::STALE_ORDER => {
                f.write_str( "8" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_OrdRejReason_Enum {
    fn default() -> Self {
        Field_OrdRejReason_Enum::Undefined
    }
}
const FIELD_IOIQUALIFIER : u32 = 104; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_IOIQualifier_Enum {
    ALL_OR_NONE, // = "A"
    AT_THE_CLOSE, // = "C"
    IN_TOUCH_WITH, // = "I"
    LIMIT, // = "L"
    MORE_BEHIND, // = "M"
    AT_THE_OPEN, // = "O"
    TAKING_A_POSITION, // = "P"
    AT_THE_MARKET, // = "Q"
    READY_TO_TRADE, // = "R"
    PORTFOLIO_SHOW_N, // = "S"
    THROUGH_THE_DAY, // = "T"
    VERSUS, // = "V"
    INDICATION, // = "W"
    CROSSING_OPPORTUNITY, // = "X"
    AT_THE_MIDPOINT, // = "Y"
    PRE_OPEN, // = "Z"

    Undefined
}

impl FromStr for Field_IOIQualifier_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Field_IOIQualifier_Enum::ALL_OR_NONE),
            "C" => Ok(Field_IOIQualifier_Enum::AT_THE_CLOSE),
            "I" => Ok(Field_IOIQualifier_Enum::IN_TOUCH_WITH),
            "L" => Ok(Field_IOIQualifier_Enum::LIMIT),
            "M" => Ok(Field_IOIQualifier_Enum::MORE_BEHIND),
            "O" => Ok(Field_IOIQualifier_Enum::AT_THE_OPEN),
            "P" => Ok(Field_IOIQualifier_Enum::TAKING_A_POSITION),
            "Q" => Ok(Field_IOIQualifier_Enum::AT_THE_MARKET),
            "R" => Ok(Field_IOIQualifier_Enum::READY_TO_TRADE),
            "S" => Ok(Field_IOIQualifier_Enum::PORTFOLIO_SHOW_N),
            "T" => Ok(Field_IOIQualifier_Enum::THROUGH_THE_DAY),
            "V" => Ok(Field_IOIQualifier_Enum::VERSUS),
            "W" => Ok(Field_IOIQualifier_Enum::INDICATION),
            "X" => Ok(Field_IOIQualifier_Enum::CROSSING_OPPORTUNITY),
            "Y" => Ok(Field_IOIQualifier_Enum::AT_THE_MIDPOINT),
            "Z" => Ok(Field_IOIQualifier_Enum::PRE_OPEN),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_IOIQualifier_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_IOIQualifier_Enum::ALL_OR_NONE => {
                f.write_str( "A" )
            },
            &Field_IOIQualifier_Enum::AT_THE_CLOSE => {
                f.write_str( "C" )
            },
            &Field_IOIQualifier_Enum::IN_TOUCH_WITH => {
                f.write_str( "I" )
            },
            &Field_IOIQualifier_Enum::LIMIT => {
                f.write_str( "L" )
            },
            &Field_IOIQualifier_Enum::MORE_BEHIND => {
                f.write_str( "M" )
            },
            &Field_IOIQualifier_Enum::AT_THE_OPEN => {
                f.write_str( "O" )
            },
            &Field_IOIQualifier_Enum::TAKING_A_POSITION => {
                f.write_str( "P" )
            },
            &Field_IOIQualifier_Enum::AT_THE_MARKET => {
                f.write_str( "Q" )
            },
            &Field_IOIQualifier_Enum::READY_TO_TRADE => {
                f.write_str( "R" )
            },
            &Field_IOIQualifier_Enum::PORTFOLIO_SHOW_N => {
                f.write_str( "S" )
            },
            &Field_IOIQualifier_Enum::THROUGH_THE_DAY => {
                f.write_str( "T" )
            },
            &Field_IOIQualifier_Enum::VERSUS => {
                f.write_str( "V" )
            },
            &Field_IOIQualifier_Enum::INDICATION => {
                f.write_str( "W" )
            },
            &Field_IOIQualifier_Enum::CROSSING_OPPORTUNITY => {
                f.write_str( "X" )
            },
            &Field_IOIQualifier_Enum::AT_THE_MIDPOINT => {
                f.write_str( "Y" )
            },
            &Field_IOIQualifier_Enum::PRE_OPEN => {
                f.write_str( "Z" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_IOIQualifier_Enum {
    fn default() -> Self {
        Field_IOIQualifier_Enum::Undefined
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_DKReason_Enum {
    UNKNOWN_SYMBOL, // = "A"
    WRONG_SIDE, // = "B"
    QUANTITY_EXCEEDS_ORDER, // = "C"
    NO_MATCHING_ORDER, // = "D"
    PRICE_EXCEEDS_LIMIT, // = "E"
    OTHER, // = "Z"

    Undefined
}

impl FromStr for Field_DKReason_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Field_DKReason_Enum::UNKNOWN_SYMBOL),
            "B" => Ok(Field_DKReason_Enum::WRONG_SIDE),
            "C" => Ok(Field_DKReason_Enum::QUANTITY_EXCEEDS_ORDER),
            "D" => Ok(Field_DKReason_Enum::NO_MATCHING_ORDER),
            "E" => Ok(Field_DKReason_Enum::PRICE_EXCEEDS_LIMIT),
            "Z" => Ok(Field_DKReason_Enum::OTHER),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_DKReason_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_DKReason_Enum::UNKNOWN_SYMBOL => {
                f.write_str( "A" )
            },
            &Field_DKReason_Enum::WRONG_SIDE => {
                f.write_str( "B" )
            },
            &Field_DKReason_Enum::QUANTITY_EXCEEDS_ORDER => {
                f.write_str( "C" )
            },
            &Field_DKReason_Enum::NO_MATCHING_ORDER => {
                f.write_str( "D" )
            },
            &Field_DKReason_Enum::PRICE_EXCEEDS_LIMIT => {
                f.write_str( "E" )
            },
            &Field_DKReason_Enum::OTHER => {
                f.write_str( "Z" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_DKReason_Enum {
    fn default() -> Self {
        Field_DKReason_Enum::Undefined
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_MiscFeeType_Enum {
    REGULATORY, // = "1"
    TAX, // = "2"
    LOCAL_COMMISSION, // = "3"
    EXCHANGE_FEES, // = "4"
    STAMP, // = "5"
    LEVY, // = "6"
    OTHER, // = "7"
    MARKUP, // = "8"
    CONSUMPTION_TAX, // = "9"

    Undefined
}

impl FromStr for Field_MiscFeeType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_MiscFeeType_Enum::REGULATORY),
            "2" => Ok(Field_MiscFeeType_Enum::TAX),
            "3" => Ok(Field_MiscFeeType_Enum::LOCAL_COMMISSION),
            "4" => Ok(Field_MiscFeeType_Enum::EXCHANGE_FEES),
            "5" => Ok(Field_MiscFeeType_Enum::STAMP),
            "6" => Ok(Field_MiscFeeType_Enum::LEVY),
            "7" => Ok(Field_MiscFeeType_Enum::OTHER),
            "8" => Ok(Field_MiscFeeType_Enum::MARKUP),
            "9" => Ok(Field_MiscFeeType_Enum::CONSUMPTION_TAX),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_MiscFeeType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_MiscFeeType_Enum::REGULATORY => {
                f.write_str( "1" )
            },
            &Field_MiscFeeType_Enum::TAX => {
                f.write_str( "2" )
            },
            &Field_MiscFeeType_Enum::LOCAL_COMMISSION => {
                f.write_str( "3" )
            },
            &Field_MiscFeeType_Enum::EXCHANGE_FEES => {
                f.write_str( "4" )
            },
            &Field_MiscFeeType_Enum::STAMP => {
                f.write_str( "5" )
            },
            &Field_MiscFeeType_Enum::LEVY => {
                f.write_str( "6" )
            },
            &Field_MiscFeeType_Enum::OTHER => {
                f.write_str( "7" )
            },
            &Field_MiscFeeType_Enum::MARKUP => {
                f.write_str( "8" )
            },
            &Field_MiscFeeType_Enum::CONSUMPTION_TAX => {
                f.write_str( "9" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_MiscFeeType_Enum {
    fn default() -> Self {
        Field_MiscFeeType_Enum::Undefined
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_ExecType_Enum {
    NEW, // = "0"
    PARTIAL_FILL, // = "1"
    FILL, // = "2"
    DONE_FOR_DAY, // = "3"
    CANCELED, // = "4"
    REPLACE, // = "5"
    PENDING_CANCEL, // = "6"
    STOPPED, // = "7"
    REJECTED, // = "8"
    SUSPENDED, // = "9"
    PENDING_NEW, // = "A"
    CALCULATED, // = "B"
    EXPIRED, // = "C"
    RESTATED, // = "D"
    PENDING_REPLACE, // = "E"

    Undefined
}

impl FromStr for Field_ExecType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_ExecType_Enum::NEW),
            "1" => Ok(Field_ExecType_Enum::PARTIAL_FILL),
            "2" => Ok(Field_ExecType_Enum::FILL),
            "3" => Ok(Field_ExecType_Enum::DONE_FOR_DAY),
            "4" => Ok(Field_ExecType_Enum::CANCELED),
            "5" => Ok(Field_ExecType_Enum::REPLACE),
            "6" => Ok(Field_ExecType_Enum::PENDING_CANCEL),
            "7" => Ok(Field_ExecType_Enum::STOPPED),
            "8" => Ok(Field_ExecType_Enum::REJECTED),
            "9" => Ok(Field_ExecType_Enum::SUSPENDED),
            "A" => Ok(Field_ExecType_Enum::PENDING_NEW),
            "B" => Ok(Field_ExecType_Enum::CALCULATED),
            "C" => Ok(Field_ExecType_Enum::EXPIRED),
            "D" => Ok(Field_ExecType_Enum::RESTATED),
            "E" => Ok(Field_ExecType_Enum::PENDING_REPLACE),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_ExecType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_ExecType_Enum::NEW => {
                f.write_str( "0" )
            },
            &Field_ExecType_Enum::PARTIAL_FILL => {
                f.write_str( "1" )
            },
            &Field_ExecType_Enum::FILL => {
                f.write_str( "2" )
            },
            &Field_ExecType_Enum::DONE_FOR_DAY => {
                f.write_str( "3" )
            },
            &Field_ExecType_Enum::CANCELED => {
                f.write_str( "4" )
            },
            &Field_ExecType_Enum::REPLACE => {
                f.write_str( "5" )
            },
            &Field_ExecType_Enum::PENDING_CANCEL => {
                f.write_str( "6" )
            },
            &Field_ExecType_Enum::STOPPED => {
                f.write_str( "7" )
            },
            &Field_ExecType_Enum::REJECTED => {
                f.write_str( "8" )
            },
            &Field_ExecType_Enum::SUSPENDED => {
                f.write_str( "9" )
            },
            &Field_ExecType_Enum::PENDING_NEW => {
                f.write_str( "A" )
            },
            &Field_ExecType_Enum::CALCULATED => {
                f.write_str( "B" )
            },
            &Field_ExecType_Enum::EXPIRED => {
                f.write_str( "C" )
            },
            &Field_ExecType_Enum::RESTATED => {
                f.write_str( "D" )
            },
            &Field_ExecType_Enum::PENDING_REPLACE => {
                f.write_str( "E" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_ExecType_Enum {
    fn default() -> Self {
        Field_ExecType_Enum::Undefined
    }
}
const FIELD_LEAVESQTY : u32 = 151; // QTY

const FIELD_CASHORDERQTY : u32 = 152; // QTY

const FIELD_ALLOCAVGPX : u32 = 153; // PRICE

const FIELD_ALLOCNETMONEY : u32 = 154; // AMT

const FIELD_SETTLCURRFXRATE : u32 = 155; // FLOAT

const FIELD_SETTLCURRFXRATECALC : u32 = 156; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_SettlCurrFxRateCalc_Enum {
    MULTIPLY, // = "M"
    DIVIDE, // = "D"

    Undefined
}

impl FromStr for Field_SettlCurrFxRateCalc_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "M" => Ok(Field_SettlCurrFxRateCalc_Enum::MULTIPLY),
            "D" => Ok(Field_SettlCurrFxRateCalc_Enum::DIVIDE),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_SettlCurrFxRateCalc_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_SettlCurrFxRateCalc_Enum::MULTIPLY => {
                f.write_str( "M" )
            },
            &Field_SettlCurrFxRateCalc_Enum::DIVIDE => {
                f.write_str( "D" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_SettlCurrFxRateCalc_Enum {
    fn default() -> Self {
        Field_SettlCurrFxRateCalc_Enum::Undefined
    }
}
const FIELD_NUMDAYSINTEREST : u32 = 157; // INT

const FIELD_ACCRUEDINTERESTRATE : u32 = 158; // FLOAT

const FIELD_ACCRUEDINTERESTAMT : u32 = 159; // AMT

const FIELD_SETTLINSTMODE : u32 = 160; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_SettlInstMode_Enum {
    DEFAULT, // = "0"
    STANDING_INSTRUCTIONS_PROVIDED, // = "1"
    SPECIFIC_ALLOCATION_ACCOUNT_OVERRIDING, // = "2"
    SPECIFIC_ALLOCATION_ACCOUNT_STANDING, // = "3"

    Undefined
}

impl FromStr for Field_SettlInstMode_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_SettlInstMode_Enum::DEFAULT),
            "1" => Ok(Field_SettlInstMode_Enum::STANDING_INSTRUCTIONS_PROVIDED),
            "2" => Ok(Field_SettlInstMode_Enum::SPECIFIC_ALLOCATION_ACCOUNT_OVERRIDING),
            "3" => Ok(Field_SettlInstMode_Enum::SPECIFIC_ALLOCATION_ACCOUNT_STANDING),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_SettlInstMode_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_SettlInstMode_Enum::DEFAULT => {
                f.write_str( "0" )
            },
            &Field_SettlInstMode_Enum::STANDING_INSTRUCTIONS_PROVIDED => {
                f.write_str( "1" )
            },
            &Field_SettlInstMode_Enum::SPECIFIC_ALLOCATION_ACCOUNT_OVERRIDING => {
                f.write_str( "2" )
            },
            &Field_SettlInstMode_Enum::SPECIFIC_ALLOCATION_ACCOUNT_STANDING => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_SettlInstMode_Enum {
    fn default() -> Self {
        Field_SettlInstMode_Enum::Undefined
    }
}
const FIELD_ALLOCTEXT : u32 = 161; // STRING

const FIELD_SETTLINSTID : u32 = 162; // STRING

const FIELD_SETTLINSTTRANSTYPE : u32 = 163; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_SettlInstTransType_Enum {
    CANCEL, // = "C"
    NEW, // = "N"
    REPLACE, // = "R"

    Undefined
}

impl FromStr for Field_SettlInstTransType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(Field_SettlInstTransType_Enum::CANCEL),
            "N" => Ok(Field_SettlInstTransType_Enum::NEW),
            "R" => Ok(Field_SettlInstTransType_Enum::REPLACE),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_SettlInstTransType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_SettlInstTransType_Enum::CANCEL => {
                f.write_str( "C" )
            },
            &Field_SettlInstTransType_Enum::NEW => {
                f.write_str( "N" )
            },
            &Field_SettlInstTransType_Enum::REPLACE => {
                f.write_str( "R" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_SettlInstTransType_Enum {
    fn default() -> Self {
        Field_SettlInstTransType_Enum::Undefined
    }
}
const FIELD_EMAILTHREADID : u32 = 164; // STRING

const FIELD_SETTLINSTSOURCE : u32 = 165; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_SettlInstSource_Enum {
    BROKERS_INSTRUCTIONS, // = "1"
    INSTITUTIONS_INSTRUCTIONS, // = "2"

    Undefined
}

impl FromStr for Field_SettlInstSource_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_SettlInstSource_Enum::BROKERS_INSTRUCTIONS),
            "2" => Ok(Field_SettlInstSource_Enum::INSTITUTIONS_INSTRUCTIONS),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_SettlInstSource_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_SettlInstSource_Enum::BROKERS_INSTRUCTIONS => {
                f.write_str( "1" )
            },
            &Field_SettlInstSource_Enum::INSTITUTIONS_INSTRUCTIONS => {
                f.write_str( "2" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_SettlInstSource_Enum {
    fn default() -> Self {
        Field_SettlInstSource_Enum::Undefined
    }
}
const FIELD_SETTLLOCATION : u32 = 166; // STRING
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_SettlLocation_Enum {
    CEDEL, // = "CED"
    DEPOSITORY_TRUST_COMPANY, // = "DTC"
    EUROCLEAR, // = "EUR"
    FEDERAL_BOOK_ENTRY, // = "FED"
    LOCAL_MARKET_SETTLE_LOCATION, // = "ISO Country Code"
    PHYSICAL, // = "PNY"
    PARTICIPANT_TRUST_COMPANY, // = "PTC"

    Undefined
}

impl FromStr for Field_SettlLocation_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CED" => Ok(Field_SettlLocation_Enum::CEDEL),
            "DTC" => Ok(Field_SettlLocation_Enum::DEPOSITORY_TRUST_COMPANY),
            "EUR" => Ok(Field_SettlLocation_Enum::EUROCLEAR),
            "FED" => Ok(Field_SettlLocation_Enum::FEDERAL_BOOK_ENTRY),
            "ISO Country Code" => Ok(Field_SettlLocation_Enum::LOCAL_MARKET_SETTLE_LOCATION),
            "PNY" => Ok(Field_SettlLocation_Enum::PHYSICAL),
            "PTC" => Ok(Field_SettlLocation_Enum::PARTICIPANT_TRUST_COMPANY),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_SettlLocation_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_SettlLocation_Enum::CEDEL => {
                f.write_str( "CED" )
            },
            &Field_SettlLocation_Enum::DEPOSITORY_TRUST_COMPANY => {
                f.write_str( "DTC" )
            },
            &Field_SettlLocation_Enum::EUROCLEAR => {
                f.write_str( "EUR" )
            },
            &Field_SettlLocation_Enum::FEDERAL_BOOK_ENTRY => {
                f.write_str( "FED" )
            },
            &Field_SettlLocation_Enum::LOCAL_MARKET_SETTLE_LOCATION => {
                f.write_str( "ISO Country Code" )
            },
            &Field_SettlLocation_Enum::PHYSICAL => {
                f.write_str( "PNY" )
            },
            &Field_SettlLocation_Enum::PARTICIPANT_TRUST_COMPANY => {
                f.write_str( "PTC" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_SettlLocation_Enum {
    fn default() -> Self {
        Field_SettlLocation_Enum::Undefined
    }
}
const FIELD_SECURITYTYPE : u32 = 167; // STRING
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_SecurityType_Enum {
    WILDCARD_ENTRY, // = "?"
    BANKERS_ACCEPTANCE, // = "BA"
    CONVERTIBLE_BOND, // = "CB"
    CERTIFICATE_OF_DEPOSIT, // = "CD"
    COLLATERALIZE_MORTGAGE_OBLIGATION, // = "CMO"
    CORPORATE_BOND, // = "CORP"
    COMMERCIAL_PAPER, // = "CP"
    CORPORATE_PRIVATE_PLACEMENT, // = "CPP"
    COMMON_STOCK, // = "CS"
    FEDERAL_HOUSING_AUTHORITY, // = "FHA"
    FEDERAL_HOME_LOAN, // = "FHL"
    FEDERAL_NATIONAL_MORTGAGE_ASSOCIATION, // = "FN"
    FOREIGN_EXCHANGE_CONTRACT, // = "FOR"
    FUTURE, // = "FUT"
    GOVERNMENT_NATIONAL_MORTGAGE_ASSOCIATION, // = "GN"
    TREASURIES_PLUS_AGENCY_DEBENTURE, // = "GOVT"
    MORTGAGE_IOETTE, // = "IET"
    MUTUAL_FUND, // = "MF"
    MORTGAGE_INTEREST_ONLY, // = "MIO"
    MORTGAGE_PRINCIPAL_ONLY, // = "MPO"
    MORTGAGE_PRIVATE_PLACEMENT, // = "MPP"
    MISCELLANEOUS_PASS_THRU, // = "MPT"
    MUNICIPAL_BOND, // = "MUNI"
    NO_ISITC_SECURITY_TYPE, // = "NONE"
    OPTION, // = "OPT"
    PREFERRED_STOCK, // = "PS"
    REPURCHASE_AGREEMENT, // = "RP"
    REVERSE_REPURCHASE_AGREEMENT, // = "RVRP"
    STUDENT_LOAN_MARKETING_ASSOCIATION, // = "SL"
    TIME_DEPOSIT, // = "TD"
    US_TREASURY_BILL, // = "USTB"
    WARRANT, // = "WAR"
    CATS_TIGERS_LIONS, // = "ZOO"

    Undefined
}

impl FromStr for Field_SecurityType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "?" => Ok(Field_SecurityType_Enum::WILDCARD_ENTRY),
            "BA" => Ok(Field_SecurityType_Enum::BANKERS_ACCEPTANCE),
            "CB" => Ok(Field_SecurityType_Enum::CONVERTIBLE_BOND),
            "CD" => Ok(Field_SecurityType_Enum::CERTIFICATE_OF_DEPOSIT),
            "CMO" => Ok(Field_SecurityType_Enum::COLLATERALIZE_MORTGAGE_OBLIGATION),
            "CORP" => Ok(Field_SecurityType_Enum::CORPORATE_BOND),
            "CP" => Ok(Field_SecurityType_Enum::COMMERCIAL_PAPER),
            "CPP" => Ok(Field_SecurityType_Enum::CORPORATE_PRIVATE_PLACEMENT),
            "CS" => Ok(Field_SecurityType_Enum::COMMON_STOCK),
            "FHA" => Ok(Field_SecurityType_Enum::FEDERAL_HOUSING_AUTHORITY),
            "FHL" => Ok(Field_SecurityType_Enum::FEDERAL_HOME_LOAN),
            "FN" => Ok(Field_SecurityType_Enum::FEDERAL_NATIONAL_MORTGAGE_ASSOCIATION),
            "FOR" => Ok(Field_SecurityType_Enum::FOREIGN_EXCHANGE_CONTRACT),
            "FUT" => Ok(Field_SecurityType_Enum::FUTURE),
            "GN" => Ok(Field_SecurityType_Enum::GOVERNMENT_NATIONAL_MORTGAGE_ASSOCIATION),
            "GOVT" => Ok(Field_SecurityType_Enum::TREASURIES_PLUS_AGENCY_DEBENTURE),
            "IET" => Ok(Field_SecurityType_Enum::MORTGAGE_IOETTE),
            "MF" => Ok(Field_SecurityType_Enum::MUTUAL_FUND),
            "MIO" => Ok(Field_SecurityType_Enum::MORTGAGE_INTEREST_ONLY),
            "MPO" => Ok(Field_SecurityType_Enum::MORTGAGE_PRINCIPAL_ONLY),
            "MPP" => Ok(Field_SecurityType_Enum::MORTGAGE_PRIVATE_PLACEMENT),
            "MPT" => Ok(Field_SecurityType_Enum::MISCELLANEOUS_PASS_THRU),
            "MUNI" => Ok(Field_SecurityType_Enum::MUNICIPAL_BOND),
            "NONE" => Ok(Field_SecurityType_Enum::NO_ISITC_SECURITY_TYPE),
            "OPT" => Ok(Field_SecurityType_Enum::OPTION),
            "PS" => Ok(Field_SecurityType_Enum::PREFERRED_STOCK),
            "RP" => Ok(Field_SecurityType_Enum::REPURCHASE_AGREEMENT),
            "RVRP" => Ok(Field_SecurityType_Enum::REVERSE_REPURCHASE_AGREEMENT),
            "SL" => Ok(Field_SecurityType_Enum::STUDENT_LOAN_MARKETING_ASSOCIATION),
            "TD" => Ok(Field_SecurityType_Enum::TIME_DEPOSIT),
            "USTB" => Ok(Field_SecurityType_Enum::US_TREASURY_BILL),
            "WAR" => Ok(Field_SecurityType_Enum::WARRANT),
            "ZOO" => Ok(Field_SecurityType_Enum::CATS_TIGERS_LIONS),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_SecurityType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_SecurityType_Enum::WILDCARD_ENTRY => {
                f.write_str( "?" )
            },
            &Field_SecurityType_Enum::BANKERS_ACCEPTANCE => {
                f.write_str( "BA" )
            },
            &Field_SecurityType_Enum::CONVERTIBLE_BOND => {
                f.write_str( "CB" )
            },
            &Field_SecurityType_Enum::CERTIFICATE_OF_DEPOSIT => {
                f.write_str( "CD" )
            },
            &Field_SecurityType_Enum::COLLATERALIZE_MORTGAGE_OBLIGATION => {
                f.write_str( "CMO" )
            },
            &Field_SecurityType_Enum::CORPORATE_BOND => {
                f.write_str( "CORP" )
            },
            &Field_SecurityType_Enum::COMMERCIAL_PAPER => {
                f.write_str( "CP" )
            },
            &Field_SecurityType_Enum::CORPORATE_PRIVATE_PLACEMENT => {
                f.write_str( "CPP" )
            },
            &Field_SecurityType_Enum::COMMON_STOCK => {
                f.write_str( "CS" )
            },
            &Field_SecurityType_Enum::FEDERAL_HOUSING_AUTHORITY => {
                f.write_str( "FHA" )
            },
            &Field_SecurityType_Enum::FEDERAL_HOME_LOAN => {
                f.write_str( "FHL" )
            },
            &Field_SecurityType_Enum::FEDERAL_NATIONAL_MORTGAGE_ASSOCIATION => {
                f.write_str( "FN" )
            },
            &Field_SecurityType_Enum::FOREIGN_EXCHANGE_CONTRACT => {
                f.write_str( "FOR" )
            },
            &Field_SecurityType_Enum::FUTURE => {
                f.write_str( "FUT" )
            },
            &Field_SecurityType_Enum::GOVERNMENT_NATIONAL_MORTGAGE_ASSOCIATION => {
                f.write_str( "GN" )
            },
            &Field_SecurityType_Enum::TREASURIES_PLUS_AGENCY_DEBENTURE => {
                f.write_str( "GOVT" )
            },
            &Field_SecurityType_Enum::MORTGAGE_IOETTE => {
                f.write_str( "IET" )
            },
            &Field_SecurityType_Enum::MUTUAL_FUND => {
                f.write_str( "MF" )
            },
            &Field_SecurityType_Enum::MORTGAGE_INTEREST_ONLY => {
                f.write_str( "MIO" )
            },
            &Field_SecurityType_Enum::MORTGAGE_PRINCIPAL_ONLY => {
                f.write_str( "MPO" )
            },
            &Field_SecurityType_Enum::MORTGAGE_PRIVATE_PLACEMENT => {
                f.write_str( "MPP" )
            },
            &Field_SecurityType_Enum::MISCELLANEOUS_PASS_THRU => {
                f.write_str( "MPT" )
            },
            &Field_SecurityType_Enum::MUNICIPAL_BOND => {
                f.write_str( "MUNI" )
            },
            &Field_SecurityType_Enum::NO_ISITC_SECURITY_TYPE => {
                f.write_str( "NONE" )
            },
            &Field_SecurityType_Enum::OPTION => {
                f.write_str( "OPT" )
            },
            &Field_SecurityType_Enum::PREFERRED_STOCK => {
                f.write_str( "PS" )
            },
            &Field_SecurityType_Enum::REPURCHASE_AGREEMENT => {
                f.write_str( "RP" )
            },
            &Field_SecurityType_Enum::REVERSE_REPURCHASE_AGREEMENT => {
                f.write_str( "RVRP" )
            },
            &Field_SecurityType_Enum::STUDENT_LOAN_MARKETING_ASSOCIATION => {
                f.write_str( "SL" )
            },
            &Field_SecurityType_Enum::TIME_DEPOSIT => {
                f.write_str( "TD" )
            },
            &Field_SecurityType_Enum::US_TREASURY_BILL => {
                f.write_str( "USTB" )
            },
            &Field_SecurityType_Enum::WARRANT => {
                f.write_str( "WAR" )
            },
            &Field_SecurityType_Enum::CATS_TIGERS_LIONS => {
                f.write_str( "ZOO" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_SecurityType_Enum {
    fn default() -> Self {
        Field_SecurityType_Enum::Undefined
    }
}
const FIELD_EFFECTIVETIME : u32 = 168; // UTCTIMESTAMP

const FIELD_STANDINSTDBTYPE : u32 = 169; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_StandInstDbType_Enum {
    OTHER, // = "0"
    DTC_SID, // = "1"
    THOMSON_ALERT, // = "2"
    A_GLOBAL_CUSTODIAN, // = "3"

    Undefined
}

impl FromStr for Field_StandInstDbType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_StandInstDbType_Enum::OTHER),
            "1" => Ok(Field_StandInstDbType_Enum::DTC_SID),
            "2" => Ok(Field_StandInstDbType_Enum::THOMSON_ALERT),
            "3" => Ok(Field_StandInstDbType_Enum::A_GLOBAL_CUSTODIAN),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_StandInstDbType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_StandInstDbType_Enum::OTHER => {
                f.write_str( "0" )
            },
            &Field_StandInstDbType_Enum::DTC_SID => {
                f.write_str( "1" )
            },
            &Field_StandInstDbType_Enum::THOMSON_ALERT => {
                f.write_str( "2" )
            },
            &Field_StandInstDbType_Enum::A_GLOBAL_CUSTODIAN => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_StandInstDbType_Enum {
    fn default() -> Self {
        Field_StandInstDbType_Enum::Undefined
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_AllocLinkType_Enum {
    F_X_NETTING, // = "0"
    F_X_SWAP, // = "1"

    Undefined
}

impl FromStr for Field_AllocLinkType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_AllocLinkType_Enum::F_X_NETTING),
            "1" => Ok(Field_AllocLinkType_Enum::F_X_SWAP),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_AllocLinkType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_AllocLinkType_Enum::F_X_NETTING => {
                f.write_str( "0" )
            },
            &Field_AllocLinkType_Enum::F_X_SWAP => {
                f.write_str( "1" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_AllocLinkType_Enum {
    fn default() -> Self {
        Field_AllocLinkType_Enum::Undefined
    }
}
const FIELD_SECONDARYORDERID : u32 = 198; // STRING

const FIELD_NOIOIQUALIFIERS : u32 = 199; // INT

const FIELD_MATURITYMONTHYEAR : u32 = 200; // MONTHYEAR

const FIELD_PUTORCALL : u32 = 201; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_PutOrCall_Enum {
    PUT, // = "0"
    CALL, // = "1"

    Undefined
}

impl FromStr for Field_PutOrCall_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_PutOrCall_Enum::PUT),
            "1" => Ok(Field_PutOrCall_Enum::CALL),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_PutOrCall_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_PutOrCall_Enum::PUT => {
                f.write_str( "0" )
            },
            &Field_PutOrCall_Enum::CALL => {
                f.write_str( "1" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_PutOrCall_Enum {
    fn default() -> Self {
        Field_PutOrCall_Enum::Undefined
    }
}
const FIELD_STRIKEPRICE : u32 = 202; // PRICE

const FIELD_COVEREDORUNCOVERED : u32 = 203; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_CoveredOrUncovered_Enum {
    COVERED, // = "0"
    UNCOVERED, // = "1"

    Undefined
}

impl FromStr for Field_CoveredOrUncovered_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_CoveredOrUncovered_Enum::COVERED),
            "1" => Ok(Field_CoveredOrUncovered_Enum::UNCOVERED),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_CoveredOrUncovered_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_CoveredOrUncovered_Enum::COVERED => {
                f.write_str( "0" )
            },
            &Field_CoveredOrUncovered_Enum::UNCOVERED => {
                f.write_str( "1" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_CoveredOrUncovered_Enum {
    fn default() -> Self {
        Field_CoveredOrUncovered_Enum::Undefined
    }
}
const FIELD_CUSTOMERORFIRM : u32 = 204; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_CustomerOrFirm_Enum {
    CUSTOMER, // = "0"
    FIRM, // = "1"

    Undefined
}

impl FromStr for Field_CustomerOrFirm_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_CustomerOrFirm_Enum::CUSTOMER),
            "1" => Ok(Field_CustomerOrFirm_Enum::FIRM),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_CustomerOrFirm_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_CustomerOrFirm_Enum::CUSTOMER => {
                f.write_str( "0" )
            },
            &Field_CustomerOrFirm_Enum::FIRM => {
                f.write_str( "1" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_CustomerOrFirm_Enum {
    fn default() -> Self {
        Field_CustomerOrFirm_Enum::Undefined
    }
}
const FIELD_MATURITYDAY : u32 = 205; // DAYOFMONTH

const FIELD_OPTATTRIBUTE : u32 = 206; // CHAR

const FIELD_SECURITYEXCHANGE : u32 = 207; // EXCHANGE

const FIELD_NOTIFYBROKEROFCREDIT : u32 = 208; // BOOLEAN

const FIELD_ALLOCHANDLINST : u32 = 209; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_AllocHandlInst_Enum {
    MATCH, // = "1"
    FORWARD, // = "2"
    FORWARD_AND_MATCH, // = "3"

    Undefined
}

impl FromStr for Field_AllocHandlInst_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_AllocHandlInst_Enum::MATCH),
            "2" => Ok(Field_AllocHandlInst_Enum::FORWARD),
            "3" => Ok(Field_AllocHandlInst_Enum::FORWARD_AND_MATCH),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_AllocHandlInst_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_AllocHandlInst_Enum::MATCH => {
                f.write_str( "1" )
            },
            &Field_AllocHandlInst_Enum::FORWARD => {
                f.write_str( "2" )
            },
            &Field_AllocHandlInst_Enum::FORWARD_AND_MATCH => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_AllocHandlInst_Enum {
    fn default() -> Self {
        Field_AllocHandlInst_Enum::Undefined
    }
}
const FIELD_MAXSHOW : u32 = 210; // QTY

const FIELD_PEGDIFFERENCE : u32 = 211; // PRICEOFFSET

const FIELD_XMLDATALEN : u32 = 212; // LENGTH

const FIELD_XMLDATA : u32 = 213; // DATA

const FIELD_SETTLINSTREFID : u32 = 214; // STRING

const FIELD_NOROUTINGIDS : u32 = 215; // INT

const FIELD_ROUTINGTYPE : u32 = 216; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_RoutingType_Enum {
    TARGET_FIRM, // = "1"
    TARGET_LIST, // = "2"
    BLOCK_FIRM, // = "3"
    BLOCK_LIST, // = "4"

    Undefined
}

impl FromStr for Field_RoutingType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_RoutingType_Enum::TARGET_FIRM),
            "2" => Ok(Field_RoutingType_Enum::TARGET_LIST),
            "3" => Ok(Field_RoutingType_Enum::BLOCK_FIRM),
            "4" => Ok(Field_RoutingType_Enum::BLOCK_LIST),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_RoutingType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_RoutingType_Enum::TARGET_FIRM => {
                f.write_str( "1" )
            },
            &Field_RoutingType_Enum::TARGET_LIST => {
                f.write_str( "2" )
            },
            &Field_RoutingType_Enum::BLOCK_FIRM => {
                f.write_str( "3" )
            },
            &Field_RoutingType_Enum::BLOCK_LIST => {
                f.write_str( "4" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_RoutingType_Enum {
    fn default() -> Self {
        Field_RoutingType_Enum::Undefined
    }
}
const FIELD_ROUTINGID : u32 = 217; // STRING

const FIELD_SPREADTOBENCHMARK : u32 = 218; // PRICEOFFSET

const FIELD_BENCHMARK : u32 = 219; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_Benchmark_Enum {
    CURVE, // = "1"
    _5_YR, // = "2"
    OLD_5, // = "3"
    _10_YR, // = "4"
    OLD_10, // = "5"
    _30_YR, // = "6"
    OLD_30, // = "7"
    _3_MO_LIBOR, // = "8"
    _6_MO_LIBOR, // = "9"

    Undefined
}

impl FromStr for Field_Benchmark_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_Benchmark_Enum::CURVE),
            "2" => Ok(Field_Benchmark_Enum::_5_YR),
            "3" => Ok(Field_Benchmark_Enum::OLD_5),
            "4" => Ok(Field_Benchmark_Enum::_10_YR),
            "5" => Ok(Field_Benchmark_Enum::OLD_10),
            "6" => Ok(Field_Benchmark_Enum::_30_YR),
            "7" => Ok(Field_Benchmark_Enum::OLD_30),
            "8" => Ok(Field_Benchmark_Enum::_3_MO_LIBOR),
            "9" => Ok(Field_Benchmark_Enum::_6_MO_LIBOR),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_Benchmark_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_Benchmark_Enum::CURVE => {
                f.write_str( "1" )
            },
            &Field_Benchmark_Enum::_5_YR => {
                f.write_str( "2" )
            },
            &Field_Benchmark_Enum::OLD_5 => {
                f.write_str( "3" )
            },
            &Field_Benchmark_Enum::_10_YR => {
                f.write_str( "4" )
            },
            &Field_Benchmark_Enum::OLD_10 => {
                f.write_str( "5" )
            },
            &Field_Benchmark_Enum::_30_YR => {
                f.write_str( "6" )
            },
            &Field_Benchmark_Enum::OLD_30 => {
                f.write_str( "7" )
            },
            &Field_Benchmark_Enum::_3_MO_LIBOR => {
                f.write_str( "8" )
            },
            &Field_Benchmark_Enum::_6_MO_LIBOR => {
                f.write_str( "9" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_Benchmark_Enum {
    fn default() -> Self {
        Field_Benchmark_Enum::Undefined
    }
}
const FIELD_COUPONRATE : u32 = 223; // FLOAT

const FIELD_CONTRACTMULTIPLIER : u32 = 231; // FLOAT

const FIELD_MDREQID : u32 = 262; // STRING

const FIELD_SUBSCRIPTIONREQUESTTYPE : u32 = 263; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_SubscriptionRequestType_Enum {
    SNAPSHOT, // = "0"
    SNAPSHOT_PLUS_UPDATES, // = "1"
    DISABLE_PREVIOUS_SNAPSHOT_PLUS_UPDATE_REQUEST, // = "2"

    Undefined
}

impl FromStr for Field_SubscriptionRequestType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_SubscriptionRequestType_Enum::SNAPSHOT),
            "1" => Ok(Field_SubscriptionRequestType_Enum::SNAPSHOT_PLUS_UPDATES),
            "2" => Ok(Field_SubscriptionRequestType_Enum::DISABLE_PREVIOUS_SNAPSHOT_PLUS_UPDATE_REQUEST),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_SubscriptionRequestType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_SubscriptionRequestType_Enum::SNAPSHOT => {
                f.write_str( "0" )
            },
            &Field_SubscriptionRequestType_Enum::SNAPSHOT_PLUS_UPDATES => {
                f.write_str( "1" )
            },
            &Field_SubscriptionRequestType_Enum::DISABLE_PREVIOUS_SNAPSHOT_PLUS_UPDATE_REQUEST => {
                f.write_str( "2" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_SubscriptionRequestType_Enum {
    fn default() -> Self {
        Field_SubscriptionRequestType_Enum::Undefined
    }
}
const FIELD_MARKETDEPTH : u32 = 264; // INT

const FIELD_MDUPDATETYPE : u32 = 265; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_MDUpdateType_Enum {
    FULL_REFRESH, // = "0"
    INCREMENTAL_REFRESH, // = "1"

    Undefined
}

impl FromStr for Field_MDUpdateType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_MDUpdateType_Enum::FULL_REFRESH),
            "1" => Ok(Field_MDUpdateType_Enum::INCREMENTAL_REFRESH),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_MDUpdateType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_MDUpdateType_Enum::FULL_REFRESH => {
                f.write_str( "0" )
            },
            &Field_MDUpdateType_Enum::INCREMENTAL_REFRESH => {
                f.write_str( "1" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_MDUpdateType_Enum {
    fn default() -> Self {
        Field_MDUpdateType_Enum::Undefined
    }
}
const FIELD_AGGREGATEDBOOK : u32 = 266; // BOOLEAN

const FIELD_NOMDENTRYTYPES : u32 = 267; // INT

const FIELD_NOMDENTRIES : u32 = 268; // INT

const FIELD_MDENTRYTYPE : u32 = 269; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_MDEntryType_Enum {
    BID, // = "0"
    OFFER, // = "1"
    TRADE, // = "2"
    INDEX_VALUE, // = "3"
    OPENING_PRICE, // = "4"
    CLOSING_PRICE, // = "5"
    SETTLEMENT_PRICE, // = "6"
    TRADING_SESSION_HIGH_PRICE, // = "7"
    TRADING_SESSION_LOW_PRICE, // = "8"
    TRADING_SESSION_VWAP_PRICE, // = "9"

    Undefined
}

impl FromStr for Field_MDEntryType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_MDEntryType_Enum::BID),
            "1" => Ok(Field_MDEntryType_Enum::OFFER),
            "2" => Ok(Field_MDEntryType_Enum::TRADE),
            "3" => Ok(Field_MDEntryType_Enum::INDEX_VALUE),
            "4" => Ok(Field_MDEntryType_Enum::OPENING_PRICE),
            "5" => Ok(Field_MDEntryType_Enum::CLOSING_PRICE),
            "6" => Ok(Field_MDEntryType_Enum::SETTLEMENT_PRICE),
            "7" => Ok(Field_MDEntryType_Enum::TRADING_SESSION_HIGH_PRICE),
            "8" => Ok(Field_MDEntryType_Enum::TRADING_SESSION_LOW_PRICE),
            "9" => Ok(Field_MDEntryType_Enum::TRADING_SESSION_VWAP_PRICE),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_MDEntryType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_MDEntryType_Enum::BID => {
                f.write_str( "0" )
            },
            &Field_MDEntryType_Enum::OFFER => {
                f.write_str( "1" )
            },
            &Field_MDEntryType_Enum::TRADE => {
                f.write_str( "2" )
            },
            &Field_MDEntryType_Enum::INDEX_VALUE => {
                f.write_str( "3" )
            },
            &Field_MDEntryType_Enum::OPENING_PRICE => {
                f.write_str( "4" )
            },
            &Field_MDEntryType_Enum::CLOSING_PRICE => {
                f.write_str( "5" )
            },
            &Field_MDEntryType_Enum::SETTLEMENT_PRICE => {
                f.write_str( "6" )
            },
            &Field_MDEntryType_Enum::TRADING_SESSION_HIGH_PRICE => {
                f.write_str( "7" )
            },
            &Field_MDEntryType_Enum::TRADING_SESSION_LOW_PRICE => {
                f.write_str( "8" )
            },
            &Field_MDEntryType_Enum::TRADING_SESSION_VWAP_PRICE => {
                f.write_str( "9" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_MDEntryType_Enum {
    fn default() -> Self {
        Field_MDEntryType_Enum::Undefined
    }
}
const FIELD_MDENTRYPX : u32 = 270; // PRICE

const FIELD_MDENTRYSIZE : u32 = 271; // QTY

const FIELD_MDENTRYDATE : u32 = 272; // UTCDATE

const FIELD_MDENTRYTIME : u32 = 273; // UTCTIMEONLY

const FIELD_TICKDIRECTION : u32 = 274; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_TickDirection_Enum {
    PLUS_TICK, // = "0"
    ZERO_PLUS_TICK, // = "1"
    MINUS_TICK, // = "2"
    ZERO_MINUS_TICK, // = "3"

    Undefined
}

impl FromStr for Field_TickDirection_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_TickDirection_Enum::PLUS_TICK),
            "1" => Ok(Field_TickDirection_Enum::ZERO_PLUS_TICK),
            "2" => Ok(Field_TickDirection_Enum::MINUS_TICK),
            "3" => Ok(Field_TickDirection_Enum::ZERO_MINUS_TICK),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_TickDirection_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_TickDirection_Enum::PLUS_TICK => {
                f.write_str( "0" )
            },
            &Field_TickDirection_Enum::ZERO_PLUS_TICK => {
                f.write_str( "1" )
            },
            &Field_TickDirection_Enum::MINUS_TICK => {
                f.write_str( "2" )
            },
            &Field_TickDirection_Enum::ZERO_MINUS_TICK => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_TickDirection_Enum {
    fn default() -> Self {
        Field_TickDirection_Enum::Undefined
    }
}
const FIELD_MDMKT : u32 = 275; // EXCHANGE

const FIELD_QUOTECONDITION : u32 = 276; // MULTIPLEVALUESTRING
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_QuoteCondition_Enum {
    OPEN, // = "A"
    CLOSED, // = "B"
    EXCHANGE_BEST, // = "C"
    CONSOLIDATED_BEST, // = "D"
    LOCKED, // = "E"
    CROSSED, // = "F"
    DEPTH, // = "G"
    FAST_TRADING, // = "H"
    NON_FIRM, // = "I"

    Undefined
}

impl FromStr for Field_QuoteCondition_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Field_QuoteCondition_Enum::OPEN),
            "B" => Ok(Field_QuoteCondition_Enum::CLOSED),
            "C" => Ok(Field_QuoteCondition_Enum::EXCHANGE_BEST),
            "D" => Ok(Field_QuoteCondition_Enum::CONSOLIDATED_BEST),
            "E" => Ok(Field_QuoteCondition_Enum::LOCKED),
            "F" => Ok(Field_QuoteCondition_Enum::CROSSED),
            "G" => Ok(Field_QuoteCondition_Enum::DEPTH),
            "H" => Ok(Field_QuoteCondition_Enum::FAST_TRADING),
            "I" => Ok(Field_QuoteCondition_Enum::NON_FIRM),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_QuoteCondition_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_QuoteCondition_Enum::OPEN => {
                f.write_str( "A" )
            },
            &Field_QuoteCondition_Enum::CLOSED => {
                f.write_str( "B" )
            },
            &Field_QuoteCondition_Enum::EXCHANGE_BEST => {
                f.write_str( "C" )
            },
            &Field_QuoteCondition_Enum::CONSOLIDATED_BEST => {
                f.write_str( "D" )
            },
            &Field_QuoteCondition_Enum::LOCKED => {
                f.write_str( "E" )
            },
            &Field_QuoteCondition_Enum::CROSSED => {
                f.write_str( "F" )
            },
            &Field_QuoteCondition_Enum::DEPTH => {
                f.write_str( "G" )
            },
            &Field_QuoteCondition_Enum::FAST_TRADING => {
                f.write_str( "H" )
            },
            &Field_QuoteCondition_Enum::NON_FIRM => {
                f.write_str( "I" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_QuoteCondition_Enum {
    fn default() -> Self {
        Field_QuoteCondition_Enum::Undefined
    }
}
const FIELD_TRADECONDITION : u32 = 277; // MULTIPLEVALUESTRING
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_TradeCondition_Enum {
    CASH, // = "A"
    AVERAGE_PRICE_TRADE, // = "B"
    CASH_TRADE, // = "C"
    NEXT_DAY, // = "D"
    OPENING, // = "E"
    INTRADAY_TRADE_DETAIL, // = "F"
    RULE_127_TRADE, // = "G"
    RULE_155_TRADE, // = "H"
    SOLD_LAST, // = "I"
    NEXT_DAY_TRADE, // = "J"
    OPENED, // = "K"
    SELLER, // = "L"
    SOLD, // = "M"
    STOPPED_STOCK, // = "N"

    Undefined
}

impl FromStr for Field_TradeCondition_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Field_TradeCondition_Enum::CASH),
            "B" => Ok(Field_TradeCondition_Enum::AVERAGE_PRICE_TRADE),
            "C" => Ok(Field_TradeCondition_Enum::CASH_TRADE),
            "D" => Ok(Field_TradeCondition_Enum::NEXT_DAY),
            "E" => Ok(Field_TradeCondition_Enum::OPENING),
            "F" => Ok(Field_TradeCondition_Enum::INTRADAY_TRADE_DETAIL),
            "G" => Ok(Field_TradeCondition_Enum::RULE_127_TRADE),
            "H" => Ok(Field_TradeCondition_Enum::RULE_155_TRADE),
            "I" => Ok(Field_TradeCondition_Enum::SOLD_LAST),
            "J" => Ok(Field_TradeCondition_Enum::NEXT_DAY_TRADE),
            "K" => Ok(Field_TradeCondition_Enum::OPENED),
            "L" => Ok(Field_TradeCondition_Enum::SELLER),
            "M" => Ok(Field_TradeCondition_Enum::SOLD),
            "N" => Ok(Field_TradeCondition_Enum::STOPPED_STOCK),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_TradeCondition_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_TradeCondition_Enum::CASH => {
                f.write_str( "A" )
            },
            &Field_TradeCondition_Enum::AVERAGE_PRICE_TRADE => {
                f.write_str( "B" )
            },
            &Field_TradeCondition_Enum::CASH_TRADE => {
                f.write_str( "C" )
            },
            &Field_TradeCondition_Enum::NEXT_DAY => {
                f.write_str( "D" )
            },
            &Field_TradeCondition_Enum::OPENING => {
                f.write_str( "E" )
            },
            &Field_TradeCondition_Enum::INTRADAY_TRADE_DETAIL => {
                f.write_str( "F" )
            },
            &Field_TradeCondition_Enum::RULE_127_TRADE => {
                f.write_str( "G" )
            },
            &Field_TradeCondition_Enum::RULE_155_TRADE => {
                f.write_str( "H" )
            },
            &Field_TradeCondition_Enum::SOLD_LAST => {
                f.write_str( "I" )
            },
            &Field_TradeCondition_Enum::NEXT_DAY_TRADE => {
                f.write_str( "J" )
            },
            &Field_TradeCondition_Enum::OPENED => {
                f.write_str( "K" )
            },
            &Field_TradeCondition_Enum::SELLER => {
                f.write_str( "L" )
            },
            &Field_TradeCondition_Enum::SOLD => {
                f.write_str( "M" )
            },
            &Field_TradeCondition_Enum::STOPPED_STOCK => {
                f.write_str( "N" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_TradeCondition_Enum {
    fn default() -> Self {
        Field_TradeCondition_Enum::Undefined
    }
}
const FIELD_MDENTRYID : u32 = 278; // STRING

const FIELD_MDUPDATEACTION : u32 = 279; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_MDUpdateAction_Enum {
    NEW, // = "0"
    CHANGE, // = "1"
    DELETE, // = "2"

    Undefined
}

impl FromStr for Field_MDUpdateAction_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_MDUpdateAction_Enum::NEW),
            "1" => Ok(Field_MDUpdateAction_Enum::CHANGE),
            "2" => Ok(Field_MDUpdateAction_Enum::DELETE),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_MDUpdateAction_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_MDUpdateAction_Enum::NEW => {
                f.write_str( "0" )
            },
            &Field_MDUpdateAction_Enum::CHANGE => {
                f.write_str( "1" )
            },
            &Field_MDUpdateAction_Enum::DELETE => {
                f.write_str( "2" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_MDUpdateAction_Enum {
    fn default() -> Self {
        Field_MDUpdateAction_Enum::Undefined
    }
}
const FIELD_MDENTRYREFID : u32 = 280; // STRING

const FIELD_MDREQREJREASON : u32 = 281; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_MDReqRejReason_Enum {
    UNKNOWN_SYMBOL, // = "0"
    DUPLICATE_MDREQID, // = "1"
    INSUFFICIENT_BANDWIDTH, // = "2"
    INSUFFICIENT_PERMISSIONS, // = "3"
    UNSUPPORTED_SUBSCRIPTIONREQUESTTYPE, // = "4"
    UNSUPPORTED_MARKETDEPTH, // = "5"
    UNSUPPORTED_MDUPDATETYPE, // = "6"
    UNSUPPORTED_AGGREGATEDBOOK, // = "7"
    UNSUPPORTED_MDENTRYTYPE, // = "8"

    Undefined
}

impl FromStr for Field_MDReqRejReason_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_MDReqRejReason_Enum::UNKNOWN_SYMBOL),
            "1" => Ok(Field_MDReqRejReason_Enum::DUPLICATE_MDREQID),
            "2" => Ok(Field_MDReqRejReason_Enum::INSUFFICIENT_BANDWIDTH),
            "3" => Ok(Field_MDReqRejReason_Enum::INSUFFICIENT_PERMISSIONS),
            "4" => Ok(Field_MDReqRejReason_Enum::UNSUPPORTED_SUBSCRIPTIONREQUESTTYPE),
            "5" => Ok(Field_MDReqRejReason_Enum::UNSUPPORTED_MARKETDEPTH),
            "6" => Ok(Field_MDReqRejReason_Enum::UNSUPPORTED_MDUPDATETYPE),
            "7" => Ok(Field_MDReqRejReason_Enum::UNSUPPORTED_AGGREGATEDBOOK),
            "8" => Ok(Field_MDReqRejReason_Enum::UNSUPPORTED_MDENTRYTYPE),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_MDReqRejReason_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_MDReqRejReason_Enum::UNKNOWN_SYMBOL => {
                f.write_str( "0" )
            },
            &Field_MDReqRejReason_Enum::DUPLICATE_MDREQID => {
                f.write_str( "1" )
            },
            &Field_MDReqRejReason_Enum::INSUFFICIENT_BANDWIDTH => {
                f.write_str( "2" )
            },
            &Field_MDReqRejReason_Enum::INSUFFICIENT_PERMISSIONS => {
                f.write_str( "3" )
            },
            &Field_MDReqRejReason_Enum::UNSUPPORTED_SUBSCRIPTIONREQUESTTYPE => {
                f.write_str( "4" )
            },
            &Field_MDReqRejReason_Enum::UNSUPPORTED_MARKETDEPTH => {
                f.write_str( "5" )
            },
            &Field_MDReqRejReason_Enum::UNSUPPORTED_MDUPDATETYPE => {
                f.write_str( "6" )
            },
            &Field_MDReqRejReason_Enum::UNSUPPORTED_AGGREGATEDBOOK => {
                f.write_str( "7" )
            },
            &Field_MDReqRejReason_Enum::UNSUPPORTED_MDENTRYTYPE => {
                f.write_str( "8" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_MDReqRejReason_Enum {
    fn default() -> Self {
        Field_MDReqRejReason_Enum::Undefined
    }
}
const FIELD_MDENTRYORIGINATOR : u32 = 282; // STRING

const FIELD_LOCATIONID : u32 = 283; // STRING

const FIELD_DESKID : u32 = 284; // STRING

const FIELD_DELETEREASON : u32 = 285; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_DeleteReason_Enum {
    CANCELATION, // = "0"
    ERROR, // = "1"

    Undefined
}

impl FromStr for Field_DeleteReason_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_DeleteReason_Enum::CANCELATION),
            "1" => Ok(Field_DeleteReason_Enum::ERROR),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_DeleteReason_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_DeleteReason_Enum::CANCELATION => {
                f.write_str( "0" )
            },
            &Field_DeleteReason_Enum::ERROR => {
                f.write_str( "1" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_DeleteReason_Enum {
    fn default() -> Self {
        Field_DeleteReason_Enum::Undefined
    }
}
const FIELD_OPENCLOSESETTLEFLAG : u32 = 286; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_OpenCloseSettleFlag_Enum {
    DAILY_OPEN, // = "0"
    SESSION_OPEN, // = "1"
    DELIVERY_SETTLEMENT_PRICE, // = "2"

    Undefined
}

impl FromStr for Field_OpenCloseSettleFlag_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_OpenCloseSettleFlag_Enum::DAILY_OPEN),
            "1" => Ok(Field_OpenCloseSettleFlag_Enum::SESSION_OPEN),
            "2" => Ok(Field_OpenCloseSettleFlag_Enum::DELIVERY_SETTLEMENT_PRICE),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_OpenCloseSettleFlag_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_OpenCloseSettleFlag_Enum::DAILY_OPEN => {
                f.write_str( "0" )
            },
            &Field_OpenCloseSettleFlag_Enum::SESSION_OPEN => {
                f.write_str( "1" )
            },
            &Field_OpenCloseSettleFlag_Enum::DELIVERY_SETTLEMENT_PRICE => {
                f.write_str( "2" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_OpenCloseSettleFlag_Enum {
    fn default() -> Self {
        Field_OpenCloseSettleFlag_Enum::Undefined
    }
}
const FIELD_SELLERDAYS : u32 = 287; // INT

const FIELD_MDENTRYBUYER : u32 = 288; // STRING

const FIELD_MDENTRYSELLER : u32 = 289; // STRING

const FIELD_MDENTRYPOSITIONNO : u32 = 290; // INT

const FIELD_FINANCIALSTATUS : u32 = 291; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_FinancialStatus_Enum {
    BANKRUPT, // = "1"

    Undefined
}

impl FromStr for Field_FinancialStatus_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_FinancialStatus_Enum::BANKRUPT),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_FinancialStatus_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_FinancialStatus_Enum::BANKRUPT => {
                f.write_str( "1" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_FinancialStatus_Enum {
    fn default() -> Self {
        Field_FinancialStatus_Enum::Undefined
    }
}
const FIELD_CORPORATEACTION : u32 = 292; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_CorporateAction_Enum {
    EX_DIVIDEND, // = "A"
    EX_DISTRIBUTION, // = "B"
    EX_RIGHTS, // = "C"
    NEW, // = "D"
    EX_INTEREST, // = "E"

    Undefined
}

impl FromStr for Field_CorporateAction_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Field_CorporateAction_Enum::EX_DIVIDEND),
            "B" => Ok(Field_CorporateAction_Enum::EX_DISTRIBUTION),
            "C" => Ok(Field_CorporateAction_Enum::EX_RIGHTS),
            "D" => Ok(Field_CorporateAction_Enum::NEW),
            "E" => Ok(Field_CorporateAction_Enum::EX_INTEREST),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_CorporateAction_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_CorporateAction_Enum::EX_DIVIDEND => {
                f.write_str( "A" )
            },
            &Field_CorporateAction_Enum::EX_DISTRIBUTION => {
                f.write_str( "B" )
            },
            &Field_CorporateAction_Enum::EX_RIGHTS => {
                f.write_str( "C" )
            },
            &Field_CorporateAction_Enum::NEW => {
                f.write_str( "D" )
            },
            &Field_CorporateAction_Enum::EX_INTEREST => {
                f.write_str( "E" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_CorporateAction_Enum {
    fn default() -> Self {
        Field_CorporateAction_Enum::Undefined
    }
}
const FIELD_DEFBIDSIZE : u32 = 293; // QTY

const FIELD_DEFOFFERSIZE : u32 = 294; // QTY

const FIELD_NOQUOTEENTRIES : u32 = 295; // INT

const FIELD_NOQUOTESETS : u32 = 296; // INT

const FIELD_QUOTEACKSTATUS : u32 = 297; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_QuoteAckStatus_Enum {
    ACCEPTED, // = "0"
    CANCELED_FOR_SYMBOL, // = "1"
    CANCELED_FOR_SECURITY_TYPE, // = "2"
    CANCELED_FOR_UNDERLYING, // = "3"
    CANCELED_ALL, // = "4"
    REJECTED, // = "5"

    Undefined
}

impl FromStr for Field_QuoteAckStatus_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_QuoteAckStatus_Enum::ACCEPTED),
            "1" => Ok(Field_QuoteAckStatus_Enum::CANCELED_FOR_SYMBOL),
            "2" => Ok(Field_QuoteAckStatus_Enum::CANCELED_FOR_SECURITY_TYPE),
            "3" => Ok(Field_QuoteAckStatus_Enum::CANCELED_FOR_UNDERLYING),
            "4" => Ok(Field_QuoteAckStatus_Enum::CANCELED_ALL),
            "5" => Ok(Field_QuoteAckStatus_Enum::REJECTED),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_QuoteAckStatus_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_QuoteAckStatus_Enum::ACCEPTED => {
                f.write_str( "0" )
            },
            &Field_QuoteAckStatus_Enum::CANCELED_FOR_SYMBOL => {
                f.write_str( "1" )
            },
            &Field_QuoteAckStatus_Enum::CANCELED_FOR_SECURITY_TYPE => {
                f.write_str( "2" )
            },
            &Field_QuoteAckStatus_Enum::CANCELED_FOR_UNDERLYING => {
                f.write_str( "3" )
            },
            &Field_QuoteAckStatus_Enum::CANCELED_ALL => {
                f.write_str( "4" )
            },
            &Field_QuoteAckStatus_Enum::REJECTED => {
                f.write_str( "5" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_QuoteAckStatus_Enum {
    fn default() -> Self {
        Field_QuoteAckStatus_Enum::Undefined
    }
}
const FIELD_QUOTECANCELTYPE : u32 = 298; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_QuoteCancelType_Enum {
    CANCEL_FOR_SYMBOL, // = "1"
    CANCEL_FOR_SECURITY_TYPE, // = "2"
    CANCEL_FOR_UNDERLYING_SYMBOL, // = "3"
    CANCEL_FOR_ALL_QUOTES, // = "4"

    Undefined
}

impl FromStr for Field_QuoteCancelType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_QuoteCancelType_Enum::CANCEL_FOR_SYMBOL),
            "2" => Ok(Field_QuoteCancelType_Enum::CANCEL_FOR_SECURITY_TYPE),
            "3" => Ok(Field_QuoteCancelType_Enum::CANCEL_FOR_UNDERLYING_SYMBOL),
            "4" => Ok(Field_QuoteCancelType_Enum::CANCEL_FOR_ALL_QUOTES),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_QuoteCancelType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_QuoteCancelType_Enum::CANCEL_FOR_SYMBOL => {
                f.write_str( "1" )
            },
            &Field_QuoteCancelType_Enum::CANCEL_FOR_SECURITY_TYPE => {
                f.write_str( "2" )
            },
            &Field_QuoteCancelType_Enum::CANCEL_FOR_UNDERLYING_SYMBOL => {
                f.write_str( "3" )
            },
            &Field_QuoteCancelType_Enum::CANCEL_FOR_ALL_QUOTES => {
                f.write_str( "4" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_QuoteCancelType_Enum {
    fn default() -> Self {
        Field_QuoteCancelType_Enum::Undefined
    }
}
const FIELD_QUOTEENTRYID : u32 = 299; // STRING

const FIELD_QUOTEREJECTREASON : u32 = 300; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_QuoteRejectReason_Enum {
    UNKNOWN_SYMBOL, // = "1"
    EXCHANGE, // = "2"
    QUOTE_REQUEST_EXCEEDS_LIMIT, // = "3"
    TOO_LATE_TO_ENTER, // = "4"
    UNKNOWN_QUOTE, // = "5"
    DUPLICATE_QUOTE, // = "6"
    INVALID_BID_ASK_SPREAD, // = "7"
    INVALID_PRICE, // = "8"
    NOT_AUTHORIZED_TO_QUOTE_SECURITY, // = "9"

    Undefined
}

impl FromStr for Field_QuoteRejectReason_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_QuoteRejectReason_Enum::UNKNOWN_SYMBOL),
            "2" => Ok(Field_QuoteRejectReason_Enum::EXCHANGE),
            "3" => Ok(Field_QuoteRejectReason_Enum::QUOTE_REQUEST_EXCEEDS_LIMIT),
            "4" => Ok(Field_QuoteRejectReason_Enum::TOO_LATE_TO_ENTER),
            "5" => Ok(Field_QuoteRejectReason_Enum::UNKNOWN_QUOTE),
            "6" => Ok(Field_QuoteRejectReason_Enum::DUPLICATE_QUOTE),
            "7" => Ok(Field_QuoteRejectReason_Enum::INVALID_BID_ASK_SPREAD),
            "8" => Ok(Field_QuoteRejectReason_Enum::INVALID_PRICE),
            "9" => Ok(Field_QuoteRejectReason_Enum::NOT_AUTHORIZED_TO_QUOTE_SECURITY),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_QuoteRejectReason_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_QuoteRejectReason_Enum::UNKNOWN_SYMBOL => {
                f.write_str( "1" )
            },
            &Field_QuoteRejectReason_Enum::EXCHANGE => {
                f.write_str( "2" )
            },
            &Field_QuoteRejectReason_Enum::QUOTE_REQUEST_EXCEEDS_LIMIT => {
                f.write_str( "3" )
            },
            &Field_QuoteRejectReason_Enum::TOO_LATE_TO_ENTER => {
                f.write_str( "4" )
            },
            &Field_QuoteRejectReason_Enum::UNKNOWN_QUOTE => {
                f.write_str( "5" )
            },
            &Field_QuoteRejectReason_Enum::DUPLICATE_QUOTE => {
                f.write_str( "6" )
            },
            &Field_QuoteRejectReason_Enum::INVALID_BID_ASK_SPREAD => {
                f.write_str( "7" )
            },
            &Field_QuoteRejectReason_Enum::INVALID_PRICE => {
                f.write_str( "8" )
            },
            &Field_QuoteRejectReason_Enum::NOT_AUTHORIZED_TO_QUOTE_SECURITY => {
                f.write_str( "9" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_QuoteRejectReason_Enum {
    fn default() -> Self {
        Field_QuoteRejectReason_Enum::Undefined
    }
}
const FIELD_QUOTERESPONSELEVEL : u32 = 301; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_QuoteResponseLevel_Enum {
    NO_ACKNOWLEDGEMENT, // = "0"
    ACKNOWLEDGE_ONLY_NEGATIVE_OR_ERRONEOUS_QUOTES, // = "1"
    ACKNOWLEDGE_EACH_QUOTE_MESSAGES, // = "2"

    Undefined
}

impl FromStr for Field_QuoteResponseLevel_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_QuoteResponseLevel_Enum::NO_ACKNOWLEDGEMENT),
            "1" => Ok(Field_QuoteResponseLevel_Enum::ACKNOWLEDGE_ONLY_NEGATIVE_OR_ERRONEOUS_QUOTES),
            "2" => Ok(Field_QuoteResponseLevel_Enum::ACKNOWLEDGE_EACH_QUOTE_MESSAGES),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_QuoteResponseLevel_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_QuoteResponseLevel_Enum::NO_ACKNOWLEDGEMENT => {
                f.write_str( "0" )
            },
            &Field_QuoteResponseLevel_Enum::ACKNOWLEDGE_ONLY_NEGATIVE_OR_ERRONEOUS_QUOTES => {
                f.write_str( "1" )
            },
            &Field_QuoteResponseLevel_Enum::ACKNOWLEDGE_EACH_QUOTE_MESSAGES => {
                f.write_str( "2" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_QuoteResponseLevel_Enum {
    fn default() -> Self {
        Field_QuoteResponseLevel_Enum::Undefined
    }
}
const FIELD_QUOTESETID : u32 = 302; // STRING

const FIELD_QUOTEREQUESTTYPE : u32 = 303; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_QuoteRequestType_Enum {
    MANUAL, // = "1"
    AUTOMATIC, // = "2"

    Undefined
}

impl FromStr for Field_QuoteRequestType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_QuoteRequestType_Enum::MANUAL),
            "2" => Ok(Field_QuoteRequestType_Enum::AUTOMATIC),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_QuoteRequestType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_QuoteRequestType_Enum::MANUAL => {
                f.write_str( "1" )
            },
            &Field_QuoteRequestType_Enum::AUTOMATIC => {
                f.write_str( "2" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_QuoteRequestType_Enum {
    fn default() -> Self {
        Field_QuoteRequestType_Enum::Undefined
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_SecurityRequestType_Enum {
    REQUEST_SECURITY_IDENTITY_AND_SPECIFICATIONS, // = "0"
    REQUEST_SECURITY_IDENTITY_FOR_THE_SPECIFICATIONS_PROVIDED, // = "1"
    REQUEST_LIST_SECURITY_TYPES, // = "2"
    REQUEST_LIST_SECURITIES, // = "3"

    Undefined
}

impl FromStr for Field_SecurityRequestType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_SecurityRequestType_Enum::REQUEST_SECURITY_IDENTITY_AND_SPECIFICATIONS),
            "1" => Ok(Field_SecurityRequestType_Enum::REQUEST_SECURITY_IDENTITY_FOR_THE_SPECIFICATIONS_PROVIDED),
            "2" => Ok(Field_SecurityRequestType_Enum::REQUEST_LIST_SECURITY_TYPES),
            "3" => Ok(Field_SecurityRequestType_Enum::REQUEST_LIST_SECURITIES),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_SecurityRequestType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_SecurityRequestType_Enum::REQUEST_SECURITY_IDENTITY_AND_SPECIFICATIONS => {
                f.write_str( "0" )
            },
            &Field_SecurityRequestType_Enum::REQUEST_SECURITY_IDENTITY_FOR_THE_SPECIFICATIONS_PROVIDED => {
                f.write_str( "1" )
            },
            &Field_SecurityRequestType_Enum::REQUEST_LIST_SECURITY_TYPES => {
                f.write_str( "2" )
            },
            &Field_SecurityRequestType_Enum::REQUEST_LIST_SECURITIES => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_SecurityRequestType_Enum {
    fn default() -> Self {
        Field_SecurityRequestType_Enum::Undefined
    }
}
const FIELD_SECURITYRESPONSEID : u32 = 322; // STRING

const FIELD_SECURITYRESPONSETYPE : u32 = 323; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_SecurityResponseType_Enum {
    ACCEPT_SECURITY_PROPOSAL_AS_IS, // = "1"
    ACCEPT_SECURITY_PROPOSAL_WITH_REVISIONS_AS_INDICATED_IN_THE_MESSAGE, // = "2"
    LIST_OF_SECURITY_TYPES_RETURNED_PER_REQUEST, // = "3"
    LIST_OF_SECURITIES_RETURNED_PER_REQUEST, // = "4"
    REJECT_SECURITY_PROPOSAL, // = "5"
    CAN_NOT_MATCH_SELECTION_CRITERIA, // = "6"

    Undefined
}

impl FromStr for Field_SecurityResponseType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_SecurityResponseType_Enum::ACCEPT_SECURITY_PROPOSAL_AS_IS),
            "2" => Ok(Field_SecurityResponseType_Enum::ACCEPT_SECURITY_PROPOSAL_WITH_REVISIONS_AS_INDICATED_IN_THE_MESSAGE),
            "3" => Ok(Field_SecurityResponseType_Enum::LIST_OF_SECURITY_TYPES_RETURNED_PER_REQUEST),
            "4" => Ok(Field_SecurityResponseType_Enum::LIST_OF_SECURITIES_RETURNED_PER_REQUEST),
            "5" => Ok(Field_SecurityResponseType_Enum::REJECT_SECURITY_PROPOSAL),
            "6" => Ok(Field_SecurityResponseType_Enum::CAN_NOT_MATCH_SELECTION_CRITERIA),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_SecurityResponseType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_SecurityResponseType_Enum::ACCEPT_SECURITY_PROPOSAL_AS_IS => {
                f.write_str( "1" )
            },
            &Field_SecurityResponseType_Enum::ACCEPT_SECURITY_PROPOSAL_WITH_REVISIONS_AS_INDICATED_IN_THE_MESSAGE => {
                f.write_str( "2" )
            },
            &Field_SecurityResponseType_Enum::LIST_OF_SECURITY_TYPES_RETURNED_PER_REQUEST => {
                f.write_str( "3" )
            },
            &Field_SecurityResponseType_Enum::LIST_OF_SECURITIES_RETURNED_PER_REQUEST => {
                f.write_str( "4" )
            },
            &Field_SecurityResponseType_Enum::REJECT_SECURITY_PROPOSAL => {
                f.write_str( "5" )
            },
            &Field_SecurityResponseType_Enum::CAN_NOT_MATCH_SELECTION_CRITERIA => {
                f.write_str( "6" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_SecurityResponseType_Enum {
    fn default() -> Self {
        Field_SecurityResponseType_Enum::Undefined
    }
}
const FIELD_SECURITYSTATUSREQID : u32 = 324; // STRING

const FIELD_UNSOLICITEDINDICATOR : u32 = 325; // BOOLEAN

const FIELD_SECURITYTRADINGSTATUS : u32 = 326; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_SecurityTradingStatus_Enum {
    OPENING_DELAY, // = "1"
    MARKET_ON_CLOSE_IMBALANCE_SELL, // = "10"
    NO_MARKET_IMBALANCE, // = "12"
    NO_MARKET_ON_CLOSE_IMBALANCE, // = "13"
    ITS_PRE_OPENING, // = "14"
    NEW_PRICE_INDICATION, // = "15"
    TRADE_DISSEMINATION_TIME, // = "16"
    READY_TO_TRADE, // = "17"
    NOT_AVAILABLE_FOR_TRADING, // = "18"
    NOT_TRADED_ON_THIS_MARKET, // = "19"
    TRADING_HALT, // = "2"
    UNKNOWN_OR_INVALID, // = "20"
    RESUME, // = "3"
    NO_OPEN_NO_RESUME, // = "4"
    PRICE_INDICATION, // = "5"
    TRADING_RANGE_INDICATION, // = "6"
    MARKET_IMBALANCE_BUY, // = "7"
    MARKET_IMBALANCE_SELL, // = "8"
    MARKET_ON_CLOSE_IMBALANCE_BUY, // = "9"

    Undefined
}

impl FromStr for Field_SecurityTradingStatus_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_SecurityTradingStatus_Enum::OPENING_DELAY),
            "10" => Ok(Field_SecurityTradingStatus_Enum::MARKET_ON_CLOSE_IMBALANCE_SELL),
            "12" => Ok(Field_SecurityTradingStatus_Enum::NO_MARKET_IMBALANCE),
            "13" => Ok(Field_SecurityTradingStatus_Enum::NO_MARKET_ON_CLOSE_IMBALANCE),
            "14" => Ok(Field_SecurityTradingStatus_Enum::ITS_PRE_OPENING),
            "15" => Ok(Field_SecurityTradingStatus_Enum::NEW_PRICE_INDICATION),
            "16" => Ok(Field_SecurityTradingStatus_Enum::TRADE_DISSEMINATION_TIME),
            "17" => Ok(Field_SecurityTradingStatus_Enum::READY_TO_TRADE),
            "18" => Ok(Field_SecurityTradingStatus_Enum::NOT_AVAILABLE_FOR_TRADING),
            "19" => Ok(Field_SecurityTradingStatus_Enum::NOT_TRADED_ON_THIS_MARKET),
            "2" => Ok(Field_SecurityTradingStatus_Enum::TRADING_HALT),
            "20" => Ok(Field_SecurityTradingStatus_Enum::UNKNOWN_OR_INVALID),
            "3" => Ok(Field_SecurityTradingStatus_Enum::RESUME),
            "4" => Ok(Field_SecurityTradingStatus_Enum::NO_OPEN_NO_RESUME),
            "5" => Ok(Field_SecurityTradingStatus_Enum::PRICE_INDICATION),
            "6" => Ok(Field_SecurityTradingStatus_Enum::TRADING_RANGE_INDICATION),
            "7" => Ok(Field_SecurityTradingStatus_Enum::MARKET_IMBALANCE_BUY),
            "8" => Ok(Field_SecurityTradingStatus_Enum::MARKET_IMBALANCE_SELL),
            "9" => Ok(Field_SecurityTradingStatus_Enum::MARKET_ON_CLOSE_IMBALANCE_BUY),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_SecurityTradingStatus_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_SecurityTradingStatus_Enum::OPENING_DELAY => {
                f.write_str( "1" )
            },
            &Field_SecurityTradingStatus_Enum::MARKET_ON_CLOSE_IMBALANCE_SELL => {
                f.write_str( "10" )
            },
            &Field_SecurityTradingStatus_Enum::NO_MARKET_IMBALANCE => {
                f.write_str( "12" )
            },
            &Field_SecurityTradingStatus_Enum::NO_MARKET_ON_CLOSE_IMBALANCE => {
                f.write_str( "13" )
            },
            &Field_SecurityTradingStatus_Enum::ITS_PRE_OPENING => {
                f.write_str( "14" )
            },
            &Field_SecurityTradingStatus_Enum::NEW_PRICE_INDICATION => {
                f.write_str( "15" )
            },
            &Field_SecurityTradingStatus_Enum::TRADE_DISSEMINATION_TIME => {
                f.write_str( "16" )
            },
            &Field_SecurityTradingStatus_Enum::READY_TO_TRADE => {
                f.write_str( "17" )
            },
            &Field_SecurityTradingStatus_Enum::NOT_AVAILABLE_FOR_TRADING => {
                f.write_str( "18" )
            },
            &Field_SecurityTradingStatus_Enum::NOT_TRADED_ON_THIS_MARKET => {
                f.write_str( "19" )
            },
            &Field_SecurityTradingStatus_Enum::TRADING_HALT => {
                f.write_str( "2" )
            },
            &Field_SecurityTradingStatus_Enum::UNKNOWN_OR_INVALID => {
                f.write_str( "20" )
            },
            &Field_SecurityTradingStatus_Enum::RESUME => {
                f.write_str( "3" )
            },
            &Field_SecurityTradingStatus_Enum::NO_OPEN_NO_RESUME => {
                f.write_str( "4" )
            },
            &Field_SecurityTradingStatus_Enum::PRICE_INDICATION => {
                f.write_str( "5" )
            },
            &Field_SecurityTradingStatus_Enum::TRADING_RANGE_INDICATION => {
                f.write_str( "6" )
            },
            &Field_SecurityTradingStatus_Enum::MARKET_IMBALANCE_BUY => {
                f.write_str( "7" )
            },
            &Field_SecurityTradingStatus_Enum::MARKET_IMBALANCE_SELL => {
                f.write_str( "8" )
            },
            &Field_SecurityTradingStatus_Enum::MARKET_ON_CLOSE_IMBALANCE_BUY => {
                f.write_str( "9" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_SecurityTradingStatus_Enum {
    fn default() -> Self {
        Field_SecurityTradingStatus_Enum::Undefined
    }
}
const FIELD_HALTREASONCHAR : u32 = 327; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_HaltReasonChar_Enum {
    NEWS_DISSEMINATION, // = "D"
    ORDER_INFLUX, // = "E"
    ORDER_IMBALANCE, // = "I"
    ADDITIONAL_INFORMATION, // = "M"
    NEWS_PENDING, // = "P"
    EQUIPMENT_CHANGEOVER, // = "X"

    Undefined
}

impl FromStr for Field_HaltReasonChar_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "D" => Ok(Field_HaltReasonChar_Enum::NEWS_DISSEMINATION),
            "E" => Ok(Field_HaltReasonChar_Enum::ORDER_INFLUX),
            "I" => Ok(Field_HaltReasonChar_Enum::ORDER_IMBALANCE),
            "M" => Ok(Field_HaltReasonChar_Enum::ADDITIONAL_INFORMATION),
            "P" => Ok(Field_HaltReasonChar_Enum::NEWS_PENDING),
            "X" => Ok(Field_HaltReasonChar_Enum::EQUIPMENT_CHANGEOVER),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_HaltReasonChar_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_HaltReasonChar_Enum::NEWS_DISSEMINATION => {
                f.write_str( "D" )
            },
            &Field_HaltReasonChar_Enum::ORDER_INFLUX => {
                f.write_str( "E" )
            },
            &Field_HaltReasonChar_Enum::ORDER_IMBALANCE => {
                f.write_str( "I" )
            },
            &Field_HaltReasonChar_Enum::ADDITIONAL_INFORMATION => {
                f.write_str( "M" )
            },
            &Field_HaltReasonChar_Enum::NEWS_PENDING => {
                f.write_str( "P" )
            },
            &Field_HaltReasonChar_Enum::EQUIPMENT_CHANGEOVER => {
                f.write_str( "X" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_HaltReasonChar_Enum {
    fn default() -> Self {
        Field_HaltReasonChar_Enum::Undefined
    }
}
const FIELD_INVIEWOFCOMMON : u32 = 328; // BOOLEAN

const FIELD_DUETORELATED : u32 = 329; // BOOLEAN

const FIELD_BUYVOLUME : u32 = 330; // QTY

const FIELD_SELLVOLUME : u32 = 331; // QTY

const FIELD_HIGHPX : u32 = 332; // PRICE

const FIELD_LOWPX : u32 = 333; // PRICE

const FIELD_ADJUSTMENT : u32 = 334; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_Adjustment_Enum {
    CANCEL, // = "1"
    ERROR, // = "2"
    CORRECTION, // = "3"

    Undefined
}

impl FromStr for Field_Adjustment_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_Adjustment_Enum::CANCEL),
            "2" => Ok(Field_Adjustment_Enum::ERROR),
            "3" => Ok(Field_Adjustment_Enum::CORRECTION),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_Adjustment_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_Adjustment_Enum::CANCEL => {
                f.write_str( "1" )
            },
            &Field_Adjustment_Enum::ERROR => {
                f.write_str( "2" )
            },
            &Field_Adjustment_Enum::CORRECTION => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_Adjustment_Enum {
    fn default() -> Self {
        Field_Adjustment_Enum::Undefined
    }
}
const FIELD_TRADSESREQID : u32 = 335; // STRING

const FIELD_TRADINGSESSIONID : u32 = 336; // STRING

const FIELD_CONTRATRADER : u32 = 337; // STRING

const FIELD_TRADSESMETHOD : u32 = 338; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_TradSesMethod_Enum {
    ELECTRONIC, // = "1"
    OPEN_OUTCRY, // = "2"
    TWO_PARTY, // = "3"

    Undefined
}

impl FromStr for Field_TradSesMethod_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_TradSesMethod_Enum::ELECTRONIC),
            "2" => Ok(Field_TradSesMethod_Enum::OPEN_OUTCRY),
            "3" => Ok(Field_TradSesMethod_Enum::TWO_PARTY),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_TradSesMethod_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_TradSesMethod_Enum::ELECTRONIC => {
                f.write_str( "1" )
            },
            &Field_TradSesMethod_Enum::OPEN_OUTCRY => {
                f.write_str( "2" )
            },
            &Field_TradSesMethod_Enum::TWO_PARTY => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_TradSesMethod_Enum {
    fn default() -> Self {
        Field_TradSesMethod_Enum::Undefined
    }
}
const FIELD_TRADSESMODE : u32 = 339; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_TradSesMode_Enum {
    TESTING, // = "1"
    SIMULATED, // = "2"
    PRODUCTION, // = "3"

    Undefined
}

impl FromStr for Field_TradSesMode_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_TradSesMode_Enum::TESTING),
            "2" => Ok(Field_TradSesMode_Enum::SIMULATED),
            "3" => Ok(Field_TradSesMode_Enum::PRODUCTION),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_TradSesMode_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_TradSesMode_Enum::TESTING => {
                f.write_str( "1" )
            },
            &Field_TradSesMode_Enum::SIMULATED => {
                f.write_str( "2" )
            },
            &Field_TradSesMode_Enum::PRODUCTION => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_TradSesMode_Enum {
    fn default() -> Self {
        Field_TradSesMode_Enum::Undefined
    }
}
const FIELD_TRADSESSTATUS : u32 = 340; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_TradSesStatus_Enum {
    HALTED, // = "1"
    OPEN, // = "2"
    CLOSED, // = "3"
    PRE_OPEN, // = "4"
    PRE_CLOSE, // = "5"

    Undefined
}

impl FromStr for Field_TradSesStatus_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_TradSesStatus_Enum::HALTED),
            "2" => Ok(Field_TradSesStatus_Enum::OPEN),
            "3" => Ok(Field_TradSesStatus_Enum::CLOSED),
            "4" => Ok(Field_TradSesStatus_Enum::PRE_OPEN),
            "5" => Ok(Field_TradSesStatus_Enum::PRE_CLOSE),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_TradSesStatus_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_TradSesStatus_Enum::HALTED => {
                f.write_str( "1" )
            },
            &Field_TradSesStatus_Enum::OPEN => {
                f.write_str( "2" )
            },
            &Field_TradSesStatus_Enum::CLOSED => {
                f.write_str( "3" )
            },
            &Field_TradSesStatus_Enum::PRE_OPEN => {
                f.write_str( "4" )
            },
            &Field_TradSesStatus_Enum::PRE_CLOSE => {
                f.write_str( "5" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_TradSesStatus_Enum {
    fn default() -> Self {
        Field_TradSesStatus_Enum::Undefined
    }
}
const FIELD_TRADSESSTARTTIME : u32 = 341; // UTCTIMESTAMP

const FIELD_TRADSESOPENTIME : u32 = 342; // UTCTIMESTAMP

const FIELD_TRADSESPRECLOSETIME : u32 = 343; // UTCTIMESTAMP

const FIELD_TRADSESCLOSETIME : u32 = 344; // UTCTIMESTAMP

const FIELD_TRADSESENDTIME : u32 = 345; // UTCTIMESTAMP

const FIELD_NUMBEROFORDERS : u32 = 346; // INT

const FIELD_MESSAGEENCODING : u32 = 347; // STRING
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_MessageEncoding_Enum {
    EUC_JP, // = "EUC-JP"
    ISO_2022_JP, // = "ISO-2022-JP"
    SHIFT_JIS, // = "SHIFT_JIS"
    UTF_8, // = "UTF-8"

    Undefined
}

impl FromStr for Field_MessageEncoding_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EUC-JP" => Ok(Field_MessageEncoding_Enum::EUC_JP),
            "ISO-2022-JP" => Ok(Field_MessageEncoding_Enum::ISO_2022_JP),
            "SHIFT_JIS" => Ok(Field_MessageEncoding_Enum::SHIFT_JIS),
            "UTF-8" => Ok(Field_MessageEncoding_Enum::UTF_8),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_MessageEncoding_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_MessageEncoding_Enum::EUC_JP => {
                f.write_str( "EUC-JP" )
            },
            &Field_MessageEncoding_Enum::ISO_2022_JP => {
                f.write_str( "ISO-2022-JP" )
            },
            &Field_MessageEncoding_Enum::SHIFT_JIS => {
                f.write_str( "SHIFT_JIS" )
            },
            &Field_MessageEncoding_Enum::UTF_8 => {
                f.write_str( "UTF-8" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_MessageEncoding_Enum {
    fn default() -> Self {
        Field_MessageEncoding_Enum::Undefined
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_QuoteEntryRejectReason_Enum {
    UNKNOWN_SYMBOL, // = "1"
    EXCHANGE, // = "2"
    QUOTE_EXCEEDS_LIMIT, // = "3"
    TOO_LATE_TO_ENTER, // = "4"
    UNKNOWN_QUOTE, // = "5"
    DUPLICATE_QUOTE, // = "6"
    INVALID_BID_ASK_SPREAD, // = "7"
    INVALID_PRICE, // = "8"
    NOT_AUTHORIZED_TO_QUOTE_SECURITY, // = "9"

    Undefined
}

impl FromStr for Field_QuoteEntryRejectReason_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_QuoteEntryRejectReason_Enum::UNKNOWN_SYMBOL),
            "2" => Ok(Field_QuoteEntryRejectReason_Enum::EXCHANGE),
            "3" => Ok(Field_QuoteEntryRejectReason_Enum::QUOTE_EXCEEDS_LIMIT),
            "4" => Ok(Field_QuoteEntryRejectReason_Enum::TOO_LATE_TO_ENTER),
            "5" => Ok(Field_QuoteEntryRejectReason_Enum::UNKNOWN_QUOTE),
            "6" => Ok(Field_QuoteEntryRejectReason_Enum::DUPLICATE_QUOTE),
            "7" => Ok(Field_QuoteEntryRejectReason_Enum::INVALID_BID_ASK_SPREAD),
            "8" => Ok(Field_QuoteEntryRejectReason_Enum::INVALID_PRICE),
            "9" => Ok(Field_QuoteEntryRejectReason_Enum::NOT_AUTHORIZED_TO_QUOTE_SECURITY),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_QuoteEntryRejectReason_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_QuoteEntryRejectReason_Enum::UNKNOWN_SYMBOL => {
                f.write_str( "1" )
            },
            &Field_QuoteEntryRejectReason_Enum::EXCHANGE => {
                f.write_str( "2" )
            },
            &Field_QuoteEntryRejectReason_Enum::QUOTE_EXCEEDS_LIMIT => {
                f.write_str( "3" )
            },
            &Field_QuoteEntryRejectReason_Enum::TOO_LATE_TO_ENTER => {
                f.write_str( "4" )
            },
            &Field_QuoteEntryRejectReason_Enum::UNKNOWN_QUOTE => {
                f.write_str( "5" )
            },
            &Field_QuoteEntryRejectReason_Enum::DUPLICATE_QUOTE => {
                f.write_str( "6" )
            },
            &Field_QuoteEntryRejectReason_Enum::INVALID_BID_ASK_SPREAD => {
                f.write_str( "7" )
            },
            &Field_QuoteEntryRejectReason_Enum::INVALID_PRICE => {
                f.write_str( "8" )
            },
            &Field_QuoteEntryRejectReason_Enum::NOT_AUTHORIZED_TO_QUOTE_SECURITY => {
                f.write_str( "9" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_QuoteEntryRejectReason_Enum {
    fn default() -> Self {
        Field_QuoteEntryRejectReason_Enum::Undefined
    }
}
const FIELD_LASTMSGSEQNUMPROCESSED : u32 = 369; // INT

const FIELD_ONBEHALFOFSENDINGTIME : u32 = 370; // UTCTIMESTAMP

const FIELD_REFTAGID : u32 = 371; // INT

const FIELD_REFMSGTYPE : u32 = 372; // STRING

const FIELD_SESSIONREJECTREASON : u32 = 373; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_SessionRejectReason_Enum {
    INVALID_TAG_NUMBER, // = "0"
    REQUIRED_TAG_MISSING, // = "1"
    SENDINGTIME_ACCURACY_PROBLEM, // = "10"
    INVALID_MSGTYPE, // = "11"
    TAG_NOT_DEFINED_FOR_THIS_MESSAGE_TYPE, // = "2"
    UNDEFINED_TAG, // = "3"
    TAG_SPECIFIED_WITHOUT_A_VALUE, // = "4"
    VALUE_IS_INCORRECT, // = "5"
    INCORRECT_DATA_FORMAT_FOR_VALUE, // = "6"
    DECRYPTION_PROBLEM, // = "7"
    SIGNATURE_PROBLEM, // = "8"
    COMPID_PROBLEM, // = "9"

    Undefined
}

impl FromStr for Field_SessionRejectReason_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_SessionRejectReason_Enum::INVALID_TAG_NUMBER),
            "1" => Ok(Field_SessionRejectReason_Enum::REQUIRED_TAG_MISSING),
            "10" => Ok(Field_SessionRejectReason_Enum::SENDINGTIME_ACCURACY_PROBLEM),
            "11" => Ok(Field_SessionRejectReason_Enum::INVALID_MSGTYPE),
            "2" => Ok(Field_SessionRejectReason_Enum::TAG_NOT_DEFINED_FOR_THIS_MESSAGE_TYPE),
            "3" => Ok(Field_SessionRejectReason_Enum::UNDEFINED_TAG),
            "4" => Ok(Field_SessionRejectReason_Enum::TAG_SPECIFIED_WITHOUT_A_VALUE),
            "5" => Ok(Field_SessionRejectReason_Enum::VALUE_IS_INCORRECT),
            "6" => Ok(Field_SessionRejectReason_Enum::INCORRECT_DATA_FORMAT_FOR_VALUE),
            "7" => Ok(Field_SessionRejectReason_Enum::DECRYPTION_PROBLEM),
            "8" => Ok(Field_SessionRejectReason_Enum::SIGNATURE_PROBLEM),
            "9" => Ok(Field_SessionRejectReason_Enum::COMPID_PROBLEM),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_SessionRejectReason_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_SessionRejectReason_Enum::INVALID_TAG_NUMBER => {
                f.write_str( "0" )
            },
            &Field_SessionRejectReason_Enum::REQUIRED_TAG_MISSING => {
                f.write_str( "1" )
            },
            &Field_SessionRejectReason_Enum::SENDINGTIME_ACCURACY_PROBLEM => {
                f.write_str( "10" )
            },
            &Field_SessionRejectReason_Enum::INVALID_MSGTYPE => {
                f.write_str( "11" )
            },
            &Field_SessionRejectReason_Enum::TAG_NOT_DEFINED_FOR_THIS_MESSAGE_TYPE => {
                f.write_str( "2" )
            },
            &Field_SessionRejectReason_Enum::UNDEFINED_TAG => {
                f.write_str( "3" )
            },
            &Field_SessionRejectReason_Enum::TAG_SPECIFIED_WITHOUT_A_VALUE => {
                f.write_str( "4" )
            },
            &Field_SessionRejectReason_Enum::VALUE_IS_INCORRECT => {
                f.write_str( "5" )
            },
            &Field_SessionRejectReason_Enum::INCORRECT_DATA_FORMAT_FOR_VALUE => {
                f.write_str( "6" )
            },
            &Field_SessionRejectReason_Enum::DECRYPTION_PROBLEM => {
                f.write_str( "7" )
            },
            &Field_SessionRejectReason_Enum::SIGNATURE_PROBLEM => {
                f.write_str( "8" )
            },
            &Field_SessionRejectReason_Enum::COMPID_PROBLEM => {
                f.write_str( "9" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_SessionRejectReason_Enum {
    fn default() -> Self {
        Field_SessionRejectReason_Enum::Undefined
    }
}
const FIELD_BIDREQUESTTRANSTYPE : u32 = 374; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_BidRequestTransType_Enum {
    CANCEL, // = "C"
    NO, // = "N"

    Undefined
}

impl FromStr for Field_BidRequestTransType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(Field_BidRequestTransType_Enum::CANCEL),
            "N" => Ok(Field_BidRequestTransType_Enum::NO),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_BidRequestTransType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_BidRequestTransType_Enum::CANCEL => {
                f.write_str( "C" )
            },
            &Field_BidRequestTransType_Enum::NO => {
                f.write_str( "N" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_BidRequestTransType_Enum {
    fn default() -> Self {
        Field_BidRequestTransType_Enum::Undefined
    }
}
const FIELD_CONTRABROKER : u32 = 375; // STRING

const FIELD_COMPLIANCEID : u32 = 376; // STRING

const FIELD_SOLICITEDFLAG : u32 = 377; // BOOLEAN

const FIELD_EXECRESTATEMENTREASON : u32 = 378; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_ExecRestatementReason_Enum {
    GT_CORPORATE_ACTION, // = "0"
    GT_RENEWAL, // = "1"
    VERBAL_CHANGE, // = "2"
    REPRICING_OF_ORDER, // = "3"
    BROKER_OPTION, // = "4"
    PARTIAL_DECLINE_OF_ORDERQTY, // = "5"

    Undefined
}

impl FromStr for Field_ExecRestatementReason_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_ExecRestatementReason_Enum::GT_CORPORATE_ACTION),
            "1" => Ok(Field_ExecRestatementReason_Enum::GT_RENEWAL),
            "2" => Ok(Field_ExecRestatementReason_Enum::VERBAL_CHANGE),
            "3" => Ok(Field_ExecRestatementReason_Enum::REPRICING_OF_ORDER),
            "4" => Ok(Field_ExecRestatementReason_Enum::BROKER_OPTION),
            "5" => Ok(Field_ExecRestatementReason_Enum::PARTIAL_DECLINE_OF_ORDERQTY),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_ExecRestatementReason_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_ExecRestatementReason_Enum::GT_CORPORATE_ACTION => {
                f.write_str( "0" )
            },
            &Field_ExecRestatementReason_Enum::GT_RENEWAL => {
                f.write_str( "1" )
            },
            &Field_ExecRestatementReason_Enum::VERBAL_CHANGE => {
                f.write_str( "2" )
            },
            &Field_ExecRestatementReason_Enum::REPRICING_OF_ORDER => {
                f.write_str( "3" )
            },
            &Field_ExecRestatementReason_Enum::BROKER_OPTION => {
                f.write_str( "4" )
            },
            &Field_ExecRestatementReason_Enum::PARTIAL_DECLINE_OF_ORDERQTY => {
                f.write_str( "5" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_ExecRestatementReason_Enum {
    fn default() -> Self {
        Field_ExecRestatementReason_Enum::Undefined
    }
}
const FIELD_BUSINESSREJECTREFID : u32 = 379; // STRING

const FIELD_BUSINESSREJECTREASON : u32 = 380; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_BusinessRejectReason_Enum {
    OTHER, // = "0"
    UNKOWN_ID, // = "1"
    UNKNOWN_SECURITY, // = "2"
    UNSUPPORTED_MESSAGE_TYPE, // = "3"
    APPLICATION_NOT_AVAILABLE, // = "4"
    CONDITIONALLY_REQUIRED_FIELD_MISSING, // = "5"

    Undefined
}

impl FromStr for Field_BusinessRejectReason_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_BusinessRejectReason_Enum::OTHER),
            "1" => Ok(Field_BusinessRejectReason_Enum::UNKOWN_ID),
            "2" => Ok(Field_BusinessRejectReason_Enum::UNKNOWN_SECURITY),
            "3" => Ok(Field_BusinessRejectReason_Enum::UNSUPPORTED_MESSAGE_TYPE),
            "4" => Ok(Field_BusinessRejectReason_Enum::APPLICATION_NOT_AVAILABLE),
            "5" => Ok(Field_BusinessRejectReason_Enum::CONDITIONALLY_REQUIRED_FIELD_MISSING),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_BusinessRejectReason_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_BusinessRejectReason_Enum::OTHER => {
                f.write_str( "0" )
            },
            &Field_BusinessRejectReason_Enum::UNKOWN_ID => {
                f.write_str( "1" )
            },
            &Field_BusinessRejectReason_Enum::UNKNOWN_SECURITY => {
                f.write_str( "2" )
            },
            &Field_BusinessRejectReason_Enum::UNSUPPORTED_MESSAGE_TYPE => {
                f.write_str( "3" )
            },
            &Field_BusinessRejectReason_Enum::APPLICATION_NOT_AVAILABLE => {
                f.write_str( "4" )
            },
            &Field_BusinessRejectReason_Enum::CONDITIONALLY_REQUIRED_FIELD_MISSING => {
                f.write_str( "5" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_BusinessRejectReason_Enum {
    fn default() -> Self {
        Field_BusinessRejectReason_Enum::Undefined
    }
}
const FIELD_GROSSTRADEAMT : u32 = 381; // AMT

const FIELD_NOCONTRABROKERS : u32 = 382; // INT

const FIELD_MAXMESSAGESIZE : u32 = 383; // INT

const FIELD_NOMSGTYPES : u32 = 384; // INT

const FIELD_MSGDIRECTION : u32 = 385; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_MsgDirection_Enum {
    RECEIVE, // = "R"
    SEND, // = "S"

    Undefined
}

impl FromStr for Field_MsgDirection_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Field_MsgDirection_Enum::RECEIVE),
            "S" => Ok(Field_MsgDirection_Enum::SEND),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_MsgDirection_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_MsgDirection_Enum::RECEIVE => {
                f.write_str( "R" )
            },
            &Field_MsgDirection_Enum::SEND => {
                f.write_str( "S" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_MsgDirection_Enum {
    fn default() -> Self {
        Field_MsgDirection_Enum::Undefined
    }
}
const FIELD_NOTRADINGSESSIONS : u32 = 386; // INT

const FIELD_TOTALVOLUMETRADED : u32 = 387; // QTY

const FIELD_DISCRETIONINST : u32 = 388; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_DiscretionInst_Enum {
    RELATED_TO_DISPLAYED_PRICE, // = "0"
    RELATED_TO_MARKET_PRICE, // = "1"
    RELATED_TO_PRIMARY_PRICE, // = "2"
    RELATED_TO_LOCAL_PRIMARY_PRICE, // = "3"
    RELATED_TO_MIDPOINT_PRICE, // = "4"
    RELATED_TO_LAST_TRADE_PRICE, // = "5"

    Undefined
}

impl FromStr for Field_DiscretionInst_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_DiscretionInst_Enum::RELATED_TO_DISPLAYED_PRICE),
            "1" => Ok(Field_DiscretionInst_Enum::RELATED_TO_MARKET_PRICE),
            "2" => Ok(Field_DiscretionInst_Enum::RELATED_TO_PRIMARY_PRICE),
            "3" => Ok(Field_DiscretionInst_Enum::RELATED_TO_LOCAL_PRIMARY_PRICE),
            "4" => Ok(Field_DiscretionInst_Enum::RELATED_TO_MIDPOINT_PRICE),
            "5" => Ok(Field_DiscretionInst_Enum::RELATED_TO_LAST_TRADE_PRICE),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_DiscretionInst_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_DiscretionInst_Enum::RELATED_TO_DISPLAYED_PRICE => {
                f.write_str( "0" )
            },
            &Field_DiscretionInst_Enum::RELATED_TO_MARKET_PRICE => {
                f.write_str( "1" )
            },
            &Field_DiscretionInst_Enum::RELATED_TO_PRIMARY_PRICE => {
                f.write_str( "2" )
            },
            &Field_DiscretionInst_Enum::RELATED_TO_LOCAL_PRIMARY_PRICE => {
                f.write_str( "3" )
            },
            &Field_DiscretionInst_Enum::RELATED_TO_MIDPOINT_PRICE => {
                f.write_str( "4" )
            },
            &Field_DiscretionInst_Enum::RELATED_TO_LAST_TRADE_PRICE => {
                f.write_str( "5" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_DiscretionInst_Enum {
    fn default() -> Self {
        Field_DiscretionInst_Enum::Undefined
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_LiquidityIndType_Enum {
    _5_DAY_MOVING_AVERAGE, // = "1"
    _20_DAY_MOVING_AVERAGE, // = "2"
    NORMAL_MARKET_SIZE, // = "3"
    OTHER, // = "4"

    Undefined
}

impl FromStr for Field_LiquidityIndType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_LiquidityIndType_Enum::_5_DAY_MOVING_AVERAGE),
            "2" => Ok(Field_LiquidityIndType_Enum::_20_DAY_MOVING_AVERAGE),
            "3" => Ok(Field_LiquidityIndType_Enum::NORMAL_MARKET_SIZE),
            "4" => Ok(Field_LiquidityIndType_Enum::OTHER),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_LiquidityIndType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_LiquidityIndType_Enum::_5_DAY_MOVING_AVERAGE => {
                f.write_str( "1" )
            },
            &Field_LiquidityIndType_Enum::_20_DAY_MOVING_AVERAGE => {
                f.write_str( "2" )
            },
            &Field_LiquidityIndType_Enum::NORMAL_MARKET_SIZE => {
                f.write_str( "3" )
            },
            &Field_LiquidityIndType_Enum::OTHER => {
                f.write_str( "4" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_LiquidityIndType_Enum {
    fn default() -> Self {
        Field_LiquidityIndType_Enum::Undefined
    }
}
const FIELD_WTAVERAGELIQUIDITY : u32 = 410; // FLOAT

const FIELD_EXCHANGEFORPHYSICAL : u32 = 411; // BOOLEAN

const FIELD_OUTMAINCNTRYUINDEX : u32 = 412; // AMT

const FIELD_CROSSPERCENT : u32 = 413; // FLOAT

const FIELD_PROGRPTREQS : u32 = 414; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_ProgRptReqs_Enum {
    BUYSIDE_EXPLICITLY_REQUESTS_STATUS_USING_STATUSREQUEST, // = "1"
    SELLSIDE_PERIODICALLY_SENDS_STATUS_USING_LISTSTATUS_PERIOD_OPTIONALLY_SPECIFIED_IN_PROGRESSPERIOD, // = "2"
    REAL_TIME_EXECUTION_REPORTS, // = "3"

    Undefined
}

impl FromStr for Field_ProgRptReqs_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_ProgRptReqs_Enum::BUYSIDE_EXPLICITLY_REQUESTS_STATUS_USING_STATUSREQUEST),
            "2" => Ok(Field_ProgRptReqs_Enum::SELLSIDE_PERIODICALLY_SENDS_STATUS_USING_LISTSTATUS_PERIOD_OPTIONALLY_SPECIFIED_IN_PROGRESSPERIOD),
            "3" => Ok(Field_ProgRptReqs_Enum::REAL_TIME_EXECUTION_REPORTS),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_ProgRptReqs_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_ProgRptReqs_Enum::BUYSIDE_EXPLICITLY_REQUESTS_STATUS_USING_STATUSREQUEST => {
                f.write_str( "1" )
            },
            &Field_ProgRptReqs_Enum::SELLSIDE_PERIODICALLY_SENDS_STATUS_USING_LISTSTATUS_PERIOD_OPTIONALLY_SPECIFIED_IN_PROGRESSPERIOD => {
                f.write_str( "2" )
            },
            &Field_ProgRptReqs_Enum::REAL_TIME_EXECUTION_REPORTS => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_ProgRptReqs_Enum {
    fn default() -> Self {
        Field_ProgRptReqs_Enum::Undefined
    }
}
const FIELD_PROGPERIODINTERVAL : u32 = 415; // INT

const FIELD_INCTAXIND : u32 = 416; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_IncTaxInd_Enum {
    NET, // = "1"
    GROSS, // = "2"

    Undefined
}

impl FromStr for Field_IncTaxInd_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_IncTaxInd_Enum::NET),
            "2" => Ok(Field_IncTaxInd_Enum::GROSS),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_IncTaxInd_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_IncTaxInd_Enum::NET => {
                f.write_str( "1" )
            },
            &Field_IncTaxInd_Enum::GROSS => {
                f.write_str( "2" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_IncTaxInd_Enum {
    fn default() -> Self {
        Field_IncTaxInd_Enum::Undefined
    }
}
const FIELD_NUMBIDDERS : u32 = 417; // INT

const FIELD_TRADETYPE : u32 = 418; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_TradeType_Enum {
    AGENCY, // = "A"
    VWAP_GUARANTEE, // = "G"
    GUARANTEED_CLOSE, // = "J"
    RISK_TRADE, // = "R"

    Undefined
}

impl FromStr for Field_TradeType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Field_TradeType_Enum::AGENCY),
            "G" => Ok(Field_TradeType_Enum::VWAP_GUARANTEE),
            "J" => Ok(Field_TradeType_Enum::GUARANTEED_CLOSE),
            "R" => Ok(Field_TradeType_Enum::RISK_TRADE),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_TradeType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_TradeType_Enum::AGENCY => {
                f.write_str( "A" )
            },
            &Field_TradeType_Enum::VWAP_GUARANTEE => {
                f.write_str( "G" )
            },
            &Field_TradeType_Enum::GUARANTEED_CLOSE => {
                f.write_str( "J" )
            },
            &Field_TradeType_Enum::RISK_TRADE => {
                f.write_str( "R" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_TradeType_Enum {
    fn default() -> Self {
        Field_TradeType_Enum::Undefined
    }
}
const FIELD_BASISPXTYPE : u32 = 419; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_BasisPxType_Enum {
    CLOSING_PRICE_AT_MORNING_SESSION, // = "2"
    CLOSING_PRICE, // = "3"
    CURRENT_PRICE, // = "4"
    SQ, // = "5"
    VWAP_THROUGH_A_DAY, // = "6"
    VWAP_THROUGH_A_MORNING_SESSION, // = "7"
    VWAP_THROUGH_AN_AFTERNOON_SESSION, // = "8"
    VWAP_THROUGH_A_DAY_EXCEPT_YORI, // = "9"
    VWAP_THROUGH_A_MORNING_SESSION_EXCEPT_YORI, // = "A"
    VWAP_THROUGH_AN_AFTERNOON_SESSION_EXCEPT_YORI, // = "B"
    STRIKE, // = "C"
    OPEN, // = "D"
    OTHERS, // = "Z"

    Undefined
}

impl FromStr for Field_BasisPxType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Field_BasisPxType_Enum::CLOSING_PRICE_AT_MORNING_SESSION),
            "3" => Ok(Field_BasisPxType_Enum::CLOSING_PRICE),
            "4" => Ok(Field_BasisPxType_Enum::CURRENT_PRICE),
            "5" => Ok(Field_BasisPxType_Enum::SQ),
            "6" => Ok(Field_BasisPxType_Enum::VWAP_THROUGH_A_DAY),
            "7" => Ok(Field_BasisPxType_Enum::VWAP_THROUGH_A_MORNING_SESSION),
            "8" => Ok(Field_BasisPxType_Enum::VWAP_THROUGH_AN_AFTERNOON_SESSION),
            "9" => Ok(Field_BasisPxType_Enum::VWAP_THROUGH_A_DAY_EXCEPT_YORI),
            "A" => Ok(Field_BasisPxType_Enum::VWAP_THROUGH_A_MORNING_SESSION_EXCEPT_YORI),
            "B" => Ok(Field_BasisPxType_Enum::VWAP_THROUGH_AN_AFTERNOON_SESSION_EXCEPT_YORI),
            "C" => Ok(Field_BasisPxType_Enum::STRIKE),
            "D" => Ok(Field_BasisPxType_Enum::OPEN),
            "Z" => Ok(Field_BasisPxType_Enum::OTHERS),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_BasisPxType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_BasisPxType_Enum::CLOSING_PRICE_AT_MORNING_SESSION => {
                f.write_str( "2" )
            },
            &Field_BasisPxType_Enum::CLOSING_PRICE => {
                f.write_str( "3" )
            },
            &Field_BasisPxType_Enum::CURRENT_PRICE => {
                f.write_str( "4" )
            },
            &Field_BasisPxType_Enum::SQ => {
                f.write_str( "5" )
            },
            &Field_BasisPxType_Enum::VWAP_THROUGH_A_DAY => {
                f.write_str( "6" )
            },
            &Field_BasisPxType_Enum::VWAP_THROUGH_A_MORNING_SESSION => {
                f.write_str( "7" )
            },
            &Field_BasisPxType_Enum::VWAP_THROUGH_AN_AFTERNOON_SESSION => {
                f.write_str( "8" )
            },
            &Field_BasisPxType_Enum::VWAP_THROUGH_A_DAY_EXCEPT_YORI => {
                f.write_str( "9" )
            },
            &Field_BasisPxType_Enum::VWAP_THROUGH_A_MORNING_SESSION_EXCEPT_YORI => {
                f.write_str( "A" )
            },
            &Field_BasisPxType_Enum::VWAP_THROUGH_AN_AFTERNOON_SESSION_EXCEPT_YORI => {
                f.write_str( "B" )
            },
            &Field_BasisPxType_Enum::STRIKE => {
                f.write_str( "C" )
            },
            &Field_BasisPxType_Enum::OPEN => {
                f.write_str( "D" )
            },
            &Field_BasisPxType_Enum::OTHERS => {
                f.write_str( "Z" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_BasisPxType_Enum {
    fn default() -> Self {
        Field_BasisPxType_Enum::Undefined
    }
}
const FIELD_NOBIDCOMPONENTS : u32 = 420; // INT

const FIELD_COUNTRY : u32 = 421; // STRING

const FIELD_TOTNOSTRIKES : u32 = 422; // INT

const FIELD_PRICETYPE : u32 = 423; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_PriceType_Enum {
    PERCENTAGE, // = "1"
    PER_SHARE, // = "2"
    FIXED_AMOUNT, // = "3"

    Undefined
}

impl FromStr for Field_PriceType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_PriceType_Enum::PERCENTAGE),
            "2" => Ok(Field_PriceType_Enum::PER_SHARE),
            "3" => Ok(Field_PriceType_Enum::FIXED_AMOUNT),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_PriceType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_PriceType_Enum::PERCENTAGE => {
                f.write_str( "1" )
            },
            &Field_PriceType_Enum::PER_SHARE => {
                f.write_str( "2" )
            },
            &Field_PriceType_Enum::FIXED_AMOUNT => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_PriceType_Enum {
    fn default() -> Self {
        Field_PriceType_Enum::Undefined
    }
}
const FIELD_DAYORDERQTY : u32 = 424; // QTY

const FIELD_DAYCUMQTY : u32 = 425; // QTY

const FIELD_DAYAVGPX : u32 = 426; // PRICE

const FIELD_GTBOOKINGINST : u32 = 427; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_GTBookingInst_Enum {
    BOOK_OUT_ALL_TRADES_ON_DAY_OF_EXECUTION, // = "0"
    ACCUMULATE_EXECUTIONS_UNTIL_ORDER_IS_FILLED_OR_EXPIRES, // = "1"
    ACCUMULATE_UNTIL_VERBALLY_NOTIFIED_OTHERWISE, // = "2"

    Undefined
}

impl FromStr for Field_GTBookingInst_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Field_GTBookingInst_Enum::BOOK_OUT_ALL_TRADES_ON_DAY_OF_EXECUTION),
            "1" => Ok(Field_GTBookingInst_Enum::ACCUMULATE_EXECUTIONS_UNTIL_ORDER_IS_FILLED_OR_EXPIRES),
            "2" => Ok(Field_GTBookingInst_Enum::ACCUMULATE_UNTIL_VERBALLY_NOTIFIED_OTHERWISE),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_GTBookingInst_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_GTBookingInst_Enum::BOOK_OUT_ALL_TRADES_ON_DAY_OF_EXECUTION => {
                f.write_str( "0" )
            },
            &Field_GTBookingInst_Enum::ACCUMULATE_EXECUTIONS_UNTIL_ORDER_IS_FILLED_OR_EXPIRES => {
                f.write_str( "1" )
            },
            &Field_GTBookingInst_Enum::ACCUMULATE_UNTIL_VERBALLY_NOTIFIED_OTHERWISE => {
                f.write_str( "2" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_GTBookingInst_Enum {
    fn default() -> Self {
        Field_GTBookingInst_Enum::Undefined
    }
}
const FIELD_NOSTRIKES : u32 = 428; // INT

const FIELD_LISTSTATUSTYPE : u32 = 429; // INT

const FIELD_NETGROSSIND : u32 = 430; // INT
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_NetGrossInd_Enum {
    NET, // = "1"
    GROSS, // = "2"

    Undefined
}

impl FromStr for Field_NetGrossInd_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_NetGrossInd_Enum::NET),
            "2" => Ok(Field_NetGrossInd_Enum::GROSS),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_NetGrossInd_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_NetGrossInd_Enum::NET => {
                f.write_str( "1" )
            },
            &Field_NetGrossInd_Enum::GROSS => {
                f.write_str( "2" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_NetGrossInd_Enum {
    fn default() -> Self {
        Field_NetGrossInd_Enum::Undefined
    }
}
const FIELD_LISTORDERSTATUS : u32 = 431; // INT

const FIELD_EXPIREDATE : u32 = 432; // LOCALMKTDATE

const FIELD_LISTEXECINSTTYPE : u32 = 433; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_ListExecInstType_Enum {
    IMMEDIATE, // = "1"
    WAIT_FOR_EXECUTE_INSTRUCTION, // = "2"

    Undefined
}

impl FromStr for Field_ListExecInstType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_ListExecInstType_Enum::IMMEDIATE),
            "2" => Ok(Field_ListExecInstType_Enum::WAIT_FOR_EXECUTE_INSTRUCTION),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_ListExecInstType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_ListExecInstType_Enum::IMMEDIATE => {
                f.write_str( "1" )
            },
            &Field_ListExecInstType_Enum::WAIT_FOR_EXECUTE_INSTRUCTION => {
                f.write_str( "2" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_ListExecInstType_Enum {
    fn default() -> Self {
        Field_ListExecInstType_Enum::Undefined
    }
}
const FIELD_CXLREJRESPONSETO : u32 = 434; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_CxlRejResponseTo_Enum {
    ORDER_CANCEL_REQUEST, // = "1"
    ORDER_CANCEL_REPLACE_REQUEST, // = "2"

    Undefined
}

impl FromStr for Field_CxlRejResponseTo_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_CxlRejResponseTo_Enum::ORDER_CANCEL_REQUEST),
            "2" => Ok(Field_CxlRejResponseTo_Enum::ORDER_CANCEL_REPLACE_REQUEST),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_CxlRejResponseTo_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_CxlRejResponseTo_Enum::ORDER_CANCEL_REQUEST => {
                f.write_str( "1" )
            },
            &Field_CxlRejResponseTo_Enum::ORDER_CANCEL_REPLACE_REQUEST => {
                f.write_str( "2" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_CxlRejResponseTo_Enum {
    fn default() -> Self {
        Field_CxlRejResponseTo_Enum::Undefined
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub enum Field_MultiLegReportingType_Enum {
    SINGLE_SECURITY, // = "1"
    INDIVIDUAL_LEG_OF_A_MULTI_LEG_SECURITY, // = "2"
    MULTI_LEG_SECURITY, // = "3"

    Undefined
}

impl FromStr for Field_MultiLegReportingType_Enum {
    type Err = OurParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Field_MultiLegReportingType_Enum::SINGLE_SECURITY),
            "2" => Ok(Field_MultiLegReportingType_Enum::INDIVIDUAL_LEG_OF_A_MULTI_LEG_SECURITY),
            "3" => Ok(Field_MultiLegReportingType_Enum::MULTI_LEG_SECURITY),

            _ => {
                Err(OurParserError { unregonized_val: s.to_string() })
            }
        }
    }
}

impl Display for Field_MultiLegReportingType_Enum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Field_MultiLegReportingType_Enum::SINGLE_SECURITY => {
                f.write_str( "1" )
            },
            &Field_MultiLegReportingType_Enum::INDIVIDUAL_LEG_OF_A_MULTI_LEG_SECURITY => {
                f.write_str( "2" )
            },
            &Field_MultiLegReportingType_Enum::MULTI_LEG_SECURITY => {
                f.write_str( "3" )
            },

            _ => {
                Err(Error { })
            }
        }
    }
}

impl Default for Field_MultiLegReportingType_Enum {
    fn default() -> Self {
        Field_MultiLegReportingType_Enum::Undefined
    }
}
const FIELD_STRIKETIME : u32 = 443; // UTCTIMESTAMP

const FIELD_LISTSTATUSTEXT : u32 = 444; // STRING

const FIELD_ENCODEDLISTSTATUSTEXTLEN : u32 = 445; // LENGTH

const FIELD_ENCODEDLISTSTATUSTEXT : u32 = 446; // DATA


// TODO impl of ToString trait for the enum. Is std::fmt::Display an alternative for less allocs?
// TODO impl of FromStr trait for the enum.
// TODO: If type=Mulstring needs to impl trait BitOr as well


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

        "6" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_ioifields( &mut consumer );
            FixMessage::IOI(Box::new( fields ))
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

        "B" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_news_fields( &mut consumer );
            FixMessage::News(Box::new( fields ))
        },

        "C" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_email_fields( &mut consumer );
            FixMessage::Email(Box::new( fields ))
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

        "J" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_allocation_fields( &mut consumer );
            FixMessage::Allocation(Box::new( fields ))
        },

        "K" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_list_cancel_request_fields( &mut consumer );
            FixMessage::ListCancelRequest(Box::new( fields ))
        },

        "L" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_list_execute_fields( &mut consumer );
            FixMessage::ListExecute(Box::new( fields ))
        },

        "M" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_list_status_request_fields( &mut consumer );
            FixMessage::ListStatusRequest(Box::new( fields ))
        },

        "N" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_list_status_fields( &mut consumer );
            FixMessage::ListStatus(Box::new( fields ))
        },

        "P" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_allocation_instruction_ack_fields( &mut consumer );
            FixMessage::AllocationInstructionAck(Box::new( fields ))
        },

        "Q" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_dont_know_trade_fields( &mut consumer );
            FixMessage::DontKnowTrade(Box::new( fields ))
        },

        "R" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_quote_request_fields( &mut consumer );
            FixMessage::QuoteRequest(Box::new( fields ))
        },

        "S" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_quote_fields( &mut consumer );
            FixMessage::Quote(Box::new( fields ))
        },

        "T" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_settlement_instructions_fields( &mut consumer );
            FixMessage::SettlementInstructions(Box::new( fields ))
        },

        "V" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_market_data_request_fields( &mut consumer );
            FixMessage::MarketDataRequest(Box::new( fields ))
        },

        "W" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_market_data_snapshot_full_refresh_fields( &mut consumer );
            FixMessage::MarketDataSnapshotFullRefresh(Box::new( fields ))
        },

        "X" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_market_data_incremental_refresh_fields( &mut consumer );
            FixMessage::MarketDataIncrementalRefresh(Box::new( fields ))
        },

        "Y" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_market_data_request_reject_fields( &mut consumer );
            FixMessage::MarketDataRequestReject(Box::new( fields ))
        },

        "Z" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_quote_cancel_fields( &mut consumer );
            FixMessage::QuoteCancel(Box::new( fields ))
        },

        "a" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_quote_status_request_fields( &mut consumer );
            FixMessage::QuoteStatusRequest(Box::new( fields ))
        },

        "b" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_quote_acknowledgement_fields( &mut consumer );
            FixMessage::QuoteAcknowledgement(Box::new( fields ))
        },

        "c" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_security_definition_request_fields( &mut consumer );
            FixMessage::SecurityDefinitionRequest(Box::new( fields ))
        },

        "d" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_security_definition_fields( &mut consumer );
            FixMessage::SecurityDefinition(Box::new( fields ))
        },

        "e" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_security_status_request_fields( &mut consumer );
            FixMessage::SecurityStatusRequest(Box::new( fields ))
        },

        "f" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_security_status_fields( &mut consumer );
            FixMessage::SecurityStatus(Box::new( fields ))
        },

        "g" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_trading_session_status_request_fields( &mut consumer );
            FixMessage::TradingSessionStatusRequest(Box::new( fields ))
        },

        "h" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_trading_session_status_fields( &mut consumer );
            FixMessage::TradingSessionStatus(Box::new( fields ))
        },

        "i" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_mass_quote_fields( &mut consumer );
            FixMessage::MassQuote(Box::new( fields ))
        },

        "j" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_business_message_reject_fields( &mut consumer );
            FixMessage::BusinessMessageReject(Box::new( fields ))
        },

        "k" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_bid_request_fields( &mut consumer );
            FixMessage::BidRequest(Box::new( fields ))
        },

        "l" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_bid_response_fields( &mut consumer );
            FixMessage::BidResponse(Box::new( fields ))
        },

        "m" => {
            let mut consumer = FixConsumer::new(flds);
            let fields = parse_message_list_strike_price_fields( &mut consumer );
            FixMessage::ListStrikePrice(Box::new( fields ))
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
    let mut session_reject_reason : Option<Field_SessionRejectReason_Enum> = None;
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

                session_reject_reason = Some( Field_SessionRejectReason_Enum::from_str(v).unwrap() );
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


fn parse_message_ioifields( consumer : &mut FixConsumer  ) -> IOIFields {
    // fields:
    let mut ioiid : Option<String> = None;
    let mut ioitrans_type : Option<Field_IOITransType_Enum> = None;
    let mut ioiref_id : Option<String> = None;
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<Field_IDSource_Enum> = None;
    let mut security_type : Option<Field_SecurityType_Enum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<Field_PutOrCall_Enum> = None;
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
    let mut side : Option<Field_Side_Enum> = None;
    let mut ioishares : Option<Field_IOIShares_Enum> = None;
    let mut price : Option<f32> = None;
    let mut currency : Option<f32> = None;
    let mut valid_until_time : Option<UtcDateTime> = None;
    let mut ioiqlty_ind : Option<Field_IOIQltyInd_Enum> = None;
    let mut ioinatural_flag : Option<bool> = None;
    let mut no_ioiqualifiers : Option<Vec<NoIOIQualifiers9Fields>> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;
    let mut transact_time : Option<UtcDateTime> = None;
    let mut urllink : Option<String> = None;
    let mut no_routing_ids : Option<Vec<NoRoutingIDs27Fields>> = None;
    let mut spread_to_benchmark : Option<f32> = None;
    let mut benchmark : Option<Field_Benchmark_Enum> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_IOIID, val: v } => {

                ioiid = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_IOITRANSTYPE, val: v } => {

                ioitrans_type = Some( Field_IOITransType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_IOIREFID, val: v } => {

                ioiref_id = Some( v.to_string() );
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

                idsource = Some( Field_IDSource_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                security_type = Some( Field_SecurityType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYMONTHYEAR, val: v } => {

                maturity_month_year = Some( UtcDate::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYDAY, val: v } => {

                maturity_day = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PUTORCALL, val: v } => {

                put_or_call = Some( Field_PutOrCall_Enum::from_str(v).unwrap() );
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

                side = Some( Field_Side_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_IOISHARES, val: v } => {

                ioishares = Some( Field_IOIShares_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PRICE, val: v } => {

                price = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CURRENCY, val: v } => {

                currency = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_VALIDUNTILTIME, val: v } => {

                valid_until_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_IOIQLTYIND, val: v } => {

                ioiqlty_ind = Some( Field_IOIQltyInd_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_IOINATURALFLAG, val: v } => {

                ioinatural_flag = Some( boolconv(v) );
            },
            &FieldVal { id: FIELD_NOIOIQUALIFIERS, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_ioiqualifiers9_fields(consumer, size);
                no_ioiqualifiers = Some(subgroup);
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
            &FieldVal { id: FIELD_TRANSACTTIME, val: v } => {

                transact_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_URLLINK, val: v } => {

                urllink = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_NOROUTINGIDS, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_routing_ids27_fields(consumer, size);
                no_routing_ids = Some(subgroup);
            },
            &FieldVal { id: FIELD_SPREADTOBENCHMARK, val: v } => {

                spread_to_benchmark = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_BENCHMARK, val: v } => {

                benchmark = Some( Field_Benchmark_Enum::from_str(v).unwrap() );
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    IOIFields {
        ioiid: ioiid.unwrap() /* better error hdl? */ ,
        ioitrans_type: ioitrans_type.unwrap() /* better error hdl? */ ,
        ioiref_id: ioiref_id,
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
        ioishares: ioishares.unwrap() /* better error hdl? */ ,
        price: price,
        currency: currency,
        valid_until_time: valid_until_time,
        ioiqlty_ind: ioiqlty_ind,
        ioinatural_flag: ioinatural_flag,
        no_ioiqualifiers: no_ioiqualifiers,
        text: text,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
        transact_time: transact_time,
        urllink: urllink,
        no_routing_ids: no_routing_ids,
        spread_to_benchmark: spread_to_benchmark,
        benchmark: benchmark,
    }
}


fn parse_message_advertisement_fields( consumer : &mut FixConsumer  ) -> AdvertisementFields {
    // fields:
    let mut adv_id : Option<String> = None;
    let mut adv_trans_type : Option<Field_AdvTransType_Enum> = None;
    let mut adv_ref_id : Option<String> = None;
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<Field_IDSource_Enum> = None;
    let mut security_type : Option<Field_SecurityType_Enum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<Field_PutOrCall_Enum> = None;
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
    let mut adv_side : Option<Field_AdvSide_Enum> = None;
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

                adv_trans_type = Some( Field_AdvTransType_Enum::from_str(v).unwrap() );
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

                idsource = Some( Field_IDSource_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                security_type = Some( Field_SecurityType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYMONTHYEAR, val: v } => {

                maturity_month_year = Some( UtcDate::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYDAY, val: v } => {

                maturity_day = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PUTORCALL, val: v } => {

                put_or_call = Some( Field_PutOrCall_Enum::from_str(v).unwrap() );
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

                adv_side = Some( Field_AdvSide_Enum::from_str(v).unwrap() );
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
    let mut no_contra_brokers : Option<Vec<NoContraBrokers7Fields>> = None;
    let mut list_id : Option<String> = None;
    let mut exec_id : Option<String> = None;
    let mut exec_trans_type : Option<Field_ExecTransType_Enum> = None;
    let mut exec_ref_id : Option<String> = None;
    let mut exec_type : Option<Field_ExecType_Enum> = None;
    let mut ord_status : Option<Field_OrdStatus_Enum> = None;
    let mut ord_rej_reason : Option<Field_OrdRejReason_Enum> = None;
    let mut exec_restatement_reason : Option<Field_ExecRestatementReason_Enum> = None;
    let mut account : Option<String> = None;
    let mut settlmnt_typ : Option<Field_SettlmntTyp_Enum> = None;
    let mut fut_sett_date : Option<UtcDateTime> = None;
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<Field_IDSource_Enum> = None;
    let mut security_type : Option<Field_SecurityType_Enum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<Field_PutOrCall_Enum> = None;
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
    let mut side : Option<Field_Side_Enum> = None;
    let mut order_qty : Option<f32> = None;
    let mut cash_order_qty : Option<f32> = None;
    let mut ord_type : Option<Field_OrdType_Enum> = None;
    let mut price : Option<f32> = None;
    let mut stop_px : Option<f32> = None;
    let mut peg_difference : Option<f32> = None;
    let mut discretion_inst : Option<Field_DiscretionInst_Enum> = None;
    let mut discretion_offset : Option<f32> = None;
    let mut currency : Option<f32> = None;
    let mut compliance_id : Option<String> = None;
    let mut solicited_flag : Option<bool> = None;
    let mut time_in_force : Option<Field_TimeInForce_Enum> = None;
    let mut effective_time : Option<UtcDateTime> = None;
    let mut expire_date : Option<UtcDateTime> = None;
    let mut expire_time : Option<UtcDateTime> = None;
    let mut exec_inst : Option<Field_ExecInst_Enum> = None;
    let mut rule80_a : Option<Field_Rule80A_Enum> = None;
    let mut last_shares : Option<f32> = None;
    let mut last_px : Option<f32> = None;
    let mut last_spot_rate : Option<f32> = None;
    let mut last_forward_points : Option<f32> = None;
    let mut last_mkt : Option<String> = None;
    let mut trading_session_id : Option<String> = None;
    let mut last_capacity : Option<Field_LastCapacity_Enum> = None;
    let mut leaves_qty : Option<f32> = None;
    let mut cum_qty : Option<f32> = None;
    let mut avg_px : Option<f32> = None;
    let mut day_order_qty : Option<f32> = None;
    let mut day_cum_qty : Option<f32> = None;
    let mut day_avg_px : Option<f32> = None;
    let mut gtbooking_inst : Option<Field_GTBookingInst_Enum> = None;
    let mut trade_date : Option<UtcDateTime> = None;
    let mut transact_time : Option<UtcDateTime> = None;
    let mut report_to_exch : Option<bool> = None;
    let mut commission : Option<f32> = None;
    let mut comm_type : Option<Field_CommType_Enum> = None;
    let mut gross_trade_amt : Option<f32> = None;
    let mut settl_curr_amt : Option<f32> = None;
    let mut settl_currency : Option<f32> = None;
    let mut settl_curr_fx_rate : Option<f32> = None;
    let mut settl_curr_fx_rate_calc : Option<Field_SettlCurrFxRateCalc_Enum> = None;
    let mut handl_inst : Option<Field_HandlInst_Enum> = None;
    let mut min_qty : Option<f32> = None;
    let mut max_floor : Option<f32> = None;
    let mut open_close : Option<Field_OpenClose_Enum> = None;
    let mut max_show : Option<f32> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;
    let mut fut_sett_date2 : Option<UtcDateTime> = None;
    let mut order_qty2 : Option<f32> = None;
    let mut clearing_firm : Option<String> = None;
    let mut clearing_account : Option<String> = None;
    let mut multi_leg_reporting_type : Option<Field_MultiLegReportingType_Enum> = None;

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
                let subgroup = build_group_no_contra_brokers7_fields(consumer, size);
                no_contra_brokers = Some(subgroup);
            },
            &FieldVal { id: FIELD_LISTID, val: v } => {

                list_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_EXECID, val: v } => {

                exec_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_EXECTRANSTYPE, val: v } => {

                exec_trans_type = Some( Field_ExecTransType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_EXECREFID, val: v } => {

                exec_ref_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_EXECTYPE, val: v } => {

                exec_type = Some( Field_ExecType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ORDSTATUS, val: v } => {

                ord_status = Some( Field_OrdStatus_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ORDREJREASON, val: v } => {

                ord_rej_reason = Some( Field_OrdRejReason_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_EXECRESTATEMENTREASON, val: v } => {

                exec_restatement_reason = Some( Field_ExecRestatementReason_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ACCOUNT, val: v } => {

                account = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SETTLMNTTYP, val: v } => {

                settlmnt_typ = Some( Field_SettlmntTyp_Enum::from_str(v).unwrap() );
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

                idsource = Some( Field_IDSource_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                security_type = Some( Field_SecurityType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYMONTHYEAR, val: v } => {

                maturity_month_year = Some( UtcDate::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYDAY, val: v } => {

                maturity_day = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PUTORCALL, val: v } => {

                put_or_call = Some( Field_PutOrCall_Enum::from_str(v).unwrap() );
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

                side = Some( Field_Side_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ORDERQTY, val: v } => {

                order_qty = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CASHORDERQTY, val: v } => {

                cash_order_qty = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ORDTYPE, val: v } => {

                ord_type = Some( Field_OrdType_Enum::from_str(v).unwrap() );
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

                discretion_inst = Some( Field_DiscretionInst_Enum::from_str(v).unwrap() );
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

                time_in_force = Some( Field_TimeInForce_Enum::from_str(v).unwrap() );
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

                exec_inst = Some( Field_ExecInst_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_RULE80A, val: v } => {

                rule80_a = Some( Field_Rule80A_Enum::from_str(v).unwrap() );
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

                last_capacity = Some( Field_LastCapacity_Enum::from_str(v).unwrap() );
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

                gtbooking_inst = Some( Field_GTBookingInst_Enum::from_str(v).unwrap() );
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

                comm_type = Some( Field_CommType_Enum::from_str(v).unwrap() );
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

                settl_curr_fx_rate_calc = Some( Field_SettlCurrFxRateCalc_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_HANDLINST, val: v } => {

                handl_inst = Some( Field_HandlInst_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MINQTY, val: v } => {

                min_qty = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MAXFLOOR, val: v } => {

                max_floor = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_OPENCLOSE, val: v } => {

                open_close = Some( Field_OpenClose_Enum::from_str(v).unwrap() );
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

                multi_leg_reporting_type = Some( Field_MultiLegReportingType_Enum::from_str(v).unwrap() );
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
    let mut ord_status : Option<Field_OrdStatus_Enum> = None;
    let mut client_id : Option<String> = None;
    let mut exec_broker : Option<String> = None;
    let mut list_id : Option<String> = None;
    let mut account : Option<String> = None;
    let mut transact_time : Option<UtcDateTime> = None;
    let mut cxl_rej_response_to : Option<Field_CxlRejResponseTo_Enum> = None;
    let mut cxl_rej_reason : Option<Field_CxlRejReason_Enum> = None;
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

                ord_status = Some( Field_OrdStatus_Enum::from_str(v).unwrap() );
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

                cxl_rej_response_to = Some( Field_CxlRejResponseTo_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CXLREJREASON, val: v } => {

                cxl_rej_reason = Some( Field_CxlRejReason_Enum::from_str(v).unwrap() );
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
    let mut encrypt_method : Option<Field_EncryptMethod_Enum> = None;
    let mut heart_bt_int : Option<i32> = None;
    let mut raw_data_length : Option<usize> = None;
    let mut raw_data : Option<String> = None;
    let mut reset_seq_num_flag : Option<bool> = None;
    let mut max_message_size : Option<i32> = None;
    let mut no_msg_types : Option<Vec<NoMsgTypes14Fields>> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_ENCRYPTMETHOD, val: v } => {

                encrypt_method = Some( Field_EncryptMethod_Enum::from_str(v).unwrap() );
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
                let subgroup = build_group_no_msg_types14_fields(consumer, size);
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


fn parse_message_news_fields( consumer : &mut FixConsumer  ) -> NewsFields {
    // fields:
    let mut orig_time : Option<UtcDateTime> = None;
    let mut urgency : Option<Field_Urgency_Enum> = None;
    let mut headline : Option<String> = None;
    let mut encoded_headline_len : Option<usize> = None;
    let mut encoded_headline : Option<String> = None;
    let mut no_routing_ids : Option<Vec<NoRoutingIDs27Fields>> = None;
    let mut no_related_sym : Option<Vec<NoRelatedSym26Fields>> = None;
    let mut lines_of_text : Option<Vec<LinesOfText1Fields>> = None;
    let mut urllink : Option<String> = None;
    let mut raw_data_length : Option<usize> = None;
    let mut raw_data : Option<String> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_ORIGTIME, val: v } => {

                orig_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_URGENCY, val: v } => {

                urgency = Some( Field_Urgency_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_HEADLINE, val: v } => {

                headline = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ENCODEDHEADLINELEN, val: v } => {

                encoded_headline_len = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ENCODEDHEADLINE, val: v } => {

                encoded_headline = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_NOROUTINGIDS, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_routing_ids27_fields(consumer, size);
                no_routing_ids = Some(subgroup);
            },
            &FieldVal { id: FIELD_NORELATEDSYM, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_related_sym26_fields(consumer, size);
                no_related_sym = Some(subgroup);
            },
            &FieldVal { id: FIELD_LINESOFTEXT, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_lines_of_text1_fields(consumer, size);
                lines_of_text = Some(subgroup);
            },
            &FieldVal { id: FIELD_URLLINK, val: v } => {

                urllink = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_RAWDATALENGTH, val: v } => {

                raw_data_length = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_RAWDATA, val: v } => {

                raw_data = Some( v.to_string() );
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    NewsFields {
        orig_time: orig_time,
        urgency: urgency,
        headline: headline.unwrap() /* better error hdl? */ ,
        encoded_headline_len: encoded_headline_len,
        encoded_headline: encoded_headline,
        no_routing_ids: no_routing_ids,
        no_related_sym: no_related_sym,
        lines_of_text: lines_of_text.unwrap() /* better error hdl? */ ,
        urllink: urllink,
        raw_data_length: raw_data_length,
        raw_data: raw_data,
    }
}


fn parse_message_email_fields( consumer : &mut FixConsumer  ) -> EmailFields {
    // fields:
    let mut email_thread_id : Option<String> = None;
    let mut email_type : Option<Field_EmailType_Enum> = None;
    let mut orig_time : Option<UtcDateTime> = None;
    let mut subject : Option<String> = None;
    let mut encoded_subject_len : Option<usize> = None;
    let mut encoded_subject : Option<String> = None;
    let mut no_routing_ids : Option<Vec<NoRoutingIDs27Fields>> = None;
    let mut no_related_sym : Option<Vec<NoRelatedSym26Fields>> = None;
    let mut order_id : Option<String> = None;
    let mut cl_ord_id : Option<String> = None;
    let mut lines_of_text : Option<Vec<LinesOfText1Fields>> = None;
    let mut raw_data_length : Option<usize> = None;
    let mut raw_data : Option<String> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_EMAILTHREADID, val: v } => {

                email_thread_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_EMAILTYPE, val: v } => {

                email_type = Some( Field_EmailType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ORIGTIME, val: v } => {

                orig_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SUBJECT, val: v } => {

                subject = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ENCODEDSUBJECTLEN, val: v } => {

                encoded_subject_len = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ENCODEDSUBJECT, val: v } => {

                encoded_subject = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_NOROUTINGIDS, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_routing_ids27_fields(consumer, size);
                no_routing_ids = Some(subgroup);
            },
            &FieldVal { id: FIELD_NORELATEDSYM, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_related_sym26_fields(consumer, size);
                no_related_sym = Some(subgroup);
            },
            &FieldVal { id: FIELD_ORDERID, val: v } => {

                order_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_CLORDID, val: v } => {

                cl_ord_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_LINESOFTEXT, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_lines_of_text1_fields(consumer, size);
                lines_of_text = Some(subgroup);
            },
            &FieldVal { id: FIELD_RAWDATALENGTH, val: v } => {

                raw_data_length = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_RAWDATA, val: v } => {

                raw_data = Some( v.to_string() );
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    EmailFields {
        email_thread_id: email_thread_id.unwrap() /* better error hdl? */ ,
        email_type: email_type.unwrap() /* better error hdl? */ ,
        orig_time: orig_time,
        subject: subject.unwrap() /* better error hdl? */ ,
        encoded_subject_len: encoded_subject_len,
        encoded_subject: encoded_subject,
        no_routing_ids: no_routing_ids,
        no_related_sym: no_related_sym,
        order_id: order_id,
        cl_ord_id: cl_ord_id,
        lines_of_text: lines_of_text.unwrap() /* better error hdl? */ ,
        raw_data_length: raw_data_length,
        raw_data: raw_data,
    }
}


fn parse_message_new_order_single_fields( consumer : &mut FixConsumer  ) -> NewOrderSingleFields {
    // fields:
    let mut cl_ord_id : Option<String> = None;
    let mut client_id : Option<String> = None;
    let mut exec_broker : Option<String> = None;
    let mut account : Option<String> = None;
    let mut no_allocs : Option<Vec<NoAllocs3Fields>> = None;
    let mut settlmnt_typ : Option<Field_SettlmntTyp_Enum> = None;
    let mut fut_sett_date : Option<UtcDateTime> = None;
    let mut handl_inst : Option<Field_HandlInst_Enum> = None;
    let mut exec_inst : Option<Field_ExecInst_Enum> = None;
    let mut min_qty : Option<f32> = None;
    let mut max_floor : Option<f32> = None;
    let mut ex_destination : Option<String> = None;
    let mut no_trading_sessions : Option<Vec<NoTradingSessions29Fields>> = None;
    let mut process_code : Option<Field_ProcessCode_Enum> = None;
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<Field_IDSource_Enum> = None;
    let mut security_type : Option<Field_SecurityType_Enum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<Field_PutOrCall_Enum> = None;
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
    let mut side : Option<Field_Side_Enum> = None;
    let mut locate_reqd : Option<bool> = None;
    let mut transact_time : Option<UtcDateTime> = None;
    let mut order_qty : Option<f32> = None;
    let mut cash_order_qty : Option<f32> = None;
    let mut ord_type : Option<Field_OrdType_Enum> = None;
    let mut price : Option<f32> = None;
    let mut stop_px : Option<f32> = None;
    let mut currency : Option<f32> = None;
    let mut compliance_id : Option<String> = None;
    let mut solicited_flag : Option<bool> = None;
    let mut ioiid : Option<String> = None;
    let mut quote_id : Option<String> = None;
    let mut time_in_force : Option<Field_TimeInForce_Enum> = None;
    let mut effective_time : Option<UtcDateTime> = None;
    let mut expire_date : Option<UtcDateTime> = None;
    let mut expire_time : Option<UtcDateTime> = None;
    let mut gtbooking_inst : Option<Field_GTBookingInst_Enum> = None;
    let mut commission : Option<f32> = None;
    let mut comm_type : Option<Field_CommType_Enum> = None;
    let mut rule80_a : Option<Field_Rule80A_Enum> = None;
    let mut forex_req : Option<bool> = None;
    let mut settl_currency : Option<f32> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;
    let mut fut_sett_date2 : Option<UtcDateTime> = None;
    let mut order_qty2 : Option<f32> = None;
    let mut open_close : Option<Field_OpenClose_Enum> = None;
    let mut covered_or_uncovered : Option<Field_CoveredOrUncovered_Enum> = None;
    let mut customer_or_firm : Option<Field_CustomerOrFirm_Enum> = None;
    let mut max_show : Option<f32> = None;
    let mut peg_difference : Option<f32> = None;
    let mut discretion_inst : Option<Field_DiscretionInst_Enum> = None;
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
                let subgroup = build_group_no_allocs3_fields(consumer, size);
                no_allocs = Some(subgroup);
            },
            &FieldVal { id: FIELD_SETTLMNTTYP, val: v } => {

                settlmnt_typ = Some( Field_SettlmntTyp_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_FUTSETTDATE, val: v } => {

                fut_sett_date = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_HANDLINST, val: v } => {

                handl_inst = Some( Field_HandlInst_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_EXECINST, val: v } => {

                exec_inst = Some( Field_ExecInst_Enum::from_str(v).unwrap() );
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
                let subgroup = build_group_no_trading_sessions29_fields(consumer, size);
                no_trading_sessions = Some(subgroup);
            },
            &FieldVal { id: FIELD_PROCESSCODE, val: v } => {

                process_code = Some( Field_ProcessCode_Enum::from_str(v).unwrap() );
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

                idsource = Some( Field_IDSource_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                security_type = Some( Field_SecurityType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYMONTHYEAR, val: v } => {

                maturity_month_year = Some( UtcDate::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYDAY, val: v } => {

                maturity_day = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PUTORCALL, val: v } => {

                put_or_call = Some( Field_PutOrCall_Enum::from_str(v).unwrap() );
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

                side = Some( Field_Side_Enum::from_str(v).unwrap() );
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

                ord_type = Some( Field_OrdType_Enum::from_str(v).unwrap() );
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

                time_in_force = Some( Field_TimeInForce_Enum::from_str(v).unwrap() );
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

                gtbooking_inst = Some( Field_GTBookingInst_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_COMMISSION, val: v } => {

                commission = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_COMMTYPE, val: v } => {

                comm_type = Some( Field_CommType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_RULE80A, val: v } => {

                rule80_a = Some( Field_Rule80A_Enum::from_str(v).unwrap() );
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

                open_close = Some( Field_OpenClose_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_COVEREDORUNCOVERED, val: v } => {

                covered_or_uncovered = Some( Field_CoveredOrUncovered_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CUSTOMERORFIRM, val: v } => {

                customer_or_firm = Some( Field_CustomerOrFirm_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MAXSHOW, val: v } => {

                max_show = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PEGDIFFERENCE, val: v } => {

                peg_difference = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_DISCRETIONINST, val: v } => {

                discretion_inst = Some( Field_DiscretionInst_Enum::from_str(v).unwrap() );
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
    let mut prog_rpt_reqs : Option<Field_ProgRptReqs_Enum> = None;
    let mut bid_type : Option<i32> = None;
    let mut prog_period_interval : Option<i32> = None;
    let mut list_exec_inst_type : Option<Field_ListExecInstType_Enum> = None;
    let mut list_exec_inst : Option<String> = None;
    let mut encoded_list_exec_inst_len : Option<usize> = None;
    let mut encoded_list_exec_inst : Option<String> = None;
    let mut tot_no_orders : Option<i32> = None;
    let mut no_orders : Option<Vec<NoOrders15Fields>> = None;

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

                prog_rpt_reqs = Some( Field_ProgRptReqs_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_BIDTYPE, val: v } => {

                bid_type = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PROGPERIODINTERVAL, val: v } => {

                prog_period_interval = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_LISTEXECINSTTYPE, val: v } => {

                list_exec_inst_type = Some( Field_ListExecInstType_Enum::from_str(v).unwrap() );
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
                let subgroup = build_group_no_orders15_fields(consumer, size);
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
    let mut idsource : Option<Field_IDSource_Enum> = None;
    let mut security_type : Option<Field_SecurityType_Enum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<Field_PutOrCall_Enum> = None;
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
    let mut side : Option<Field_Side_Enum> = None;
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

                idsource = Some( Field_IDSource_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                security_type = Some( Field_SecurityType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYMONTHYEAR, val: v } => {

                maturity_month_year = Some( UtcDate::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYDAY, val: v } => {

                maturity_day = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PUTORCALL, val: v } => {

                put_or_call = Some( Field_PutOrCall_Enum::from_str(v).unwrap() );
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

                side = Some( Field_Side_Enum::from_str(v).unwrap() );
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
    let mut no_allocs : Option<Vec<NoAllocs3Fields>> = None;
    let mut settlmnt_typ : Option<Field_SettlmntTyp_Enum> = None;
    let mut fut_sett_date : Option<UtcDateTime> = None;
    let mut handl_inst : Option<Field_HandlInst_Enum> = None;
    let mut exec_inst : Option<Field_ExecInst_Enum> = None;
    let mut min_qty : Option<f32> = None;
    let mut max_floor : Option<f32> = None;
    let mut ex_destination : Option<String> = None;
    let mut no_trading_sessions : Option<Vec<NoTradingSessions29Fields>> = None;
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<Field_IDSource_Enum> = None;
    let mut security_type : Option<Field_SecurityType_Enum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<Field_PutOrCall_Enum> = None;
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
    let mut side : Option<Field_Side_Enum> = None;
    let mut transact_time : Option<UtcDateTime> = None;
    let mut order_qty : Option<f32> = None;
    let mut cash_order_qty : Option<f32> = None;
    let mut ord_type : Option<Field_OrdType_Enum> = None;
    let mut price : Option<f32> = None;
    let mut stop_px : Option<f32> = None;
    let mut peg_difference : Option<f32> = None;
    let mut discretion_inst : Option<Field_DiscretionInst_Enum> = None;
    let mut discretion_offset : Option<f32> = None;
    let mut compliance_id : Option<String> = None;
    let mut solicited_flag : Option<bool> = None;
    let mut currency : Option<f32> = None;
    let mut time_in_force : Option<Field_TimeInForce_Enum> = None;
    let mut effective_time : Option<UtcDateTime> = None;
    let mut expire_date : Option<UtcDateTime> = None;
    let mut expire_time : Option<UtcDateTime> = None;
    let mut gtbooking_inst : Option<Field_GTBookingInst_Enum> = None;
    let mut commission : Option<f32> = None;
    let mut comm_type : Option<Field_CommType_Enum> = None;
    let mut rule80_a : Option<Field_Rule80A_Enum> = None;
    let mut forex_req : Option<bool> = None;
    let mut settl_currency : Option<f32> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;
    let mut fut_sett_date2 : Option<UtcDateTime> = None;
    let mut order_qty2 : Option<f32> = None;
    let mut open_close : Option<Field_OpenClose_Enum> = None;
    let mut covered_or_uncovered : Option<Field_CoveredOrUncovered_Enum> = None;
    let mut customer_or_firm : Option<Field_CustomerOrFirm_Enum> = None;
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
                let subgroup = build_group_no_allocs3_fields(consumer, size);
                no_allocs = Some(subgroup);
            },
            &FieldVal { id: FIELD_SETTLMNTTYP, val: v } => {

                settlmnt_typ = Some( Field_SettlmntTyp_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_FUTSETTDATE, val: v } => {

                fut_sett_date = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_HANDLINST, val: v } => {

                handl_inst = Some( Field_HandlInst_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_EXECINST, val: v } => {

                exec_inst = Some( Field_ExecInst_Enum::from_str(v).unwrap() );
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
                let subgroup = build_group_no_trading_sessions29_fields(consumer, size);
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

                idsource = Some( Field_IDSource_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                security_type = Some( Field_SecurityType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYMONTHYEAR, val: v } => {

                maturity_month_year = Some( UtcDate::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYDAY, val: v } => {

                maturity_day = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PUTORCALL, val: v } => {

                put_or_call = Some( Field_PutOrCall_Enum::from_str(v).unwrap() );
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

                side = Some( Field_Side_Enum::from_str(v).unwrap() );
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

                ord_type = Some( Field_OrdType_Enum::from_str(v).unwrap() );
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

                discretion_inst = Some( Field_DiscretionInst_Enum::from_str(v).unwrap() );
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

                time_in_force = Some( Field_TimeInForce_Enum::from_str(v).unwrap() );
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

                gtbooking_inst = Some( Field_GTBookingInst_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_COMMISSION, val: v } => {

                commission = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_COMMTYPE, val: v } => {

                comm_type = Some( Field_CommType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_RULE80A, val: v } => {

                rule80_a = Some( Field_Rule80A_Enum::from_str(v).unwrap() );
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

                open_close = Some( Field_OpenClose_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_COVEREDORUNCOVERED, val: v } => {

                covered_or_uncovered = Some( Field_CoveredOrUncovered_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CUSTOMERORFIRM, val: v } => {

                customer_or_firm = Some( Field_CustomerOrFirm_Enum::from_str(v).unwrap() );
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
    let mut idsource : Option<Field_IDSource_Enum> = None;
    let mut security_type : Option<Field_SecurityType_Enum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<Field_PutOrCall_Enum> = None;
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
    let mut side : Option<Field_Side_Enum> = None;

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

                idsource = Some( Field_IDSource_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                security_type = Some( Field_SecurityType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYMONTHYEAR, val: v } => {

                maturity_month_year = Some( UtcDate::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYDAY, val: v } => {

                maturity_day = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PUTORCALL, val: v } => {

                put_or_call = Some( Field_PutOrCall_Enum::from_str(v).unwrap() );
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

                side = Some( Field_Side_Enum::from_str(v).unwrap() );
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


fn parse_message_allocation_fields( consumer : &mut FixConsumer  ) -> AllocationFields {
    // fields:
    let mut alloc_id : Option<String> = None;
    let mut alloc_trans_type : Option<Field_AllocTransType_Enum> = None;
    let mut ref_alloc_id : Option<String> = None;
    let mut alloc_link_id : Option<String> = None;
    let mut alloc_link_type : Option<Field_AllocLinkType_Enum> = None;
    let mut no_orders : Option<Vec<NoOrders16Fields>> = None;
    let mut no_execs : Option<Vec<NoExecs8Fields>> = None;
    let mut side : Option<Field_Side_Enum> = None;
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<Field_IDSource_Enum> = None;
    let mut security_type : Option<Field_SecurityType_Enum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<Field_PutOrCall_Enum> = None;
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
    let mut shares : Option<f32> = None;
    let mut last_mkt : Option<String> = None;
    let mut trading_session_id : Option<String> = None;
    let mut avg_px : Option<f32> = None;
    let mut currency : Option<f32> = None;
    let mut avg_prx_precision : Option<i32> = None;
    let mut trade_date : Option<UtcDateTime> = None;
    let mut transact_time : Option<UtcDateTime> = None;
    let mut settlmnt_typ : Option<Field_SettlmntTyp_Enum> = None;
    let mut fut_sett_date : Option<UtcDateTime> = None;
    let mut gross_trade_amt : Option<f32> = None;
    let mut net_money : Option<f32> = None;
    let mut open_close : Option<Field_OpenClose_Enum> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;
    let mut num_days_interest : Option<i32> = None;
    let mut accrued_interest_rate : Option<f32> = None;
    let mut no_allocs : Option<Vec<NoAllocs2Fields>> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_ALLOCID, val: v } => {

                alloc_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ALLOCTRANSTYPE, val: v } => {

                alloc_trans_type = Some( Field_AllocTransType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_REFALLOCID, val: v } => {

                ref_alloc_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ALLOCLINKID, val: v } => {

                alloc_link_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ALLOCLINKTYPE, val: v } => {

                alloc_link_type = Some( Field_AllocLinkType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_NOORDERS, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_orders16_fields(consumer, size);
                no_orders = Some(subgroup);
            },
            &FieldVal { id: FIELD_NOEXECS, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_execs8_fields(consumer, size);
                no_execs = Some(subgroup);
            },
            &FieldVal { id: FIELD_SIDE, val: v } => {

                side = Some( Field_Side_Enum::from_str(v).unwrap() );
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

                idsource = Some( Field_IDSource_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                security_type = Some( Field_SecurityType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYMONTHYEAR, val: v } => {

                maturity_month_year = Some( UtcDate::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYDAY, val: v } => {

                maturity_day = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PUTORCALL, val: v } => {

                put_or_call = Some( Field_PutOrCall_Enum::from_str(v).unwrap() );
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
            &FieldVal { id: FIELD_SHARES, val: v } => {

                shares = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_LASTMKT, val: v } => {

                last_mkt = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_TRADINGSESSIONID, val: v } => {

                trading_session_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_AVGPX, val: v } => {

                avg_px = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CURRENCY, val: v } => {

                currency = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_AVGPRXPRECISION, val: v } => {

                avg_prx_precision = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRADEDATE, val: v } => {

                trade_date = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRANSACTTIME, val: v } => {

                transact_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SETTLMNTTYP, val: v } => {

                settlmnt_typ = Some( Field_SettlmntTyp_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_FUTSETTDATE, val: v } => {

                fut_sett_date = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_GROSSTRADEAMT, val: v } => {

                gross_trade_amt = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_NETMONEY, val: v } => {

                net_money = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_OPENCLOSE, val: v } => {

                open_close = Some( Field_OpenClose_Enum::from_str(v).unwrap() );
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
            &FieldVal { id: FIELD_NUMDAYSINTEREST, val: v } => {

                num_days_interest = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ACCRUEDINTERESTRATE, val: v } => {

                accrued_interest_rate = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_NOALLOCS, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_allocs2_fields(consumer, size);
                no_allocs = Some(subgroup);
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    AllocationFields {
        alloc_id: alloc_id.unwrap() /* better error hdl? */ ,
        alloc_trans_type: alloc_trans_type.unwrap() /* better error hdl? */ ,
        ref_alloc_id: ref_alloc_id,
        alloc_link_id: alloc_link_id,
        alloc_link_type: alloc_link_type,
        no_orders: no_orders,
        no_execs: no_execs,
        side: side.unwrap() /* better error hdl? */ ,
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
        shares: shares.unwrap() /* better error hdl? */ ,
        last_mkt: last_mkt,
        trading_session_id: trading_session_id,
        avg_px: avg_px.unwrap() /* better error hdl? */ ,
        currency: currency,
        avg_prx_precision: avg_prx_precision,
        trade_date: trade_date.unwrap() /* better error hdl? */ ,
        transact_time: transact_time,
        settlmnt_typ: settlmnt_typ,
        fut_sett_date: fut_sett_date,
        gross_trade_amt: gross_trade_amt,
        net_money: net_money,
        open_close: open_close,
        text: text,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
        num_days_interest: num_days_interest,
        accrued_interest_rate: accrued_interest_rate,
        no_allocs: no_allocs,
    }
}


fn parse_message_list_cancel_request_fields( consumer : &mut FixConsumer  ) -> ListCancelRequestFields {
    // fields:
    let mut list_id : Option<String> = None;
    let mut transact_time : Option<UtcDateTime> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_LISTID, val: v } => {

                list_id = Some( v.to_string() );
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
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    ListCancelRequestFields {
        list_id: list_id.unwrap() /* better error hdl? */ ,
        transact_time: transact_time.unwrap() /* better error hdl? */ ,
        text: text,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
    }
}


fn parse_message_list_execute_fields( consumer : &mut FixConsumer  ) -> ListExecuteFields {
    // fields:
    let mut list_id : Option<String> = None;
    let mut client_bid_id : Option<String> = None;
    let mut bid_id : Option<String> = None;
    let mut transact_time : Option<UtcDateTime> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_LISTID, val: v } => {

                list_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_CLIENTBIDID, val: v } => {

                client_bid_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_BIDID, val: v } => {

                bid_id = Some( v.to_string() );
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
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    ListExecuteFields {
        list_id: list_id.unwrap() /* better error hdl? */ ,
        client_bid_id: client_bid_id,
        bid_id: bid_id,
        transact_time: transact_time.unwrap() /* better error hdl? */ ,
        text: text,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
    }
}


fn parse_message_list_status_request_fields( consumer : &mut FixConsumer  ) -> ListStatusRequestFields {
    // fields:
    let mut list_id : Option<String> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_LISTID, val: v } => {

                list_id = Some( v.to_string() );
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
    ListStatusRequestFields {
        list_id: list_id.unwrap() /* better error hdl? */ ,
        text: text,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
    }
}


fn parse_message_list_status_fields( consumer : &mut FixConsumer  ) -> ListStatusFields {
    // fields:
    let mut list_id : Option<String> = None;
    let mut list_status_type : Option<i32> = None;
    let mut no_rpts : Option<i32> = None;
    let mut list_order_status : Option<i32> = None;
    let mut rpt_seq : Option<i32> = None;
    let mut list_status_text : Option<String> = None;
    let mut encoded_list_status_text_len : Option<usize> = None;
    let mut encoded_list_status_text : Option<String> = None;
    let mut transact_time : Option<UtcDateTime> = None;
    let mut tot_no_orders : Option<i32> = None;
    let mut no_orders : Option<Vec<NoOrders17Fields>> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_LISTID, val: v } => {

                list_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_LISTSTATUSTYPE, val: v } => {

                list_status_type = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_NORPTS, val: v } => {

                no_rpts = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_LISTORDERSTATUS, val: v } => {

                list_order_status = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_RPTSEQ, val: v } => {

                rpt_seq = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_LISTSTATUSTEXT, val: v } => {

                list_status_text = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ENCODEDLISTSTATUSTEXTLEN, val: v } => {

                encoded_list_status_text_len = Some( usize::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ENCODEDLISTSTATUSTEXT, val: v } => {

                encoded_list_status_text = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_TRANSACTTIME, val: v } => {

                transact_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TOTNOORDERS, val: v } => {

                tot_no_orders = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_NOORDERS, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_orders17_fields(consumer, size);
                no_orders = Some(subgroup);
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    ListStatusFields {
        list_id: list_id.unwrap() /* better error hdl? */ ,
        list_status_type: list_status_type.unwrap() /* better error hdl? */ ,
        no_rpts: no_rpts.unwrap() /* better error hdl? */ ,
        list_order_status: list_order_status.unwrap() /* better error hdl? */ ,
        rpt_seq: rpt_seq.unwrap() /* better error hdl? */ ,
        list_status_text: list_status_text,
        encoded_list_status_text_len: encoded_list_status_text_len,
        encoded_list_status_text: encoded_list_status_text,
        transact_time: transact_time,
        tot_no_orders: tot_no_orders.unwrap() /* better error hdl? */ ,
        no_orders: no_orders.unwrap() /* better error hdl? */ ,
    }
}


fn parse_message_allocation_instruction_ack_fields( consumer : &mut FixConsumer  ) -> AllocationInstructionAckFields {
    // fields:
    let mut client_id : Option<String> = None;
    let mut exec_broker : Option<String> = None;
    let mut alloc_id : Option<String> = None;
    let mut trade_date : Option<UtcDateTime> = None;
    let mut transact_time : Option<UtcDateTime> = None;
    let mut alloc_status : Option<Field_AllocStatus_Enum> = None;
    let mut alloc_rej_code : Option<Field_AllocRejCode_Enum> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_CLIENTID, val: v } => {

                client_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_EXECBROKER, val: v } => {

                exec_broker = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ALLOCID, val: v } => {

                alloc_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_TRADEDATE, val: v } => {

                trade_date = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRANSACTTIME, val: v } => {

                transact_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ALLOCSTATUS, val: v } => {

                alloc_status = Some( Field_AllocStatus_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ALLOCREJCODE, val: v } => {

                alloc_rej_code = Some( Field_AllocRejCode_Enum::from_str(v).unwrap() );
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
    AllocationInstructionAckFields {
        client_id: client_id,
        exec_broker: exec_broker,
        alloc_id: alloc_id.unwrap() /* better error hdl? */ ,
        trade_date: trade_date.unwrap() /* better error hdl? */ ,
        transact_time: transact_time,
        alloc_status: alloc_status.unwrap() /* better error hdl? */ ,
        alloc_rej_code: alloc_rej_code,
        text: text,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
    }
}


fn parse_message_dont_know_trade_fields( consumer : &mut FixConsumer  ) -> DontKnowTradeFields {
    // fields:
    let mut order_id : Option<String> = None;
    let mut exec_id : Option<String> = None;
    let mut dkreason : Option<Field_DKReason_Enum> = None;
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<Field_IDSource_Enum> = None;
    let mut security_type : Option<Field_SecurityType_Enum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<Field_PutOrCall_Enum> = None;
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
    let mut side : Option<Field_Side_Enum> = None;
    let mut order_qty : Option<f32> = None;
    let mut cash_order_qty : Option<f32> = None;
    let mut last_shares : Option<f32> = None;
    let mut last_px : Option<f32> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_ORDERID, val: v } => {

                order_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_EXECID, val: v } => {

                exec_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_DKREASON, val: v } => {

                dkreason = Some( Field_DKReason_Enum::from_str(v).unwrap() );
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

                idsource = Some( Field_IDSource_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                security_type = Some( Field_SecurityType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYMONTHYEAR, val: v } => {

                maturity_month_year = Some( UtcDate::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYDAY, val: v } => {

                maturity_day = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PUTORCALL, val: v } => {

                put_or_call = Some( Field_PutOrCall_Enum::from_str(v).unwrap() );
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

                side = Some( Field_Side_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ORDERQTY, val: v } => {

                order_qty = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CASHORDERQTY, val: v } => {

                cash_order_qty = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_LASTSHARES, val: v } => {

                last_shares = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_LASTPX, val: v } => {

                last_px = Some( f32::from_str(v).unwrap() );
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
    DontKnowTradeFields {
        order_id: order_id.unwrap() /* better error hdl? */ ,
        exec_id: exec_id.unwrap() /* better error hdl? */ ,
        dkreason: dkreason.unwrap() /* better error hdl? */ ,
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
        last_shares: last_shares,
        last_px: last_px,
        text: text,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
    }
}


fn parse_message_quote_request_fields( consumer : &mut FixConsumer  ) -> QuoteRequestFields {
    // fields:
    let mut quote_req_id : Option<String> = None;
    let mut no_related_sym : Option<Vec<NoRelatedSym25Fields>> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_QUOTEREQID, val: v } => {

                quote_req_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_NORELATEDSYM, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_related_sym25_fields(consumer, size);
                no_related_sym = Some(subgroup);
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    QuoteRequestFields {
        quote_req_id: quote_req_id.unwrap() /* better error hdl? */ ,
        no_related_sym: no_related_sym.unwrap() /* better error hdl? */ ,
    }
}


fn parse_message_quote_fields( consumer : &mut FixConsumer  ) -> QuoteFields {
    // fields:
    let mut quote_req_id : Option<String> = None;
    let mut quote_id : Option<String> = None;
    let mut quote_response_level : Option<Field_QuoteResponseLevel_Enum> = None;
    let mut trading_session_id : Option<String> = None;
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<Field_IDSource_Enum> = None;
    let mut security_type : Option<Field_SecurityType_Enum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<Field_PutOrCall_Enum> = None;
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
    let mut bid_px : Option<f32> = None;
    let mut offer_px : Option<f32> = None;
    let mut bid_size : Option<f32> = None;
    let mut offer_size : Option<f32> = None;
    let mut valid_until_time : Option<UtcDateTime> = None;
    let mut bid_spot_rate : Option<f32> = None;
    let mut offer_spot_rate : Option<f32> = None;
    let mut bid_forward_points : Option<f32> = None;
    let mut offer_forward_points : Option<f32> = None;
    let mut transact_time : Option<UtcDateTime> = None;
    let mut fut_sett_date : Option<UtcDateTime> = None;
    let mut ord_type : Option<Field_OrdType_Enum> = None;
    let mut fut_sett_date2 : Option<UtcDateTime> = None;
    let mut order_qty2 : Option<f32> = None;
    let mut currency : Option<f32> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_QUOTEREQID, val: v } => {

                quote_req_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_QUOTEID, val: v } => {

                quote_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_QUOTERESPONSELEVEL, val: v } => {

                quote_response_level = Some( Field_QuoteResponseLevel_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRADINGSESSIONID, val: v } => {

                trading_session_id = Some( v.to_string() );
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

                idsource = Some( Field_IDSource_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                security_type = Some( Field_SecurityType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYMONTHYEAR, val: v } => {

                maturity_month_year = Some( UtcDate::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYDAY, val: v } => {

                maturity_day = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PUTORCALL, val: v } => {

                put_or_call = Some( Field_PutOrCall_Enum::from_str(v).unwrap() );
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
            &FieldVal { id: FIELD_BIDPX, val: v } => {

                bid_px = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_OFFERPX, val: v } => {

                offer_px = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_BIDSIZE, val: v } => {

                bid_size = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_OFFERSIZE, val: v } => {

                offer_size = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_VALIDUNTILTIME, val: v } => {

                valid_until_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_BIDSPOTRATE, val: v } => {

                bid_spot_rate = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_OFFERSPOTRATE, val: v } => {

                offer_spot_rate = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_BIDFORWARDPOINTS, val: v } => {

                bid_forward_points = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_OFFERFORWARDPOINTS, val: v } => {

                offer_forward_points = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRANSACTTIME, val: v } => {

                transact_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_FUTSETTDATE, val: v } => {

                fut_sett_date = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ORDTYPE, val: v } => {

                ord_type = Some( Field_OrdType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_FUTSETTDATE2, val: v } => {

                fut_sett_date2 = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ORDERQTY2, val: v } => {

                order_qty2 = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CURRENCY, val: v } => {

                currency = Some( f32::from_str(v).unwrap() );
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    QuoteFields {
        quote_req_id: quote_req_id,
        quote_id: quote_id.unwrap() /* better error hdl? */ ,
        quote_response_level: quote_response_level,
        trading_session_id: trading_session_id,
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
        bid_px: bid_px,
        offer_px: offer_px,
        bid_size: bid_size,
        offer_size: offer_size,
        valid_until_time: valid_until_time,
        bid_spot_rate: bid_spot_rate,
        offer_spot_rate: offer_spot_rate,
        bid_forward_points: bid_forward_points,
        offer_forward_points: offer_forward_points,
        transact_time: transact_time,
        fut_sett_date: fut_sett_date,
        ord_type: ord_type,
        fut_sett_date2: fut_sett_date2,
        order_qty2: order_qty2,
        currency: currency,
    }
}


fn parse_message_settlement_instructions_fields( consumer : &mut FixConsumer  ) -> SettlementInstructionsFields {
    // fields:
    let mut settl_inst_id : Option<String> = None;
    let mut settl_inst_trans_type : Option<Field_SettlInstTransType_Enum> = None;
    let mut settl_inst_ref_id : Option<String> = None;
    let mut settl_inst_mode : Option<Field_SettlInstMode_Enum> = None;
    let mut settl_inst_source : Option<Field_SettlInstSource_Enum> = None;
    let mut alloc_account : Option<String> = None;
    let mut settl_location : Option<Field_SettlLocation_Enum> = None;
    let mut trade_date : Option<UtcDateTime> = None;
    let mut alloc_id : Option<String> = None;
    let mut last_mkt : Option<String> = None;
    let mut trading_session_id : Option<String> = None;
    let mut side : Option<Field_Side_Enum> = None;
    let mut security_type : Option<Field_SecurityType_Enum> = None;
    let mut effective_time : Option<UtcDateTime> = None;
    let mut transact_time : Option<UtcDateTime> = None;
    let mut client_id : Option<String> = None;
    let mut exec_broker : Option<String> = None;
    let mut stand_inst_db_type : Option<Field_StandInstDbType_Enum> = None;
    let mut stand_inst_db_name : Option<String> = None;
    let mut stand_inst_db_id : Option<String> = None;
    let mut settl_delivery_type : Option<i32> = None;
    let mut settl_depository_code : Option<String> = None;
    let mut settl_brkr_code : Option<String> = None;
    let mut settl_inst_code : Option<String> = None;
    let mut security_settl_agent_name : Option<String> = None;
    let mut security_settl_agent_code : Option<String> = None;
    let mut security_settl_agent_acct_num : Option<String> = None;
    let mut security_settl_agent_acct_name : Option<String> = None;
    let mut security_settl_agent_contact_name : Option<String> = None;
    let mut security_settl_agent_contact_phone : Option<String> = None;
    let mut cash_settl_agent_name : Option<String> = None;
    let mut cash_settl_agent_code : Option<String> = None;
    let mut cash_settl_agent_acct_num : Option<String> = None;
    let mut cash_settl_agent_acct_name : Option<String> = None;
    let mut cash_settl_agent_contact_name : Option<String> = None;
    let mut cash_settl_agent_contact_phone : Option<String> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_SETTLINSTID, val: v } => {

                settl_inst_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SETTLINSTTRANSTYPE, val: v } => {

                settl_inst_trans_type = Some( Field_SettlInstTransType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SETTLINSTREFID, val: v } => {

                settl_inst_ref_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SETTLINSTMODE, val: v } => {

                settl_inst_mode = Some( Field_SettlInstMode_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SETTLINSTSOURCE, val: v } => {

                settl_inst_source = Some( Field_SettlInstSource_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ALLOCACCOUNT, val: v } => {

                alloc_account = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SETTLLOCATION, val: v } => {

                settl_location = Some( Field_SettlLocation_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRADEDATE, val: v } => {

                trade_date = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ALLOCID, val: v } => {

                alloc_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_LASTMKT, val: v } => {

                last_mkt = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_TRADINGSESSIONID, val: v } => {

                trading_session_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SIDE, val: v } => {

                side = Some( Field_Side_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                security_type = Some( Field_SecurityType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_EFFECTIVETIME, val: v } => {

                effective_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRANSACTTIME, val: v } => {

                transact_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CLIENTID, val: v } => {

                client_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_EXECBROKER, val: v } => {

                exec_broker = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_STANDINSTDBTYPE, val: v } => {

                stand_inst_db_type = Some( Field_StandInstDbType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_STANDINSTDBNAME, val: v } => {

                stand_inst_db_name = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_STANDINSTDBID, val: v } => {

                stand_inst_db_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SETTLDELIVERYTYPE, val: v } => {

                settl_delivery_type = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SETTLDEPOSITORYCODE, val: v } => {

                settl_depository_code = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SETTLBRKRCODE, val: v } => {

                settl_brkr_code = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SETTLINSTCODE, val: v } => {

                settl_inst_code = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SECURITYSETTLAGENTNAME, val: v } => {

                security_settl_agent_name = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SECURITYSETTLAGENTCODE, val: v } => {

                security_settl_agent_code = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SECURITYSETTLAGENTACCTNUM, val: v } => {

                security_settl_agent_acct_num = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SECURITYSETTLAGENTACCTNAME, val: v } => {

                security_settl_agent_acct_name = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SECURITYSETTLAGENTCONTACTNAME, val: v } => {

                security_settl_agent_contact_name = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SECURITYSETTLAGENTCONTACTPHONE, val: v } => {

                security_settl_agent_contact_phone = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_CASHSETTLAGENTNAME, val: v } => {

                cash_settl_agent_name = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_CASHSETTLAGENTCODE, val: v } => {

                cash_settl_agent_code = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_CASHSETTLAGENTACCTNUM, val: v } => {

                cash_settl_agent_acct_num = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_CASHSETTLAGENTACCTNAME, val: v } => {

                cash_settl_agent_acct_name = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_CASHSETTLAGENTCONTACTNAME, val: v } => {

                cash_settl_agent_contact_name = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_CASHSETTLAGENTCONTACTPHONE, val: v } => {

                cash_settl_agent_contact_phone = Some( v.to_string() );
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    SettlementInstructionsFields {
        settl_inst_id: settl_inst_id.unwrap() /* better error hdl? */ ,
        settl_inst_trans_type: settl_inst_trans_type.unwrap() /* better error hdl? */ ,
        settl_inst_ref_id: settl_inst_ref_id.unwrap() /* better error hdl? */ ,
        settl_inst_mode: settl_inst_mode.unwrap() /* better error hdl? */ ,
        settl_inst_source: settl_inst_source.unwrap() /* better error hdl? */ ,
        alloc_account: alloc_account.unwrap() /* better error hdl? */ ,
        settl_location: settl_location,
        trade_date: trade_date,
        alloc_id: alloc_id,
        last_mkt: last_mkt,
        trading_session_id: trading_session_id,
        side: side,
        security_type: security_type,
        effective_time: effective_time,
        transact_time: transact_time.unwrap() /* better error hdl? */ ,
        client_id: client_id,
        exec_broker: exec_broker,
        stand_inst_db_type: stand_inst_db_type,
        stand_inst_db_name: stand_inst_db_name,
        stand_inst_db_id: stand_inst_db_id,
        settl_delivery_type: settl_delivery_type,
        settl_depository_code: settl_depository_code,
        settl_brkr_code: settl_brkr_code,
        settl_inst_code: settl_inst_code,
        security_settl_agent_name: security_settl_agent_name,
        security_settl_agent_code: security_settl_agent_code,
        security_settl_agent_acct_num: security_settl_agent_acct_num,
        security_settl_agent_acct_name: security_settl_agent_acct_name,
        security_settl_agent_contact_name: security_settl_agent_contact_name,
        security_settl_agent_contact_phone: security_settl_agent_contact_phone,
        cash_settl_agent_name: cash_settl_agent_name,
        cash_settl_agent_code: cash_settl_agent_code,
        cash_settl_agent_acct_num: cash_settl_agent_acct_num,
        cash_settl_agent_acct_name: cash_settl_agent_acct_name,
        cash_settl_agent_contact_name: cash_settl_agent_contact_name,
        cash_settl_agent_contact_phone: cash_settl_agent_contact_phone,
    }
}


fn parse_message_market_data_request_fields( consumer : &mut FixConsumer  ) -> MarketDataRequestFields {
    // fields:
    let mut mdreq_id : Option<String> = None;
    let mut subscription_request_type : Option<Field_SubscriptionRequestType_Enum> = None;
    let mut market_depth : Option<i32> = None;
    let mut mdupdate_type : Option<Field_MDUpdateType_Enum> = None;
    let mut aggregated_book : Option<bool> = None;
    let mut no_mdentry_types : Option<Vec<NoMDEntryTypes12Fields>> = None;
    let mut no_related_sym : Option<Vec<NoRelatedSym24Fields>> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_MDREQID, val: v } => {

                mdreq_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SUBSCRIPTIONREQUESTTYPE, val: v } => {

                subscription_request_type = Some( Field_SubscriptionRequestType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MARKETDEPTH, val: v } => {

                market_depth = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MDUPDATETYPE, val: v } => {

                mdupdate_type = Some( Field_MDUpdateType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_AGGREGATEDBOOK, val: v } => {

                aggregated_book = Some( boolconv(v) );
            },
            &FieldVal { id: FIELD_NOMDENTRYTYPES, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_mdentry_types12_fields(consumer, size);
                no_mdentry_types = Some(subgroup);
            },
            &FieldVal { id: FIELD_NORELATEDSYM, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_related_sym24_fields(consumer, size);
                no_related_sym = Some(subgroup);
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    MarketDataRequestFields {
        mdreq_id: mdreq_id.unwrap() /* better error hdl? */ ,
        subscription_request_type: subscription_request_type.unwrap() /* better error hdl? */ ,
        market_depth: market_depth.unwrap() /* better error hdl? */ ,
        mdupdate_type: mdupdate_type,
        aggregated_book: aggregated_book,
        no_mdentry_types: no_mdentry_types.unwrap() /* better error hdl? */ ,
        no_related_sym: no_related_sym.unwrap() /* better error hdl? */ ,
    }
}


fn parse_message_market_data_snapshot_full_refresh_fields( consumer : &mut FixConsumer  ) -> MarketDataSnapshotFullRefreshFields {
    // fields:
    let mut mdreq_id : Option<String> = None;
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<Field_IDSource_Enum> = None;
    let mut security_type : Option<Field_SecurityType_Enum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<Field_PutOrCall_Enum> = None;
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
    let mut financial_status : Option<Field_FinancialStatus_Enum> = None;
    let mut corporate_action : Option<Field_CorporateAction_Enum> = None;
    let mut total_volume_traded : Option<f32> = None;
    let mut no_mdentries : Option<Vec<NoMDEntries10Fields>> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_MDREQID, val: v } => {

                mdreq_id = Some( v.to_string() );
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

                idsource = Some( Field_IDSource_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                security_type = Some( Field_SecurityType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYMONTHYEAR, val: v } => {

                maturity_month_year = Some( UtcDate::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYDAY, val: v } => {

                maturity_day = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PUTORCALL, val: v } => {

                put_or_call = Some( Field_PutOrCall_Enum::from_str(v).unwrap() );
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
            &FieldVal { id: FIELD_FINANCIALSTATUS, val: v } => {

                financial_status = Some( Field_FinancialStatus_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CORPORATEACTION, val: v } => {

                corporate_action = Some( Field_CorporateAction_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TOTALVOLUMETRADED, val: v } => {

                total_volume_traded = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_NOMDENTRIES, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_mdentries10_fields(consumer, size);
                no_mdentries = Some(subgroup);
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    MarketDataSnapshotFullRefreshFields {
        mdreq_id: mdreq_id,
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
        financial_status: financial_status,
        corporate_action: corporate_action,
        total_volume_traded: total_volume_traded,
        no_mdentries: no_mdentries.unwrap() /* better error hdl? */ ,
    }
}


fn parse_message_market_data_incremental_refresh_fields( consumer : &mut FixConsumer  ) -> MarketDataIncrementalRefreshFields {
    // fields:
    let mut mdreq_id : Option<String> = None;
    let mut no_mdentries : Option<Vec<NoMDEntries11Fields>> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_MDREQID, val: v } => {

                mdreq_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_NOMDENTRIES, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_mdentries11_fields(consumer, size);
                no_mdentries = Some(subgroup);
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    MarketDataIncrementalRefreshFields {
        mdreq_id: mdreq_id,
        no_mdentries: no_mdentries.unwrap() /* better error hdl? */ ,
    }
}


fn parse_message_market_data_request_reject_fields( consumer : &mut FixConsumer  ) -> MarketDataRequestRejectFields {
    // fields:
    let mut mdreq_id : Option<String> = None;
    let mut mdreq_rej_reason : Option<Field_MDReqRejReason_Enum> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_MDREQID, val: v } => {

                mdreq_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_MDREQREJREASON, val: v } => {

                mdreq_rej_reason = Some( Field_MDReqRejReason_Enum::from_str(v).unwrap() );
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
    MarketDataRequestRejectFields {
        mdreq_id: mdreq_id.unwrap() /* better error hdl? */ ,
        mdreq_rej_reason: mdreq_rej_reason,
        text: text,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
    }
}


fn parse_message_quote_cancel_fields( consumer : &mut FixConsumer  ) -> QuoteCancelFields {
    // fields:
    let mut quote_req_id : Option<String> = None;
    let mut quote_id : Option<String> = None;
    let mut quote_cancel_type : Option<Field_QuoteCancelType_Enum> = None;
    let mut quote_response_level : Option<Field_QuoteResponseLevel_Enum> = None;
    let mut trading_session_id : Option<String> = None;
    let mut no_quote_entries : Option<Vec<NoQuoteEntries18Fields>> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_QUOTEREQID, val: v } => {

                quote_req_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_QUOTEID, val: v } => {

                quote_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_QUOTECANCELTYPE, val: v } => {

                quote_cancel_type = Some( Field_QuoteCancelType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_QUOTERESPONSELEVEL, val: v } => {

                quote_response_level = Some( Field_QuoteResponseLevel_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRADINGSESSIONID, val: v } => {

                trading_session_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_NOQUOTEENTRIES, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_quote_entries18_fields(consumer, size);
                no_quote_entries = Some(subgroup);
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    QuoteCancelFields {
        quote_req_id: quote_req_id,
        quote_id: quote_id.unwrap() /* better error hdl? */ ,
        quote_cancel_type: quote_cancel_type.unwrap() /* better error hdl? */ ,
        quote_response_level: quote_response_level,
        trading_session_id: trading_session_id,
        no_quote_entries: no_quote_entries.unwrap() /* better error hdl? */ ,
    }
}


fn parse_message_quote_status_request_fields( consumer : &mut FixConsumer  ) -> QuoteStatusRequestFields {
    // fields:
    let mut quote_id : Option<String> = None;
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<Field_IDSource_Enum> = None;
    let mut security_type : Option<Field_SecurityType_Enum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<Field_PutOrCall_Enum> = None;
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
    let mut side : Option<Field_Side_Enum> = None;
    let mut trading_session_id : Option<String> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_QUOTEID, val: v } => {

                quote_id = Some( v.to_string() );
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

                idsource = Some( Field_IDSource_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                security_type = Some( Field_SecurityType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYMONTHYEAR, val: v } => {

                maturity_month_year = Some( UtcDate::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYDAY, val: v } => {

                maturity_day = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PUTORCALL, val: v } => {

                put_or_call = Some( Field_PutOrCall_Enum::from_str(v).unwrap() );
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

                side = Some( Field_Side_Enum::from_str(v).unwrap() );
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
    QuoteStatusRequestFields {
        quote_id: quote_id,
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
        side: side,
        trading_session_id: trading_session_id,
    }
}


fn parse_message_quote_acknowledgement_fields( consumer : &mut FixConsumer  ) -> QuoteAcknowledgementFields {
    // fields:
    let mut quote_req_id : Option<String> = None;
    let mut quote_id : Option<String> = None;
    let mut quote_ack_status : Option<Field_QuoteAckStatus_Enum> = None;
    let mut quote_reject_reason : Option<Field_QuoteRejectReason_Enum> = None;
    let mut quote_response_level : Option<Field_QuoteResponseLevel_Enum> = None;
    let mut trading_session_id : Option<String> = None;
    let mut text : Option<String> = None;
    let mut no_quote_sets : Option<Vec<NoQuoteSets21Fields>> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_QUOTEREQID, val: v } => {

                quote_req_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_QUOTEID, val: v } => {

                quote_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_QUOTEACKSTATUS, val: v } => {

                quote_ack_status = Some( Field_QuoteAckStatus_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_QUOTEREJECTREASON, val: v } => {

                quote_reject_reason = Some( Field_QuoteRejectReason_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_QUOTERESPONSELEVEL, val: v } => {

                quote_response_level = Some( Field_QuoteResponseLevel_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRADINGSESSIONID, val: v } => {

                trading_session_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_TEXT, val: v } => {

                text = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_NOQUOTESETS, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_quote_sets21_fields(consumer, size);
                no_quote_sets = Some(subgroup);
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    QuoteAcknowledgementFields {
        quote_req_id: quote_req_id,
        quote_id: quote_id,
        quote_ack_status: quote_ack_status.unwrap() /* better error hdl? */ ,
        quote_reject_reason: quote_reject_reason,
        quote_response_level: quote_response_level,
        trading_session_id: trading_session_id,
        text: text,
        no_quote_sets: no_quote_sets,
    }
}


fn parse_message_security_definition_request_fields( consumer : &mut FixConsumer  ) -> SecurityDefinitionRequestFields {
    // fields:
    let mut security_req_id : Option<String> = None;
    let mut security_request_type : Option<Field_SecurityRequestType_Enum> = None;
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<Field_IDSource_Enum> = None;
    let mut security_type : Option<Field_SecurityType_Enum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<Field_PutOrCall_Enum> = None;
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
    let mut currency : Option<f32> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;
    let mut trading_session_id : Option<String> = None;
    let mut no_related_sym : Option<Vec<NoRelatedSym23Fields>> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_SECURITYREQID, val: v } => {

                security_req_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SECURITYREQUESTTYPE, val: v } => {

                security_request_type = Some( Field_SecurityRequestType_Enum::from_str(v).unwrap() );
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

                idsource = Some( Field_IDSource_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                security_type = Some( Field_SecurityType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYMONTHYEAR, val: v } => {

                maturity_month_year = Some( UtcDate::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYDAY, val: v } => {

                maturity_day = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PUTORCALL, val: v } => {

                put_or_call = Some( Field_PutOrCall_Enum::from_str(v).unwrap() );
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
            &FieldVal { id: FIELD_CURRENCY, val: v } => {

                currency = Some( f32::from_str(v).unwrap() );
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
            &FieldVal { id: FIELD_TRADINGSESSIONID, val: v } => {

                trading_session_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_NORELATEDSYM, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_related_sym23_fields(consumer, size);
                no_related_sym = Some(subgroup);
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    SecurityDefinitionRequestFields {
        security_req_id: security_req_id.unwrap() /* better error hdl? */ ,
        security_request_type: security_request_type.unwrap() /* better error hdl? */ ,
        symbol: symbol,
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
        currency: currency,
        text: text,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
        trading_session_id: trading_session_id,
        no_related_sym: no_related_sym,
    }
}


fn parse_message_security_definition_fields( consumer : &mut FixConsumer  ) -> SecurityDefinitionFields {
    // fields:
    let mut security_req_id : Option<String> = None;
    let mut security_response_id : Option<String> = None;
    let mut security_response_type : Option<Field_SecurityResponseType_Enum> = None;
    let mut total_num_securities : Option<i32> = None;
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<Field_IDSource_Enum> = None;
    let mut security_type : Option<Field_SecurityType_Enum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<Field_PutOrCall_Enum> = None;
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
    let mut currency : Option<f32> = None;
    let mut trading_session_id : Option<String> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;
    let mut no_related_sym : Option<Vec<NoRelatedSym23Fields>> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_SECURITYREQID, val: v } => {

                security_req_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SECURITYRESPONSEID, val: v } => {

                security_response_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SECURITYRESPONSETYPE, val: v } => {

                security_response_type = Some( Field_SecurityResponseType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TOTALNUMSECURITIES, val: v } => {

                total_num_securities = Some( i32::from_str(v).unwrap() );
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

                idsource = Some( Field_IDSource_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                security_type = Some( Field_SecurityType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYMONTHYEAR, val: v } => {

                maturity_month_year = Some( UtcDate::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYDAY, val: v } => {

                maturity_day = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PUTORCALL, val: v } => {

                put_or_call = Some( Field_PutOrCall_Enum::from_str(v).unwrap() );
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
            &FieldVal { id: FIELD_CURRENCY, val: v } => {

                currency = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRADINGSESSIONID, val: v } => {

                trading_session_id = Some( v.to_string() );
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
            &FieldVal { id: FIELD_NORELATEDSYM, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_related_sym23_fields(consumer, size);
                no_related_sym = Some(subgroup);
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    SecurityDefinitionFields {
        security_req_id: security_req_id.unwrap() /* better error hdl? */ ,
        security_response_id: security_response_id.unwrap() /* better error hdl? */ ,
        security_response_type: security_response_type,
        total_num_securities: total_num_securities.unwrap() /* better error hdl? */ ,
        symbol: symbol,
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
        currency: currency,
        trading_session_id: trading_session_id,
        text: text,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
        no_related_sym: no_related_sym,
    }
}


fn parse_message_security_status_request_fields( consumer : &mut FixConsumer  ) -> SecurityStatusRequestFields {
    // fields:
    let mut security_status_req_id : Option<String> = None;
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<Field_IDSource_Enum> = None;
    let mut security_type : Option<Field_SecurityType_Enum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<Field_PutOrCall_Enum> = None;
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
    let mut currency : Option<f32> = None;
    let mut subscription_request_type : Option<Field_SubscriptionRequestType_Enum> = None;
    let mut trading_session_id : Option<String> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_SECURITYSTATUSREQID, val: v } => {

                security_status_req_id = Some( v.to_string() );
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

                idsource = Some( Field_IDSource_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                security_type = Some( Field_SecurityType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYMONTHYEAR, val: v } => {

                maturity_month_year = Some( UtcDate::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYDAY, val: v } => {

                maturity_day = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PUTORCALL, val: v } => {

                put_or_call = Some( Field_PutOrCall_Enum::from_str(v).unwrap() );
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
            &FieldVal { id: FIELD_CURRENCY, val: v } => {

                currency = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SUBSCRIPTIONREQUESTTYPE, val: v } => {

                subscription_request_type = Some( Field_SubscriptionRequestType_Enum::from_str(v).unwrap() );
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
    SecurityStatusRequestFields {
        security_status_req_id: security_status_req_id.unwrap() /* better error hdl? */ ,
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
        currency: currency,
        subscription_request_type: subscription_request_type.unwrap() /* better error hdl? */ ,
        trading_session_id: trading_session_id,
    }
}


fn parse_message_security_status_fields( consumer : &mut FixConsumer  ) -> SecurityStatusFields {
    // fields:
    let mut security_status_req_id : Option<String> = None;
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<Field_IDSource_Enum> = None;
    let mut security_type : Option<Field_SecurityType_Enum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<Field_PutOrCall_Enum> = None;
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
    let mut currency : Option<f32> = None;
    let mut trading_session_id : Option<String> = None;
    let mut unsolicited_indicator : Option<bool> = None;
    let mut security_trading_status : Option<Field_SecurityTradingStatus_Enum> = None;
    let mut financial_status : Option<Field_FinancialStatus_Enum> = None;
    let mut corporate_action : Option<Field_CorporateAction_Enum> = None;
    let mut halt_reason_char : Option<Field_HaltReasonChar_Enum> = None;
    let mut in_view_of_common : Option<bool> = None;
    let mut due_to_related : Option<bool> = None;
    let mut buy_volume : Option<f32> = None;
    let mut sell_volume : Option<f32> = None;
    let mut high_px : Option<f32> = None;
    let mut low_px : Option<f32> = None;
    let mut last_px : Option<f32> = None;
    let mut transact_time : Option<UtcDateTime> = None;
    let mut adjustment : Option<Field_Adjustment_Enum> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_SECURITYSTATUSREQID, val: v } => {

                security_status_req_id = Some( v.to_string() );
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

                idsource = Some( Field_IDSource_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                security_type = Some( Field_SecurityType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYMONTHYEAR, val: v } => {

                maturity_month_year = Some( UtcDate::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MATURITYDAY, val: v } => {

                maturity_day = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PUTORCALL, val: v } => {

                put_or_call = Some( Field_PutOrCall_Enum::from_str(v).unwrap() );
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
            &FieldVal { id: FIELD_CURRENCY, val: v } => {

                currency = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRADINGSESSIONID, val: v } => {

                trading_session_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_UNSOLICITEDINDICATOR, val: v } => {

                unsolicited_indicator = Some( boolconv(v) );
            },
            &FieldVal { id: FIELD_SECURITYTRADINGSTATUS, val: v } => {

                security_trading_status = Some( Field_SecurityTradingStatus_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_FINANCIALSTATUS, val: v } => {

                financial_status = Some( Field_FinancialStatus_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CORPORATEACTION, val: v } => {

                corporate_action = Some( Field_CorporateAction_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_HALTREASONCHAR, val: v } => {

                halt_reason_char = Some( Field_HaltReasonChar_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_INVIEWOFCOMMON, val: v } => {

                in_view_of_common = Some( boolconv(v) );
            },
            &FieldVal { id: FIELD_DUETORELATED, val: v } => {

                due_to_related = Some( boolconv(v) );
            },
            &FieldVal { id: FIELD_BUYVOLUME, val: v } => {

                buy_volume = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SELLVOLUME, val: v } => {

                sell_volume = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_HIGHPX, val: v } => {

                high_px = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_LOWPX, val: v } => {

                low_px = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_LASTPX, val: v } => {

                last_px = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRANSACTTIME, val: v } => {

                transact_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ADJUSTMENT, val: v } => {

                adjustment = Some( Field_Adjustment_Enum::from_str(v).unwrap() );
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    SecurityStatusFields {
        security_status_req_id: security_status_req_id,
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
        currency: currency,
        trading_session_id: trading_session_id,
        unsolicited_indicator: unsolicited_indicator,
        security_trading_status: security_trading_status,
        financial_status: financial_status,
        corporate_action: corporate_action,
        halt_reason_char: halt_reason_char,
        in_view_of_common: in_view_of_common,
        due_to_related: due_to_related,
        buy_volume: buy_volume,
        sell_volume: sell_volume,
        high_px: high_px,
        low_px: low_px,
        last_px: last_px,
        transact_time: transact_time,
        adjustment: adjustment,
    }
}


fn parse_message_trading_session_status_request_fields( consumer : &mut FixConsumer  ) -> TradingSessionStatusRequestFields {
    // fields:
    let mut trad_ses_req_id : Option<String> = None;
    let mut trading_session_id : Option<String> = None;
    let mut trad_ses_method : Option<Field_TradSesMethod_Enum> = None;
    let mut trad_ses_mode : Option<Field_TradSesMode_Enum> = None;
    let mut subscription_request_type : Option<Field_SubscriptionRequestType_Enum> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_TRADSESREQID, val: v } => {

                trad_ses_req_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_TRADINGSESSIONID, val: v } => {

                trading_session_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_TRADSESMETHOD, val: v } => {

                trad_ses_method = Some( Field_TradSesMethod_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRADSESMODE, val: v } => {

                trad_ses_mode = Some( Field_TradSesMode_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SUBSCRIPTIONREQUESTTYPE, val: v } => {

                subscription_request_type = Some( Field_SubscriptionRequestType_Enum::from_str(v).unwrap() );
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    TradingSessionStatusRequestFields {
        trad_ses_req_id: trad_ses_req_id.unwrap() /* better error hdl? */ ,
        trading_session_id: trading_session_id,
        trad_ses_method: trad_ses_method,
        trad_ses_mode: trad_ses_mode,
        subscription_request_type: subscription_request_type.unwrap() /* better error hdl? */ ,
    }
}


fn parse_message_trading_session_status_fields( consumer : &mut FixConsumer  ) -> TradingSessionStatusFields {
    // fields:
    let mut trad_ses_req_id : Option<String> = None;
    let mut trading_session_id : Option<String> = None;
    let mut trad_ses_method : Option<Field_TradSesMethod_Enum> = None;
    let mut trad_ses_mode : Option<Field_TradSesMode_Enum> = None;
    let mut unsolicited_indicator : Option<bool> = None;
    let mut trad_ses_status : Option<Field_TradSesStatus_Enum> = None;
    let mut trad_ses_start_time : Option<UtcDateTime> = None;
    let mut trad_ses_open_time : Option<UtcDateTime> = None;
    let mut trad_ses_pre_close_time : Option<UtcDateTime> = None;
    let mut trad_ses_close_time : Option<UtcDateTime> = None;
    let mut trad_ses_end_time : Option<UtcDateTime> = None;
    let mut total_volume_traded : Option<f32> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_TRADSESREQID, val: v } => {

                trad_ses_req_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_TRADINGSESSIONID, val: v } => {

                trading_session_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_TRADSESMETHOD, val: v } => {

                trad_ses_method = Some( Field_TradSesMethod_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRADSESMODE, val: v } => {

                trad_ses_mode = Some( Field_TradSesMode_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_UNSOLICITEDINDICATOR, val: v } => {

                unsolicited_indicator = Some( boolconv(v) );
            },
            &FieldVal { id: FIELD_TRADSESSTATUS, val: v } => {

                trad_ses_status = Some( Field_TradSesStatus_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRADSESSTARTTIME, val: v } => {

                trad_ses_start_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRADSESOPENTIME, val: v } => {

                trad_ses_open_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRADSESPRECLOSETIME, val: v } => {

                trad_ses_pre_close_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRADSESCLOSETIME, val: v } => {

                trad_ses_close_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRADSESENDTIME, val: v } => {

                trad_ses_end_time = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TOTALVOLUMETRADED, val: v } => {

                total_volume_traded = Some( f32::from_str(v).unwrap() );
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
    TradingSessionStatusFields {
        trad_ses_req_id: trad_ses_req_id,
        trading_session_id: trading_session_id.unwrap() /* better error hdl? */ ,
        trad_ses_method: trad_ses_method,
        trad_ses_mode: trad_ses_mode,
        unsolicited_indicator: unsolicited_indicator,
        trad_ses_status: trad_ses_status.unwrap() /* better error hdl? */ ,
        trad_ses_start_time: trad_ses_start_time,
        trad_ses_open_time: trad_ses_open_time,
        trad_ses_pre_close_time: trad_ses_pre_close_time,
        trad_ses_close_time: trad_ses_close_time,
        trad_ses_end_time: trad_ses_end_time,
        total_volume_traded: total_volume_traded,
        text: text,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
    }
}


fn parse_message_mass_quote_fields( consumer : &mut FixConsumer  ) -> MassQuoteFields {
    // fields:
    let mut quote_req_id : Option<String> = None;
    let mut quote_id : Option<String> = None;
    let mut quote_response_level : Option<Field_QuoteResponseLevel_Enum> = None;
    let mut def_bid_size : Option<f32> = None;
    let mut def_offer_size : Option<f32> = None;
    let mut no_quote_sets : Option<Vec<NoQuoteSets22Fields>> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_QUOTEREQID, val: v } => {

                quote_req_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_QUOTEID, val: v } => {

                quote_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_QUOTERESPONSELEVEL, val: v } => {

                quote_response_level = Some( Field_QuoteResponseLevel_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_DEFBIDSIZE, val: v } => {

                def_bid_size = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_DEFOFFERSIZE, val: v } => {

                def_offer_size = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_NOQUOTESETS, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_quote_sets22_fields(consumer, size);
                no_quote_sets = Some(subgroup);
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    MassQuoteFields {
        quote_req_id: quote_req_id,
        quote_id: quote_id.unwrap() /* better error hdl? */ ,
        quote_response_level: quote_response_level,
        def_bid_size: def_bid_size,
        def_offer_size: def_offer_size,
        no_quote_sets: no_quote_sets.unwrap() /* better error hdl? */ ,
    }
}


fn parse_message_business_message_reject_fields( consumer : &mut FixConsumer  ) -> BusinessMessageRejectFields {
    // fields:
    let mut ref_seq_num : Option<i32> = None;
    let mut ref_msg_type : Option<String> = None;
    let mut business_reject_ref_id : Option<String> = None;
    let mut business_reject_reason : Option<Field_BusinessRejectReason_Enum> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_REFSEQNUM, val: v } => {

                ref_seq_num = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_REFMSGTYPE, val: v } => {

                ref_msg_type = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_BUSINESSREJECTREFID, val: v } => {

                business_reject_ref_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_BUSINESSREJECTREASON, val: v } => {

                business_reject_reason = Some( Field_BusinessRejectReason_Enum::from_str(v).unwrap() );
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
    BusinessMessageRejectFields {
        ref_seq_num: ref_seq_num,
        ref_msg_type: ref_msg_type.unwrap() /* better error hdl? */ ,
        business_reject_ref_id: business_reject_ref_id,
        business_reject_reason: business_reject_reason.unwrap() /* better error hdl? */ ,
        text: text,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
    }
}


fn parse_message_bid_request_fields( consumer : &mut FixConsumer  ) -> BidRequestFields {
    // fields:
    let mut bid_id : Option<String> = None;
    let mut client_bid_id : Option<String> = None;
    let mut bid_request_trans_type : Option<Field_BidRequestTransType_Enum> = None;
    let mut list_name : Option<String> = None;
    let mut total_num_securities : Option<i32> = None;
    let mut bid_type : Option<i32> = None;
    let mut num_tickets : Option<i32> = None;
    let mut currency : Option<f32> = None;
    let mut side_value1 : Option<f32> = None;
    let mut side_value2 : Option<f32> = None;
    let mut no_bid_descriptors : Option<Vec<NoBidDescriptors6Fields>> = None;
    let mut no_bid_components : Option<Vec<NoBidComponents4Fields>> = None;
    let mut liquidity_ind_type : Option<Field_LiquidityIndType_Enum> = None;
    let mut wt_average_liquidity : Option<f32> = None;
    let mut exchange_for_physical : Option<bool> = None;
    let mut out_main_cntry_uindex : Option<f32> = None;
    let mut cross_percent : Option<f32> = None;
    let mut prog_rpt_reqs : Option<Field_ProgRptReqs_Enum> = None;
    let mut prog_period_interval : Option<i32> = None;
    let mut inc_tax_ind : Option<Field_IncTaxInd_Enum> = None;
    let mut forex_req : Option<bool> = None;
    let mut num_bidders : Option<i32> = None;
    let mut trade_date : Option<UtcDateTime> = None;
    let mut trade_type : Option<Field_TradeType_Enum> = None;
    let mut basis_px_type : Option<Field_BasisPxType_Enum> = None;
    let mut strike_time : Option<UtcDateTime> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_BIDID, val: v } => {

                bid_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_CLIENTBIDID, val: v } => {

                client_bid_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_BIDREQUESTTRANSTYPE, val: v } => {

                bid_request_trans_type = Some( Field_BidRequestTransType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_LISTNAME, val: v } => {

                list_name = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_TOTALNUMSECURITIES, val: v } => {

                total_num_securities = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_BIDTYPE, val: v } => {

                bid_type = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_NUMTICKETS, val: v } => {

                num_tickets = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CURRENCY, val: v } => {

                currency = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SIDEVALUE1, val: v } => {

                side_value1 = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SIDEVALUE2, val: v } => {

                side_value2 = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_NOBIDDESCRIPTORS, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_bid_descriptors6_fields(consumer, size);
                no_bid_descriptors = Some(subgroup);
            },
            &FieldVal { id: FIELD_NOBIDCOMPONENTS, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_bid_components4_fields(consumer, size);
                no_bid_components = Some(subgroup);
            },
            &FieldVal { id: FIELD_LIQUIDITYINDTYPE, val: v } => {

                liquidity_ind_type = Some( Field_LiquidityIndType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_WTAVERAGELIQUIDITY, val: v } => {

                wt_average_liquidity = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_EXCHANGEFORPHYSICAL, val: v } => {

                exchange_for_physical = Some( boolconv(v) );
            },
            &FieldVal { id: FIELD_OUTMAINCNTRYUINDEX, val: v } => {

                out_main_cntry_uindex = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CROSSPERCENT, val: v } => {

                cross_percent = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PROGRPTREQS, val: v } => {

                prog_rpt_reqs = Some( Field_ProgRptReqs_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PROGPERIODINTERVAL, val: v } => {

                prog_period_interval = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_INCTAXIND, val: v } => {

                inc_tax_ind = Some( Field_IncTaxInd_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_FOREXREQ, val: v } => {

                forex_req = Some( boolconv(v) );
            },
            &FieldVal { id: FIELD_NUMBIDDERS, val: v } => {

                num_bidders = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRADEDATE, val: v } => {

                trade_date = Some( UtcDateTime::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRADETYPE, val: v } => {

                trade_type = Some( Field_TradeType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_BASISPXTYPE, val: v } => {

                basis_px_type = Some( Field_BasisPxType_Enum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_STRIKETIME, val: v } => {

                strike_time = Some( UtcDateTime::from_str(v).unwrap() );
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
    BidRequestFields {
        bid_id: bid_id,
        client_bid_id: client_bid_id.unwrap() /* better error hdl? */ ,
        bid_request_trans_type: bid_request_trans_type.unwrap() /* better error hdl? */ ,
        list_name: list_name,
        total_num_securities: total_num_securities.unwrap() /* better error hdl? */ ,
        bid_type: bid_type.unwrap() /* better error hdl? */ ,
        num_tickets: num_tickets,
        currency: currency,
        side_value1: side_value1,
        side_value2: side_value2,
        no_bid_descriptors: no_bid_descriptors,
        no_bid_components: no_bid_components,
        liquidity_ind_type: liquidity_ind_type,
        wt_average_liquidity: wt_average_liquidity,
        exchange_for_physical: exchange_for_physical,
        out_main_cntry_uindex: out_main_cntry_uindex,
        cross_percent: cross_percent,
        prog_rpt_reqs: prog_rpt_reqs,
        prog_period_interval: prog_period_interval,
        inc_tax_ind: inc_tax_ind,
        forex_req: forex_req,
        num_bidders: num_bidders,
        trade_date: trade_date,
        trade_type: trade_type.unwrap() /* better error hdl? */ ,
        basis_px_type: basis_px_type.unwrap() /* better error hdl? */ ,
        strike_time: strike_time,
        text: text,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
    }
}


fn parse_message_bid_response_fields( consumer : &mut FixConsumer  ) -> BidResponseFields {
    // fields:
    let mut bid_id : Option<String> = None;
    let mut client_bid_id : Option<String> = None;
    let mut no_bid_components : Option<Vec<NoBidComponents5Fields>> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_BIDID, val: v } => {

                bid_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_CLIENTBIDID, val: v } => {

                client_bid_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_NOBIDCOMPONENTS, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_bid_components5_fields(consumer, size);
                no_bid_components = Some(subgroup);
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    BidResponseFields {
        bid_id: bid_id,
        client_bid_id: client_bid_id,
        no_bid_components: no_bid_components.unwrap() /* better error hdl? */ ,
    }
}


fn parse_message_list_strike_price_fields( consumer : &mut FixConsumer  ) -> ListStrikePriceFields {
    // fields:
    let mut list_id : Option<String> = None;
    let mut tot_no_strikes : Option<i32> = None;
    let mut no_strikes : Option<Vec<NoStrikes28Fields>> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_LISTID, val: v } => {

                list_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_TOTNOSTRIKES, val: v } => {

                tot_no_strikes = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_NOSTRIKES, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_strikes28_fields(consumer, size);
                no_strikes = Some(subgroup);
            },
            _ => {
                // unknown field? why? should it be added raw to some list?
            }
        }
    }

    // construction
    ListStrikePriceFields {
        list_id: list_id.unwrap() /* better error hdl? */ ,
        tot_no_strikes: tot_no_strikes.unwrap() /* better error hdl? */ ,
        no_strikes: no_strikes.unwrap() /* better error hdl? */ ,
    }
}





fn build_group_no_allocs3_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoAllocs3Fields> {
    let mut items : Vec<NoAllocs3Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_allocs3_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_allocs3_fields_line(consumer: &mut FixConsumer) -> NoAllocs3Fields {
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
    NoAllocs3Fields {
        alloc_account: alloc_account,
        alloc_shares: alloc_shares,
    }
}


fn build_group_no_mdentries10_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoMDEntries10Fields> {
    let mut items : Vec<NoMDEntries10Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_mdentries10_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_mdentries10_fields_line(consumer: &mut FixConsumer) -> NoMDEntries10Fields {
    // fields
    let mut mdentry_type : Option<Field_MDEntryType_Enum> = None;
    let mut mdentry_px : Option<f32> = None;
    let mut currency : Option<f32> = None;
    let mut mdentry_size : Option<f32> = None;
    let mut mdentry_date : Option<UtcDate> = None;
    let mut mdentry_time : Option<UtcTime> = None;
    let mut tick_direction : Option<Field_TickDirection_Enum> = None;
    let mut mdmkt : Option<String> = None;
    let mut trading_session_id : Option<String> = None;
    let mut quote_condition : Option<Field_QuoteCondition_Enum> = None;
    let mut trade_condition : Option<Field_TradeCondition_Enum> = None;
    let mut mdentry_originator : Option<String> = None;
    let mut location_id : Option<String> = None;
    let mut desk_id : Option<String> = None;
    let mut open_close_settle_flag : Option<Field_OpenCloseSettleFlag_Enum> = None;
    let mut time_in_force : Option<Field_TimeInForce_Enum> = None;
    let mut expire_date : Option<UtcDateTime> = None;
    let mut expire_time : Option<UtcDateTime> = None;
    let mut min_qty : Option<f32> = None;
    let mut exec_inst : Option<Field_ExecInst_Enum> = None;
    let mut seller_days : Option<i32> = None;
    let mut order_id : Option<String> = None;
    let mut quote_entry_id : Option<String> = None;
    let mut mdentry_buyer : Option<String> = None;
    let mut mdentry_seller : Option<String> = None;
    let mut number_of_orders : Option<i32> = None;
    let mut mdentry_position_no : Option<i32> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_MDENTRYTYPE, val: v } => {

                if mdentry_type.is_some() { break; }

                mdentry_type = Some( Field_MDEntryType_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_MDENTRYPX, val: v } => {

                if mdentry_px.is_some() { break; }

                mdentry_px = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_CURRENCY, val: v } => {

                if currency.is_some() { break; }

                currency = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_MDENTRYSIZE, val: v } => {

                if mdentry_size.is_some() { break; }

                mdentry_size = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_MDENTRYDATE, val: v } => {

                if mdentry_date.is_some() { break; }

                mdentry_date = Some( UtcDate::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_MDENTRYTIME, val: v } => {

                if mdentry_time.is_some() { break; }

                mdentry_time = Some( UtcTime::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_TICKDIRECTION, val: v } => {

                if tick_direction.is_some() { break; }

                tick_direction = Some( Field_TickDirection_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_MDMKT, val: v } => {

                if mdmkt.is_some() { break; }

                mdmkt = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_TRADINGSESSIONID, val: v } => {

                if trading_session_id.is_some() { break; }

                trading_session_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_QUOTECONDITION, val: v } => {

                if quote_condition.is_some() { break; }

                quote_condition = Some( Field_QuoteCondition_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_TRADECONDITION, val: v } => {

                if trade_condition.is_some() { break; }

                trade_condition = Some( Field_TradeCondition_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_MDENTRYORIGINATOR, val: v } => {

                if mdentry_originator.is_some() { break; }

                mdentry_originator = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_LOCATIONID, val: v } => {

                if location_id.is_some() { break; }

                location_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_DESKID, val: v } => {

                if desk_id.is_some() { break; }

                desk_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_OPENCLOSESETTLEFLAG, val: v } => {

                if open_close_settle_flag.is_some() { break; }

                open_close_settle_flag = Some( Field_OpenCloseSettleFlag_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_TIMEINFORCE, val: v } => {

                if time_in_force.is_some() { break; }

                time_in_force = Some( Field_TimeInForce_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_EXPIREDATE, val: v } => {

                if expire_date.is_some() { break; }

                expire_date = Some( UtcDateTime::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_EXPIRETIME, val: v } => {

                if expire_time.is_some() { break; }

                expire_time = Some( UtcDateTime::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_MINQTY, val: v } => {

                if min_qty.is_some() { break; }

                min_qty = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_EXECINST, val: v } => {

                if exec_inst.is_some() { break; }

                exec_inst = Some( Field_ExecInst_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SELLERDAYS, val: v } => {

                if seller_days.is_some() { break; }

                seller_days = Some( i32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ORDERID, val: v } => {

                if order_id.is_some() { break; }

                order_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_QUOTEENTRYID, val: v } => {

                if quote_entry_id.is_some() { break; }

                quote_entry_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_MDENTRYBUYER, val: v } => {

                if mdentry_buyer.is_some() { break; }

                mdentry_buyer = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_MDENTRYSELLER, val: v } => {

                if mdentry_seller.is_some() { break; }

                mdentry_seller = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_NUMBEROFORDERS, val: v } => {

                if number_of_orders.is_some() { break; }

                number_of_orders = Some( i32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_MDENTRYPOSITIONNO, val: v } => {

                if mdentry_position_no.is_some() { break; }

                mdentry_position_no = Some( i32::from_str(v).unwrap() );
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


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoMDEntries10Fields {
        mdentry_type: mdentry_type.unwrap() ,
        mdentry_px: mdentry_px.unwrap() ,
        currency: currency,
        mdentry_size: mdentry_size,
        mdentry_date: mdentry_date,
        mdentry_time: mdentry_time,
        tick_direction: tick_direction,
        mdmkt: mdmkt,
        trading_session_id: trading_session_id,
        quote_condition: quote_condition,
        trade_condition: trade_condition,
        mdentry_originator: mdentry_originator,
        location_id: location_id,
        desk_id: desk_id,
        open_close_settle_flag: open_close_settle_flag,
        time_in_force: time_in_force,
        expire_date: expire_date,
        expire_time: expire_time,
        min_qty: min_qty,
        exec_inst: exec_inst,
        seller_days: seller_days,
        order_id: order_id,
        quote_entry_id: quote_entry_id,
        mdentry_buyer: mdentry_buyer,
        mdentry_seller: mdentry_seller,
        number_of_orders: number_of_orders,
        mdentry_position_no: mdentry_position_no,
        text: text,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
    }
}


fn build_group_no_related_sym23_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoRelatedSym23Fields> {
    let mut items : Vec<NoRelatedSym23Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_related_sym23_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_related_sym23_fields_line(consumer: &mut FixConsumer) -> NoRelatedSym23Fields {
    // fields
    let mut underlying_symbol : Option<String> = None;
    let mut underlying_symbol_sfx : Option<String> = None;
    let mut underlying_security_id : Option<String> = None;
    let mut underlying_idsource : Option<String> = None;
    let mut underlying_security_type : Option<String> = None;
    let mut underlying_maturity_month_year : Option<UtcDate> = None;
    let mut underlying_maturity_day : Option<i32> = None;
    let mut underlying_put_or_call : Option<i32> = None;
    let mut underlying_strike_price : Option<f32> = None;
    let mut underlying_opt_attribute : Option<char> = None;
    let mut underlying_contract_multiplier : Option<f32> = None;
    let mut underlying_coupon_rate : Option<f32> = None;
    let mut underlying_security_exchange : Option<String> = None;
    let mut underlying_issuer : Option<String> = None;
    let mut encoded_underlying_issuer_len : Option<usize> = None;
    let mut encoded_underlying_issuer : Option<String> = None;
    let mut underlying_security_desc : Option<String> = None;
    let mut encoded_underlying_security_desc_len : Option<usize> = None;
    let mut encoded_underlying_security_desc : Option<String> = None;
    let mut ratio_qty : Option<f32> = None;
    let mut side : Option<Field_Side_Enum> = None;
    let mut underlying_currency : Option<f32> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_UNDERLYINGSYMBOL, val: v } => {

                if underlying_symbol.is_some() { break; }

                underlying_symbol = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_UNDERLYINGSYMBOLSFX, val: v } => {

                if underlying_symbol_sfx.is_some() { break; }

                underlying_symbol_sfx = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_UNDERLYINGSECURITYID, val: v } => {

                if underlying_security_id.is_some() { break; }

                underlying_security_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_UNDERLYINGIDSOURCE, val: v } => {

                if underlying_idsource.is_some() { break; }

                underlying_idsource = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_UNDERLYINGSECURITYTYPE, val: v } => {

                if underlying_security_type.is_some() { break; }

                underlying_security_type = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_UNDERLYINGMATURITYMONTHYEAR, val: v } => {

                if underlying_maturity_month_year.is_some() { break; }

                underlying_maturity_month_year = Some( UtcDate::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_UNDERLYINGMATURITYDAY, val: v } => {

                if underlying_maturity_day.is_some() { break; }

                underlying_maturity_day = Some( i32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_UNDERLYINGPUTORCALL, val: v } => {

                if underlying_put_or_call.is_some() { break; }

                underlying_put_or_call = Some( i32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_UNDERLYINGSTRIKEPRICE, val: v } => {

                if underlying_strike_price.is_some() { break; }

                underlying_strike_price = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_UNDERLYINGOPTATTRIBUTE, val: v } => {

                if underlying_opt_attribute.is_some() { break; }

                underlying_opt_attribute = Some( char::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_UNDERLYINGCONTRACTMULTIPLIER, val: v } => {

                if underlying_contract_multiplier.is_some() { break; }

                underlying_contract_multiplier = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_UNDERLYINGCOUPONRATE, val: v } => {

                if underlying_coupon_rate.is_some() { break; }

                underlying_coupon_rate = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_UNDERLYINGSECURITYEXCHANGE, val: v } => {

                if underlying_security_exchange.is_some() { break; }

                underlying_security_exchange = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_UNDERLYINGISSUER, val: v } => {

                if underlying_issuer.is_some() { break; }

                underlying_issuer = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_ENCODEDUNDERLYINGISSUERLEN, val: v } => {

                if encoded_underlying_issuer_len.is_some() { break; }

                encoded_underlying_issuer_len = Some( usize::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ENCODEDUNDERLYINGISSUER, val: v } => {

                if encoded_underlying_issuer.is_some() { break; }

                encoded_underlying_issuer = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_UNDERLYINGSECURITYDESC, val: v } => {

                if underlying_security_desc.is_some() { break; }

                underlying_security_desc = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_ENCODEDUNDERLYINGSECURITYDESCLEN, val: v } => {

                if encoded_underlying_security_desc_len.is_some() { break; }

                encoded_underlying_security_desc_len = Some( usize::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ENCODEDUNDERLYINGSECURITYDESC, val: v } => {

                if encoded_underlying_security_desc.is_some() { break; }

                encoded_underlying_security_desc = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_RATIOQTY, val: v } => {

                if ratio_qty.is_some() { break; }

                ratio_qty = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SIDE, val: v } => {

                if side.is_some() { break; }

                side = Some( Field_Side_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_UNDERLYINGCURRENCY, val: v } => {

                if underlying_currency.is_some() { break; }

                underlying_currency = Some( f32::from_str(v).unwrap() );
            },


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoRelatedSym23Fields {
        underlying_symbol: underlying_symbol,
        underlying_symbol_sfx: underlying_symbol_sfx,
        underlying_security_id: underlying_security_id,
        underlying_idsource: underlying_idsource,
        underlying_security_type: underlying_security_type,
        underlying_maturity_month_year: underlying_maturity_month_year,
        underlying_maturity_day: underlying_maturity_day,
        underlying_put_or_call: underlying_put_or_call,
        underlying_strike_price: underlying_strike_price,
        underlying_opt_attribute: underlying_opt_attribute,
        underlying_contract_multiplier: underlying_contract_multiplier,
        underlying_coupon_rate: underlying_coupon_rate,
        underlying_security_exchange: underlying_security_exchange,
        underlying_issuer: underlying_issuer,
        encoded_underlying_issuer_len: encoded_underlying_issuer_len,
        encoded_underlying_issuer: encoded_underlying_issuer,
        underlying_security_desc: underlying_security_desc,
        encoded_underlying_security_desc_len: encoded_underlying_security_desc_len,
        encoded_underlying_security_desc: encoded_underlying_security_desc,
        ratio_qty: ratio_qty,
        side: side,
        underlying_currency: underlying_currency,
    }
}


fn build_group_no_mdentry_types12_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoMDEntryTypes12Fields> {
    let mut items : Vec<NoMDEntryTypes12Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_mdentry_types12_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_mdentry_types12_fields_line(consumer: &mut FixConsumer) -> NoMDEntryTypes12Fields {
    // fields
    let mut mdentry_type : Option<Field_MDEntryType_Enum> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_MDENTRYTYPE, val: v } => {

                if mdentry_type.is_some() { break; }

                mdentry_type = Some( Field_MDEntryType_Enum::from_str(v).unwrap() );
            },


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoMDEntryTypes12Fields {
        mdentry_type: mdentry_type.unwrap() ,
    }
}


fn build_group_no_quote_entries18_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoQuoteEntries18Fields> {
    let mut items : Vec<NoQuoteEntries18Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_quote_entries18_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_quote_entries18_fields_line(consumer: &mut FixConsumer) -> NoQuoteEntries18Fields {
    // fields
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<Field_IDSource_Enum> = None;
    let mut security_type : Option<Field_SecurityType_Enum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<Field_PutOrCall_Enum> = None;
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
    let mut underlying_symbol : Option<String> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
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

                idsource = Some( Field_IDSource_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                if security_type.is_some() { break; }

                security_type = Some( Field_SecurityType_Enum::from_str(v).unwrap() );
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

                put_or_call = Some( Field_PutOrCall_Enum::from_str(v).unwrap() );
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

            &FieldVal { id: FIELD_UNDERLYINGSYMBOL, val: v } => {

                if underlying_symbol.is_some() { break; }

                underlying_symbol = Some( v.to_string() );
            },


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoQuoteEntries18Fields {
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
        underlying_symbol: underlying_symbol,
    }
}


fn build_group_no_quote_entries20_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoQuoteEntries20Fields> {
    let mut items : Vec<NoQuoteEntries20Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_quote_entries20_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_quote_entries20_fields_line(consumer: &mut FixConsumer) -> NoQuoteEntries20Fields {
    // fields
    let mut quote_entry_id : Option<String> = None;
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<Field_IDSource_Enum> = None;
    let mut security_type : Option<Field_SecurityType_Enum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<Field_PutOrCall_Enum> = None;
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
    let mut bid_px : Option<f32> = None;
    let mut offer_px : Option<f32> = None;
    let mut bid_size : Option<f32> = None;
    let mut offer_size : Option<f32> = None;
    let mut valid_until_time : Option<UtcDateTime> = None;
    let mut bid_spot_rate : Option<f32> = None;
    let mut offer_spot_rate : Option<f32> = None;
    let mut bid_forward_points : Option<f32> = None;
    let mut offer_forward_points : Option<f32> = None;
    let mut transact_time : Option<UtcDateTime> = None;
    let mut trading_session_id : Option<String> = None;
    let mut fut_sett_date : Option<UtcDateTime> = None;
    let mut ord_type : Option<Field_OrdType_Enum> = None;
    let mut fut_sett_date2 : Option<UtcDateTime> = None;
    let mut order_qty2 : Option<f32> = None;
    let mut currency : Option<f32> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_QUOTEENTRYID, val: v } => {

                if quote_entry_id.is_some() { break; }

                quote_entry_id = Some( v.to_string() );
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

                idsource = Some( Field_IDSource_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                if security_type.is_some() { break; }

                security_type = Some( Field_SecurityType_Enum::from_str(v).unwrap() );
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

                put_or_call = Some( Field_PutOrCall_Enum::from_str(v).unwrap() );
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

            &FieldVal { id: FIELD_BIDPX, val: v } => {

                if bid_px.is_some() { break; }

                bid_px = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_OFFERPX, val: v } => {

                if offer_px.is_some() { break; }

                offer_px = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_BIDSIZE, val: v } => {

                if bid_size.is_some() { break; }

                bid_size = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_OFFERSIZE, val: v } => {

                if offer_size.is_some() { break; }

                offer_size = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_VALIDUNTILTIME, val: v } => {

                if valid_until_time.is_some() { break; }

                valid_until_time = Some( UtcDateTime::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_BIDSPOTRATE, val: v } => {

                if bid_spot_rate.is_some() { break; }

                bid_spot_rate = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_OFFERSPOTRATE, val: v } => {

                if offer_spot_rate.is_some() { break; }

                offer_spot_rate = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_BIDFORWARDPOINTS, val: v } => {

                if bid_forward_points.is_some() { break; }

                bid_forward_points = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_OFFERFORWARDPOINTS, val: v } => {

                if offer_forward_points.is_some() { break; }

                offer_forward_points = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_TRANSACTTIME, val: v } => {

                if transact_time.is_some() { break; }

                transact_time = Some( UtcDateTime::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_TRADINGSESSIONID, val: v } => {

                if trading_session_id.is_some() { break; }

                trading_session_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_FUTSETTDATE, val: v } => {

                if fut_sett_date.is_some() { break; }

                fut_sett_date = Some( UtcDateTime::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ORDTYPE, val: v } => {

                if ord_type.is_some() { break; }

                ord_type = Some( Field_OrdType_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_FUTSETTDATE2, val: v } => {

                if fut_sett_date2.is_some() { break; }

                fut_sett_date2 = Some( UtcDateTime::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ORDERQTY2, val: v } => {

                if order_qty2.is_some() { break; }

                order_qty2 = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_CURRENCY, val: v } => {

                if currency.is_some() { break; }

                currency = Some( f32::from_str(v).unwrap() );
            },


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoQuoteEntries20Fields {
        quote_entry_id: quote_entry_id.unwrap() ,
        symbol: symbol,
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
        bid_px: bid_px,
        offer_px: offer_px,
        bid_size: bid_size,
        offer_size: offer_size,
        valid_until_time: valid_until_time,
        bid_spot_rate: bid_spot_rate,
        offer_spot_rate: offer_spot_rate,
        bid_forward_points: bid_forward_points,
        offer_forward_points: offer_forward_points,
        transact_time: transact_time,
        trading_session_id: trading_session_id,
        fut_sett_date: fut_sett_date,
        ord_type: ord_type,
        fut_sett_date2: fut_sett_date2,
        order_qty2: order_qty2,
        currency: currency,
    }
}


fn build_group_no_mdentries11_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoMDEntries11Fields> {
    let mut items : Vec<NoMDEntries11Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_mdentries11_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_mdentries11_fields_line(consumer: &mut FixConsumer) -> NoMDEntries11Fields {
    // fields
    let mut mdupdate_action : Option<Field_MDUpdateAction_Enum> = None;
    let mut delete_reason : Option<Field_DeleteReason_Enum> = None;
    let mut mdentry_type : Option<Field_MDEntryType_Enum> = None;
    let mut mdentry_id : Option<String> = None;
    let mut mdentry_ref_id : Option<String> = None;
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<Field_IDSource_Enum> = None;
    let mut security_type : Option<Field_SecurityType_Enum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<Field_PutOrCall_Enum> = None;
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
    let mut financial_status : Option<Field_FinancialStatus_Enum> = None;
    let mut corporate_action : Option<Field_CorporateAction_Enum> = None;
    let mut mdentry_px : Option<f32> = None;
    let mut currency : Option<f32> = None;
    let mut mdentry_size : Option<f32> = None;
    let mut mdentry_date : Option<UtcDate> = None;
    let mut mdentry_time : Option<UtcTime> = None;
    let mut tick_direction : Option<Field_TickDirection_Enum> = None;
    let mut mdmkt : Option<String> = None;
    let mut trading_session_id : Option<String> = None;
    let mut quote_condition : Option<Field_QuoteCondition_Enum> = None;
    let mut trade_condition : Option<Field_TradeCondition_Enum> = None;
    let mut mdentry_originator : Option<String> = None;
    let mut location_id : Option<String> = None;
    let mut desk_id : Option<String> = None;
    let mut open_close_settle_flag : Option<Field_OpenCloseSettleFlag_Enum> = None;
    let mut time_in_force : Option<Field_TimeInForce_Enum> = None;
    let mut expire_date : Option<UtcDateTime> = None;
    let mut expire_time : Option<UtcDateTime> = None;
    let mut min_qty : Option<f32> = None;
    let mut exec_inst : Option<Field_ExecInst_Enum> = None;
    let mut seller_days : Option<i32> = None;
    let mut order_id : Option<String> = None;
    let mut quote_entry_id : Option<String> = None;
    let mut mdentry_buyer : Option<String> = None;
    let mut mdentry_seller : Option<String> = None;
    let mut number_of_orders : Option<i32> = None;
    let mut mdentry_position_no : Option<i32> = None;
    let mut total_volume_traded : Option<f32> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_MDUPDATEACTION, val: v } => {

                if mdupdate_action.is_some() { break; }

                mdupdate_action = Some( Field_MDUpdateAction_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_DELETEREASON, val: v } => {

                if delete_reason.is_some() { break; }

                delete_reason = Some( Field_DeleteReason_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_MDENTRYTYPE, val: v } => {

                if mdentry_type.is_some() { break; }

                mdentry_type = Some( Field_MDEntryType_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_MDENTRYID, val: v } => {

                if mdentry_id.is_some() { break; }

                mdentry_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_MDENTRYREFID, val: v } => {

                if mdentry_ref_id.is_some() { break; }

                mdentry_ref_id = Some( v.to_string() );
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

                idsource = Some( Field_IDSource_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                if security_type.is_some() { break; }

                security_type = Some( Field_SecurityType_Enum::from_str(v).unwrap() );
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

                put_or_call = Some( Field_PutOrCall_Enum::from_str(v).unwrap() );
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

            &FieldVal { id: FIELD_FINANCIALSTATUS, val: v } => {

                if financial_status.is_some() { break; }

                financial_status = Some( Field_FinancialStatus_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_CORPORATEACTION, val: v } => {

                if corporate_action.is_some() { break; }

                corporate_action = Some( Field_CorporateAction_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_MDENTRYPX, val: v } => {

                if mdentry_px.is_some() { break; }

                mdentry_px = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_CURRENCY, val: v } => {

                if currency.is_some() { break; }

                currency = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_MDENTRYSIZE, val: v } => {

                if mdentry_size.is_some() { break; }

                mdentry_size = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_MDENTRYDATE, val: v } => {

                if mdentry_date.is_some() { break; }

                mdentry_date = Some( UtcDate::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_MDENTRYTIME, val: v } => {

                if mdentry_time.is_some() { break; }

                mdentry_time = Some( UtcTime::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_TICKDIRECTION, val: v } => {

                if tick_direction.is_some() { break; }

                tick_direction = Some( Field_TickDirection_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_MDMKT, val: v } => {

                if mdmkt.is_some() { break; }

                mdmkt = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_TRADINGSESSIONID, val: v } => {

                if trading_session_id.is_some() { break; }

                trading_session_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_QUOTECONDITION, val: v } => {

                if quote_condition.is_some() { break; }

                quote_condition = Some( Field_QuoteCondition_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_TRADECONDITION, val: v } => {

                if trade_condition.is_some() { break; }

                trade_condition = Some( Field_TradeCondition_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_MDENTRYORIGINATOR, val: v } => {

                if mdentry_originator.is_some() { break; }

                mdentry_originator = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_LOCATIONID, val: v } => {

                if location_id.is_some() { break; }

                location_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_DESKID, val: v } => {

                if desk_id.is_some() { break; }

                desk_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_OPENCLOSESETTLEFLAG, val: v } => {

                if open_close_settle_flag.is_some() { break; }

                open_close_settle_flag = Some( Field_OpenCloseSettleFlag_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_TIMEINFORCE, val: v } => {

                if time_in_force.is_some() { break; }

                time_in_force = Some( Field_TimeInForce_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_EXPIREDATE, val: v } => {

                if expire_date.is_some() { break; }

                expire_date = Some( UtcDateTime::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_EXPIRETIME, val: v } => {

                if expire_time.is_some() { break; }

                expire_time = Some( UtcDateTime::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_MINQTY, val: v } => {

                if min_qty.is_some() { break; }

                min_qty = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_EXECINST, val: v } => {

                if exec_inst.is_some() { break; }

                exec_inst = Some( Field_ExecInst_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SELLERDAYS, val: v } => {

                if seller_days.is_some() { break; }

                seller_days = Some( i32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ORDERID, val: v } => {

                if order_id.is_some() { break; }

                order_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_QUOTEENTRYID, val: v } => {

                if quote_entry_id.is_some() { break; }

                quote_entry_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_MDENTRYBUYER, val: v } => {

                if mdentry_buyer.is_some() { break; }

                mdentry_buyer = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_MDENTRYSELLER, val: v } => {

                if mdentry_seller.is_some() { break; }

                mdentry_seller = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_NUMBEROFORDERS, val: v } => {

                if number_of_orders.is_some() { break; }

                number_of_orders = Some( i32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_MDENTRYPOSITIONNO, val: v } => {

                if mdentry_position_no.is_some() { break; }

                mdentry_position_no = Some( i32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_TOTALVOLUMETRADED, val: v } => {

                if total_volume_traded.is_some() { break; }

                total_volume_traded = Some( f32::from_str(v).unwrap() );
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


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoMDEntries11Fields {
        mdupdate_action: mdupdate_action.unwrap() ,
        delete_reason: delete_reason,
        mdentry_type: mdentry_type,
        mdentry_id: mdentry_id,
        mdentry_ref_id: mdentry_ref_id,
        symbol: symbol,
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
        financial_status: financial_status,
        corporate_action: corporate_action,
        mdentry_px: mdentry_px,
        currency: currency,
        mdentry_size: mdentry_size,
        mdentry_date: mdentry_date,
        mdentry_time: mdentry_time,
        tick_direction: tick_direction,
        mdmkt: mdmkt,
        trading_session_id: trading_session_id,
        quote_condition: quote_condition,
        trade_condition: trade_condition,
        mdentry_originator: mdentry_originator,
        location_id: location_id,
        desk_id: desk_id,
        open_close_settle_flag: open_close_settle_flag,
        time_in_force: time_in_force,
        expire_date: expire_date,
        expire_time: expire_time,
        min_qty: min_qty,
        exec_inst: exec_inst,
        seller_days: seller_days,
        order_id: order_id,
        quote_entry_id: quote_entry_id,
        mdentry_buyer: mdentry_buyer,
        mdentry_seller: mdentry_seller,
        number_of_orders: number_of_orders,
        mdentry_position_no: mdentry_position_no,
        total_volume_traded: total_volume_traded,
        text: text,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
    }
}


fn build_group_no_bid_components4_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoBidComponents4Fields> {
    let mut items : Vec<NoBidComponents4Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_bid_components4_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_bid_components4_fields_line(consumer: &mut FixConsumer) -> NoBidComponents4Fields {
    // fields
    let mut list_id : Option<String> = None;
    let mut side : Option<Field_Side_Enum> = None;
    let mut trading_session_id : Option<String> = None;
    let mut net_gross_ind : Option<Field_NetGrossInd_Enum> = None;
    let mut settlmnt_typ : Option<Field_SettlmntTyp_Enum> = None;
    let mut fut_sett_date : Option<UtcDateTime> = None;
    let mut account : Option<String> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_LISTID, val: v } => {

                if list_id.is_some() { break; }

                list_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_SIDE, val: v } => {

                if side.is_some() { break; }

                side = Some( Field_Side_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_TRADINGSESSIONID, val: v } => {

                if trading_session_id.is_some() { break; }

                trading_session_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_NETGROSSIND, val: v } => {

                if net_gross_ind.is_some() { break; }

                net_gross_ind = Some( Field_NetGrossInd_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SETTLMNTTYP, val: v } => {

                if settlmnt_typ.is_some() { break; }

                settlmnt_typ = Some( Field_SettlmntTyp_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_FUTSETTDATE, val: v } => {

                if fut_sett_date.is_some() { break; }

                fut_sett_date = Some( UtcDateTime::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ACCOUNT, val: v } => {

                if account.is_some() { break; }

                account = Some( v.to_string() );
            },


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoBidComponents4Fields {
        list_id: list_id,
        side: side,
        trading_session_id: trading_session_id,
        net_gross_ind: net_gross_ind,
        settlmnt_typ: settlmnt_typ,
        fut_sett_date: fut_sett_date,
        account: account,
    }
}


fn build_group_no_execs8_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoExecs8Fields> {
    let mut items : Vec<NoExecs8Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_execs8_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_execs8_fields_line(consumer: &mut FixConsumer) -> NoExecs8Fields {
    // fields
    let mut last_shares : Option<f32> = None;
    let mut exec_id : Option<String> = None;
    let mut last_px : Option<f32> = None;
    let mut last_capacity : Option<Field_LastCapacity_Enum> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_LASTSHARES, val: v } => {

                if last_shares.is_some() { break; }

                last_shares = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_EXECID, val: v } => {

                if exec_id.is_some() { break; }

                exec_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_LASTPX, val: v } => {

                if last_px.is_some() { break; }

                last_px = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_LASTCAPACITY, val: v } => {

                if last_capacity.is_some() { break; }

                last_capacity = Some( Field_LastCapacity_Enum::from_str(v).unwrap() );
            },


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoExecs8Fields {
        last_shares: last_shares,
        exec_id: exec_id,
        last_px: last_px,
        last_capacity: last_capacity,
    }
}


fn build_group_no_contra_brokers7_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoContraBrokers7Fields> {
    let mut items : Vec<NoContraBrokers7Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_contra_brokers7_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_contra_brokers7_fields_line(consumer: &mut FixConsumer) -> NoContraBrokers7Fields {
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
    NoContraBrokers7Fields {
        contra_broker: contra_broker,
        contra_trader: contra_trader,
        contra_trade_qty: contra_trade_qty,
        contra_trade_time: contra_trade_time,
    }
}


fn build_group_no_quote_entries19_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoQuoteEntries19Fields> {
    let mut items : Vec<NoQuoteEntries19Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_quote_entries19_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_quote_entries19_fields_line(consumer: &mut FixConsumer) -> NoQuoteEntries19Fields {
    // fields
    let mut quote_entry_id : Option<String> = None;
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<Field_IDSource_Enum> = None;
    let mut security_type : Option<Field_SecurityType_Enum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<Field_PutOrCall_Enum> = None;
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
    let mut quote_entry_reject_reason : Option<Field_QuoteEntryRejectReason_Enum> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_QUOTEENTRYID, val: v } => {

                if quote_entry_id.is_some() { break; }

                quote_entry_id = Some( v.to_string() );
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

                idsource = Some( Field_IDSource_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                if security_type.is_some() { break; }

                security_type = Some( Field_SecurityType_Enum::from_str(v).unwrap() );
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

                put_or_call = Some( Field_PutOrCall_Enum::from_str(v).unwrap() );
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

            &FieldVal { id: FIELD_QUOTEENTRYREJECTREASON, val: v } => {

                if quote_entry_reject_reason.is_some() { break; }

                quote_entry_reject_reason = Some( Field_QuoteEntryRejectReason_Enum::from_str(v).unwrap() );
            },


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoQuoteEntries19Fields {
        quote_entry_id: quote_entry_id,
        symbol: symbol,
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
        quote_entry_reject_reason: quote_entry_reject_reason,
    }
}


fn build_group_no_trading_sessions29_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoTradingSessions29Fields> {
    let mut items : Vec<NoTradingSessions29Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_trading_sessions29_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_trading_sessions29_fields_line(consumer: &mut FixConsumer) -> NoTradingSessions29Fields {
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
    NoTradingSessions29Fields {
        trading_session_id: trading_session_id,
    }
}


fn build_group_no_ioiqualifiers9_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoIOIQualifiers9Fields> {
    let mut items : Vec<NoIOIQualifiers9Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_ioiqualifiers9_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_ioiqualifiers9_fields_line(consumer: &mut FixConsumer) -> NoIOIQualifiers9Fields {
    // fields
    let mut ioiqualifier : Option<Field_IOIQualifier_Enum> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_IOIQUALIFIER, val: v } => {

                if ioiqualifier.is_some() { break; }

                ioiqualifier = Some( Field_IOIQualifier_Enum::from_str(v).unwrap() );
            },


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoIOIQualifiers9Fields {
        ioiqualifier: ioiqualifier,
    }
}


fn build_group_no_msg_types14_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoMsgTypes14Fields> {
    let mut items : Vec<NoMsgTypes14Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_msg_types14_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_msg_types14_fields_line(consumer: &mut FixConsumer) -> NoMsgTypes14Fields {
    // fields
    let mut ref_msg_type : Option<String> = None;
    let mut msg_direction : Option<Field_MsgDirection_Enum> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_REFMSGTYPE, val: v } => {

                if ref_msg_type.is_some() { break; }

                ref_msg_type = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_MSGDIRECTION, val: v } => {

                if msg_direction.is_some() { break; }

                msg_direction = Some( Field_MsgDirection_Enum::from_str(v).unwrap() );
            },


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoMsgTypes14Fields {
        ref_msg_type: ref_msg_type,
        msg_direction: msg_direction,
    }
}


fn build_group_lines_of_text1_fields(consumer: &mut FixConsumer, size: usize) -> Vec<LinesOfText1Fields> {
    let mut items : Vec<LinesOfText1Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_lines_of_text1_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_lines_of_text1_fields_line(consumer: &mut FixConsumer) -> LinesOfText1Fields {
    // fields
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
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


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    LinesOfText1Fields {
        text: text.unwrap() ,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
    }
}


fn build_group_no_allocs2_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoAllocs2Fields> {
    let mut items : Vec<NoAllocs2Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_allocs2_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_allocs2_fields_line(consumer: &mut FixConsumer) -> NoAllocs2Fields {
    // fields
    let mut alloc_account : Option<String> = None;
    let mut alloc_price : Option<f32> = None;
    let mut alloc_shares : Option<f32> = None;
    let mut process_code : Option<Field_ProcessCode_Enum> = None;
    let mut broker_of_credit : Option<String> = None;
    let mut notify_broker_of_credit : Option<bool> = None;
    let mut alloc_handl_inst : Option<Field_AllocHandlInst_Enum> = None;
    let mut alloc_text : Option<String> = None;
    let mut encoded_alloc_text_len : Option<usize> = None;
    let mut encoded_alloc_text : Option<String> = None;
    let mut exec_broker : Option<String> = None;
    let mut client_id : Option<String> = None;
    let mut commission : Option<f32> = None;
    let mut comm_type : Option<Field_CommType_Enum> = None;
    let mut alloc_avg_px : Option<f32> = None;
    let mut alloc_net_money : Option<f32> = None;
    let mut settl_curr_amt : Option<f32> = None;
    let mut settl_currency : Option<f32> = None;
    let mut settl_curr_fx_rate : Option<f32> = None;
    let mut settl_curr_fx_rate_calc : Option<Field_SettlCurrFxRateCalc_Enum> = None;
    let mut accrued_interest_amt : Option<f32> = None;
    let mut settl_inst_mode : Option<Field_SettlInstMode_Enum> = None;
    let mut no_misc_fees : Option<Vec<NoMiscFees13Fields>> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_ALLOCACCOUNT, val: v } => {

                if alloc_account.is_some() { break; }

                alloc_account = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_ALLOCPRICE, val: v } => {

                if alloc_price.is_some() { break; }

                alloc_price = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ALLOCSHARES, val: v } => {

                if alloc_shares.is_some() { break; }

                alloc_shares = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_PROCESSCODE, val: v } => {

                if process_code.is_some() { break; }

                process_code = Some( Field_ProcessCode_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_BROKEROFCREDIT, val: v } => {

                if broker_of_credit.is_some() { break; }

                broker_of_credit = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_NOTIFYBROKEROFCREDIT, val: v } => {

                if notify_broker_of_credit.is_some() { break; }

                notify_broker_of_credit = Some( boolconv(v) );
            },

            &FieldVal { id: FIELD_ALLOCHANDLINST, val: v } => {

                if alloc_handl_inst.is_some() { break; }

                alloc_handl_inst = Some( Field_AllocHandlInst_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ALLOCTEXT, val: v } => {

                if alloc_text.is_some() { break; }

                alloc_text = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_ENCODEDALLOCTEXTLEN, val: v } => {

                if encoded_alloc_text_len.is_some() { break; }

                encoded_alloc_text_len = Some( usize::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ENCODEDALLOCTEXT, val: v } => {

                if encoded_alloc_text.is_some() { break; }

                encoded_alloc_text = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_EXECBROKER, val: v } => {

                if exec_broker.is_some() { break; }

                exec_broker = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_CLIENTID, val: v } => {

                if client_id.is_some() { break; }

                client_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_COMMISSION, val: v } => {

                if commission.is_some() { break; }

                commission = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_COMMTYPE, val: v } => {

                if comm_type.is_some() { break; }

                comm_type = Some( Field_CommType_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ALLOCAVGPX, val: v } => {

                if alloc_avg_px.is_some() { break; }

                alloc_avg_px = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ALLOCNETMONEY, val: v } => {

                if alloc_net_money.is_some() { break; }

                alloc_net_money = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SETTLCURRAMT, val: v } => {

                if settl_curr_amt.is_some() { break; }

                settl_curr_amt = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SETTLCURRENCY, val: v } => {

                if settl_currency.is_some() { break; }

                settl_currency = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SETTLCURRFXRATE, val: v } => {

                if settl_curr_fx_rate.is_some() { break; }

                settl_curr_fx_rate = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SETTLCURRFXRATECALC, val: v } => {

                if settl_curr_fx_rate_calc.is_some() { break; }

                settl_curr_fx_rate_calc = Some( Field_SettlCurrFxRateCalc_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ACCRUEDINTERESTAMT, val: v } => {

                if accrued_interest_amt.is_some() { break; }

                accrued_interest_amt = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SETTLINSTMODE, val: v } => {

                if settl_inst_mode.is_some() { break; }

                settl_inst_mode = Some( Field_SettlInstMode_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_NOMISCFEES, val: v } => {

                if no_misc_fees.is_some() { break; }

                let size = usize::from_str(v).unwrap();
                let items = build_group_no_misc_fees13_fields(consumer, size);
                no_misc_fees = Some(items);
                continue;
            },


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoAllocs2Fields {
        alloc_account: alloc_account,
        alloc_price: alloc_price,
        alloc_shares: alloc_shares.unwrap() ,
        process_code: process_code,
        broker_of_credit: broker_of_credit,
        notify_broker_of_credit: notify_broker_of_credit,
        alloc_handl_inst: alloc_handl_inst,
        alloc_text: alloc_text,
        encoded_alloc_text_len: encoded_alloc_text_len,
        encoded_alloc_text: encoded_alloc_text,
        exec_broker: exec_broker,
        client_id: client_id,
        commission: commission,
        comm_type: comm_type,
        alloc_avg_px: alloc_avg_px,
        alloc_net_money: alloc_net_money,
        settl_curr_amt: settl_curr_amt,
        settl_currency: settl_currency,
        settl_curr_fx_rate: settl_curr_fx_rate,
        settl_curr_fx_rate_calc: settl_curr_fx_rate_calc,
        accrued_interest_amt: accrued_interest_amt,
        settl_inst_mode: settl_inst_mode,
        no_misc_fees: no_misc_fees,
    }
}


fn build_group_no_orders17_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoOrders17Fields> {
    let mut items : Vec<NoOrders17Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_orders17_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_orders17_fields_line(consumer: &mut FixConsumer) -> NoOrders17Fields {
    // fields
    let mut cl_ord_id : Option<String> = None;
    let mut cum_qty : Option<f32> = None;
    let mut ord_status : Option<Field_OrdStatus_Enum> = None;
    let mut leaves_qty : Option<f32> = None;
    let mut cxl_qty : Option<f32> = None;
    let mut avg_px : Option<f32> = None;
    let mut ord_rej_reason : Option<Field_OrdRejReason_Enum> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_CLORDID, val: v } => {

                if cl_ord_id.is_some() { break; }

                cl_ord_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_CUMQTY, val: v } => {

                if cum_qty.is_some() { break; }

                cum_qty = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ORDSTATUS, val: v } => {

                if ord_status.is_some() { break; }

                ord_status = Some( Field_OrdStatus_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_LEAVESQTY, val: v } => {

                if leaves_qty.is_some() { break; }

                leaves_qty = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_CXLQTY, val: v } => {

                if cxl_qty.is_some() { break; }

                cxl_qty = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_AVGPX, val: v } => {

                if avg_px.is_some() { break; }

                avg_px = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ORDREJREASON, val: v } => {

                if ord_rej_reason.is_some() { break; }

                ord_rej_reason = Some( Field_OrdRejReason_Enum::from_str(v).unwrap() );
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


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoOrders17Fields {
        cl_ord_id: cl_ord_id.unwrap() ,
        cum_qty: cum_qty.unwrap() ,
        ord_status: ord_status.unwrap() ,
        leaves_qty: leaves_qty.unwrap() ,
        cxl_qty: cxl_qty.unwrap() ,
        avg_px: avg_px.unwrap() ,
        ord_rej_reason: ord_rej_reason,
        text: text,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
    }
}


fn build_group_no_strikes28_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoStrikes28Fields> {
    let mut items : Vec<NoStrikes28Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_strikes28_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_strikes28_fields_line(consumer: &mut FixConsumer) -> NoStrikes28Fields {
    // fields
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<Field_IDSource_Enum> = None;
    let mut security_type : Option<Field_SecurityType_Enum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<Field_PutOrCall_Enum> = None;
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
    let mut cl_ord_id : Option<String> = None;
    let mut side : Option<Field_Side_Enum> = None;
    let mut price : Option<f32> = None;
    let mut currency : Option<f32> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
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

                idsource = Some( Field_IDSource_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                if security_type.is_some() { break; }

                security_type = Some( Field_SecurityType_Enum::from_str(v).unwrap() );
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

                put_or_call = Some( Field_PutOrCall_Enum::from_str(v).unwrap() );
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

            &FieldVal { id: FIELD_CLORDID, val: v } => {

                if cl_ord_id.is_some() { break; }

                cl_ord_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_SIDE, val: v } => {

                if side.is_some() { break; }

                side = Some( Field_Side_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_PRICE, val: v } => {

                if price.is_some() { break; }

                price = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_CURRENCY, val: v } => {

                if currency.is_some() { break; }

                currency = Some( f32::from_str(v).unwrap() );
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


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoStrikes28Fields {
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
        cl_ord_id: cl_ord_id,
        side: side,
        price: price.unwrap() ,
        currency: currency,
        text: text,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
    }
}


fn build_group_no_orders16_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoOrders16Fields> {
    let mut items : Vec<NoOrders16Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_orders16_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_orders16_fields_line(consumer: &mut FixConsumer) -> NoOrders16Fields {
    // fields
    let mut cl_ord_id : Option<String> = None;
    let mut order_id : Option<String> = None;
    let mut secondary_order_id : Option<String> = None;
    let mut list_id : Option<String> = None;
    let mut wave_no : Option<String> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_CLORDID, val: v } => {

                if cl_ord_id.is_some() { break; }

                cl_ord_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_ORDERID, val: v } => {

                if order_id.is_some() { break; }

                order_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_SECONDARYORDERID, val: v } => {

                if secondary_order_id.is_some() { break; }

                secondary_order_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_LISTID, val: v } => {

                if list_id.is_some() { break; }

                list_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_WAVENO, val: v } => {

                if wave_no.is_some() { break; }

                wave_no = Some( v.to_string() );
            },


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoOrders16Fields {
        cl_ord_id: cl_ord_id,
        order_id: order_id,
        secondary_order_id: secondary_order_id,
        list_id: list_id,
        wave_no: wave_no,
    }
}


fn build_group_no_related_sym25_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoRelatedSym25Fields> {
    let mut items : Vec<NoRelatedSym25Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_related_sym25_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_related_sym25_fields_line(consumer: &mut FixConsumer) -> NoRelatedSym25Fields {
    // fields
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<Field_IDSource_Enum> = None;
    let mut security_type : Option<Field_SecurityType_Enum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<Field_PutOrCall_Enum> = None;
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
    let mut quote_request_type : Option<Field_QuoteRequestType_Enum> = None;
    let mut trading_session_id : Option<String> = None;
    let mut side : Option<Field_Side_Enum> = None;
    let mut order_qty : Option<f32> = None;
    let mut fut_sett_date : Option<UtcDateTime> = None;
    let mut ord_type : Option<Field_OrdType_Enum> = None;
    let mut fut_sett_date2 : Option<UtcDateTime> = None;
    let mut order_qty2 : Option<f32> = None;
    let mut expire_time : Option<UtcDateTime> = None;
    let mut transact_time : Option<UtcDateTime> = None;
    let mut currency : Option<f32> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
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

                idsource = Some( Field_IDSource_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                if security_type.is_some() { break; }

                security_type = Some( Field_SecurityType_Enum::from_str(v).unwrap() );
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

                put_or_call = Some( Field_PutOrCall_Enum::from_str(v).unwrap() );
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

            &FieldVal { id: FIELD_QUOTEREQUESTTYPE, val: v } => {

                if quote_request_type.is_some() { break; }

                quote_request_type = Some( Field_QuoteRequestType_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_TRADINGSESSIONID, val: v } => {

                if trading_session_id.is_some() { break; }

                trading_session_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_SIDE, val: v } => {

                if side.is_some() { break; }

                side = Some( Field_Side_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ORDERQTY, val: v } => {

                if order_qty.is_some() { break; }

                order_qty = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_FUTSETTDATE, val: v } => {

                if fut_sett_date.is_some() { break; }

                fut_sett_date = Some( UtcDateTime::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ORDTYPE, val: v } => {

                if ord_type.is_some() { break; }

                ord_type = Some( Field_OrdType_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_FUTSETTDATE2, val: v } => {

                if fut_sett_date2.is_some() { break; }

                fut_sett_date2 = Some( UtcDateTime::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ORDERQTY2, val: v } => {

                if order_qty2.is_some() { break; }

                order_qty2 = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_EXPIRETIME, val: v } => {

                if expire_time.is_some() { break; }

                expire_time = Some( UtcDateTime::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_TRANSACTTIME, val: v } => {

                if transact_time.is_some() { break; }

                transact_time = Some( UtcDateTime::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_CURRENCY, val: v } => {

                if currency.is_some() { break; }

                currency = Some( f32::from_str(v).unwrap() );
            },


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoRelatedSym25Fields {
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
        quote_request_type: quote_request_type,
        trading_session_id: trading_session_id,
        side: side,
        order_qty: order_qty,
        fut_sett_date: fut_sett_date,
        ord_type: ord_type,
        fut_sett_date2: fut_sett_date2,
        order_qty2: order_qty2,
        expire_time: expire_time,
        transact_time: transact_time,
        currency: currency,
    }
}


fn build_group_no_quote_sets22_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoQuoteSets22Fields> {
    let mut items : Vec<NoQuoteSets22Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_quote_sets22_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_quote_sets22_fields_line(consumer: &mut FixConsumer) -> NoQuoteSets22Fields {
    // fields
    let mut quote_set_id : Option<String> = None;
    let mut underlying_symbol : Option<String> = None;
    let mut underlying_symbol_sfx : Option<String> = None;
    let mut underlying_security_id : Option<String> = None;
    let mut underlying_idsource : Option<String> = None;
    let mut underlying_security_type : Option<String> = None;
    let mut underlying_maturity_month_year : Option<UtcDate> = None;
    let mut underlying_maturity_day : Option<i32> = None;
    let mut underlying_put_or_call : Option<i32> = None;
    let mut underlying_strike_price : Option<f32> = None;
    let mut underlying_opt_attribute : Option<char> = None;
    let mut underlying_contract_multiplier : Option<f32> = None;
    let mut underlying_coupon_rate : Option<f32> = None;
    let mut underlying_security_exchange : Option<String> = None;
    let mut underlying_issuer : Option<String> = None;
    let mut encoded_underlying_issuer_len : Option<usize> = None;
    let mut encoded_underlying_issuer : Option<String> = None;
    let mut underlying_security_desc : Option<String> = None;
    let mut encoded_underlying_security_desc_len : Option<usize> = None;
    let mut encoded_underlying_security_desc : Option<String> = None;
    let mut quote_set_valid_until_time : Option<UtcDateTime> = None;
    let mut tot_quote_entries : Option<i32> = None;
    let mut no_quote_entries : Option<Vec<NoQuoteEntries20Fields>> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_QUOTESETID, val: v } => {

                if quote_set_id.is_some() { break; }

                quote_set_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_UNDERLYINGSYMBOL, val: v } => {

                if underlying_symbol.is_some() { break; }

                underlying_symbol = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_UNDERLYINGSYMBOLSFX, val: v } => {

                if underlying_symbol_sfx.is_some() { break; }

                underlying_symbol_sfx = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_UNDERLYINGSECURITYID, val: v } => {

                if underlying_security_id.is_some() { break; }

                underlying_security_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_UNDERLYINGIDSOURCE, val: v } => {

                if underlying_idsource.is_some() { break; }

                underlying_idsource = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_UNDERLYINGSECURITYTYPE, val: v } => {

                if underlying_security_type.is_some() { break; }

                underlying_security_type = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_UNDERLYINGMATURITYMONTHYEAR, val: v } => {

                if underlying_maturity_month_year.is_some() { break; }

                underlying_maturity_month_year = Some( UtcDate::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_UNDERLYINGMATURITYDAY, val: v } => {

                if underlying_maturity_day.is_some() { break; }

                underlying_maturity_day = Some( i32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_UNDERLYINGPUTORCALL, val: v } => {

                if underlying_put_or_call.is_some() { break; }

                underlying_put_or_call = Some( i32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_UNDERLYINGSTRIKEPRICE, val: v } => {

                if underlying_strike_price.is_some() { break; }

                underlying_strike_price = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_UNDERLYINGOPTATTRIBUTE, val: v } => {

                if underlying_opt_attribute.is_some() { break; }

                underlying_opt_attribute = Some( char::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_UNDERLYINGCONTRACTMULTIPLIER, val: v } => {

                if underlying_contract_multiplier.is_some() { break; }

                underlying_contract_multiplier = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_UNDERLYINGCOUPONRATE, val: v } => {

                if underlying_coupon_rate.is_some() { break; }

                underlying_coupon_rate = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_UNDERLYINGSECURITYEXCHANGE, val: v } => {

                if underlying_security_exchange.is_some() { break; }

                underlying_security_exchange = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_UNDERLYINGISSUER, val: v } => {

                if underlying_issuer.is_some() { break; }

                underlying_issuer = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_ENCODEDUNDERLYINGISSUERLEN, val: v } => {

                if encoded_underlying_issuer_len.is_some() { break; }

                encoded_underlying_issuer_len = Some( usize::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ENCODEDUNDERLYINGISSUER, val: v } => {

                if encoded_underlying_issuer.is_some() { break; }

                encoded_underlying_issuer = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_UNDERLYINGSECURITYDESC, val: v } => {

                if underlying_security_desc.is_some() { break; }

                underlying_security_desc = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_ENCODEDUNDERLYINGSECURITYDESCLEN, val: v } => {

                if encoded_underlying_security_desc_len.is_some() { break; }

                encoded_underlying_security_desc_len = Some( usize::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ENCODEDUNDERLYINGSECURITYDESC, val: v } => {

                if encoded_underlying_security_desc.is_some() { break; }

                encoded_underlying_security_desc = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_QUOTESETVALIDUNTILTIME, val: v } => {

                if quote_set_valid_until_time.is_some() { break; }

                quote_set_valid_until_time = Some( UtcDateTime::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_TOTQUOTEENTRIES, val: v } => {

                if tot_quote_entries.is_some() { break; }

                tot_quote_entries = Some( i32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_NOQUOTEENTRIES, val: v } => {

                if no_quote_entries.is_some() { break; }

                let size = usize::from_str(v).unwrap();
                let items = build_group_no_quote_entries20_fields(consumer, size);
                no_quote_entries = Some(items);
                continue;
            },


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoQuoteSets22Fields {
        quote_set_id: quote_set_id.unwrap() ,
        underlying_symbol: underlying_symbol.unwrap() ,
        underlying_symbol_sfx: underlying_symbol_sfx,
        underlying_security_id: underlying_security_id,
        underlying_idsource: underlying_idsource,
        underlying_security_type: underlying_security_type,
        underlying_maturity_month_year: underlying_maturity_month_year,
        underlying_maturity_day: underlying_maturity_day,
        underlying_put_or_call: underlying_put_or_call,
        underlying_strike_price: underlying_strike_price,
        underlying_opt_attribute: underlying_opt_attribute,
        underlying_contract_multiplier: underlying_contract_multiplier,
        underlying_coupon_rate: underlying_coupon_rate,
        underlying_security_exchange: underlying_security_exchange,
        underlying_issuer: underlying_issuer,
        encoded_underlying_issuer_len: encoded_underlying_issuer_len,
        encoded_underlying_issuer: encoded_underlying_issuer,
        underlying_security_desc: underlying_security_desc,
        encoded_underlying_security_desc_len: encoded_underlying_security_desc_len,
        encoded_underlying_security_desc: encoded_underlying_security_desc,
        quote_set_valid_until_time: quote_set_valid_until_time,
        tot_quote_entries: tot_quote_entries.unwrap() ,
        no_quote_entries: no_quote_entries.unwrap() ,
    }
}


fn build_group_no_related_sym24_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoRelatedSym24Fields> {
    let mut items : Vec<NoRelatedSym24Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_related_sym24_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_related_sym24_fields_line(consumer: &mut FixConsumer) -> NoRelatedSym24Fields {
    // fields
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<Field_IDSource_Enum> = None;
    let mut security_type : Option<Field_SecurityType_Enum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<Field_PutOrCall_Enum> = None;
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
    let mut trading_session_id : Option<String> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
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

                idsource = Some( Field_IDSource_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                if security_type.is_some() { break; }

                security_type = Some( Field_SecurityType_Enum::from_str(v).unwrap() );
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

                put_or_call = Some( Field_PutOrCall_Enum::from_str(v).unwrap() );
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
    NoRelatedSym24Fields {
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
        trading_session_id: trading_session_id,
    }
}


fn build_group_no_bid_components5_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoBidComponents5Fields> {
    let mut items : Vec<NoBidComponents5Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_bid_components5_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_bid_components5_fields_line(consumer: &mut FixConsumer) -> NoBidComponents5Fields {
    // fields
    let mut commission : Option<f32> = None;
    let mut comm_type : Option<Field_CommType_Enum> = None;
    let mut list_id : Option<String> = None;
    let mut country : Option<String> = None;
    let mut side : Option<Field_Side_Enum> = None;
    let mut price : Option<f32> = None;
    let mut price_type : Option<Field_PriceType_Enum> = None;
    let mut fair_value : Option<f32> = None;
    let mut net_gross_ind : Option<Field_NetGrossInd_Enum> = None;
    let mut settlmnt_typ : Option<Field_SettlmntTyp_Enum> = None;
    let mut fut_sett_date : Option<UtcDateTime> = None;
    let mut trading_session_id : Option<String> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_COMMISSION, val: v } => {

                if commission.is_some() { break; }

                commission = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_COMMTYPE, val: v } => {

                if comm_type.is_some() { break; }

                comm_type = Some( Field_CommType_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_LISTID, val: v } => {

                if list_id.is_some() { break; }

                list_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_COUNTRY, val: v } => {

                if country.is_some() { break; }

                country = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_SIDE, val: v } => {

                if side.is_some() { break; }

                side = Some( Field_Side_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_PRICE, val: v } => {

                if price.is_some() { break; }

                price = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_PRICETYPE, val: v } => {

                if price_type.is_some() { break; }

                price_type = Some( Field_PriceType_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_FAIRVALUE, val: v } => {

                if fair_value.is_some() { break; }

                fair_value = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_NETGROSSIND, val: v } => {

                if net_gross_ind.is_some() { break; }

                net_gross_ind = Some( Field_NetGrossInd_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SETTLMNTTYP, val: v } => {

                if settlmnt_typ.is_some() { break; }

                settlmnt_typ = Some( Field_SettlmntTyp_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_FUTSETTDATE, val: v } => {

                if fut_sett_date.is_some() { break; }

                fut_sett_date = Some( UtcDateTime::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_TRADINGSESSIONID, val: v } => {

                if trading_session_id.is_some() { break; }

                trading_session_id = Some( v.to_string() );
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


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoBidComponents5Fields {
        commission: commission.unwrap() ,
        comm_type: comm_type.unwrap() ,
        list_id: list_id,
        country: country,
        side: side,
        price: price,
        price_type: price_type,
        fair_value: fair_value,
        net_gross_ind: net_gross_ind,
        settlmnt_typ: settlmnt_typ,
        fut_sett_date: fut_sett_date,
        trading_session_id: trading_session_id,
        text: text,
        encoded_text_len: encoded_text_len,
        encoded_text: encoded_text,
    }
}


fn build_group_no_orders15_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoOrders15Fields> {
    let mut items : Vec<NoOrders15Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_orders15_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_orders15_fields_line(consumer: &mut FixConsumer) -> NoOrders15Fields {
    // fields
    let mut cl_ord_id : Option<String> = None;
    let mut list_seq_no : Option<i32> = None;
    let mut settl_inst_mode : Option<Field_SettlInstMode_Enum> = None;
    let mut client_id : Option<String> = None;
    let mut exec_broker : Option<String> = None;
    let mut account : Option<String> = None;
    let mut no_allocs : Option<Vec<NoAllocs3Fields>> = None;
    let mut settlmnt_typ : Option<Field_SettlmntTyp_Enum> = None;
    let mut fut_sett_date : Option<UtcDateTime> = None;
    let mut handl_inst : Option<Field_HandlInst_Enum> = None;
    let mut exec_inst : Option<Field_ExecInst_Enum> = None;
    let mut min_qty : Option<f32> = None;
    let mut max_floor : Option<f32> = None;
    let mut ex_destination : Option<String> = None;
    let mut no_trading_sessions : Option<Vec<NoTradingSessions29Fields>> = None;
    let mut process_code : Option<Field_ProcessCode_Enum> = None;
    let mut symbol : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<Field_IDSource_Enum> = None;
    let mut security_type : Option<Field_SecurityType_Enum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<Field_PutOrCall_Enum> = None;
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
    let mut side : Option<Field_Side_Enum> = None;
    let mut side_value_ind : Option<i32> = None;
    let mut locate_reqd : Option<bool> = None;
    let mut transact_time : Option<UtcDateTime> = None;
    let mut order_qty : Option<f32> = None;
    let mut cash_order_qty : Option<f32> = None;
    let mut ord_type : Option<Field_OrdType_Enum> = None;
    let mut price : Option<f32> = None;
    let mut stop_px : Option<f32> = None;
    let mut currency : Option<f32> = None;
    let mut compliance_id : Option<String> = None;
    let mut solicited_flag : Option<bool> = None;
    let mut ioiid : Option<String> = None;
    let mut quote_id : Option<String> = None;
    let mut time_in_force : Option<Field_TimeInForce_Enum> = None;
    let mut effective_time : Option<UtcDateTime> = None;
    let mut expire_date : Option<UtcDateTime> = None;
    let mut expire_time : Option<UtcDateTime> = None;
    let mut gtbooking_inst : Option<Field_GTBookingInst_Enum> = None;
    let mut commission : Option<f32> = None;
    let mut comm_type : Option<Field_CommType_Enum> = None;
    let mut rule80_a : Option<Field_Rule80A_Enum> = None;
    let mut forex_req : Option<bool> = None;
    let mut settl_currency : Option<f32> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;
    let mut fut_sett_date2 : Option<UtcDateTime> = None;
    let mut order_qty2 : Option<f32> = None;
    let mut open_close : Option<Field_OpenClose_Enum> = None;
    let mut covered_or_uncovered : Option<Field_CoveredOrUncovered_Enum> = None;
    let mut customer_or_firm : Option<Field_CustomerOrFirm_Enum> = None;
    let mut max_show : Option<f32> = None;
    let mut peg_difference : Option<f32> = None;
    let mut discretion_inst : Option<Field_DiscretionInst_Enum> = None;
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

                settl_inst_mode = Some( Field_SettlInstMode_Enum::from_str(v).unwrap() );
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
                let items = build_group_no_allocs3_fields(consumer, size);
                no_allocs = Some(items);
                continue;
            },

            &FieldVal { id: FIELD_SETTLMNTTYP, val: v } => {

                if settlmnt_typ.is_some() { break; }

                settlmnt_typ = Some( Field_SettlmntTyp_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_FUTSETTDATE, val: v } => {

                if fut_sett_date.is_some() { break; }

                fut_sett_date = Some( UtcDateTime::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_HANDLINST, val: v } => {

                if handl_inst.is_some() { break; }

                handl_inst = Some( Field_HandlInst_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_EXECINST, val: v } => {

                if exec_inst.is_some() { break; }

                exec_inst = Some( Field_ExecInst_Enum::from_str(v).unwrap() );
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
                let items = build_group_no_trading_sessions29_fields(consumer, size);
                no_trading_sessions = Some(items);
                continue;
            },

            &FieldVal { id: FIELD_PROCESSCODE, val: v } => {

                if process_code.is_some() { break; }

                process_code = Some( Field_ProcessCode_Enum::from_str(v).unwrap() );
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

                idsource = Some( Field_IDSource_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                if security_type.is_some() { break; }

                security_type = Some( Field_SecurityType_Enum::from_str(v).unwrap() );
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

                put_or_call = Some( Field_PutOrCall_Enum::from_str(v).unwrap() );
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

                side = Some( Field_Side_Enum::from_str(v).unwrap() );
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

                ord_type = Some( Field_OrdType_Enum::from_str(v).unwrap() );
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

                time_in_force = Some( Field_TimeInForce_Enum::from_str(v).unwrap() );
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

                gtbooking_inst = Some( Field_GTBookingInst_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_COMMISSION, val: v } => {

                if commission.is_some() { break; }

                commission = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_COMMTYPE, val: v } => {

                if comm_type.is_some() { break; }

                comm_type = Some( Field_CommType_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_RULE80A, val: v } => {

                if rule80_a.is_some() { break; }

                rule80_a = Some( Field_Rule80A_Enum::from_str(v).unwrap() );
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

                open_close = Some( Field_OpenClose_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_COVEREDORUNCOVERED, val: v } => {

                if covered_or_uncovered.is_some() { break; }

                covered_or_uncovered = Some( Field_CoveredOrUncovered_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_CUSTOMERORFIRM, val: v } => {

                if customer_or_firm.is_some() { break; }

                customer_or_firm = Some( Field_CustomerOrFirm_Enum::from_str(v).unwrap() );
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

                discretion_inst = Some( Field_DiscretionInst_Enum::from_str(v).unwrap() );
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
    NoOrders15Fields {
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


fn build_group_no_related_sym26_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoRelatedSym26Fields> {
    let mut items : Vec<NoRelatedSym26Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_related_sym26_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_related_sym26_fields_line(consumer: &mut FixConsumer) -> NoRelatedSym26Fields {
    // fields
    let mut relatd_sym : Option<String> = None;
    let mut symbol_sfx : Option<String> = None;
    let mut security_id : Option<String> = None;
    let mut idsource : Option<Field_IDSource_Enum> = None;
    let mut security_type : Option<Field_SecurityType_Enum> = None;
    let mut maturity_month_year : Option<UtcDate> = None;
    let mut maturity_day : Option<i32> = None;
    let mut put_or_call : Option<Field_PutOrCall_Enum> = None;
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

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_RELATDSYM, val: v } => {

                if relatd_sym.is_some() { break; }

                relatd_sym = Some( v.to_string() );
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

                idsource = Some( Field_IDSource_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                if security_type.is_some() { break; }

                security_type = Some( Field_SecurityType_Enum::from_str(v).unwrap() );
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

                put_or_call = Some( Field_PutOrCall_Enum::from_str(v).unwrap() );
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


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoRelatedSym26Fields {
        relatd_sym: relatd_sym,
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
    }
}


fn build_group_no_bid_descriptors6_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoBidDescriptors6Fields> {
    let mut items : Vec<NoBidDescriptors6Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_bid_descriptors6_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_bid_descriptors6_fields_line(consumer: &mut FixConsumer) -> NoBidDescriptors6Fields {
    // fields
    let mut bid_descriptor_type : Option<i32> = None;
    let mut bid_descriptor : Option<String> = None;
    let mut side_value_ind : Option<i32> = None;
    let mut liquidity_value : Option<f32> = None;
    let mut liquidity_num_securities : Option<i32> = None;
    let mut liquidity_pct_low : Option<f32> = None;
    let mut liquidity_pct_high : Option<f32> = None;
    let mut efptracking_error : Option<f32> = None;
    let mut fair_value : Option<f32> = None;
    let mut outside_index_pct : Option<f32> = None;
    let mut value_of_futures : Option<f32> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_BIDDESCRIPTORTYPE, val: v } => {

                if bid_descriptor_type.is_some() { break; }

                bid_descriptor_type = Some( i32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_BIDDESCRIPTOR, val: v } => {

                if bid_descriptor.is_some() { break; }

                bid_descriptor = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_SIDEVALUEIND, val: v } => {

                if side_value_ind.is_some() { break; }

                side_value_ind = Some( i32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_LIQUIDITYVALUE, val: v } => {

                if liquidity_value.is_some() { break; }

                liquidity_value = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_LIQUIDITYNUMSECURITIES, val: v } => {

                if liquidity_num_securities.is_some() { break; }

                liquidity_num_securities = Some( i32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_LIQUIDITYPCTLOW, val: v } => {

                if liquidity_pct_low.is_some() { break; }

                liquidity_pct_low = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_LIQUIDITYPCTHIGH, val: v } => {

                if liquidity_pct_high.is_some() { break; }

                liquidity_pct_high = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_EFPTRACKINGERROR, val: v } => {

                if efptracking_error.is_some() { break; }

                efptracking_error = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_FAIRVALUE, val: v } => {

                if fair_value.is_some() { break; }

                fair_value = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_OUTSIDEINDEXPCT, val: v } => {

                if outside_index_pct.is_some() { break; }

                outside_index_pct = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_VALUEOFFUTURES, val: v } => {

                if value_of_futures.is_some() { break; }

                value_of_futures = Some( f32::from_str(v).unwrap() );
            },


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoBidDescriptors6Fields {
        bid_descriptor_type: bid_descriptor_type,
        bid_descriptor: bid_descriptor,
        side_value_ind: side_value_ind,
        liquidity_value: liquidity_value,
        liquidity_num_securities: liquidity_num_securities,
        liquidity_pct_low: liquidity_pct_low,
        liquidity_pct_high: liquidity_pct_high,
        efptracking_error: efptracking_error,
        fair_value: fair_value,
        outside_index_pct: outside_index_pct,
        value_of_futures: value_of_futures,
    }
}


fn build_group_no_quote_sets21_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoQuoteSets21Fields> {
    let mut items : Vec<NoQuoteSets21Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_quote_sets21_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_quote_sets21_fields_line(consumer: &mut FixConsumer) -> NoQuoteSets21Fields {
    // fields
    let mut quote_set_id : Option<String> = None;
    let mut underlying_symbol : Option<String> = None;
    let mut underlying_symbol_sfx : Option<String> = None;
    let mut underlying_security_id : Option<String> = None;
    let mut underlying_idsource : Option<String> = None;
    let mut underlying_security_type : Option<String> = None;
    let mut underlying_maturity_month_year : Option<UtcDate> = None;
    let mut underlying_maturity_day : Option<i32> = None;
    let mut underlying_put_or_call : Option<i32> = None;
    let mut underlying_strike_price : Option<f32> = None;
    let mut underlying_opt_attribute : Option<char> = None;
    let mut underlying_contract_multiplier : Option<f32> = None;
    let mut underlying_coupon_rate : Option<f32> = None;
    let mut underlying_security_exchange : Option<String> = None;
    let mut underlying_issuer : Option<String> = None;
    let mut encoded_underlying_issuer_len : Option<usize> = None;
    let mut encoded_underlying_issuer : Option<String> = None;
    let mut underlying_security_desc : Option<String> = None;
    let mut encoded_underlying_security_desc_len : Option<usize> = None;
    let mut encoded_underlying_security_desc : Option<String> = None;
    let mut tot_quote_entries : Option<i32> = None;
    let mut no_quote_entries : Option<Vec<NoQuoteEntries19Fields>> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_QUOTESETID, val: v } => {

                if quote_set_id.is_some() { break; }

                quote_set_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_UNDERLYINGSYMBOL, val: v } => {

                if underlying_symbol.is_some() { break; }

                underlying_symbol = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_UNDERLYINGSYMBOLSFX, val: v } => {

                if underlying_symbol_sfx.is_some() { break; }

                underlying_symbol_sfx = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_UNDERLYINGSECURITYID, val: v } => {

                if underlying_security_id.is_some() { break; }

                underlying_security_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_UNDERLYINGIDSOURCE, val: v } => {

                if underlying_idsource.is_some() { break; }

                underlying_idsource = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_UNDERLYINGSECURITYTYPE, val: v } => {

                if underlying_security_type.is_some() { break; }

                underlying_security_type = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_UNDERLYINGMATURITYMONTHYEAR, val: v } => {

                if underlying_maturity_month_year.is_some() { break; }

                underlying_maturity_month_year = Some( UtcDate::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_UNDERLYINGMATURITYDAY, val: v } => {

                if underlying_maturity_day.is_some() { break; }

                underlying_maturity_day = Some( i32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_UNDERLYINGPUTORCALL, val: v } => {

                if underlying_put_or_call.is_some() { break; }

                underlying_put_or_call = Some( i32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_UNDERLYINGSTRIKEPRICE, val: v } => {

                if underlying_strike_price.is_some() { break; }

                underlying_strike_price = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_UNDERLYINGOPTATTRIBUTE, val: v } => {

                if underlying_opt_attribute.is_some() { break; }

                underlying_opt_attribute = Some( char::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_UNDERLYINGCONTRACTMULTIPLIER, val: v } => {

                if underlying_contract_multiplier.is_some() { break; }

                underlying_contract_multiplier = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_UNDERLYINGCOUPONRATE, val: v } => {

                if underlying_coupon_rate.is_some() { break; }

                underlying_coupon_rate = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_UNDERLYINGSECURITYEXCHANGE, val: v } => {

                if underlying_security_exchange.is_some() { break; }

                underlying_security_exchange = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_UNDERLYINGISSUER, val: v } => {

                if underlying_issuer.is_some() { break; }

                underlying_issuer = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_ENCODEDUNDERLYINGISSUERLEN, val: v } => {

                if encoded_underlying_issuer_len.is_some() { break; }

                encoded_underlying_issuer_len = Some( usize::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ENCODEDUNDERLYINGISSUER, val: v } => {

                if encoded_underlying_issuer.is_some() { break; }

                encoded_underlying_issuer = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_UNDERLYINGSECURITYDESC, val: v } => {

                if underlying_security_desc.is_some() { break; }

                underlying_security_desc = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_ENCODEDUNDERLYINGSECURITYDESCLEN, val: v } => {

                if encoded_underlying_security_desc_len.is_some() { break; }

                encoded_underlying_security_desc_len = Some( usize::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ENCODEDUNDERLYINGSECURITYDESC, val: v } => {

                if encoded_underlying_security_desc.is_some() { break; }

                encoded_underlying_security_desc = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_TOTQUOTEENTRIES, val: v } => {

                if tot_quote_entries.is_some() { break; }

                tot_quote_entries = Some( i32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_NOQUOTEENTRIES, val: v } => {

                if no_quote_entries.is_some() { break; }

                let size = usize::from_str(v).unwrap();
                let items = build_group_no_quote_entries19_fields(consumer, size);
                no_quote_entries = Some(items);
                continue;
            },


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoQuoteSets21Fields {
        quote_set_id: quote_set_id,
        underlying_symbol: underlying_symbol,
        underlying_symbol_sfx: underlying_symbol_sfx,
        underlying_security_id: underlying_security_id,
        underlying_idsource: underlying_idsource,
        underlying_security_type: underlying_security_type,
        underlying_maturity_month_year: underlying_maturity_month_year,
        underlying_maturity_day: underlying_maturity_day,
        underlying_put_or_call: underlying_put_or_call,
        underlying_strike_price: underlying_strike_price,
        underlying_opt_attribute: underlying_opt_attribute,
        underlying_contract_multiplier: underlying_contract_multiplier,
        underlying_coupon_rate: underlying_coupon_rate,
        underlying_security_exchange: underlying_security_exchange,
        underlying_issuer: underlying_issuer,
        encoded_underlying_issuer_len: encoded_underlying_issuer_len,
        encoded_underlying_issuer: encoded_underlying_issuer,
        underlying_security_desc: underlying_security_desc,
        encoded_underlying_security_desc_len: encoded_underlying_security_desc_len,
        encoded_underlying_security_desc: encoded_underlying_security_desc,
        tot_quote_entries: tot_quote_entries,
        no_quote_entries: no_quote_entries,
    }
}


fn build_group_no_routing_ids27_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoRoutingIDs27Fields> {
    let mut items : Vec<NoRoutingIDs27Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_routing_ids27_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_routing_ids27_fields_line(consumer: &mut FixConsumer) -> NoRoutingIDs27Fields {
    // fields
    let mut routing_type : Option<Field_RoutingType_Enum> = None;
    let mut routing_id : Option<String> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_ROUTINGTYPE, val: v } => {

                if routing_type.is_some() { break; }

                routing_type = Some( Field_RoutingType_Enum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ROUTINGID, val: v } => {

                if routing_id.is_some() { break; }

                routing_id = Some( v.to_string() );
            },


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoRoutingIDs27Fields {
        routing_type: routing_type,
        routing_id: routing_id,
    }
}


fn build_group_no_misc_fees13_fields(consumer: &mut FixConsumer, size: usize) -> Vec<NoMiscFees13Fields> {
    let mut items : Vec<NoMiscFees13Fields> = Vec::with_capacity(size);

    for _ in 0..size {
        let party = build_group_no_misc_fees13_fields_line( consumer );
        items.push(party);
    }

    items
}

fn build_group_no_misc_fees13_fields_line(consumer: &mut FixConsumer) -> NoMiscFees13Fields {
    // fields
    let mut misc_fee_amt : Option<f32> = None;
    let mut misc_fee_curr : Option<f32> = None;
    let mut misc_fee_type : Option<Field_MiscFeeType_Enum> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_MISCFEEAMT, val: v } => {

                if misc_fee_amt.is_some() { break; }

                misc_fee_amt = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_MISCFEECURR, val: v } => {

                if misc_fee_curr.is_some() { break; }

                misc_fee_curr = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_MISCFEETYPE, val: v } => {

                if misc_fee_type.is_some() { break; }

                misc_fee_type = Some( Field_MiscFeeType_Enum::from_str(v).unwrap() );
            },


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoMiscFees13Fields {
        misc_fee_amt: misc_fee_amt,
        misc_fee_curr: misc_fee_curr,
        misc_fee_type: misc_fee_type,
    }
}


use bytes::{BytesMut, BufMut};
use std::io::{Write};
use std::result::{Result};

struct WriteWrapper<'a> {
    buf: &'a mut BytesMut
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



pub fn write_fix_message(msg: &FixMessage, ts: &UtcDateTime, seq: u32, sender: &str, target: &str, sep: char, buf: &mut BytesMut) -> Result<(), io::Error> {

    match msg {

        // type: 0
        &FixMessage::Heartbeat(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=0{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_heartbeat_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: 1
        &FixMessage::TestRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=1{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_test_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: 2
        &FixMessage::ResendRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=2{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_resend_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: 3
        &FixMessage::Reject(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=3{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_reject_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: 4
        &FixMessage::SequenceReset(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=4{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_sequence_reset_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: 5
        &FixMessage::Logout(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=5{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_logout_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: 6
        &FixMessage::IOI(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=6{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_ioifields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: 7
        &FixMessage::Advertisement(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=7{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_advertisement_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: 8
        &FixMessage::ExecutionReport(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=8{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_execution_report_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: 9
        &FixMessage::OrderCancelReject(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=9{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_order_cancel_reject_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: A
        &FixMessage::Logon(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=A{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_logon_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: B
        &FixMessage::News(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=B{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_news_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: C
        &FixMessage::Email(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=C{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_email_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: D
        &FixMessage::NewOrderSingle(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=D{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_new_order_single_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: E
        &FixMessage::NewOrderList(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=E{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_new_order_list_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: F
        &FixMessage::OrderCancelRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=F{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_order_cancel_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: G
        &FixMessage::OrderCancelReplaceRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=G{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_order_cancel_replace_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: H
        &FixMessage::OrderStatusRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=H{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_order_status_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: J
        &FixMessage::Allocation(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=J{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_allocation_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: K
        &FixMessage::ListCancelRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=K{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_list_cancel_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: L
        &FixMessage::ListExecute(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=L{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_list_execute_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: M
        &FixMessage::ListStatusRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=M{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_list_status_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: N
        &FixMessage::ListStatus(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=N{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_list_status_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: P
        &FixMessage::AllocationInstructionAck(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=P{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_allocation_instruction_ack_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: Q
        &FixMessage::DontKnowTrade(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=Q{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_dont_know_trade_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: R
        &FixMessage::QuoteRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=R{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_quote_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: S
        &FixMessage::Quote(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=S{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_quote_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: T
        &FixMessage::SettlementInstructions(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=T{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_settlement_instructions_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: V
        &FixMessage::MarketDataRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=V{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_market_data_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: W
        &FixMessage::MarketDataSnapshotFullRefresh(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=W{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_market_data_snapshot_full_refresh_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: X
        &FixMessage::MarketDataIncrementalRefresh(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=X{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_market_data_incremental_refresh_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: Y
        &FixMessage::MarketDataRequestReject(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=Y{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_market_data_request_reject_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: Z
        &FixMessage::QuoteCancel(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=Z{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_quote_cancel_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: a
        &FixMessage::QuoteStatusRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=a{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_quote_status_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: b
        &FixMessage::QuoteAcknowledgement(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=b{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_quote_acknowledgement_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: c
        &FixMessage::SecurityDefinitionRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=c{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_security_definition_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: d
        &FixMessage::SecurityDefinition(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=d{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_security_definition_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: e
        &FixMessage::SecurityStatusRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=e{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_security_status_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: f
        &FixMessage::SecurityStatus(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=f{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_security_status_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: g
        &FixMessage::TradingSessionStatusRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=g{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_trading_session_status_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: h
        &FixMessage::TradingSessionStatus(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=h{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_trading_session_status_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: i
        &FixMessage::MassQuote(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=i{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_mass_quote_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: j
        &FixMessage::BusinessMessageReject(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=j{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_business_message_reject_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: k
        &FixMessage::BidRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=k{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_bid_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: l
        &FixMessage::BidResponse(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=l{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_bid_response_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: m
        &FixMessage::ListStrikePrice(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=m{sep}34={seq}{sep}49={sender}{sep}52={ts}{sep}56={target}{sep}",
                   ts= ts, seq= seq, sender= sender, target= target, sep= sep )?;
            write_list_strike_price_fields(box_flds, &mut writer_wrapper)?;
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


fn write_ioifields(flds: &IOIFields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.ioiid;

        write!(output, "23={}\u{01}", val )?; // FIELD_IOIID
    }
    {
        let val = &flds.ioitrans_type;

        write!(output, "28={}\u{01}", val )?; // FIELD_IOITRANSTYPE
    }
    if flds.ioiref_id.is_some() {
        let val = flds.ioiref_id.as_ref().unwrap();

        write!(output, "26={}\u{01}", val )?; // FIELD_IOIREFID
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
        let val = &flds.ioishares;

        write!(output, "27={}\u{01}", val )?; // FIELD_IOISHARES
    }
    if flds.price.is_some() {
        let val = flds.price.as_ref().unwrap();

        write!(output, "44={}\u{01}", val )?; // FIELD_PRICE
    }
    if flds.currency.is_some() {
        let val = flds.currency.as_ref().unwrap();

        write!(output, "15={}\u{01}", val )?; // FIELD_CURRENCY
    }
    if flds.valid_until_time.is_some() {
        let val = flds.valid_until_time.as_ref().unwrap();

        write!(output, "62={}\u{01}", val )?; // FIELD_VALIDUNTILTIME
    }
    if flds.ioiqlty_ind.is_some() {
        let val = flds.ioiqlty_ind.as_ref().unwrap();

        write!(output, "25={}\u{01}", val )?; // FIELD_IOIQLTYIND
    }
    if flds.ioinatural_flag.is_some() {
        let val = flds.ioinatural_flag.as_ref().unwrap();

        write!(output, "130={}\u{01}", to_boolconv(val) )?; // FIELD_IOINATURALFLAG
    }
    if flds.no_ioiqualifiers.is_some() {
        let val = flds.no_ioiqualifiers.as_ref().unwrap();

        write_group_no_ioiqualifiers9_fields( val, output )?;
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
    if flds.transact_time.is_some() {
        let val = flds.transact_time.as_ref().unwrap();

        write!(output, "60={}\u{01}", val )?; // FIELD_TRANSACTTIME
    }
    if flds.urllink.is_some() {
        let val = flds.urllink.as_ref().unwrap();

        write!(output, "149={}\u{01}", val )?; // FIELD_URLLINK
    }
    if flds.no_routing_ids.is_some() {
        let val = flds.no_routing_ids.as_ref().unwrap();

        write_group_no_routing_ids27_fields( val, output )?;
    }
    if flds.spread_to_benchmark.is_some() {
        let val = flds.spread_to_benchmark.as_ref().unwrap();

        write!(output, "218={}\u{01}", val )?; // FIELD_SPREADTOBENCHMARK
    }
    if flds.benchmark.is_some() {
        let val = flds.benchmark.as_ref().unwrap();

        write!(output, "219={}\u{01}", val )?; // FIELD_BENCHMARK
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

        write_group_no_contra_brokers7_fields( val, output )?;
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

        write_group_no_msg_types14_fields( val, output )?;
    }
    Ok( () )
}


fn write_news_fields(flds: &NewsFields, output: &mut Write) -> Result<(), io::Error> {

    if flds.orig_time.is_some() {
        let val = flds.orig_time.as_ref().unwrap();

        write!(output, "42={}\u{01}", val )?; // FIELD_ORIGTIME
    }
    if flds.urgency.is_some() {
        let val = flds.urgency.as_ref().unwrap();

        write!(output, "61={}\u{01}", val )?; // FIELD_URGENCY
    }
    {
        let val = &flds.headline;

        write!(output, "148={}\u{01}", val )?; // FIELD_HEADLINE
    }
    if flds.encoded_headline_len.is_some() {
        let val = flds.encoded_headline_len.as_ref().unwrap();

        write!(output, "358={}\u{01}", val )?; // FIELD_ENCODEDHEADLINELEN
    }
    if flds.encoded_headline.is_some() {
        let val = flds.encoded_headline.as_ref().unwrap();

        write!(output, "359={}\u{01}", val )?; // FIELD_ENCODEDHEADLINE
    }
    if flds.no_routing_ids.is_some() {
        let val = flds.no_routing_ids.as_ref().unwrap();

        write_group_no_routing_ids27_fields( val, output )?;
    }
    if flds.no_related_sym.is_some() {
        let val = flds.no_related_sym.as_ref().unwrap();

        write_group_no_related_sym26_fields( val, output )?;
    }
    {
        let val = &flds.lines_of_text;

        write_group_lines_of_text1_fields( val, output )?;
    }
    if flds.urllink.is_some() {
        let val = flds.urllink.as_ref().unwrap();

        write!(output, "149={}\u{01}", val )?; // FIELD_URLLINK
    }
    if flds.raw_data_length.is_some() {
        let val = flds.raw_data_length.as_ref().unwrap();

        write!(output, "95={}\u{01}", val )?; // FIELD_RAWDATALENGTH
    }
    if flds.raw_data.is_some() {
        let val = flds.raw_data.as_ref().unwrap();

        write!(output, "96={}\u{01}", val )?; // FIELD_RAWDATA
    }
    Ok( () )
}


fn write_email_fields(flds: &EmailFields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.email_thread_id;

        write!(output, "164={}\u{01}", val )?; // FIELD_EMAILTHREADID
    }
    {
        let val = &flds.email_type;

        write!(output, "94={}\u{01}", val )?; // FIELD_EMAILTYPE
    }
    if flds.orig_time.is_some() {
        let val = flds.orig_time.as_ref().unwrap();

        write!(output, "42={}\u{01}", val )?; // FIELD_ORIGTIME
    }
    {
        let val = &flds.subject;

        write!(output, "147={}\u{01}", val )?; // FIELD_SUBJECT
    }
    if flds.encoded_subject_len.is_some() {
        let val = flds.encoded_subject_len.as_ref().unwrap();

        write!(output, "356={}\u{01}", val )?; // FIELD_ENCODEDSUBJECTLEN
    }
    if flds.encoded_subject.is_some() {
        let val = flds.encoded_subject.as_ref().unwrap();

        write!(output, "357={}\u{01}", val )?; // FIELD_ENCODEDSUBJECT
    }
    if flds.no_routing_ids.is_some() {
        let val = flds.no_routing_ids.as_ref().unwrap();

        write_group_no_routing_ids27_fields( val, output )?;
    }
    if flds.no_related_sym.is_some() {
        let val = flds.no_related_sym.as_ref().unwrap();

        write_group_no_related_sym26_fields( val, output )?;
    }
    if flds.order_id.is_some() {
        let val = flds.order_id.as_ref().unwrap();

        write!(output, "37={}\u{01}", val )?; // FIELD_ORDERID
    }
    if flds.cl_ord_id.is_some() {
        let val = flds.cl_ord_id.as_ref().unwrap();

        write!(output, "11={}\u{01}", val )?; // FIELD_CLORDID
    }
    {
        let val = &flds.lines_of_text;

        write_group_lines_of_text1_fields( val, output )?;
    }
    if flds.raw_data_length.is_some() {
        let val = flds.raw_data_length.as_ref().unwrap();

        write!(output, "95={}\u{01}", val )?; // FIELD_RAWDATALENGTH
    }
    if flds.raw_data.is_some() {
        let val = flds.raw_data.as_ref().unwrap();

        write!(output, "96={}\u{01}", val )?; // FIELD_RAWDATA
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

        write_group_no_allocs3_fields( val, output )?;
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

        write_group_no_trading_sessions29_fields( val, output )?;
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

        write_group_no_orders15_fields( val, output )?;
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

        write_group_no_allocs3_fields( val, output )?;
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

        write_group_no_trading_sessions29_fields( val, output )?;
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


fn write_allocation_fields(flds: &AllocationFields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.alloc_id;

        write!(output, "70={}\u{01}", val )?; // FIELD_ALLOCID
    }
    {
        let val = &flds.alloc_trans_type;

        write!(output, "71={}\u{01}", val )?; // FIELD_ALLOCTRANSTYPE
    }
    if flds.ref_alloc_id.is_some() {
        let val = flds.ref_alloc_id.as_ref().unwrap();

        write!(output, "72={}\u{01}", val )?; // FIELD_REFALLOCID
    }
    if flds.alloc_link_id.is_some() {
        let val = flds.alloc_link_id.as_ref().unwrap();

        write!(output, "196={}\u{01}", val )?; // FIELD_ALLOCLINKID
    }
    if flds.alloc_link_type.is_some() {
        let val = flds.alloc_link_type.as_ref().unwrap();

        write!(output, "197={}\u{01}", val )?; // FIELD_ALLOCLINKTYPE
    }
    if flds.no_orders.is_some() {
        let val = flds.no_orders.as_ref().unwrap();

        write_group_no_orders16_fields( val, output )?;
    }
    if flds.no_execs.is_some() {
        let val = flds.no_execs.as_ref().unwrap();

        write_group_no_execs8_fields( val, output )?;
    }
    {
        let val = &flds.side;

        write!(output, "54={}\u{01}", val )?; // FIELD_SIDE
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
        let val = &flds.shares;

        write!(output, "53={}\u{01}", val )?; // FIELD_SHARES
    }
    if flds.last_mkt.is_some() {
        let val = flds.last_mkt.as_ref().unwrap();

        write!(output, "30={}\u{01}", val )?; // FIELD_LASTMKT
    }
    if flds.trading_session_id.is_some() {
        let val = flds.trading_session_id.as_ref().unwrap();

        write!(output, "336={}\u{01}", val )?; // FIELD_TRADINGSESSIONID
    }
    {
        let val = &flds.avg_px;

        write!(output, "6={}\u{01}", val )?; // FIELD_AVGPX
    }
    if flds.currency.is_some() {
        let val = flds.currency.as_ref().unwrap();

        write!(output, "15={}\u{01}", val )?; // FIELD_CURRENCY
    }
    if flds.avg_prx_precision.is_some() {
        let val = flds.avg_prx_precision.as_ref().unwrap();

        write!(output, "74={}\u{01}", val )?; // FIELD_AVGPRXPRECISION
    }
    {
        let val = &flds.trade_date;

        write!(output, "75={}\u{01}", val )?; // FIELD_TRADEDATE
    }
    if flds.transact_time.is_some() {
        let val = flds.transact_time.as_ref().unwrap();

        write!(output, "60={}\u{01}", val )?; // FIELD_TRANSACTTIME
    }
    if flds.settlmnt_typ.is_some() {
        let val = flds.settlmnt_typ.as_ref().unwrap();

        write!(output, "63={}\u{01}", val )?; // FIELD_SETTLMNTTYP
    }
    if flds.fut_sett_date.is_some() {
        let val = flds.fut_sett_date.as_ref().unwrap();

        write!(output, "64={}\u{01}", val )?; // FIELD_FUTSETTDATE
    }
    if flds.gross_trade_amt.is_some() {
        let val = flds.gross_trade_amt.as_ref().unwrap();

        write!(output, "381={}\u{01}", val )?; // FIELD_GROSSTRADEAMT
    }
    if flds.net_money.is_some() {
        let val = flds.net_money.as_ref().unwrap();

        write!(output, "118={}\u{01}", val )?; // FIELD_NETMONEY
    }
    if flds.open_close.is_some() {
        let val = flds.open_close.as_ref().unwrap();

        write!(output, "77={}\u{01}", val )?; // FIELD_OPENCLOSE
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
    if flds.num_days_interest.is_some() {
        let val = flds.num_days_interest.as_ref().unwrap();

        write!(output, "157={}\u{01}", val )?; // FIELD_NUMDAYSINTEREST
    }
    if flds.accrued_interest_rate.is_some() {
        let val = flds.accrued_interest_rate.as_ref().unwrap();

        write!(output, "158={}\u{01}", val )?; // FIELD_ACCRUEDINTERESTRATE
    }
    if flds.no_allocs.is_some() {
        let val = flds.no_allocs.as_ref().unwrap();

        write_group_no_allocs2_fields( val, output )?;
    }
    Ok( () )
}


fn write_list_cancel_request_fields(flds: &ListCancelRequestFields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.list_id;

        write!(output, "66={}\u{01}", val )?; // FIELD_LISTID
    }
    {
        let val = &flds.transact_time;

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
    Ok( () )
}


fn write_list_execute_fields(flds: &ListExecuteFields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.list_id;

        write!(output, "66={}\u{01}", val )?; // FIELD_LISTID
    }
    if flds.client_bid_id.is_some() {
        let val = flds.client_bid_id.as_ref().unwrap();

        write!(output, "391={}\u{01}", val )?; // FIELD_CLIENTBIDID
    }
    if flds.bid_id.is_some() {
        let val = flds.bid_id.as_ref().unwrap();

        write!(output, "390={}\u{01}", val )?; // FIELD_BIDID
    }
    {
        let val = &flds.transact_time;

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
    Ok( () )
}


fn write_list_status_request_fields(flds: &ListStatusRequestFields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.list_id;

        write!(output, "66={}\u{01}", val )?; // FIELD_LISTID
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


fn write_list_status_fields(flds: &ListStatusFields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.list_id;

        write!(output, "66={}\u{01}", val )?; // FIELD_LISTID
    }
    {
        let val = &flds.list_status_type;

        write!(output, "429={}\u{01}", val )?; // FIELD_LISTSTATUSTYPE
    }
    {
        let val = &flds.no_rpts;

        write!(output, "82={}\u{01}", val )?; // FIELD_NORPTS
    }
    {
        let val = &flds.list_order_status;

        write!(output, "431={}\u{01}", val )?; // FIELD_LISTORDERSTATUS
    }
    {
        let val = &flds.rpt_seq;

        write!(output, "83={}\u{01}", val )?; // FIELD_RPTSEQ
    }
    if flds.list_status_text.is_some() {
        let val = flds.list_status_text.as_ref().unwrap();

        write!(output, "444={}\u{01}", val )?; // FIELD_LISTSTATUSTEXT
    }
    if flds.encoded_list_status_text_len.is_some() {
        let val = flds.encoded_list_status_text_len.as_ref().unwrap();

        write!(output, "445={}\u{01}", val )?; // FIELD_ENCODEDLISTSTATUSTEXTLEN
    }
    if flds.encoded_list_status_text.is_some() {
        let val = flds.encoded_list_status_text.as_ref().unwrap();

        write!(output, "446={}\u{01}", val )?; // FIELD_ENCODEDLISTSTATUSTEXT
    }
    if flds.transact_time.is_some() {
        let val = flds.transact_time.as_ref().unwrap();

        write!(output, "60={}\u{01}", val )?; // FIELD_TRANSACTTIME
    }
    {
        let val = &flds.tot_no_orders;

        write!(output, "68={}\u{01}", val )?; // FIELD_TOTNOORDERS
    }
    {
        let val = &flds.no_orders;

        write_group_no_orders17_fields( val, output )?;
    }
    Ok( () )
}


fn write_allocation_instruction_ack_fields(flds: &AllocationInstructionAckFields, output: &mut Write) -> Result<(), io::Error> {

    if flds.client_id.is_some() {
        let val = flds.client_id.as_ref().unwrap();

        write!(output, "109={}\u{01}", val )?; // FIELD_CLIENTID
    }
    if flds.exec_broker.is_some() {
        let val = flds.exec_broker.as_ref().unwrap();

        write!(output, "76={}\u{01}", val )?; // FIELD_EXECBROKER
    }
    {
        let val = &flds.alloc_id;

        write!(output, "70={}\u{01}", val )?; // FIELD_ALLOCID
    }
    {
        let val = &flds.trade_date;

        write!(output, "75={}\u{01}", val )?; // FIELD_TRADEDATE
    }
    if flds.transact_time.is_some() {
        let val = flds.transact_time.as_ref().unwrap();

        write!(output, "60={}\u{01}", val )?; // FIELD_TRANSACTTIME
    }
    {
        let val = &flds.alloc_status;

        write!(output, "87={}\u{01}", val )?; // FIELD_ALLOCSTATUS
    }
    if flds.alloc_rej_code.is_some() {
        let val = flds.alloc_rej_code.as_ref().unwrap();

        write!(output, "88={}\u{01}", val )?; // FIELD_ALLOCREJCODE
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


fn write_dont_know_trade_fields(flds: &DontKnowTradeFields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.order_id;

        write!(output, "37={}\u{01}", val )?; // FIELD_ORDERID
    }
    {
        let val = &flds.exec_id;

        write!(output, "17={}\u{01}", val )?; // FIELD_EXECID
    }
    {
        let val = &flds.dkreason;

        write!(output, "127={}\u{01}", val )?; // FIELD_DKREASON
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
    if flds.last_shares.is_some() {
        let val = flds.last_shares.as_ref().unwrap();

        write!(output, "32={}\u{01}", val )?; // FIELD_LASTSHARES
    }
    if flds.last_px.is_some() {
        let val = flds.last_px.as_ref().unwrap();

        write!(output, "31={}\u{01}", val )?; // FIELD_LASTPX
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


fn write_quote_request_fields(flds: &QuoteRequestFields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.quote_req_id;

        write!(output, "131={}\u{01}", val )?; // FIELD_QUOTEREQID
    }
    {
        let val = &flds.no_related_sym;

        write_group_no_related_sym25_fields( val, output )?;
    }
    Ok( () )
}


fn write_quote_fields(flds: &QuoteFields, output: &mut Write) -> Result<(), io::Error> {

    if flds.quote_req_id.is_some() {
        let val = flds.quote_req_id.as_ref().unwrap();

        write!(output, "131={}\u{01}", val )?; // FIELD_QUOTEREQID
    }
    {
        let val = &flds.quote_id;

        write!(output, "117={}\u{01}", val )?; // FIELD_QUOTEID
    }
    if flds.quote_response_level.is_some() {
        let val = flds.quote_response_level.as_ref().unwrap();

        write!(output, "301={}\u{01}", val )?; // FIELD_QUOTERESPONSELEVEL
    }
    if flds.trading_session_id.is_some() {
        let val = flds.trading_session_id.as_ref().unwrap();

        write!(output, "336={}\u{01}", val )?; // FIELD_TRADINGSESSIONID
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
    if flds.bid_px.is_some() {
        let val = flds.bid_px.as_ref().unwrap();

        write!(output, "132={}\u{01}", val )?; // FIELD_BIDPX
    }
    if flds.offer_px.is_some() {
        let val = flds.offer_px.as_ref().unwrap();

        write!(output, "133={}\u{01}", val )?; // FIELD_OFFERPX
    }
    if flds.bid_size.is_some() {
        let val = flds.bid_size.as_ref().unwrap();

        write!(output, "134={}\u{01}", val )?; // FIELD_BIDSIZE
    }
    if flds.offer_size.is_some() {
        let val = flds.offer_size.as_ref().unwrap();

        write!(output, "135={}\u{01}", val )?; // FIELD_OFFERSIZE
    }
    if flds.valid_until_time.is_some() {
        let val = flds.valid_until_time.as_ref().unwrap();

        write!(output, "62={}\u{01}", val )?; // FIELD_VALIDUNTILTIME
    }
    if flds.bid_spot_rate.is_some() {
        let val = flds.bid_spot_rate.as_ref().unwrap();

        write!(output, "188={}\u{01}", val )?; // FIELD_BIDSPOTRATE
    }
    if flds.offer_spot_rate.is_some() {
        let val = flds.offer_spot_rate.as_ref().unwrap();

        write!(output, "190={}\u{01}", val )?; // FIELD_OFFERSPOTRATE
    }
    if flds.bid_forward_points.is_some() {
        let val = flds.bid_forward_points.as_ref().unwrap();

        write!(output, "189={}\u{01}", val )?; // FIELD_BIDFORWARDPOINTS
    }
    if flds.offer_forward_points.is_some() {
        let val = flds.offer_forward_points.as_ref().unwrap();

        write!(output, "191={}\u{01}", val )?; // FIELD_OFFERFORWARDPOINTS
    }
    if flds.transact_time.is_some() {
        let val = flds.transact_time.as_ref().unwrap();

        write!(output, "60={}\u{01}", val )?; // FIELD_TRANSACTTIME
    }
    if flds.fut_sett_date.is_some() {
        let val = flds.fut_sett_date.as_ref().unwrap();

        write!(output, "64={}\u{01}", val )?; // FIELD_FUTSETTDATE
    }
    if flds.ord_type.is_some() {
        let val = flds.ord_type.as_ref().unwrap();

        write!(output, "40={}\u{01}", val )?; // FIELD_ORDTYPE
    }
    if flds.fut_sett_date2.is_some() {
        let val = flds.fut_sett_date2.as_ref().unwrap();

        write!(output, "193={}\u{01}", val )?; // FIELD_FUTSETTDATE2
    }
    if flds.order_qty2.is_some() {
        let val = flds.order_qty2.as_ref().unwrap();

        write!(output, "192={}\u{01}", val )?; // FIELD_ORDERQTY2
    }
    if flds.currency.is_some() {
        let val = flds.currency.as_ref().unwrap();

        write!(output, "15={}\u{01}", val )?; // FIELD_CURRENCY
    }
    Ok( () )
}


fn write_settlement_instructions_fields(flds: &SettlementInstructionsFields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.settl_inst_id;

        write!(output, "162={}\u{01}", val )?; // FIELD_SETTLINSTID
    }
    {
        let val = &flds.settl_inst_trans_type;

        write!(output, "163={}\u{01}", val )?; // FIELD_SETTLINSTTRANSTYPE
    }
    {
        let val = &flds.settl_inst_ref_id;

        write!(output, "214={}\u{01}", val )?; // FIELD_SETTLINSTREFID
    }
    {
        let val = &flds.settl_inst_mode;

        write!(output, "160={}\u{01}", val )?; // FIELD_SETTLINSTMODE
    }
    {
        let val = &flds.settl_inst_source;

        write!(output, "165={}\u{01}", val )?; // FIELD_SETTLINSTSOURCE
    }
    {
        let val = &flds.alloc_account;

        write!(output, "79={}\u{01}", val )?; // FIELD_ALLOCACCOUNT
    }
    if flds.settl_location.is_some() {
        let val = flds.settl_location.as_ref().unwrap();

        write!(output, "166={}\u{01}", val )?; // FIELD_SETTLLOCATION
    }
    if flds.trade_date.is_some() {
        let val = flds.trade_date.as_ref().unwrap();

        write!(output, "75={}\u{01}", val )?; // FIELD_TRADEDATE
    }
    if flds.alloc_id.is_some() {
        let val = flds.alloc_id.as_ref().unwrap();

        write!(output, "70={}\u{01}", val )?; // FIELD_ALLOCID
    }
    if flds.last_mkt.is_some() {
        let val = flds.last_mkt.as_ref().unwrap();

        write!(output, "30={}\u{01}", val )?; // FIELD_LASTMKT
    }
    if flds.trading_session_id.is_some() {
        let val = flds.trading_session_id.as_ref().unwrap();

        write!(output, "336={}\u{01}", val )?; // FIELD_TRADINGSESSIONID
    }
    if flds.side.is_some() {
        let val = flds.side.as_ref().unwrap();

        write!(output, "54={}\u{01}", val )?; // FIELD_SIDE
    }
    if flds.security_type.is_some() {
        let val = flds.security_type.as_ref().unwrap();

        write!(output, "167={}\u{01}", val )?; // FIELD_SECURITYTYPE
    }
    if flds.effective_time.is_some() {
        let val = flds.effective_time.as_ref().unwrap();

        write!(output, "168={}\u{01}", val )?; // FIELD_EFFECTIVETIME
    }
    {
        let val = &flds.transact_time;

        write!(output, "60={}\u{01}", val )?; // FIELD_TRANSACTTIME
    }
    if flds.client_id.is_some() {
        let val = flds.client_id.as_ref().unwrap();

        write!(output, "109={}\u{01}", val )?; // FIELD_CLIENTID
    }
    if flds.exec_broker.is_some() {
        let val = flds.exec_broker.as_ref().unwrap();

        write!(output, "76={}\u{01}", val )?; // FIELD_EXECBROKER
    }
    if flds.stand_inst_db_type.is_some() {
        let val = flds.stand_inst_db_type.as_ref().unwrap();

        write!(output, "169={}\u{01}", val )?; // FIELD_STANDINSTDBTYPE
    }
    if flds.stand_inst_db_name.is_some() {
        let val = flds.stand_inst_db_name.as_ref().unwrap();

        write!(output, "170={}\u{01}", val )?; // FIELD_STANDINSTDBNAME
    }
    if flds.stand_inst_db_id.is_some() {
        let val = flds.stand_inst_db_id.as_ref().unwrap();

        write!(output, "171={}\u{01}", val )?; // FIELD_STANDINSTDBID
    }
    if flds.settl_delivery_type.is_some() {
        let val = flds.settl_delivery_type.as_ref().unwrap();

        write!(output, "172={}\u{01}", val )?; // FIELD_SETTLDELIVERYTYPE
    }
    if flds.settl_depository_code.is_some() {
        let val = flds.settl_depository_code.as_ref().unwrap();

        write!(output, "173={}\u{01}", val )?; // FIELD_SETTLDEPOSITORYCODE
    }
    if flds.settl_brkr_code.is_some() {
        let val = flds.settl_brkr_code.as_ref().unwrap();

        write!(output, "174={}\u{01}", val )?; // FIELD_SETTLBRKRCODE
    }
    if flds.settl_inst_code.is_some() {
        let val = flds.settl_inst_code.as_ref().unwrap();

        write!(output, "175={}\u{01}", val )?; // FIELD_SETTLINSTCODE
    }
    if flds.security_settl_agent_name.is_some() {
        let val = flds.security_settl_agent_name.as_ref().unwrap();

        write!(output, "176={}\u{01}", val )?; // FIELD_SECURITYSETTLAGENTNAME
    }
    if flds.security_settl_agent_code.is_some() {
        let val = flds.security_settl_agent_code.as_ref().unwrap();

        write!(output, "177={}\u{01}", val )?; // FIELD_SECURITYSETTLAGENTCODE
    }
    if flds.security_settl_agent_acct_num.is_some() {
        let val = flds.security_settl_agent_acct_num.as_ref().unwrap();

        write!(output, "178={}\u{01}", val )?; // FIELD_SECURITYSETTLAGENTACCTNUM
    }
    if flds.security_settl_agent_acct_name.is_some() {
        let val = flds.security_settl_agent_acct_name.as_ref().unwrap();

        write!(output, "179={}\u{01}", val )?; // FIELD_SECURITYSETTLAGENTACCTNAME
    }
    if flds.security_settl_agent_contact_name.is_some() {
        let val = flds.security_settl_agent_contact_name.as_ref().unwrap();

        write!(output, "180={}\u{01}", val )?; // FIELD_SECURITYSETTLAGENTCONTACTNAME
    }
    if flds.security_settl_agent_contact_phone.is_some() {
        let val = flds.security_settl_agent_contact_phone.as_ref().unwrap();

        write!(output, "181={}\u{01}", val )?; // FIELD_SECURITYSETTLAGENTCONTACTPHONE
    }
    if flds.cash_settl_agent_name.is_some() {
        let val = flds.cash_settl_agent_name.as_ref().unwrap();

        write!(output, "182={}\u{01}", val )?; // FIELD_CASHSETTLAGENTNAME
    }
    if flds.cash_settl_agent_code.is_some() {
        let val = flds.cash_settl_agent_code.as_ref().unwrap();

        write!(output, "183={}\u{01}", val )?; // FIELD_CASHSETTLAGENTCODE
    }
    if flds.cash_settl_agent_acct_num.is_some() {
        let val = flds.cash_settl_agent_acct_num.as_ref().unwrap();

        write!(output, "184={}\u{01}", val )?; // FIELD_CASHSETTLAGENTACCTNUM
    }
    if flds.cash_settl_agent_acct_name.is_some() {
        let val = flds.cash_settl_agent_acct_name.as_ref().unwrap();

        write!(output, "185={}\u{01}", val )?; // FIELD_CASHSETTLAGENTACCTNAME
    }
    if flds.cash_settl_agent_contact_name.is_some() {
        let val = flds.cash_settl_agent_contact_name.as_ref().unwrap();

        write!(output, "186={}\u{01}", val )?; // FIELD_CASHSETTLAGENTCONTACTNAME
    }
    if flds.cash_settl_agent_contact_phone.is_some() {
        let val = flds.cash_settl_agent_contact_phone.as_ref().unwrap();

        write!(output, "187={}\u{01}", val )?; // FIELD_CASHSETTLAGENTCONTACTPHONE
    }
    Ok( () )
}


fn write_market_data_request_fields(flds: &MarketDataRequestFields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.mdreq_id;

        write!(output, "262={}\u{01}", val )?; // FIELD_MDREQID
    }
    {
        let val = &flds.subscription_request_type;

        write!(output, "263={}\u{01}", val )?; // FIELD_SUBSCRIPTIONREQUESTTYPE
    }
    {
        let val = &flds.market_depth;

        write!(output, "264={}\u{01}", val )?; // FIELD_MARKETDEPTH
    }
    if flds.mdupdate_type.is_some() {
        let val = flds.mdupdate_type.as_ref().unwrap();

        write!(output, "265={}\u{01}", val )?; // FIELD_MDUPDATETYPE
    }
    if flds.aggregated_book.is_some() {
        let val = flds.aggregated_book.as_ref().unwrap();

        write!(output, "266={}\u{01}", to_boolconv(val) )?; // FIELD_AGGREGATEDBOOK
    }
    {
        let val = &flds.no_mdentry_types;

        write_group_no_mdentry_types12_fields( val, output )?;
    }
    {
        let val = &flds.no_related_sym;

        write_group_no_related_sym24_fields( val, output )?;
    }
    Ok( () )
}


fn write_market_data_snapshot_full_refresh_fields(flds: &MarketDataSnapshotFullRefreshFields, output: &mut Write) -> Result<(), io::Error> {

    if flds.mdreq_id.is_some() {
        let val = flds.mdreq_id.as_ref().unwrap();

        write!(output, "262={}\u{01}", val )?; // FIELD_MDREQID
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
    if flds.financial_status.is_some() {
        let val = flds.financial_status.as_ref().unwrap();

        write!(output, "291={}\u{01}", val )?; // FIELD_FINANCIALSTATUS
    }
    if flds.corporate_action.is_some() {
        let val = flds.corporate_action.as_ref().unwrap();

        write!(output, "292={}\u{01}", val )?; // FIELD_CORPORATEACTION
    }
    if flds.total_volume_traded.is_some() {
        let val = flds.total_volume_traded.as_ref().unwrap();

        write!(output, "387={}\u{01}", val )?; // FIELD_TOTALVOLUMETRADED
    }
    {
        let val = &flds.no_mdentries;

        write_group_no_mdentries10_fields( val, output )?;
    }
    Ok( () )
}


fn write_market_data_incremental_refresh_fields(flds: &MarketDataIncrementalRefreshFields, output: &mut Write) -> Result<(), io::Error> {

    if flds.mdreq_id.is_some() {
        let val = flds.mdreq_id.as_ref().unwrap();

        write!(output, "262={}\u{01}", val )?; // FIELD_MDREQID
    }
    {
        let val = &flds.no_mdentries;

        write_group_no_mdentries11_fields( val, output )?;
    }
    Ok( () )
}


fn write_market_data_request_reject_fields(flds: &MarketDataRequestRejectFields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.mdreq_id;

        write!(output, "262={}\u{01}", val )?; // FIELD_MDREQID
    }
    if flds.mdreq_rej_reason.is_some() {
        let val = flds.mdreq_rej_reason.as_ref().unwrap();

        write!(output, "281={}\u{01}", val )?; // FIELD_MDREQREJREASON
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


fn write_quote_cancel_fields(flds: &QuoteCancelFields, output: &mut Write) -> Result<(), io::Error> {

    if flds.quote_req_id.is_some() {
        let val = flds.quote_req_id.as_ref().unwrap();

        write!(output, "131={}\u{01}", val )?; // FIELD_QUOTEREQID
    }
    {
        let val = &flds.quote_id;

        write!(output, "117={}\u{01}", val )?; // FIELD_QUOTEID
    }
    {
        let val = &flds.quote_cancel_type;

        write!(output, "298={}\u{01}", val )?; // FIELD_QUOTECANCELTYPE
    }
    if flds.quote_response_level.is_some() {
        let val = flds.quote_response_level.as_ref().unwrap();

        write!(output, "301={}\u{01}", val )?; // FIELD_QUOTERESPONSELEVEL
    }
    if flds.trading_session_id.is_some() {
        let val = flds.trading_session_id.as_ref().unwrap();

        write!(output, "336={}\u{01}", val )?; // FIELD_TRADINGSESSIONID
    }
    {
        let val = &flds.no_quote_entries;

        write_group_no_quote_entries18_fields( val, output )?;
    }
    Ok( () )
}


fn write_quote_status_request_fields(flds: &QuoteStatusRequestFields, output: &mut Write) -> Result<(), io::Error> {

    if flds.quote_id.is_some() {
        let val = flds.quote_id.as_ref().unwrap();

        write!(output, "117={}\u{01}", val )?; // FIELD_QUOTEID
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
    if flds.side.is_some() {
        let val = flds.side.as_ref().unwrap();

        write!(output, "54={}\u{01}", val )?; // FIELD_SIDE
    }
    if flds.trading_session_id.is_some() {
        let val = flds.trading_session_id.as_ref().unwrap();

        write!(output, "336={}\u{01}", val )?; // FIELD_TRADINGSESSIONID
    }
    Ok( () )
}


fn write_quote_acknowledgement_fields(flds: &QuoteAcknowledgementFields, output: &mut Write) -> Result<(), io::Error> {

    if flds.quote_req_id.is_some() {
        let val = flds.quote_req_id.as_ref().unwrap();

        write!(output, "131={}\u{01}", val )?; // FIELD_QUOTEREQID
    }
    if flds.quote_id.is_some() {
        let val = flds.quote_id.as_ref().unwrap();

        write!(output, "117={}\u{01}", val )?; // FIELD_QUOTEID
    }
    {
        let val = &flds.quote_ack_status;

        write!(output, "297={}\u{01}", val )?; // FIELD_QUOTEACKSTATUS
    }
    if flds.quote_reject_reason.is_some() {
        let val = flds.quote_reject_reason.as_ref().unwrap();

        write!(output, "300={}\u{01}", val )?; // FIELD_QUOTEREJECTREASON
    }
    if flds.quote_response_level.is_some() {
        let val = flds.quote_response_level.as_ref().unwrap();

        write!(output, "301={}\u{01}", val )?; // FIELD_QUOTERESPONSELEVEL
    }
    if flds.trading_session_id.is_some() {
        let val = flds.trading_session_id.as_ref().unwrap();

        write!(output, "336={}\u{01}", val )?; // FIELD_TRADINGSESSIONID
    }
    if flds.text.is_some() {
        let val = flds.text.as_ref().unwrap();

        write!(output, "58={}\u{01}", val )?; // FIELD_TEXT
    }
    if flds.no_quote_sets.is_some() {
        let val = flds.no_quote_sets.as_ref().unwrap();

        write_group_no_quote_sets21_fields( val, output )?;
    }
    Ok( () )
}


fn write_security_definition_request_fields(flds: &SecurityDefinitionRequestFields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.security_req_id;

        write!(output, "320={}\u{01}", val )?; // FIELD_SECURITYREQID
    }
    {
        let val = &flds.security_request_type;

        write!(output, "321={}\u{01}", val )?; // FIELD_SECURITYREQUESTTYPE
    }
    if flds.symbol.is_some() {
        let val = flds.symbol.as_ref().unwrap();

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
    if flds.currency.is_some() {
        let val = flds.currency.as_ref().unwrap();

        write!(output, "15={}\u{01}", val )?; // FIELD_CURRENCY
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
    if flds.trading_session_id.is_some() {
        let val = flds.trading_session_id.as_ref().unwrap();

        write!(output, "336={}\u{01}", val )?; // FIELD_TRADINGSESSIONID
    }
    if flds.no_related_sym.is_some() {
        let val = flds.no_related_sym.as_ref().unwrap();

        write_group_no_related_sym23_fields( val, output )?;
    }
    Ok( () )
}


fn write_security_definition_fields(flds: &SecurityDefinitionFields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.security_req_id;

        write!(output, "320={}\u{01}", val )?; // FIELD_SECURITYREQID
    }
    {
        let val = &flds.security_response_id;

        write!(output, "322={}\u{01}", val )?; // FIELD_SECURITYRESPONSEID
    }
    if flds.security_response_type.is_some() {
        let val = flds.security_response_type.as_ref().unwrap();

        write!(output, "323={}\u{01}", val )?; // FIELD_SECURITYRESPONSETYPE
    }
    {
        let val = &flds.total_num_securities;

        write!(output, "393={}\u{01}", val )?; // FIELD_TOTALNUMSECURITIES
    }
    if flds.symbol.is_some() {
        let val = flds.symbol.as_ref().unwrap();

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
    if flds.currency.is_some() {
        let val = flds.currency.as_ref().unwrap();

        write!(output, "15={}\u{01}", val )?; // FIELD_CURRENCY
    }
    if flds.trading_session_id.is_some() {
        let val = flds.trading_session_id.as_ref().unwrap();

        write!(output, "336={}\u{01}", val )?; // FIELD_TRADINGSESSIONID
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
    if flds.no_related_sym.is_some() {
        let val = flds.no_related_sym.as_ref().unwrap();

        write_group_no_related_sym23_fields( val, output )?;
    }
    Ok( () )
}


fn write_security_status_request_fields(flds: &SecurityStatusRequestFields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.security_status_req_id;

        write!(output, "324={}\u{01}", val )?; // FIELD_SECURITYSTATUSREQID
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
    if flds.currency.is_some() {
        let val = flds.currency.as_ref().unwrap();

        write!(output, "15={}\u{01}", val )?; // FIELD_CURRENCY
    }
    {
        let val = &flds.subscription_request_type;

        write!(output, "263={}\u{01}", val )?; // FIELD_SUBSCRIPTIONREQUESTTYPE
    }
    if flds.trading_session_id.is_some() {
        let val = flds.trading_session_id.as_ref().unwrap();

        write!(output, "336={}\u{01}", val )?; // FIELD_TRADINGSESSIONID
    }
    Ok( () )
}


fn write_security_status_fields(flds: &SecurityStatusFields, output: &mut Write) -> Result<(), io::Error> {

    if flds.security_status_req_id.is_some() {
        let val = flds.security_status_req_id.as_ref().unwrap();

        write!(output, "324={}\u{01}", val )?; // FIELD_SECURITYSTATUSREQID
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
    if flds.currency.is_some() {
        let val = flds.currency.as_ref().unwrap();

        write!(output, "15={}\u{01}", val )?; // FIELD_CURRENCY
    }
    if flds.trading_session_id.is_some() {
        let val = flds.trading_session_id.as_ref().unwrap();

        write!(output, "336={}\u{01}", val )?; // FIELD_TRADINGSESSIONID
    }
    if flds.unsolicited_indicator.is_some() {
        let val = flds.unsolicited_indicator.as_ref().unwrap();

        write!(output, "325={}\u{01}", to_boolconv(val) )?; // FIELD_UNSOLICITEDINDICATOR
    }
    if flds.security_trading_status.is_some() {
        let val = flds.security_trading_status.as_ref().unwrap();

        write!(output, "326={}\u{01}", val )?; // FIELD_SECURITYTRADINGSTATUS
    }
    if flds.financial_status.is_some() {
        let val = flds.financial_status.as_ref().unwrap();

        write!(output, "291={}\u{01}", val )?; // FIELD_FINANCIALSTATUS
    }
    if flds.corporate_action.is_some() {
        let val = flds.corporate_action.as_ref().unwrap();

        write!(output, "292={}\u{01}", val )?; // FIELD_CORPORATEACTION
    }
    if flds.halt_reason_char.is_some() {
        let val = flds.halt_reason_char.as_ref().unwrap();

        write!(output, "327={}\u{01}", val )?; // FIELD_HALTREASONCHAR
    }
    if flds.in_view_of_common.is_some() {
        let val = flds.in_view_of_common.as_ref().unwrap();

        write!(output, "328={}\u{01}", to_boolconv(val) )?; // FIELD_INVIEWOFCOMMON
    }
    if flds.due_to_related.is_some() {
        let val = flds.due_to_related.as_ref().unwrap();

        write!(output, "329={}\u{01}", to_boolconv(val) )?; // FIELD_DUETORELATED
    }
    if flds.buy_volume.is_some() {
        let val = flds.buy_volume.as_ref().unwrap();

        write!(output, "330={}\u{01}", val )?; // FIELD_BUYVOLUME
    }
    if flds.sell_volume.is_some() {
        let val = flds.sell_volume.as_ref().unwrap();

        write!(output, "331={}\u{01}", val )?; // FIELD_SELLVOLUME
    }
    if flds.high_px.is_some() {
        let val = flds.high_px.as_ref().unwrap();

        write!(output, "332={}\u{01}", val )?; // FIELD_HIGHPX
    }
    if flds.low_px.is_some() {
        let val = flds.low_px.as_ref().unwrap();

        write!(output, "333={}\u{01}", val )?; // FIELD_LOWPX
    }
    if flds.last_px.is_some() {
        let val = flds.last_px.as_ref().unwrap();

        write!(output, "31={}\u{01}", val )?; // FIELD_LASTPX
    }
    if flds.transact_time.is_some() {
        let val = flds.transact_time.as_ref().unwrap();

        write!(output, "60={}\u{01}", val )?; // FIELD_TRANSACTTIME
    }
    if flds.adjustment.is_some() {
        let val = flds.adjustment.as_ref().unwrap();

        write!(output, "334={}\u{01}", val )?; // FIELD_ADJUSTMENT
    }
    Ok( () )
}


fn write_trading_session_status_request_fields(flds: &TradingSessionStatusRequestFields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.trad_ses_req_id;

        write!(output, "335={}\u{01}", val )?; // FIELD_TRADSESREQID
    }
    if flds.trading_session_id.is_some() {
        let val = flds.trading_session_id.as_ref().unwrap();

        write!(output, "336={}\u{01}", val )?; // FIELD_TRADINGSESSIONID
    }
    if flds.trad_ses_method.is_some() {
        let val = flds.trad_ses_method.as_ref().unwrap();

        write!(output, "338={}\u{01}", val )?; // FIELD_TRADSESMETHOD
    }
    if flds.trad_ses_mode.is_some() {
        let val = flds.trad_ses_mode.as_ref().unwrap();

        write!(output, "339={}\u{01}", val )?; // FIELD_TRADSESMODE
    }
    {
        let val = &flds.subscription_request_type;

        write!(output, "263={}\u{01}", val )?; // FIELD_SUBSCRIPTIONREQUESTTYPE
    }
    Ok( () )
}


fn write_trading_session_status_fields(flds: &TradingSessionStatusFields, output: &mut Write) -> Result<(), io::Error> {

    if flds.trad_ses_req_id.is_some() {
        let val = flds.trad_ses_req_id.as_ref().unwrap();

        write!(output, "335={}\u{01}", val )?; // FIELD_TRADSESREQID
    }
    {
        let val = &flds.trading_session_id;

        write!(output, "336={}\u{01}", val )?; // FIELD_TRADINGSESSIONID
    }
    if flds.trad_ses_method.is_some() {
        let val = flds.trad_ses_method.as_ref().unwrap();

        write!(output, "338={}\u{01}", val )?; // FIELD_TRADSESMETHOD
    }
    if flds.trad_ses_mode.is_some() {
        let val = flds.trad_ses_mode.as_ref().unwrap();

        write!(output, "339={}\u{01}", val )?; // FIELD_TRADSESMODE
    }
    if flds.unsolicited_indicator.is_some() {
        let val = flds.unsolicited_indicator.as_ref().unwrap();

        write!(output, "325={}\u{01}", to_boolconv(val) )?; // FIELD_UNSOLICITEDINDICATOR
    }
    {
        let val = &flds.trad_ses_status;

        write!(output, "340={}\u{01}", val )?; // FIELD_TRADSESSTATUS
    }
    if flds.trad_ses_start_time.is_some() {
        let val = flds.trad_ses_start_time.as_ref().unwrap();

        write!(output, "341={}\u{01}", val )?; // FIELD_TRADSESSTARTTIME
    }
    if flds.trad_ses_open_time.is_some() {
        let val = flds.trad_ses_open_time.as_ref().unwrap();

        write!(output, "342={}\u{01}", val )?; // FIELD_TRADSESOPENTIME
    }
    if flds.trad_ses_pre_close_time.is_some() {
        let val = flds.trad_ses_pre_close_time.as_ref().unwrap();

        write!(output, "343={}\u{01}", val )?; // FIELD_TRADSESPRECLOSETIME
    }
    if flds.trad_ses_close_time.is_some() {
        let val = flds.trad_ses_close_time.as_ref().unwrap();

        write!(output, "344={}\u{01}", val )?; // FIELD_TRADSESCLOSETIME
    }
    if flds.trad_ses_end_time.is_some() {
        let val = flds.trad_ses_end_time.as_ref().unwrap();

        write!(output, "345={}\u{01}", val )?; // FIELD_TRADSESENDTIME
    }
    if flds.total_volume_traded.is_some() {
        let val = flds.total_volume_traded.as_ref().unwrap();

        write!(output, "387={}\u{01}", val )?; // FIELD_TOTALVOLUMETRADED
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


fn write_mass_quote_fields(flds: &MassQuoteFields, output: &mut Write) -> Result<(), io::Error> {

    if flds.quote_req_id.is_some() {
        let val = flds.quote_req_id.as_ref().unwrap();

        write!(output, "131={}\u{01}", val )?; // FIELD_QUOTEREQID
    }
    {
        let val = &flds.quote_id;

        write!(output, "117={}\u{01}", val )?; // FIELD_QUOTEID
    }
    if flds.quote_response_level.is_some() {
        let val = flds.quote_response_level.as_ref().unwrap();

        write!(output, "301={}\u{01}", val )?; // FIELD_QUOTERESPONSELEVEL
    }
    if flds.def_bid_size.is_some() {
        let val = flds.def_bid_size.as_ref().unwrap();

        write!(output, "293={}\u{01}", val )?; // FIELD_DEFBIDSIZE
    }
    if flds.def_offer_size.is_some() {
        let val = flds.def_offer_size.as_ref().unwrap();

        write!(output, "294={}\u{01}", val )?; // FIELD_DEFOFFERSIZE
    }
    {
        let val = &flds.no_quote_sets;

        write_group_no_quote_sets22_fields( val, output )?;
    }
    Ok( () )
}


fn write_business_message_reject_fields(flds: &BusinessMessageRejectFields, output: &mut Write) -> Result<(), io::Error> {

    if flds.ref_seq_num.is_some() {
        let val = flds.ref_seq_num.as_ref().unwrap();

        write!(output, "45={}\u{01}", val )?; // FIELD_REFSEQNUM
    }
    {
        let val = &flds.ref_msg_type;

        write!(output, "372={}\u{01}", val )?; // FIELD_REFMSGTYPE
    }
    if flds.business_reject_ref_id.is_some() {
        let val = flds.business_reject_ref_id.as_ref().unwrap();

        write!(output, "379={}\u{01}", val )?; // FIELD_BUSINESSREJECTREFID
    }
    {
        let val = &flds.business_reject_reason;

        write!(output, "380={}\u{01}", val )?; // FIELD_BUSINESSREJECTREASON
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


fn write_bid_request_fields(flds: &BidRequestFields, output: &mut Write) -> Result<(), io::Error> {

    if flds.bid_id.is_some() {
        let val = flds.bid_id.as_ref().unwrap();

        write!(output, "390={}\u{01}", val )?; // FIELD_BIDID
    }
    {
        let val = &flds.client_bid_id;

        write!(output, "391={}\u{01}", val )?; // FIELD_CLIENTBIDID
    }
    {
        let val = &flds.bid_request_trans_type;

        write!(output, "374={}\u{01}", val )?; // FIELD_BIDREQUESTTRANSTYPE
    }
    if flds.list_name.is_some() {
        let val = flds.list_name.as_ref().unwrap();

        write!(output, "392={}\u{01}", val )?; // FIELD_LISTNAME
    }
    {
        let val = &flds.total_num_securities;

        write!(output, "393={}\u{01}", val )?; // FIELD_TOTALNUMSECURITIES
    }
    {
        let val = &flds.bid_type;

        write!(output, "394={}\u{01}", val )?; // FIELD_BIDTYPE
    }
    if flds.num_tickets.is_some() {
        let val = flds.num_tickets.as_ref().unwrap();

        write!(output, "395={}\u{01}", val )?; // FIELD_NUMTICKETS
    }
    if flds.currency.is_some() {
        let val = flds.currency.as_ref().unwrap();

        write!(output, "15={}\u{01}", val )?; // FIELD_CURRENCY
    }
    if flds.side_value1.is_some() {
        let val = flds.side_value1.as_ref().unwrap();

        write!(output, "396={}\u{01}", val )?; // FIELD_SIDEVALUE1
    }
    if flds.side_value2.is_some() {
        let val = flds.side_value2.as_ref().unwrap();

        write!(output, "397={}\u{01}", val )?; // FIELD_SIDEVALUE2
    }
    if flds.no_bid_descriptors.is_some() {
        let val = flds.no_bid_descriptors.as_ref().unwrap();

        write_group_no_bid_descriptors6_fields( val, output )?;
    }
    if flds.no_bid_components.is_some() {
        let val = flds.no_bid_components.as_ref().unwrap();

        write_group_no_bid_components4_fields( val, output )?;
    }
    if flds.liquidity_ind_type.is_some() {
        let val = flds.liquidity_ind_type.as_ref().unwrap();

        write!(output, "409={}\u{01}", val )?; // FIELD_LIQUIDITYINDTYPE
    }
    if flds.wt_average_liquidity.is_some() {
        let val = flds.wt_average_liquidity.as_ref().unwrap();

        write!(output, "410={}\u{01}", val )?; // FIELD_WTAVERAGELIQUIDITY
    }
    if flds.exchange_for_physical.is_some() {
        let val = flds.exchange_for_physical.as_ref().unwrap();

        write!(output, "411={}\u{01}", to_boolconv(val) )?; // FIELD_EXCHANGEFORPHYSICAL
    }
    if flds.out_main_cntry_uindex.is_some() {
        let val = flds.out_main_cntry_uindex.as_ref().unwrap();

        write!(output, "412={}\u{01}", val )?; // FIELD_OUTMAINCNTRYUINDEX
    }
    if flds.cross_percent.is_some() {
        let val = flds.cross_percent.as_ref().unwrap();

        write!(output, "413={}\u{01}", val )?; // FIELD_CROSSPERCENT
    }
    if flds.prog_rpt_reqs.is_some() {
        let val = flds.prog_rpt_reqs.as_ref().unwrap();

        write!(output, "414={}\u{01}", val )?; // FIELD_PROGRPTREQS
    }
    if flds.prog_period_interval.is_some() {
        let val = flds.prog_period_interval.as_ref().unwrap();

        write!(output, "415={}\u{01}", val )?; // FIELD_PROGPERIODINTERVAL
    }
    if flds.inc_tax_ind.is_some() {
        let val = flds.inc_tax_ind.as_ref().unwrap();

        write!(output, "416={}\u{01}", val )?; // FIELD_INCTAXIND
    }
    if flds.forex_req.is_some() {
        let val = flds.forex_req.as_ref().unwrap();

        write!(output, "121={}\u{01}", to_boolconv(val) )?; // FIELD_FOREXREQ
    }
    if flds.num_bidders.is_some() {
        let val = flds.num_bidders.as_ref().unwrap();

        write!(output, "417={}\u{01}", val )?; // FIELD_NUMBIDDERS
    }
    if flds.trade_date.is_some() {
        let val = flds.trade_date.as_ref().unwrap();

        write!(output, "75={}\u{01}", val )?; // FIELD_TRADEDATE
    }
    {
        let val = &flds.trade_type;

        write!(output, "418={}\u{01}", val )?; // FIELD_TRADETYPE
    }
    {
        let val = &flds.basis_px_type;

        write!(output, "419={}\u{01}", val )?; // FIELD_BASISPXTYPE
    }
    if flds.strike_time.is_some() {
        let val = flds.strike_time.as_ref().unwrap();

        write!(output, "443={}\u{01}", val )?; // FIELD_STRIKETIME
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


fn write_bid_response_fields(flds: &BidResponseFields, output: &mut Write) -> Result<(), io::Error> {

    if flds.bid_id.is_some() {
        let val = flds.bid_id.as_ref().unwrap();

        write!(output, "390={}\u{01}", val )?; // FIELD_BIDID
    }
    if flds.client_bid_id.is_some() {
        let val = flds.client_bid_id.as_ref().unwrap();

        write!(output, "391={}\u{01}", val )?; // FIELD_CLIENTBIDID
    }
    {
        let val = &flds.no_bid_components;

        write_group_no_bid_components5_fields( val, output )?;
    }
    Ok( () )
}


fn write_list_strike_price_fields(flds: &ListStrikePriceFields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.list_id;

        write!(output, "66={}\u{01}", val )?; // FIELD_LISTID
    }
    {
        let val = &flds.tot_no_strikes;

        write!(output, "422={}\u{01}", val )?; // FIELD_TOTNOSTRIKES
    }
    {
        let val = &flds.no_strikes;

        write_group_no_strikes28_fields( val, output )?;
    }
    Ok( () )
}








fn write_group_no_allocs3_fields( group: &Vec<NoAllocs3Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOALLOCS, len )?;

    for g in group {
        write_group_no_allocs3_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_allocs3_fields_line( flds: &NoAllocs3Fields, output: &mut Write) -> Result<(), io::Error> {

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



fn write_group_no_mdentries10_fields( group: &Vec<NoMDEntries10Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOMDENTRIES, len )?;

    for g in group {
        write_group_no_mdentries10_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_mdentries10_fields_line( flds: &NoMDEntries10Fields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.mdentry_type;

        write!(output, "269={}\u{01}", val )?; // FIELD_MDENTRYTYPE
    }
    {
        let val = &flds.mdentry_px;

        write!(output, "270={}\u{01}", val )?; // FIELD_MDENTRYPX
    }
    if flds.currency.is_some() {
        let val = flds.currency.as_ref().unwrap();

        write!(output, "15={}\u{01}", val )?; // FIELD_CURRENCY
    }
    if flds.mdentry_size.is_some() {
        let val = flds.mdentry_size.as_ref().unwrap();

        write!(output, "271={}\u{01}", val )?; // FIELD_MDENTRYSIZE
    }
    if flds.mdentry_date.is_some() {
        let val = flds.mdentry_date.as_ref().unwrap();

        write!(output, "272={}\u{01}", val )?; // FIELD_MDENTRYDATE
    }
    if flds.mdentry_time.is_some() {
        let val = flds.mdentry_time.as_ref().unwrap();

        write!(output, "273={}\u{01}", val )?; // FIELD_MDENTRYTIME
    }
    if flds.tick_direction.is_some() {
        let val = flds.tick_direction.as_ref().unwrap();

        write!(output, "274={}\u{01}", val )?; // FIELD_TICKDIRECTION
    }
    if flds.mdmkt.is_some() {
        let val = flds.mdmkt.as_ref().unwrap();

        write!(output, "275={}\u{01}", val )?; // FIELD_MDMKT
    }
    if flds.trading_session_id.is_some() {
        let val = flds.trading_session_id.as_ref().unwrap();

        write!(output, "336={}\u{01}", val )?; // FIELD_TRADINGSESSIONID
    }
    if flds.quote_condition.is_some() {
        let val = flds.quote_condition.as_ref().unwrap();

        write!(output, "276={}\u{01}", val )?; // FIELD_QUOTECONDITION
    }
    if flds.trade_condition.is_some() {
        let val = flds.trade_condition.as_ref().unwrap();

        write!(output, "277={}\u{01}", val )?; // FIELD_TRADECONDITION
    }
    if flds.mdentry_originator.is_some() {
        let val = flds.mdentry_originator.as_ref().unwrap();

        write!(output, "282={}\u{01}", val )?; // FIELD_MDENTRYORIGINATOR
    }
    if flds.location_id.is_some() {
        let val = flds.location_id.as_ref().unwrap();

        write!(output, "283={}\u{01}", val )?; // FIELD_LOCATIONID
    }
    if flds.desk_id.is_some() {
        let val = flds.desk_id.as_ref().unwrap();

        write!(output, "284={}\u{01}", val )?; // FIELD_DESKID
    }
    if flds.open_close_settle_flag.is_some() {
        let val = flds.open_close_settle_flag.as_ref().unwrap();

        write!(output, "286={}\u{01}", val )?; // FIELD_OPENCLOSESETTLEFLAG
    }
    if flds.time_in_force.is_some() {
        let val = flds.time_in_force.as_ref().unwrap();

        write!(output, "59={}\u{01}", val )?; // FIELD_TIMEINFORCE
    }
    if flds.expire_date.is_some() {
        let val = flds.expire_date.as_ref().unwrap();

        write!(output, "432={}\u{01}", val )?; // FIELD_EXPIREDATE
    }
    if flds.expire_time.is_some() {
        let val = flds.expire_time.as_ref().unwrap();

        write!(output, "126={}\u{01}", val )?; // FIELD_EXPIRETIME
    }
    if flds.min_qty.is_some() {
        let val = flds.min_qty.as_ref().unwrap();

        write!(output, "110={}\u{01}", val )?; // FIELD_MINQTY
    }
    if flds.exec_inst.is_some() {
        let val = flds.exec_inst.as_ref().unwrap();

        write!(output, "18={}\u{01}", val )?; // FIELD_EXECINST
    }
    if flds.seller_days.is_some() {
        let val = flds.seller_days.as_ref().unwrap();

        write!(output, "287={}\u{01}", val )?; // FIELD_SELLERDAYS
    }
    if flds.order_id.is_some() {
        let val = flds.order_id.as_ref().unwrap();

        write!(output, "37={}\u{01}", val )?; // FIELD_ORDERID
    }
    if flds.quote_entry_id.is_some() {
        let val = flds.quote_entry_id.as_ref().unwrap();

        write!(output, "299={}\u{01}", val )?; // FIELD_QUOTEENTRYID
    }
    if flds.mdentry_buyer.is_some() {
        let val = flds.mdentry_buyer.as_ref().unwrap();

        write!(output, "288={}\u{01}", val )?; // FIELD_MDENTRYBUYER
    }
    if flds.mdentry_seller.is_some() {
        let val = flds.mdentry_seller.as_ref().unwrap();

        write!(output, "289={}\u{01}", val )?; // FIELD_MDENTRYSELLER
    }
    if flds.number_of_orders.is_some() {
        let val = flds.number_of_orders.as_ref().unwrap();

        write!(output, "346={}\u{01}", val )?; // FIELD_NUMBEROFORDERS
    }
    if flds.mdentry_position_no.is_some() {
        let val = flds.mdentry_position_no.as_ref().unwrap();

        write!(output, "290={}\u{01}", val )?; // FIELD_MDENTRYPOSITIONNO
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



fn write_group_no_related_sym23_fields( group: &Vec<NoRelatedSym23Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NORELATEDSYM, len )?;

    for g in group {
        write_group_no_related_sym23_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_related_sym23_fields_line( flds: &NoRelatedSym23Fields, output: &mut Write) -> Result<(), io::Error> {

    if flds.underlying_symbol.is_some() {
        let val = flds.underlying_symbol.as_ref().unwrap();

        write!(output, "311={}\u{01}", val )?; // FIELD_UNDERLYINGSYMBOL
    }
    if flds.underlying_symbol_sfx.is_some() {
        let val = flds.underlying_symbol_sfx.as_ref().unwrap();

        write!(output, "312={}\u{01}", val )?; // FIELD_UNDERLYINGSYMBOLSFX
    }
    if flds.underlying_security_id.is_some() {
        let val = flds.underlying_security_id.as_ref().unwrap();

        write!(output, "309={}\u{01}", val )?; // FIELD_UNDERLYINGSECURITYID
    }
    if flds.underlying_idsource.is_some() {
        let val = flds.underlying_idsource.as_ref().unwrap();

        write!(output, "305={}\u{01}", val )?; // FIELD_UNDERLYINGIDSOURCE
    }
    if flds.underlying_security_type.is_some() {
        let val = flds.underlying_security_type.as_ref().unwrap();

        write!(output, "310={}\u{01}", val )?; // FIELD_UNDERLYINGSECURITYTYPE
    }
    if flds.underlying_maturity_month_year.is_some() {
        let val = flds.underlying_maturity_month_year.as_ref().unwrap();

        write!(output, "313={}\u{01}", val )?; // FIELD_UNDERLYINGMATURITYMONTHYEAR
    }
    if flds.underlying_maturity_day.is_some() {
        let val = flds.underlying_maturity_day.as_ref().unwrap();

        write!(output, "314={}\u{01}", val )?; // FIELD_UNDERLYINGMATURITYDAY
    }
    if flds.underlying_put_or_call.is_some() {
        let val = flds.underlying_put_or_call.as_ref().unwrap();

        write!(output, "315={}\u{01}", val )?; // FIELD_UNDERLYINGPUTORCALL
    }
    if flds.underlying_strike_price.is_some() {
        let val = flds.underlying_strike_price.as_ref().unwrap();

        write!(output, "316={}\u{01}", val )?; // FIELD_UNDERLYINGSTRIKEPRICE
    }
    if flds.underlying_opt_attribute.is_some() {
        let val = flds.underlying_opt_attribute.as_ref().unwrap();

        write!(output, "317={}\u{01}", val )?; // FIELD_UNDERLYINGOPTATTRIBUTE
    }
    if flds.underlying_contract_multiplier.is_some() {
        let val = flds.underlying_contract_multiplier.as_ref().unwrap();

        write!(output, "436={}\u{01}", val )?; // FIELD_UNDERLYINGCONTRACTMULTIPLIER
    }
    if flds.underlying_coupon_rate.is_some() {
        let val = flds.underlying_coupon_rate.as_ref().unwrap();

        write!(output, "435={}\u{01}", val )?; // FIELD_UNDERLYINGCOUPONRATE
    }
    if flds.underlying_security_exchange.is_some() {
        let val = flds.underlying_security_exchange.as_ref().unwrap();

        write!(output, "308={}\u{01}", val )?; // FIELD_UNDERLYINGSECURITYEXCHANGE
    }
    if flds.underlying_issuer.is_some() {
        let val = flds.underlying_issuer.as_ref().unwrap();

        write!(output, "306={}\u{01}", val )?; // FIELD_UNDERLYINGISSUER
    }
    if flds.encoded_underlying_issuer_len.is_some() {
        let val = flds.encoded_underlying_issuer_len.as_ref().unwrap();

        write!(output, "362={}\u{01}", val )?; // FIELD_ENCODEDUNDERLYINGISSUERLEN
    }
    if flds.encoded_underlying_issuer.is_some() {
        let val = flds.encoded_underlying_issuer.as_ref().unwrap();

        write!(output, "363={}\u{01}", val )?; // FIELD_ENCODEDUNDERLYINGISSUER
    }
    if flds.underlying_security_desc.is_some() {
        let val = flds.underlying_security_desc.as_ref().unwrap();

        write!(output, "307={}\u{01}", val )?; // FIELD_UNDERLYINGSECURITYDESC
    }
    if flds.encoded_underlying_security_desc_len.is_some() {
        let val = flds.encoded_underlying_security_desc_len.as_ref().unwrap();

        write!(output, "364={}\u{01}", val )?; // FIELD_ENCODEDUNDERLYINGSECURITYDESCLEN
    }
    if flds.encoded_underlying_security_desc.is_some() {
        let val = flds.encoded_underlying_security_desc.as_ref().unwrap();

        write!(output, "365={}\u{01}", val )?; // FIELD_ENCODEDUNDERLYINGSECURITYDESC
    }
    if flds.ratio_qty.is_some() {
        let val = flds.ratio_qty.as_ref().unwrap();

        write!(output, "319={}\u{01}", val )?; // FIELD_RATIOQTY
    }
    if flds.side.is_some() {
        let val = flds.side.as_ref().unwrap();

        write!(output, "54={}\u{01}", val )?; // FIELD_SIDE
    }
    if flds.underlying_currency.is_some() {
        let val = flds.underlying_currency.as_ref().unwrap();

        write!(output, "318={}\u{01}", val )?; // FIELD_UNDERLYINGCURRENCY
    }

    Ok( () )
}



fn write_group_no_mdentry_types12_fields( group: &Vec<NoMDEntryTypes12Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOMDENTRYTYPES, len )?;

    for g in group {
        write_group_no_mdentry_types12_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_mdentry_types12_fields_line( flds: &NoMDEntryTypes12Fields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.mdentry_type;

        write!(output, "269={}\u{01}", val )?; // FIELD_MDENTRYTYPE
    }

    Ok( () )
}



fn write_group_no_quote_entries18_fields( group: &Vec<NoQuoteEntries18Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOQUOTEENTRIES, len )?;

    for g in group {
        write_group_no_quote_entries18_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_quote_entries18_fields_line( flds: &NoQuoteEntries18Fields, output: &mut Write) -> Result<(), io::Error> {

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
    if flds.underlying_symbol.is_some() {
        let val = flds.underlying_symbol.as_ref().unwrap();

        write!(output, "311={}\u{01}", val )?; // FIELD_UNDERLYINGSYMBOL
    }

    Ok( () )
}



fn write_group_no_quote_entries20_fields( group: &Vec<NoQuoteEntries20Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOQUOTEENTRIES, len )?;

    for g in group {
        write_group_no_quote_entries20_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_quote_entries20_fields_line( flds: &NoQuoteEntries20Fields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.quote_entry_id;

        write!(output, "299={}\u{01}", val )?; // FIELD_QUOTEENTRYID
    }
    if flds.symbol.is_some() {
        let val = flds.symbol.as_ref().unwrap();

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
    if flds.bid_px.is_some() {
        let val = flds.bid_px.as_ref().unwrap();

        write!(output, "132={}\u{01}", val )?; // FIELD_BIDPX
    }
    if flds.offer_px.is_some() {
        let val = flds.offer_px.as_ref().unwrap();

        write!(output, "133={}\u{01}", val )?; // FIELD_OFFERPX
    }
    if flds.bid_size.is_some() {
        let val = flds.bid_size.as_ref().unwrap();

        write!(output, "134={}\u{01}", val )?; // FIELD_BIDSIZE
    }
    if flds.offer_size.is_some() {
        let val = flds.offer_size.as_ref().unwrap();

        write!(output, "135={}\u{01}", val )?; // FIELD_OFFERSIZE
    }
    if flds.valid_until_time.is_some() {
        let val = flds.valid_until_time.as_ref().unwrap();

        write!(output, "62={}\u{01}", val )?; // FIELD_VALIDUNTILTIME
    }
    if flds.bid_spot_rate.is_some() {
        let val = flds.bid_spot_rate.as_ref().unwrap();

        write!(output, "188={}\u{01}", val )?; // FIELD_BIDSPOTRATE
    }
    if flds.offer_spot_rate.is_some() {
        let val = flds.offer_spot_rate.as_ref().unwrap();

        write!(output, "190={}\u{01}", val )?; // FIELD_OFFERSPOTRATE
    }
    if flds.bid_forward_points.is_some() {
        let val = flds.bid_forward_points.as_ref().unwrap();

        write!(output, "189={}\u{01}", val )?; // FIELD_BIDFORWARDPOINTS
    }
    if flds.offer_forward_points.is_some() {
        let val = flds.offer_forward_points.as_ref().unwrap();

        write!(output, "191={}\u{01}", val )?; // FIELD_OFFERFORWARDPOINTS
    }
    if flds.transact_time.is_some() {
        let val = flds.transact_time.as_ref().unwrap();

        write!(output, "60={}\u{01}", val )?; // FIELD_TRANSACTTIME
    }
    if flds.trading_session_id.is_some() {
        let val = flds.trading_session_id.as_ref().unwrap();

        write!(output, "336={}\u{01}", val )?; // FIELD_TRADINGSESSIONID
    }
    if flds.fut_sett_date.is_some() {
        let val = flds.fut_sett_date.as_ref().unwrap();

        write!(output, "64={}\u{01}", val )?; // FIELD_FUTSETTDATE
    }
    if flds.ord_type.is_some() {
        let val = flds.ord_type.as_ref().unwrap();

        write!(output, "40={}\u{01}", val )?; // FIELD_ORDTYPE
    }
    if flds.fut_sett_date2.is_some() {
        let val = flds.fut_sett_date2.as_ref().unwrap();

        write!(output, "193={}\u{01}", val )?; // FIELD_FUTSETTDATE2
    }
    if flds.order_qty2.is_some() {
        let val = flds.order_qty2.as_ref().unwrap();

        write!(output, "192={}\u{01}", val )?; // FIELD_ORDERQTY2
    }
    if flds.currency.is_some() {
        let val = flds.currency.as_ref().unwrap();

        write!(output, "15={}\u{01}", val )?; // FIELD_CURRENCY
    }

    Ok( () )
}



fn write_group_no_mdentries11_fields( group: &Vec<NoMDEntries11Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOMDENTRIES, len )?;

    for g in group {
        write_group_no_mdentries11_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_mdentries11_fields_line( flds: &NoMDEntries11Fields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.mdupdate_action;

        write!(output, "279={}\u{01}", val )?; // FIELD_MDUPDATEACTION
    }
    if flds.delete_reason.is_some() {
        let val = flds.delete_reason.as_ref().unwrap();

        write!(output, "285={}\u{01}", val )?; // FIELD_DELETEREASON
    }
    if flds.mdentry_type.is_some() {
        let val = flds.mdentry_type.as_ref().unwrap();

        write!(output, "269={}\u{01}", val )?; // FIELD_MDENTRYTYPE
    }
    if flds.mdentry_id.is_some() {
        let val = flds.mdentry_id.as_ref().unwrap();

        write!(output, "278={}\u{01}", val )?; // FIELD_MDENTRYID
    }
    if flds.mdentry_ref_id.is_some() {
        let val = flds.mdentry_ref_id.as_ref().unwrap();

        write!(output, "280={}\u{01}", val )?; // FIELD_MDENTRYREFID
    }
    if flds.symbol.is_some() {
        let val = flds.symbol.as_ref().unwrap();

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
    if flds.financial_status.is_some() {
        let val = flds.financial_status.as_ref().unwrap();

        write!(output, "291={}\u{01}", val )?; // FIELD_FINANCIALSTATUS
    }
    if flds.corporate_action.is_some() {
        let val = flds.corporate_action.as_ref().unwrap();

        write!(output, "292={}\u{01}", val )?; // FIELD_CORPORATEACTION
    }
    if flds.mdentry_px.is_some() {
        let val = flds.mdentry_px.as_ref().unwrap();

        write!(output, "270={}\u{01}", val )?; // FIELD_MDENTRYPX
    }
    if flds.currency.is_some() {
        let val = flds.currency.as_ref().unwrap();

        write!(output, "15={}\u{01}", val )?; // FIELD_CURRENCY
    }
    if flds.mdentry_size.is_some() {
        let val = flds.mdentry_size.as_ref().unwrap();

        write!(output, "271={}\u{01}", val )?; // FIELD_MDENTRYSIZE
    }
    if flds.mdentry_date.is_some() {
        let val = flds.mdentry_date.as_ref().unwrap();

        write!(output, "272={}\u{01}", val )?; // FIELD_MDENTRYDATE
    }
    if flds.mdentry_time.is_some() {
        let val = flds.mdentry_time.as_ref().unwrap();

        write!(output, "273={}\u{01}", val )?; // FIELD_MDENTRYTIME
    }
    if flds.tick_direction.is_some() {
        let val = flds.tick_direction.as_ref().unwrap();

        write!(output, "274={}\u{01}", val )?; // FIELD_TICKDIRECTION
    }
    if flds.mdmkt.is_some() {
        let val = flds.mdmkt.as_ref().unwrap();

        write!(output, "275={}\u{01}", val )?; // FIELD_MDMKT
    }
    if flds.trading_session_id.is_some() {
        let val = flds.trading_session_id.as_ref().unwrap();

        write!(output, "336={}\u{01}", val )?; // FIELD_TRADINGSESSIONID
    }
    if flds.quote_condition.is_some() {
        let val = flds.quote_condition.as_ref().unwrap();

        write!(output, "276={}\u{01}", val )?; // FIELD_QUOTECONDITION
    }
    if flds.trade_condition.is_some() {
        let val = flds.trade_condition.as_ref().unwrap();

        write!(output, "277={}\u{01}", val )?; // FIELD_TRADECONDITION
    }
    if flds.mdentry_originator.is_some() {
        let val = flds.mdentry_originator.as_ref().unwrap();

        write!(output, "282={}\u{01}", val )?; // FIELD_MDENTRYORIGINATOR
    }
    if flds.location_id.is_some() {
        let val = flds.location_id.as_ref().unwrap();

        write!(output, "283={}\u{01}", val )?; // FIELD_LOCATIONID
    }
    if flds.desk_id.is_some() {
        let val = flds.desk_id.as_ref().unwrap();

        write!(output, "284={}\u{01}", val )?; // FIELD_DESKID
    }
    if flds.open_close_settle_flag.is_some() {
        let val = flds.open_close_settle_flag.as_ref().unwrap();

        write!(output, "286={}\u{01}", val )?; // FIELD_OPENCLOSESETTLEFLAG
    }
    if flds.time_in_force.is_some() {
        let val = flds.time_in_force.as_ref().unwrap();

        write!(output, "59={}\u{01}", val )?; // FIELD_TIMEINFORCE
    }
    if flds.expire_date.is_some() {
        let val = flds.expire_date.as_ref().unwrap();

        write!(output, "432={}\u{01}", val )?; // FIELD_EXPIREDATE
    }
    if flds.expire_time.is_some() {
        let val = flds.expire_time.as_ref().unwrap();

        write!(output, "126={}\u{01}", val )?; // FIELD_EXPIRETIME
    }
    if flds.min_qty.is_some() {
        let val = flds.min_qty.as_ref().unwrap();

        write!(output, "110={}\u{01}", val )?; // FIELD_MINQTY
    }
    if flds.exec_inst.is_some() {
        let val = flds.exec_inst.as_ref().unwrap();

        write!(output, "18={}\u{01}", val )?; // FIELD_EXECINST
    }
    if flds.seller_days.is_some() {
        let val = flds.seller_days.as_ref().unwrap();

        write!(output, "287={}\u{01}", val )?; // FIELD_SELLERDAYS
    }
    if flds.order_id.is_some() {
        let val = flds.order_id.as_ref().unwrap();

        write!(output, "37={}\u{01}", val )?; // FIELD_ORDERID
    }
    if flds.quote_entry_id.is_some() {
        let val = flds.quote_entry_id.as_ref().unwrap();

        write!(output, "299={}\u{01}", val )?; // FIELD_QUOTEENTRYID
    }
    if flds.mdentry_buyer.is_some() {
        let val = flds.mdentry_buyer.as_ref().unwrap();

        write!(output, "288={}\u{01}", val )?; // FIELD_MDENTRYBUYER
    }
    if flds.mdentry_seller.is_some() {
        let val = flds.mdentry_seller.as_ref().unwrap();

        write!(output, "289={}\u{01}", val )?; // FIELD_MDENTRYSELLER
    }
    if flds.number_of_orders.is_some() {
        let val = flds.number_of_orders.as_ref().unwrap();

        write!(output, "346={}\u{01}", val )?; // FIELD_NUMBEROFORDERS
    }
    if flds.mdentry_position_no.is_some() {
        let val = flds.mdentry_position_no.as_ref().unwrap();

        write!(output, "290={}\u{01}", val )?; // FIELD_MDENTRYPOSITIONNO
    }
    if flds.total_volume_traded.is_some() {
        let val = flds.total_volume_traded.as_ref().unwrap();

        write!(output, "387={}\u{01}", val )?; // FIELD_TOTALVOLUMETRADED
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



fn write_group_no_bid_components4_fields( group: &Vec<NoBidComponents4Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOBIDCOMPONENTS, len )?;

    for g in group {
        write_group_no_bid_components4_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_bid_components4_fields_line( flds: &NoBidComponents4Fields, output: &mut Write) -> Result<(), io::Error> {

    if flds.list_id.is_some() {
        let val = flds.list_id.as_ref().unwrap();

        write!(output, "66={}\u{01}", val )?; // FIELD_LISTID
    }
    if flds.side.is_some() {
        let val = flds.side.as_ref().unwrap();

        write!(output, "54={}\u{01}", val )?; // FIELD_SIDE
    }
    if flds.trading_session_id.is_some() {
        let val = flds.trading_session_id.as_ref().unwrap();

        write!(output, "336={}\u{01}", val )?; // FIELD_TRADINGSESSIONID
    }
    if flds.net_gross_ind.is_some() {
        let val = flds.net_gross_ind.as_ref().unwrap();

        write!(output, "430={}\u{01}", val )?; // FIELD_NETGROSSIND
    }
    if flds.settlmnt_typ.is_some() {
        let val = flds.settlmnt_typ.as_ref().unwrap();

        write!(output, "63={}\u{01}", val )?; // FIELD_SETTLMNTTYP
    }
    if flds.fut_sett_date.is_some() {
        let val = flds.fut_sett_date.as_ref().unwrap();

        write!(output, "64={}\u{01}", val )?; // FIELD_FUTSETTDATE
    }
    if flds.account.is_some() {
        let val = flds.account.as_ref().unwrap();

        write!(output, "1={}\u{01}", val )?; // FIELD_ACCOUNT
    }

    Ok( () )
}



fn write_group_no_execs8_fields( group: &Vec<NoExecs8Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOEXECS, len )?;

    for g in group {
        write_group_no_execs8_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_execs8_fields_line( flds: &NoExecs8Fields, output: &mut Write) -> Result<(), io::Error> {

    if flds.last_shares.is_some() {
        let val = flds.last_shares.as_ref().unwrap();

        write!(output, "32={}\u{01}", val )?; // FIELD_LASTSHARES
    }
    if flds.exec_id.is_some() {
        let val = flds.exec_id.as_ref().unwrap();

        write!(output, "17={}\u{01}", val )?; // FIELD_EXECID
    }
    if flds.last_px.is_some() {
        let val = flds.last_px.as_ref().unwrap();

        write!(output, "31={}\u{01}", val )?; // FIELD_LASTPX
    }
    if flds.last_capacity.is_some() {
        let val = flds.last_capacity.as_ref().unwrap();

        write!(output, "29={}\u{01}", val )?; // FIELD_LASTCAPACITY
    }

    Ok( () )
}



fn write_group_no_contra_brokers7_fields( group: &Vec<NoContraBrokers7Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOCONTRABROKERS, len )?;

    for g in group {
        write_group_no_contra_brokers7_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_contra_brokers7_fields_line( flds: &NoContraBrokers7Fields, output: &mut Write) -> Result<(), io::Error> {

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



fn write_group_no_quote_entries19_fields( group: &Vec<NoQuoteEntries19Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOQUOTEENTRIES, len )?;

    for g in group {
        write_group_no_quote_entries19_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_quote_entries19_fields_line( flds: &NoQuoteEntries19Fields, output: &mut Write) -> Result<(), io::Error> {

    if flds.quote_entry_id.is_some() {
        let val = flds.quote_entry_id.as_ref().unwrap();

        write!(output, "299={}\u{01}", val )?; // FIELD_QUOTEENTRYID
    }
    if flds.symbol.is_some() {
        let val = flds.symbol.as_ref().unwrap();

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
    if flds.quote_entry_reject_reason.is_some() {
        let val = flds.quote_entry_reject_reason.as_ref().unwrap();

        write!(output, "368={}\u{01}", val )?; // FIELD_QUOTEENTRYREJECTREASON
    }

    Ok( () )
}



fn write_group_no_trading_sessions29_fields( group: &Vec<NoTradingSessions29Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOTRADINGSESSIONS, len )?;

    for g in group {
        write_group_no_trading_sessions29_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_trading_sessions29_fields_line( flds: &NoTradingSessions29Fields, output: &mut Write) -> Result<(), io::Error> {

    if flds.trading_session_id.is_some() {
        let val = flds.trading_session_id.as_ref().unwrap();

        write!(output, "336={}\u{01}", val )?; // FIELD_TRADINGSESSIONID
    }

    Ok( () )
}



fn write_group_no_ioiqualifiers9_fields( group: &Vec<NoIOIQualifiers9Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOIOIQUALIFIERS, len )?;

    for g in group {
        write_group_no_ioiqualifiers9_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_ioiqualifiers9_fields_line( flds: &NoIOIQualifiers9Fields, output: &mut Write) -> Result<(), io::Error> {

    if flds.ioiqualifier.is_some() {
        let val = flds.ioiqualifier.as_ref().unwrap();

        write!(output, "104={}\u{01}", val )?; // FIELD_IOIQUALIFIER
    }

    Ok( () )
}



fn write_group_no_msg_types14_fields( group: &Vec<NoMsgTypes14Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOMSGTYPES, len )?;

    for g in group {
        write_group_no_msg_types14_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_msg_types14_fields_line( flds: &NoMsgTypes14Fields, output: &mut Write) -> Result<(), io::Error> {

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



fn write_group_lines_of_text1_fields( group: &Vec<LinesOfText1Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_LINESOFTEXT, len )?;

    for g in group {
        write_group_lines_of_text1_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_lines_of_text1_fields_line( flds: &LinesOfText1Fields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.text;

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



fn write_group_no_allocs2_fields( group: &Vec<NoAllocs2Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOALLOCS, len )?;

    for g in group {
        write_group_no_allocs2_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_allocs2_fields_line( flds: &NoAllocs2Fields, output: &mut Write) -> Result<(), io::Error> {

    if flds.alloc_account.is_some() {
        let val = flds.alloc_account.as_ref().unwrap();

        write!(output, "79={}\u{01}", val )?; // FIELD_ALLOCACCOUNT
    }
    if flds.alloc_price.is_some() {
        let val = flds.alloc_price.as_ref().unwrap();

        write!(output, "366={}\u{01}", val )?; // FIELD_ALLOCPRICE
    }
    {
        let val = &flds.alloc_shares;

        write!(output, "80={}\u{01}", val )?; // FIELD_ALLOCSHARES
    }
    if flds.process_code.is_some() {
        let val = flds.process_code.as_ref().unwrap();

        write!(output, "81={}\u{01}", val )?; // FIELD_PROCESSCODE
    }
    if flds.broker_of_credit.is_some() {
        let val = flds.broker_of_credit.as_ref().unwrap();

        write!(output, "92={}\u{01}", val )?; // FIELD_BROKEROFCREDIT
    }
    if flds.notify_broker_of_credit.is_some() {
        let val = flds.notify_broker_of_credit.as_ref().unwrap();

        write!(output, "208={}\u{01}", to_boolconv(val) )?; // FIELD_NOTIFYBROKEROFCREDIT
    }
    if flds.alloc_handl_inst.is_some() {
        let val = flds.alloc_handl_inst.as_ref().unwrap();

        write!(output, "209={}\u{01}", val )?; // FIELD_ALLOCHANDLINST
    }
    if flds.alloc_text.is_some() {
        let val = flds.alloc_text.as_ref().unwrap();

        write!(output, "161={}\u{01}", val )?; // FIELD_ALLOCTEXT
    }
    if flds.encoded_alloc_text_len.is_some() {
        let val = flds.encoded_alloc_text_len.as_ref().unwrap();

        write!(output, "360={}\u{01}", val )?; // FIELD_ENCODEDALLOCTEXTLEN
    }
    if flds.encoded_alloc_text.is_some() {
        let val = flds.encoded_alloc_text.as_ref().unwrap();

        write!(output, "361={}\u{01}", val )?; // FIELD_ENCODEDALLOCTEXT
    }
    if flds.exec_broker.is_some() {
        let val = flds.exec_broker.as_ref().unwrap();

        write!(output, "76={}\u{01}", val )?; // FIELD_EXECBROKER
    }
    if flds.client_id.is_some() {
        let val = flds.client_id.as_ref().unwrap();

        write!(output, "109={}\u{01}", val )?; // FIELD_CLIENTID
    }
    if flds.commission.is_some() {
        let val = flds.commission.as_ref().unwrap();

        write!(output, "12={}\u{01}", val )?; // FIELD_COMMISSION
    }
    if flds.comm_type.is_some() {
        let val = flds.comm_type.as_ref().unwrap();

        write!(output, "13={}\u{01}", val )?; // FIELD_COMMTYPE
    }
    if flds.alloc_avg_px.is_some() {
        let val = flds.alloc_avg_px.as_ref().unwrap();

        write!(output, "153={}\u{01}", val )?; // FIELD_ALLOCAVGPX
    }
    if flds.alloc_net_money.is_some() {
        let val = flds.alloc_net_money.as_ref().unwrap();

        write!(output, "154={}\u{01}", val )?; // FIELD_ALLOCNETMONEY
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
    if flds.accrued_interest_amt.is_some() {
        let val = flds.accrued_interest_amt.as_ref().unwrap();

        write!(output, "159={}\u{01}", val )?; // FIELD_ACCRUEDINTERESTAMT
    }
    if flds.settl_inst_mode.is_some() {
        let val = flds.settl_inst_mode.as_ref().unwrap();

        write!(output, "160={}\u{01}", val )?; // FIELD_SETTLINSTMODE
    }
    if flds.no_misc_fees.is_some() {
        let val = flds.no_misc_fees.as_ref().unwrap();

        write_group_no_misc_fees13_fields( val, output )?;
    }

    Ok( () )
}



fn write_group_no_orders17_fields( group: &Vec<NoOrders17Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOORDERS, len )?;

    for g in group {
        write_group_no_orders17_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_orders17_fields_line( flds: &NoOrders17Fields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.cl_ord_id;

        write!(output, "11={}\u{01}", val )?; // FIELD_CLORDID
    }
    {
        let val = &flds.cum_qty;

        write!(output, "14={}\u{01}", val )?; // FIELD_CUMQTY
    }
    {
        let val = &flds.ord_status;

        write!(output, "39={}\u{01}", val )?; // FIELD_ORDSTATUS
    }
    {
        let val = &flds.leaves_qty;

        write!(output, "151={}\u{01}", val )?; // FIELD_LEAVESQTY
    }
    {
        let val = &flds.cxl_qty;

        write!(output, "84={}\u{01}", val )?; // FIELD_CXLQTY
    }
    {
        let val = &flds.avg_px;

        write!(output, "6={}\u{01}", val )?; // FIELD_AVGPX
    }
    if flds.ord_rej_reason.is_some() {
        let val = flds.ord_rej_reason.as_ref().unwrap();

        write!(output, "103={}\u{01}", val )?; // FIELD_ORDREJREASON
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



fn write_group_no_strikes28_fields( group: &Vec<NoStrikes28Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOSTRIKES, len )?;

    for g in group {
        write_group_no_strikes28_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_strikes28_fields_line( flds: &NoStrikes28Fields, output: &mut Write) -> Result<(), io::Error> {

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
    if flds.cl_ord_id.is_some() {
        let val = flds.cl_ord_id.as_ref().unwrap();

        write!(output, "11={}\u{01}", val )?; // FIELD_CLORDID
    }
    if flds.side.is_some() {
        let val = flds.side.as_ref().unwrap();

        write!(output, "54={}\u{01}", val )?; // FIELD_SIDE
    }
    {
        let val = &flds.price;

        write!(output, "44={}\u{01}", val )?; // FIELD_PRICE
    }
    if flds.currency.is_some() {
        let val = flds.currency.as_ref().unwrap();

        write!(output, "15={}\u{01}", val )?; // FIELD_CURRENCY
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



fn write_group_no_orders16_fields( group: &Vec<NoOrders16Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOORDERS, len )?;

    for g in group {
        write_group_no_orders16_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_orders16_fields_line( flds: &NoOrders16Fields, output: &mut Write) -> Result<(), io::Error> {

    if flds.cl_ord_id.is_some() {
        let val = flds.cl_ord_id.as_ref().unwrap();

        write!(output, "11={}\u{01}", val )?; // FIELD_CLORDID
    }
    if flds.order_id.is_some() {
        let val = flds.order_id.as_ref().unwrap();

        write!(output, "37={}\u{01}", val )?; // FIELD_ORDERID
    }
    if flds.secondary_order_id.is_some() {
        let val = flds.secondary_order_id.as_ref().unwrap();

        write!(output, "198={}\u{01}", val )?; // FIELD_SECONDARYORDERID
    }
    if flds.list_id.is_some() {
        let val = flds.list_id.as_ref().unwrap();

        write!(output, "66={}\u{01}", val )?; // FIELD_LISTID
    }
    if flds.wave_no.is_some() {
        let val = flds.wave_no.as_ref().unwrap();

        write!(output, "105={}\u{01}", val )?; // FIELD_WAVENO
    }

    Ok( () )
}



fn write_group_no_related_sym25_fields( group: &Vec<NoRelatedSym25Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NORELATEDSYM, len )?;

    for g in group {
        write_group_no_related_sym25_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_related_sym25_fields_line( flds: &NoRelatedSym25Fields, output: &mut Write) -> Result<(), io::Error> {

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
    if flds.quote_request_type.is_some() {
        let val = flds.quote_request_type.as_ref().unwrap();

        write!(output, "303={}\u{01}", val )?; // FIELD_QUOTEREQUESTTYPE
    }
    if flds.trading_session_id.is_some() {
        let val = flds.trading_session_id.as_ref().unwrap();

        write!(output, "336={}\u{01}", val )?; // FIELD_TRADINGSESSIONID
    }
    if flds.side.is_some() {
        let val = flds.side.as_ref().unwrap();

        write!(output, "54={}\u{01}", val )?; // FIELD_SIDE
    }
    if flds.order_qty.is_some() {
        let val = flds.order_qty.as_ref().unwrap();

        write!(output, "38={}\u{01}", val )?; // FIELD_ORDERQTY
    }
    if flds.fut_sett_date.is_some() {
        let val = flds.fut_sett_date.as_ref().unwrap();

        write!(output, "64={}\u{01}", val )?; // FIELD_FUTSETTDATE
    }
    if flds.ord_type.is_some() {
        let val = flds.ord_type.as_ref().unwrap();

        write!(output, "40={}\u{01}", val )?; // FIELD_ORDTYPE
    }
    if flds.fut_sett_date2.is_some() {
        let val = flds.fut_sett_date2.as_ref().unwrap();

        write!(output, "193={}\u{01}", val )?; // FIELD_FUTSETTDATE2
    }
    if flds.order_qty2.is_some() {
        let val = flds.order_qty2.as_ref().unwrap();

        write!(output, "192={}\u{01}", val )?; // FIELD_ORDERQTY2
    }
    if flds.expire_time.is_some() {
        let val = flds.expire_time.as_ref().unwrap();

        write!(output, "126={}\u{01}", val )?; // FIELD_EXPIRETIME
    }
    if flds.transact_time.is_some() {
        let val = flds.transact_time.as_ref().unwrap();

        write!(output, "60={}\u{01}", val )?; // FIELD_TRANSACTTIME
    }
    if flds.currency.is_some() {
        let val = flds.currency.as_ref().unwrap();

        write!(output, "15={}\u{01}", val )?; // FIELD_CURRENCY
    }

    Ok( () )
}



fn write_group_no_quote_sets22_fields( group: &Vec<NoQuoteSets22Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOQUOTESETS, len )?;

    for g in group {
        write_group_no_quote_sets22_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_quote_sets22_fields_line( flds: &NoQuoteSets22Fields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.quote_set_id;

        write!(output, "302={}\u{01}", val )?; // FIELD_QUOTESETID
    }
    {
        let val = &flds.underlying_symbol;

        write!(output, "311={}\u{01}", val )?; // FIELD_UNDERLYINGSYMBOL
    }
    if flds.underlying_symbol_sfx.is_some() {
        let val = flds.underlying_symbol_sfx.as_ref().unwrap();

        write!(output, "312={}\u{01}", val )?; // FIELD_UNDERLYINGSYMBOLSFX
    }
    if flds.underlying_security_id.is_some() {
        let val = flds.underlying_security_id.as_ref().unwrap();

        write!(output, "309={}\u{01}", val )?; // FIELD_UNDERLYINGSECURITYID
    }
    if flds.underlying_idsource.is_some() {
        let val = flds.underlying_idsource.as_ref().unwrap();

        write!(output, "305={}\u{01}", val )?; // FIELD_UNDERLYINGIDSOURCE
    }
    if flds.underlying_security_type.is_some() {
        let val = flds.underlying_security_type.as_ref().unwrap();

        write!(output, "310={}\u{01}", val )?; // FIELD_UNDERLYINGSECURITYTYPE
    }
    if flds.underlying_maturity_month_year.is_some() {
        let val = flds.underlying_maturity_month_year.as_ref().unwrap();

        write!(output, "313={}\u{01}", val )?; // FIELD_UNDERLYINGMATURITYMONTHYEAR
    }
    if flds.underlying_maturity_day.is_some() {
        let val = flds.underlying_maturity_day.as_ref().unwrap();

        write!(output, "314={}\u{01}", val )?; // FIELD_UNDERLYINGMATURITYDAY
    }
    if flds.underlying_put_or_call.is_some() {
        let val = flds.underlying_put_or_call.as_ref().unwrap();

        write!(output, "315={}\u{01}", val )?; // FIELD_UNDERLYINGPUTORCALL
    }
    if flds.underlying_strike_price.is_some() {
        let val = flds.underlying_strike_price.as_ref().unwrap();

        write!(output, "316={}\u{01}", val )?; // FIELD_UNDERLYINGSTRIKEPRICE
    }
    if flds.underlying_opt_attribute.is_some() {
        let val = flds.underlying_opt_attribute.as_ref().unwrap();

        write!(output, "317={}\u{01}", val )?; // FIELD_UNDERLYINGOPTATTRIBUTE
    }
    if flds.underlying_contract_multiplier.is_some() {
        let val = flds.underlying_contract_multiplier.as_ref().unwrap();

        write!(output, "436={}\u{01}", val )?; // FIELD_UNDERLYINGCONTRACTMULTIPLIER
    }
    if flds.underlying_coupon_rate.is_some() {
        let val = flds.underlying_coupon_rate.as_ref().unwrap();

        write!(output, "435={}\u{01}", val )?; // FIELD_UNDERLYINGCOUPONRATE
    }
    if flds.underlying_security_exchange.is_some() {
        let val = flds.underlying_security_exchange.as_ref().unwrap();

        write!(output, "308={}\u{01}", val )?; // FIELD_UNDERLYINGSECURITYEXCHANGE
    }
    if flds.underlying_issuer.is_some() {
        let val = flds.underlying_issuer.as_ref().unwrap();

        write!(output, "306={}\u{01}", val )?; // FIELD_UNDERLYINGISSUER
    }
    if flds.encoded_underlying_issuer_len.is_some() {
        let val = flds.encoded_underlying_issuer_len.as_ref().unwrap();

        write!(output, "362={}\u{01}", val )?; // FIELD_ENCODEDUNDERLYINGISSUERLEN
    }
    if flds.encoded_underlying_issuer.is_some() {
        let val = flds.encoded_underlying_issuer.as_ref().unwrap();

        write!(output, "363={}\u{01}", val )?; // FIELD_ENCODEDUNDERLYINGISSUER
    }
    if flds.underlying_security_desc.is_some() {
        let val = flds.underlying_security_desc.as_ref().unwrap();

        write!(output, "307={}\u{01}", val )?; // FIELD_UNDERLYINGSECURITYDESC
    }
    if flds.encoded_underlying_security_desc_len.is_some() {
        let val = flds.encoded_underlying_security_desc_len.as_ref().unwrap();

        write!(output, "364={}\u{01}", val )?; // FIELD_ENCODEDUNDERLYINGSECURITYDESCLEN
    }
    if flds.encoded_underlying_security_desc.is_some() {
        let val = flds.encoded_underlying_security_desc.as_ref().unwrap();

        write!(output, "365={}\u{01}", val )?; // FIELD_ENCODEDUNDERLYINGSECURITYDESC
    }
    if flds.quote_set_valid_until_time.is_some() {
        let val = flds.quote_set_valid_until_time.as_ref().unwrap();

        write!(output, "367={}\u{01}", val )?; // FIELD_QUOTESETVALIDUNTILTIME
    }
    {
        let val = &flds.tot_quote_entries;

        write!(output, "304={}\u{01}", val )?; // FIELD_TOTQUOTEENTRIES
    }
    {
        let val = &flds.no_quote_entries;

        write_group_no_quote_entries20_fields( val, output )?;
    }

    Ok( () )
}



fn write_group_no_related_sym24_fields( group: &Vec<NoRelatedSym24Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NORELATEDSYM, len )?;

    for g in group {
        write_group_no_related_sym24_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_related_sym24_fields_line( flds: &NoRelatedSym24Fields, output: &mut Write) -> Result<(), io::Error> {

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
    if flds.trading_session_id.is_some() {
        let val = flds.trading_session_id.as_ref().unwrap();

        write!(output, "336={}\u{01}", val )?; // FIELD_TRADINGSESSIONID
    }

    Ok( () )
}



fn write_group_no_bid_components5_fields( group: &Vec<NoBidComponents5Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOBIDCOMPONENTS, len )?;

    for g in group {
        write_group_no_bid_components5_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_bid_components5_fields_line( flds: &NoBidComponents5Fields, output: &mut Write) -> Result<(), io::Error> {

    {
        let val = &flds.commission;

        write!(output, "12={}\u{01}", val )?; // FIELD_COMMISSION
    }
    {
        let val = &flds.comm_type;

        write!(output, "13={}\u{01}", val )?; // FIELD_COMMTYPE
    }
    if flds.list_id.is_some() {
        let val = flds.list_id.as_ref().unwrap();

        write!(output, "66={}\u{01}", val )?; // FIELD_LISTID
    }
    if flds.country.is_some() {
        let val = flds.country.as_ref().unwrap();

        write!(output, "421={}\u{01}", val )?; // FIELD_COUNTRY
    }
    if flds.side.is_some() {
        let val = flds.side.as_ref().unwrap();

        write!(output, "54={}\u{01}", val )?; // FIELD_SIDE
    }
    if flds.price.is_some() {
        let val = flds.price.as_ref().unwrap();

        write!(output, "44={}\u{01}", val )?; // FIELD_PRICE
    }
    if flds.price_type.is_some() {
        let val = flds.price_type.as_ref().unwrap();

        write!(output, "423={}\u{01}", val )?; // FIELD_PRICETYPE
    }
    if flds.fair_value.is_some() {
        let val = flds.fair_value.as_ref().unwrap();

        write!(output, "406={}\u{01}", val )?; // FIELD_FAIRVALUE
    }
    if flds.net_gross_ind.is_some() {
        let val = flds.net_gross_ind.as_ref().unwrap();

        write!(output, "430={}\u{01}", val )?; // FIELD_NETGROSSIND
    }
    if flds.settlmnt_typ.is_some() {
        let val = flds.settlmnt_typ.as_ref().unwrap();

        write!(output, "63={}\u{01}", val )?; // FIELD_SETTLMNTTYP
    }
    if flds.fut_sett_date.is_some() {
        let val = flds.fut_sett_date.as_ref().unwrap();

        write!(output, "64={}\u{01}", val )?; // FIELD_FUTSETTDATE
    }
    if flds.trading_session_id.is_some() {
        let val = flds.trading_session_id.as_ref().unwrap();

        write!(output, "336={}\u{01}", val )?; // FIELD_TRADINGSESSIONID
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



fn write_group_no_orders15_fields( group: &Vec<NoOrders15Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOORDERS, len )?;

    for g in group {
        write_group_no_orders15_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_orders15_fields_line( flds: &NoOrders15Fields, output: &mut Write) -> Result<(), io::Error> {

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

        write_group_no_allocs3_fields( val, output )?;
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

        write_group_no_trading_sessions29_fields( val, output )?;
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



fn write_group_no_related_sym26_fields( group: &Vec<NoRelatedSym26Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NORELATEDSYM, len )?;

    for g in group {
        write_group_no_related_sym26_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_related_sym26_fields_line( flds: &NoRelatedSym26Fields, output: &mut Write) -> Result<(), io::Error> {

    if flds.relatd_sym.is_some() {
        let val = flds.relatd_sym.as_ref().unwrap();

        write!(output, "46={}\u{01}", val )?; // FIELD_RELATDSYM
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

    Ok( () )
}



fn write_group_no_bid_descriptors6_fields( group: &Vec<NoBidDescriptors6Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOBIDDESCRIPTORS, len )?;

    for g in group {
        write_group_no_bid_descriptors6_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_bid_descriptors6_fields_line( flds: &NoBidDescriptors6Fields, output: &mut Write) -> Result<(), io::Error> {

    if flds.bid_descriptor_type.is_some() {
        let val = flds.bid_descriptor_type.as_ref().unwrap();

        write!(output, "399={}\u{01}", val )?; // FIELD_BIDDESCRIPTORTYPE
    }
    if flds.bid_descriptor.is_some() {
        let val = flds.bid_descriptor.as_ref().unwrap();

        write!(output, "400={}\u{01}", val )?; // FIELD_BIDDESCRIPTOR
    }
    if flds.side_value_ind.is_some() {
        let val = flds.side_value_ind.as_ref().unwrap();

        write!(output, "401={}\u{01}", val )?; // FIELD_SIDEVALUEIND
    }
    if flds.liquidity_value.is_some() {
        let val = flds.liquidity_value.as_ref().unwrap();

        write!(output, "404={}\u{01}", val )?; // FIELD_LIQUIDITYVALUE
    }
    if flds.liquidity_num_securities.is_some() {
        let val = flds.liquidity_num_securities.as_ref().unwrap();

        write!(output, "441={}\u{01}", val )?; // FIELD_LIQUIDITYNUMSECURITIES
    }
    if flds.liquidity_pct_low.is_some() {
        let val = flds.liquidity_pct_low.as_ref().unwrap();

        write!(output, "402={}\u{01}", val )?; // FIELD_LIQUIDITYPCTLOW
    }
    if flds.liquidity_pct_high.is_some() {
        let val = flds.liquidity_pct_high.as_ref().unwrap();

        write!(output, "403={}\u{01}", val )?; // FIELD_LIQUIDITYPCTHIGH
    }
    if flds.efptracking_error.is_some() {
        let val = flds.efptracking_error.as_ref().unwrap();

        write!(output, "405={}\u{01}", val )?; // FIELD_EFPTRACKINGERROR
    }
    if flds.fair_value.is_some() {
        let val = flds.fair_value.as_ref().unwrap();

        write!(output, "406={}\u{01}", val )?; // FIELD_FAIRVALUE
    }
    if flds.outside_index_pct.is_some() {
        let val = flds.outside_index_pct.as_ref().unwrap();

        write!(output, "407={}\u{01}", val )?; // FIELD_OUTSIDEINDEXPCT
    }
    if flds.value_of_futures.is_some() {
        let val = flds.value_of_futures.as_ref().unwrap();

        write!(output, "408={}\u{01}", val )?; // FIELD_VALUEOFFUTURES
    }

    Ok( () )
}



fn write_group_no_quote_sets21_fields( group: &Vec<NoQuoteSets21Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOQUOTESETS, len )?;

    for g in group {
        write_group_no_quote_sets21_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_quote_sets21_fields_line( flds: &NoQuoteSets21Fields, output: &mut Write) -> Result<(), io::Error> {

    if flds.quote_set_id.is_some() {
        let val = flds.quote_set_id.as_ref().unwrap();

        write!(output, "302={}\u{01}", val )?; // FIELD_QUOTESETID
    }
    if flds.underlying_symbol.is_some() {
        let val = flds.underlying_symbol.as_ref().unwrap();

        write!(output, "311={}\u{01}", val )?; // FIELD_UNDERLYINGSYMBOL
    }
    if flds.underlying_symbol_sfx.is_some() {
        let val = flds.underlying_symbol_sfx.as_ref().unwrap();

        write!(output, "312={}\u{01}", val )?; // FIELD_UNDERLYINGSYMBOLSFX
    }
    if flds.underlying_security_id.is_some() {
        let val = flds.underlying_security_id.as_ref().unwrap();

        write!(output, "309={}\u{01}", val )?; // FIELD_UNDERLYINGSECURITYID
    }
    if flds.underlying_idsource.is_some() {
        let val = flds.underlying_idsource.as_ref().unwrap();

        write!(output, "305={}\u{01}", val )?; // FIELD_UNDERLYINGIDSOURCE
    }
    if flds.underlying_security_type.is_some() {
        let val = flds.underlying_security_type.as_ref().unwrap();

        write!(output, "310={}\u{01}", val )?; // FIELD_UNDERLYINGSECURITYTYPE
    }
    if flds.underlying_maturity_month_year.is_some() {
        let val = flds.underlying_maturity_month_year.as_ref().unwrap();

        write!(output, "313={}\u{01}", val )?; // FIELD_UNDERLYINGMATURITYMONTHYEAR
    }
    if flds.underlying_maturity_day.is_some() {
        let val = flds.underlying_maturity_day.as_ref().unwrap();

        write!(output, "314={}\u{01}", val )?; // FIELD_UNDERLYINGMATURITYDAY
    }
    if flds.underlying_put_or_call.is_some() {
        let val = flds.underlying_put_or_call.as_ref().unwrap();

        write!(output, "315={}\u{01}", val )?; // FIELD_UNDERLYINGPUTORCALL
    }
    if flds.underlying_strike_price.is_some() {
        let val = flds.underlying_strike_price.as_ref().unwrap();

        write!(output, "316={}\u{01}", val )?; // FIELD_UNDERLYINGSTRIKEPRICE
    }
    if flds.underlying_opt_attribute.is_some() {
        let val = flds.underlying_opt_attribute.as_ref().unwrap();

        write!(output, "317={}\u{01}", val )?; // FIELD_UNDERLYINGOPTATTRIBUTE
    }
    if flds.underlying_contract_multiplier.is_some() {
        let val = flds.underlying_contract_multiplier.as_ref().unwrap();

        write!(output, "436={}\u{01}", val )?; // FIELD_UNDERLYINGCONTRACTMULTIPLIER
    }
    if flds.underlying_coupon_rate.is_some() {
        let val = flds.underlying_coupon_rate.as_ref().unwrap();

        write!(output, "435={}\u{01}", val )?; // FIELD_UNDERLYINGCOUPONRATE
    }
    if flds.underlying_security_exchange.is_some() {
        let val = flds.underlying_security_exchange.as_ref().unwrap();

        write!(output, "308={}\u{01}", val )?; // FIELD_UNDERLYINGSECURITYEXCHANGE
    }
    if flds.underlying_issuer.is_some() {
        let val = flds.underlying_issuer.as_ref().unwrap();

        write!(output, "306={}\u{01}", val )?; // FIELD_UNDERLYINGISSUER
    }
    if flds.encoded_underlying_issuer_len.is_some() {
        let val = flds.encoded_underlying_issuer_len.as_ref().unwrap();

        write!(output, "362={}\u{01}", val )?; // FIELD_ENCODEDUNDERLYINGISSUERLEN
    }
    if flds.encoded_underlying_issuer.is_some() {
        let val = flds.encoded_underlying_issuer.as_ref().unwrap();

        write!(output, "363={}\u{01}", val )?; // FIELD_ENCODEDUNDERLYINGISSUER
    }
    if flds.underlying_security_desc.is_some() {
        let val = flds.underlying_security_desc.as_ref().unwrap();

        write!(output, "307={}\u{01}", val )?; // FIELD_UNDERLYINGSECURITYDESC
    }
    if flds.encoded_underlying_security_desc_len.is_some() {
        let val = flds.encoded_underlying_security_desc_len.as_ref().unwrap();

        write!(output, "364={}\u{01}", val )?; // FIELD_ENCODEDUNDERLYINGSECURITYDESCLEN
    }
    if flds.encoded_underlying_security_desc.is_some() {
        let val = flds.encoded_underlying_security_desc.as_ref().unwrap();

        write!(output, "365={}\u{01}", val )?; // FIELD_ENCODEDUNDERLYINGSECURITYDESC
    }
    if flds.tot_quote_entries.is_some() {
        let val = flds.tot_quote_entries.as_ref().unwrap();

        write!(output, "304={}\u{01}", val )?; // FIELD_TOTQUOTEENTRIES
    }
    if flds.no_quote_entries.is_some() {
        let val = flds.no_quote_entries.as_ref().unwrap();

        write_group_no_quote_entries19_fields( val, output )?;
    }

    Ok( () )
}



fn write_group_no_routing_ids27_fields( group: &Vec<NoRoutingIDs27Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOROUTINGIDS, len )?;

    for g in group {
        write_group_no_routing_ids27_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_routing_ids27_fields_line( flds: &NoRoutingIDs27Fields, output: &mut Write) -> Result<(), io::Error> {

    if flds.routing_type.is_some() {
        let val = flds.routing_type.as_ref().unwrap();

        write!(output, "216={}\u{01}", val )?; // FIELD_ROUTINGTYPE
    }
    if flds.routing_id.is_some() {
        let val = flds.routing_id.as_ref().unwrap();

        write!(output, "217={}\u{01}", val )?; // FIELD_ROUTINGID
    }

    Ok( () )
}



fn write_group_no_misc_fees13_fields( group: &Vec<NoMiscFees13Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOMISCFEES, len )?;

    for g in group {
        write_group_no_misc_fees13_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_misc_fees13_fields_line( flds: &NoMiscFees13Fields, output: &mut Write) -> Result<(), io::Error> {

    if flds.misc_fee_amt.is_some() {
        let val = flds.misc_fee_amt.as_ref().unwrap();

        write!(output, "137={}\u{01}", val )?; // FIELD_MISCFEEAMT
    }
    if flds.misc_fee_curr.is_some() {
        let val = flds.misc_fee_curr.as_ref().unwrap();

        write!(output, "138={}\u{01}", val )?; // FIELD_MISCFEECURR
    }
    if flds.misc_fee_type.is_some() {
        let val = flds.misc_fee_type.as_ref().unwrap();

        write!(output, "139={}\u{01}", val )?; // FIELD_MISCFEETYPE
    }

    Ok( () )
}









