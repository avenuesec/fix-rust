// Generated file.
// Structs, enums to support parsing/generation of fix messages

#![ignore(unused_imports)]
#![ignore(unused_variables)]
#![ignore(dead_code)]

use std::str::{FromStr};
use std::{io, str, char, i32};
use std::default::{Default};

use chrono::prelude::*;  // DateTime


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
pub struct IOIFields {
    pub ioiid : String, // FIELD_IOIID 23
    pub ioitrans_type : FieldIOITransTypeEnum, // FIELD_IOITRANSTYPE 28
    pub ioiref_id : Option<String>, // FIELD_IOIREFID 26
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
    pub ioishares : FieldIOISharesEnum, // FIELD_IOISHARES 27
    pub price : Option<f32>, // FIELD_PRICE 44
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub valid_until_time : Option<UtcDateTime>, // FIELD_VALIDUNTILTIME 62
    pub ioiqlty_ind : Option<FieldIOIQltyIndEnum>, // FIELD_IOIQLTYIND 25
    pub ioinatural_flag : Option<bool>, // FIELD_IOINATURALFLAG 130
    pub no_ioiqualifiers : Option<Vec<NoIOIQualifiers9Fields>>, // FIELD_NOIOIQUALIFIERS 0
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355
    pub transact_time : Option<UtcDateTime>, // FIELD_TRANSACTTIME 60
    pub urllink : Option<String>, // FIELD_URLLINK 149
    pub no_routing_ids : Option<Vec<NoRoutingIDs27Fields>>, // FIELD_NOROUTINGIDS 0
    pub spread_to_benchmark : Option<f32>, // FIELD_SPREADTOBENCHMARK 218
    pub benchmark : Option<FieldBenchmarkEnum>, // FIELD_BENCHMARK 219

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
    pub no_contra_brokers : Option<Vec<NoContraBrokers7Fields>>, // FIELD_NOCONTRABROKERS 0
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
    pub no_msg_types : Option<Vec<NoMsgTypes14Fields>>, // FIELD_NOMSGTYPES 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NewsFields {
    pub orig_time : Option<UtcDateTime>, // FIELD_ORIGTIME 42
    pub urgency : Option<FieldUrgencyEnum>, // FIELD_URGENCY 61
    pub headline : String, // FIELD_HEADLINE 148
    pub encoded_headline_len : Option<usize>, // FIELD_ENCODEDHEADLINELEN 358
    pub encoded_headline : Option<String>, // FIELD_ENCODEDHEADLINE 359
    pub no_routing_ids : Option<Vec<NoRoutingIDs27Fields>>, // FIELD_NOROUTINGIDS 0
    pub no_related_sym : Option<Vec<NoRelatedSym25Fields>>, // FIELD_NORELATEDSYM 0
    pub lines_of_text : Vec<LinesOfText1Fields>, // FIELD_LINESOFTEXT 0
    pub urllink : Option<String>, // FIELD_URLLINK 149
    pub raw_data_length : Option<usize>, // FIELD_RAWDATALENGTH 95
    pub raw_data : Option<String>, // FIELD_RAWDATA 96

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct EmailFields {
    pub email_thread_id : String, // FIELD_EMAILTHREADID 164
    pub email_type : FieldEmailTypeEnum, // FIELD_EMAILTYPE 94
    pub orig_time : Option<UtcDateTime>, // FIELD_ORIGTIME 42
    pub subject : String, // FIELD_SUBJECT 147
    pub encoded_subject_len : Option<usize>, // FIELD_ENCODEDSUBJECTLEN 356
    pub encoded_subject : Option<String>, // FIELD_ENCODEDSUBJECT 357
    pub no_routing_ids : Option<Vec<NoRoutingIDs27Fields>>, // FIELD_NOROUTINGIDS 0
    pub no_related_sym : Option<Vec<NoRelatedSym25Fields>>, // FIELD_NORELATEDSYM 0
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
    pub no_allocs : Option<Vec<NoAllocs2Fields>>, // FIELD_NOALLOCS 0
    pub settlmnt_typ : Option<FieldSettlmntTypEnum>, // FIELD_SETTLMNTTYP 63
    pub fut_sett_date : Option<UtcDateTime>, // FIELD_FUTSETTDATE 64
    pub handl_inst : FieldHandlInstEnum, // FIELD_HANDLINST 21
    pub exec_inst : Option<FieldExecInstEnum>, // FIELD_EXECINST 18
    pub min_qty : Option<f32>, // FIELD_MINQTY 110
    pub max_floor : Option<f32>, // FIELD_MAXFLOOR 111
    pub ex_destination : Option<String>, // FIELD_EXDESTINATION 100
    pub no_trading_sessions : Option<Vec<NoTradingSessions29Fields>>, // FIELD_NOTRADINGSESSIONS 0
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
    pub no_allocs : Option<Vec<NoAllocs2Fields>>, // FIELD_NOALLOCS 0
    pub settlmnt_typ : Option<FieldSettlmntTypEnum>, // FIELD_SETTLMNTTYP 63
    pub fut_sett_date : Option<UtcDateTime>, // FIELD_FUTSETTDATE 64
    pub handl_inst : FieldHandlInstEnum, // FIELD_HANDLINST 21
    pub exec_inst : Option<FieldExecInstEnum>, // FIELD_EXECINST 18
    pub min_qty : Option<f32>, // FIELD_MINQTY 110
    pub max_floor : Option<f32>, // FIELD_MAXFLOOR 111
    pub ex_destination : Option<String>, // FIELD_EXDESTINATION 100
    pub no_trading_sessions : Option<Vec<NoTradingSessions29Fields>>, // FIELD_NOTRADINGSESSIONS 0
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



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct AllocationFields {
    pub alloc_id : String, // FIELD_ALLOCID 70
    pub alloc_trans_type : FieldAllocTransTypeEnum, // FIELD_ALLOCTRANSTYPE 71
    pub ref_alloc_id : Option<String>, // FIELD_REFALLOCID 72
    pub alloc_link_id : Option<String>, // FIELD_ALLOCLINKID 196
    pub alloc_link_type : Option<FieldAllocLinkTypeEnum>, // FIELD_ALLOCLINKTYPE 197
    pub no_orders : Option<Vec<NoOrders17Fields>>, // FIELD_NOORDERS 0
    pub no_execs : Option<Vec<NoExecs8Fields>>, // FIELD_NOEXECS 0
    pub side : FieldSideEnum, // FIELD_SIDE 54
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
    pub shares : f32, // FIELD_SHARES 53
    pub last_mkt : Option<String>, // FIELD_LASTMKT 30
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub avg_px : f32, // FIELD_AVGPX 6
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub avg_prx_precision : Option<i32>, // FIELD_AVGPRXPRECISION 74
    pub trade_date : UtcDateTime, // FIELD_TRADEDATE 75
    pub transact_time : Option<UtcDateTime>, // FIELD_TRANSACTTIME 60
    pub settlmnt_typ : Option<FieldSettlmntTypEnum>, // FIELD_SETTLMNTTYP 63
    pub fut_sett_date : Option<UtcDateTime>, // FIELD_FUTSETTDATE 64
    pub gross_trade_amt : Option<f32>, // FIELD_GROSSTRADEAMT 381
    pub net_money : Option<f32>, // FIELD_NETMONEY 118
    pub open_close : Option<FieldOpenCloseEnum>, // FIELD_OPENCLOSE 77
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355
    pub num_days_interest : Option<i32>, // FIELD_NUMDAYSINTEREST 157
    pub accrued_interest_rate : Option<f32>, // FIELD_ACCRUEDINTERESTRATE 158
    pub no_allocs : Option<Vec<NoAllocs3Fields>>, // FIELD_NOALLOCS 0

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
    pub no_orders : Vec<NoOrders16Fields>, // FIELD_NOORDERS 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct AllocationInstructionAckFields {
    pub client_id : Option<String>, // FIELD_CLIENTID 109
    pub exec_broker : Option<String>, // FIELD_EXECBROKER 76
    pub alloc_id : String, // FIELD_ALLOCID 70
    pub trade_date : UtcDateTime, // FIELD_TRADEDATE 75
    pub transact_time : Option<UtcDateTime>, // FIELD_TRANSACTTIME 60
    pub alloc_status : FieldAllocStatusEnum, // FIELD_ALLOCSTATUS 87
    pub alloc_rej_code : Option<FieldAllocRejCodeEnum>, // FIELD_ALLOCREJCODE 88
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct DontKnowTradeFields {
    pub order_id : String, // FIELD_ORDERID 37
    pub exec_id : String, // FIELD_EXECID 17
    pub dkreason : FieldDKReasonEnum, // FIELD_DKREASON 127
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
    pub last_shares : Option<f32>, // FIELD_LASTSHARES 32
    pub last_px : Option<f32>, // FIELD_LASTPX 31
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct QuoteRequestFields {
    pub quote_req_id : String, // FIELD_QUOTEREQID 131
    pub no_related_sym : Vec<NoRelatedSym24Fields>, // FIELD_NORELATEDSYM 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct QuoteFields {
    pub quote_req_id : Option<String>, // FIELD_QUOTEREQID 131
    pub quote_id : String, // FIELD_QUOTEID 117
    pub quote_response_level : Option<FieldQuoteResponseLevelEnum>, // FIELD_QUOTERESPONSELEVEL 301
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
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
    pub ord_type : Option<FieldOrdTypeEnum>, // FIELD_ORDTYPE 40
    pub fut_sett_date2 : Option<UtcDateTime>, // FIELD_FUTSETTDATE2 193
    pub order_qty2 : Option<f32>, // FIELD_ORDERQTY2 192
    pub currency : Option<f32>, // FIELD_CURRENCY 15

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct SettlementInstructionsFields {
    pub settl_inst_id : String, // FIELD_SETTLINSTID 162
    pub settl_inst_trans_type : FieldSettlInstTransTypeEnum, // FIELD_SETTLINSTTRANSTYPE 163
    pub settl_inst_ref_id : String, // FIELD_SETTLINSTREFID 214
    pub settl_inst_mode : FieldSettlInstModeEnum, // FIELD_SETTLINSTMODE 160
    pub settl_inst_source : FieldSettlInstSourceEnum, // FIELD_SETTLINSTSOURCE 165
    pub alloc_account : String, // FIELD_ALLOCACCOUNT 79
    pub settl_location : Option<FieldSettlLocationEnum>, // FIELD_SETTLLOCATION 166
    pub trade_date : Option<UtcDateTime>, // FIELD_TRADEDATE 75
    pub alloc_id : Option<String>, // FIELD_ALLOCID 70
    pub last_mkt : Option<String>, // FIELD_LASTMKT 30
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub side : Option<FieldSideEnum>, // FIELD_SIDE 54
    pub security_type : Option<FieldSecurityTypeEnum>, // FIELD_SECURITYTYPE 167
    pub effective_time : Option<UtcDateTime>, // FIELD_EFFECTIVETIME 168
    pub transact_time : UtcDateTime, // FIELD_TRANSACTTIME 60
    pub client_id : Option<String>, // FIELD_CLIENTID 109
    pub exec_broker : Option<String>, // FIELD_EXECBROKER 76
    pub stand_inst_db_type : Option<FieldStandInstDbTypeEnum>, // FIELD_STANDINSTDBTYPE 169
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
    pub subscription_request_type : FieldSubscriptionRequestTypeEnum, // FIELD_SUBSCRIPTIONREQUESTTYPE 263
    pub market_depth : i32, // FIELD_MARKETDEPTH 264
    pub mdupdate_type : Option<FieldMDUpdateTypeEnum>, // FIELD_MDUPDATETYPE 265
    pub aggregated_book : Option<bool>, // FIELD_AGGREGATEDBOOK 266
    pub no_mdentry_types : Vec<NoMDEntryTypes12Fields>, // FIELD_NOMDENTRYTYPES 0
    pub no_related_sym : Vec<NoRelatedSym23Fields>, // FIELD_NORELATEDSYM 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct MarketDataSnapshotFullRefreshFields {
    pub mdreq_id : Option<String>, // FIELD_MDREQID 262
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
    pub financial_status : Option<FieldFinancialStatusEnum>, // FIELD_FINANCIALSTATUS 291
    pub corporate_action : Option<FieldCorporateActionEnum>, // FIELD_CORPORATEACTION 292
    pub total_volume_traded : Option<f32>, // FIELD_TOTALVOLUMETRADED 387
    pub no_mdentries : Vec<NoMDEntries11Fields>, // FIELD_NOMDENTRIES 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct MarketDataIncrementalRefreshFields {
    pub mdreq_id : Option<String>, // FIELD_MDREQID 262
    pub no_mdentries : Vec<NoMDEntries10Fields>, // FIELD_NOMDENTRIES 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct MarketDataRequestRejectFields {
    pub mdreq_id : String, // FIELD_MDREQID 262
    pub mdreq_rej_reason : Option<FieldMDReqRejReasonEnum>, // FIELD_MDREQREJREASON 281
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct QuoteCancelFields {
    pub quote_req_id : Option<String>, // FIELD_QUOTEREQID 131
    pub quote_id : String, // FIELD_QUOTEID 117
    pub quote_cancel_type : FieldQuoteCancelTypeEnum, // FIELD_QUOTECANCELTYPE 298
    pub quote_response_level : Option<FieldQuoteResponseLevelEnum>, // FIELD_QUOTERESPONSELEVEL 301
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub no_quote_entries : Vec<NoQuoteEntries18Fields>, // FIELD_NOQUOTEENTRIES 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct QuoteStatusRequestFields {
    pub quote_id : Option<String>, // FIELD_QUOTEID 117
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
    pub side : Option<FieldSideEnum>, // FIELD_SIDE 54
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct QuoteAcknowledgementFields {
    pub quote_req_id : Option<String>, // FIELD_QUOTEREQID 131
    pub quote_id : Option<String>, // FIELD_QUOTEID 117
    pub quote_ack_status : FieldQuoteAckStatusEnum, // FIELD_QUOTEACKSTATUS 297
    pub quote_reject_reason : Option<FieldQuoteRejectReasonEnum>, // FIELD_QUOTEREJECTREASON 300
    pub quote_response_level : Option<FieldQuoteResponseLevelEnum>, // FIELD_QUOTERESPONSELEVEL 301
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub text : Option<String>, // FIELD_TEXT 58
    pub no_quote_sets : Option<Vec<NoQuoteSets22Fields>>, // FIELD_NOQUOTESETS 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct SecurityDefinitionRequestFields {
    pub security_req_id : String, // FIELD_SECURITYREQID 320
    pub security_request_type : FieldSecurityRequestTypeEnum, // FIELD_SECURITYREQUESTTYPE 321
    pub symbol : Option<String>, // FIELD_SYMBOL 55
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
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub no_related_sym : Option<Vec<NoRelatedSym26Fields>>, // FIELD_NORELATEDSYM 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct SecurityDefinitionFields {
    pub security_req_id : String, // FIELD_SECURITYREQID 320
    pub security_response_id : String, // FIELD_SECURITYRESPONSEID 322
    pub security_response_type : Option<FieldSecurityResponseTypeEnum>, // FIELD_SECURITYRESPONSETYPE 323
    pub total_num_securities : i32, // FIELD_TOTALNUMSECURITIES 393
    pub symbol : Option<String>, // FIELD_SYMBOL 55
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
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355
    pub no_related_sym : Option<Vec<NoRelatedSym26Fields>>, // FIELD_NORELATEDSYM 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct SecurityStatusRequestFields {
    pub security_status_req_id : String, // FIELD_SECURITYSTATUSREQID 324
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
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub subscription_request_type : FieldSubscriptionRequestTypeEnum, // FIELD_SUBSCRIPTIONREQUESTTYPE 263
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct SecurityStatusFields {
    pub security_status_req_id : Option<String>, // FIELD_SECURITYSTATUSREQID 324
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
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub unsolicited_indicator : Option<bool>, // FIELD_UNSOLICITEDINDICATOR 325
    pub security_trading_status : Option<FieldSecurityTradingStatusEnum>, // FIELD_SECURITYTRADINGSTATUS 326
    pub financial_status : Option<FieldFinancialStatusEnum>, // FIELD_FINANCIALSTATUS 291
    pub corporate_action : Option<FieldCorporateActionEnum>, // FIELD_CORPORATEACTION 292
    pub halt_reason_char : Option<FieldHaltReasonCharEnum>, // FIELD_HALTREASONCHAR 327
    pub in_view_of_common : Option<bool>, // FIELD_INVIEWOFCOMMON 328
    pub due_to_related : Option<bool>, // FIELD_DUETORELATED 329
    pub buy_volume : Option<f32>, // FIELD_BUYVOLUME 330
    pub sell_volume : Option<f32>, // FIELD_SELLVOLUME 331
    pub high_px : Option<f32>, // FIELD_HIGHPX 332
    pub low_px : Option<f32>, // FIELD_LOWPX 333
    pub last_px : Option<f32>, // FIELD_LASTPX 31
    pub transact_time : Option<UtcDateTime>, // FIELD_TRANSACTTIME 60
    pub adjustment : Option<FieldAdjustmentEnum>, // FIELD_ADJUSTMENT 334

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct TradingSessionStatusRequestFields {
    pub trad_ses_req_id : String, // FIELD_TRADSESREQID 335
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub trad_ses_method : Option<FieldTradSesMethodEnum>, // FIELD_TRADSESMETHOD 338
    pub trad_ses_mode : Option<FieldTradSesModeEnum>, // FIELD_TRADSESMODE 339
    pub subscription_request_type : FieldSubscriptionRequestTypeEnum, // FIELD_SUBSCRIPTIONREQUESTTYPE 263

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct TradingSessionStatusFields {
    pub trad_ses_req_id : Option<String>, // FIELD_TRADSESREQID 335
    pub trading_session_id : String, // FIELD_TRADINGSESSIONID 336
    pub trad_ses_method : Option<FieldTradSesMethodEnum>, // FIELD_TRADSESMETHOD 338
    pub trad_ses_mode : Option<FieldTradSesModeEnum>, // FIELD_TRADSESMODE 339
    pub unsolicited_indicator : Option<bool>, // FIELD_UNSOLICITEDINDICATOR 325
    pub trad_ses_status : FieldTradSesStatusEnum, // FIELD_TRADSESSTATUS 340
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
    pub quote_response_level : Option<FieldQuoteResponseLevelEnum>, // FIELD_QUOTERESPONSELEVEL 301
    pub def_bid_size : Option<f32>, // FIELD_DEFBIDSIZE 293
    pub def_offer_size : Option<f32>, // FIELD_DEFOFFERSIZE 294
    pub no_quote_sets : Vec<NoQuoteSets21Fields>, // FIELD_NOQUOTESETS 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct BusinessMessageRejectFields {
    pub ref_seq_num : Option<i32>, // FIELD_REFSEQNUM 45
    pub ref_msg_type : String, // FIELD_REFMSGTYPE 372
    pub business_reject_ref_id : Option<String>, // FIELD_BUSINESSREJECTREFID 379
    pub business_reject_reason : FieldBusinessRejectReasonEnum, // FIELD_BUSINESSREJECTREASON 380
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct BidRequestFields {
    pub bid_id : Option<String>, // FIELD_BIDID 390
    pub client_bid_id : String, // FIELD_CLIENTBIDID 391
    pub bid_request_trans_type : FieldBidRequestTransTypeEnum, // FIELD_BIDREQUESTTRANSTYPE 374
    pub list_name : Option<String>, // FIELD_LISTNAME 392
    pub total_num_securities : i32, // FIELD_TOTALNUMSECURITIES 393
    pub bid_type : i32, // FIELD_BIDTYPE 394
    pub num_tickets : Option<i32>, // FIELD_NUMTICKETS 395
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub side_value1 : Option<f32>, // FIELD_SIDEVALUE1 396
    pub side_value2 : Option<f32>, // FIELD_SIDEVALUE2 397
    pub no_bid_descriptors : Option<Vec<NoBidDescriptors6Fields>>, // FIELD_NOBIDDESCRIPTORS 0
    pub no_bid_components : Option<Vec<NoBidComponents5Fields>>, // FIELD_NOBIDCOMPONENTS 0
    pub liquidity_ind_type : Option<FieldLiquidityIndTypeEnum>, // FIELD_LIQUIDITYINDTYPE 409
    pub wt_average_liquidity : Option<f32>, // FIELD_WTAVERAGELIQUIDITY 410
    pub exchange_for_physical : Option<bool>, // FIELD_EXCHANGEFORPHYSICAL 411
    pub out_main_cntry_uindex : Option<f32>, // FIELD_OUTMAINCNTRYUINDEX 412
    pub cross_percent : Option<f32>, // FIELD_CROSSPERCENT 413
    pub prog_rpt_reqs : Option<FieldProgRptReqsEnum>, // FIELD_PROGRPTREQS 414
    pub prog_period_interval : Option<i32>, // FIELD_PROGPERIODINTERVAL 415
    pub inc_tax_ind : Option<FieldIncTaxIndEnum>, // FIELD_INCTAXIND 416
    pub forex_req : Option<bool>, // FIELD_FOREXREQ 121
    pub num_bidders : Option<i32>, // FIELD_NUMBIDDERS 417
    pub trade_date : Option<UtcDateTime>, // FIELD_TRADEDATE 75
    pub trade_type : FieldTradeTypeEnum, // FIELD_TRADETYPE 418
    pub basis_px_type : FieldBasisPxTypeEnum, // FIELD_BASISPXTYPE 419
    pub strike_time : Option<UtcDateTime>, // FIELD_STRIKETIME 443
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct BidResponseFields {
    pub bid_id : Option<String>, // FIELD_BIDID 390
    pub client_bid_id : Option<String>, // FIELD_CLIENTBIDID 391
    pub no_bid_components : Vec<NoBidComponents4Fields>, // FIELD_NOBIDCOMPONENTS 0

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
pub struct NoAllocs2Fields {
    pub alloc_account : Option<String>, // FIELD_ALLOCACCOUNT 79
    pub alloc_shares : Option<f32>, // FIELD_ALLOCSHARES 80

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoRelatedSym23Fields {
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
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoStrikes28Fields {
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
    pub cl_ord_id : Option<String>, // FIELD_CLORDID 11
    pub side : Option<FieldSideEnum>, // FIELD_SIDE 54
    pub price : f32, // FIELD_PRICE 44
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

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
pub struct NoMiscFees13Fields {
    pub misc_fee_amt : Option<f32>, // FIELD_MISCFEEAMT 137
    pub misc_fee_curr : Option<f32>, // FIELD_MISCFEECURR 138
    pub misc_fee_type : Option<FieldMiscFeeTypeEnum>, // FIELD_MISCFEETYPE 139

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoQuoteEntries18Fields {
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
    pub underlying_symbol : Option<String>, // FIELD_UNDERLYINGSYMBOL 311

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoContraBrokers7Fields {
    pub contra_broker : Option<String>, // FIELD_CONTRABROKER 375
    pub contra_trader : Option<String>, // FIELD_CONTRATRADER 337
    pub contra_trade_qty : Option<f32>, // FIELD_CONTRATRADEQTY 437
    pub contra_trade_time : Option<UtcDateTime>, // FIELD_CONTRATRADETIME 438

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoRelatedSym25Fields {
    pub relatd_sym : Option<String>, // FIELD_RELATDSYM 46
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

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoBidComponents4Fields {
    pub commission : f32, // FIELD_COMMISSION 12
    pub comm_type : FieldCommTypeEnum, // FIELD_COMMTYPE 13
    pub list_id : Option<String>, // FIELD_LISTID 66
    pub country : Option<String>, // FIELD_COUNTRY 421
    pub side : Option<FieldSideEnum>, // FIELD_SIDE 54
    pub price : Option<f32>, // FIELD_PRICE 44
    pub price_type : Option<FieldPriceTypeEnum>, // FIELD_PRICETYPE 423
    pub fair_value : Option<f32>, // FIELD_FAIRVALUE 406
    pub net_gross_ind : Option<FieldNetGrossIndEnum>, // FIELD_NETGROSSIND 430
    pub settlmnt_typ : Option<FieldSettlmntTypEnum>, // FIELD_SETTLMNTTYP 63
    pub fut_sett_date : Option<UtcDateTime>, // FIELD_FUTSETTDATE 64
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoExecs8Fields {
    pub last_shares : Option<f32>, // FIELD_LASTSHARES 32
    pub exec_id : Option<String>, // FIELD_EXECID 17
    pub last_px : Option<f32>, // FIELD_LASTPX 31
    pub last_capacity : Option<FieldLastCapacityEnum>, // FIELD_LASTCAPACITY 29

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct LinesOfText1Fields {
    pub text : String, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoAllocs3Fields {
    pub alloc_account : Option<String>, // FIELD_ALLOCACCOUNT 79
    pub alloc_price : Option<f32>, // FIELD_ALLOCPRICE 366
    pub alloc_shares : f32, // FIELD_ALLOCSHARES 80
    pub process_code : Option<FieldProcessCodeEnum>, // FIELD_PROCESSCODE 81
    pub broker_of_credit : Option<String>, // FIELD_BROKEROFCREDIT 92
    pub notify_broker_of_credit : Option<bool>, // FIELD_NOTIFYBROKEROFCREDIT 208
    pub alloc_handl_inst : Option<FieldAllocHandlInstEnum>, // FIELD_ALLOCHANDLINST 209
    pub alloc_text : Option<String>, // FIELD_ALLOCTEXT 161
    pub encoded_alloc_text_len : Option<usize>, // FIELD_ENCODEDALLOCTEXTLEN 360
    pub encoded_alloc_text : Option<String>, // FIELD_ENCODEDALLOCTEXT 361
    pub exec_broker : Option<String>, // FIELD_EXECBROKER 76
    pub client_id : Option<String>, // FIELD_CLIENTID 109
    pub commission : Option<f32>, // FIELD_COMMISSION 12
    pub comm_type : Option<FieldCommTypeEnum>, // FIELD_COMMTYPE 13
    pub alloc_avg_px : Option<f32>, // FIELD_ALLOCAVGPX 153
    pub alloc_net_money : Option<f32>, // FIELD_ALLOCNETMONEY 154
    pub settl_curr_amt : Option<f32>, // FIELD_SETTLCURRAMT 119
    pub settl_currency : Option<f32>, // FIELD_SETTLCURRENCY 120
    pub settl_curr_fx_rate : Option<f32>, // FIELD_SETTLCURRFXRATE 155
    pub settl_curr_fx_rate_calc : Option<FieldSettlCurrFxRateCalcEnum>, // FIELD_SETTLCURRFXRATECALC 156
    pub accrued_interest_amt : Option<f32>, // FIELD_ACCRUEDINTERESTAMT 159
    pub settl_inst_mode : Option<FieldSettlInstModeEnum>, // FIELD_SETTLINSTMODE 160
    pub no_misc_fees : Option<Vec<NoMiscFees13Fields>>, // FIELD_NOMISCFEES 0

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoIOIQualifiers9Fields {
    pub ioiqualifier : Option<FieldIOIQualifierEnum>, // FIELD_IOIQUALIFIER 104

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoRelatedSym26Fields {
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
    pub side : Option<FieldSideEnum>, // FIELD_SIDE 54
    pub underlying_currency : Option<f32>, // FIELD_UNDERLYINGCURRENCY 318

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoQuoteSets22Fields {
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
pub struct NoTradingSessions29Fields {
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoOrders17Fields {
    pub cl_ord_id : Option<String>, // FIELD_CLORDID 11
    pub order_id : Option<String>, // FIELD_ORDERID 37
    pub secondary_order_id : Option<String>, // FIELD_SECONDARYORDERID 198
    pub list_id : Option<String>, // FIELD_LISTID 66
    pub wave_no : Option<String>, // FIELD_WAVENO 105

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoOrders15Fields {
    pub cl_ord_id : String, // FIELD_CLORDID 11
    pub list_seq_no : i32, // FIELD_LISTSEQNO 67
    pub settl_inst_mode : Option<FieldSettlInstModeEnum>, // FIELD_SETTLINSTMODE 160
    pub client_id : Option<String>, // FIELD_CLIENTID 109
    pub exec_broker : Option<String>, // FIELD_EXECBROKER 76
    pub account : Option<String>, // FIELD_ACCOUNT 1
    pub no_allocs : Option<Vec<NoAllocs2Fields>>, // FIELD_NOALLOCS 0
    pub settlmnt_typ : Option<FieldSettlmntTypEnum>, // FIELD_SETTLMNTTYP 63
    pub fut_sett_date : Option<UtcDateTime>, // FIELD_FUTSETTDATE 64
    pub handl_inst : Option<FieldHandlInstEnum>, // FIELD_HANDLINST 21
    pub exec_inst : Option<FieldExecInstEnum>, // FIELD_EXECINST 18
    pub min_qty : Option<f32>, // FIELD_MINQTY 110
    pub max_floor : Option<f32>, // FIELD_MAXFLOOR 111
    pub ex_destination : Option<String>, // FIELD_EXDESTINATION 100
    pub no_trading_sessions : Option<Vec<NoTradingSessions29Fields>>, // FIELD_NOTRADINGSESSIONS 0
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



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoMsgTypes14Fields {
    pub ref_msg_type : Option<String>, // FIELD_REFMSGTYPE 372
    pub msg_direction : Option<FieldMsgDirectionEnum>, // FIELD_MSGDIRECTION 385

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoBidComponents5Fields {
    pub list_id : Option<String>, // FIELD_LISTID 66
    pub side : Option<FieldSideEnum>, // FIELD_SIDE 54
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub net_gross_ind : Option<FieldNetGrossIndEnum>, // FIELD_NETGROSSIND 430
    pub settlmnt_typ : Option<FieldSettlmntTypEnum>, // FIELD_SETTLMNTTYP 63
    pub fut_sett_date : Option<UtcDateTime>, // FIELD_FUTSETTDATE 64
    pub account : Option<String>, // FIELD_ACCOUNT 1

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoMDEntryTypes12Fields {
    pub mdentry_type : FieldMDEntryTypeEnum, // FIELD_MDENTRYTYPE 269

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoQuoteSets21Fields {
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
pub struct NoRoutingIDs27Fields {
    pub routing_type : Option<FieldRoutingTypeEnum>, // FIELD_ROUTINGTYPE 216
    pub routing_id : Option<String>, // FIELD_ROUTINGID 217

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoOrders16Fields {
    pub cl_ord_id : String, // FIELD_CLORDID 11
    pub cum_qty : f32, // FIELD_CUMQTY 14
    pub ord_status : FieldOrdStatusEnum, // FIELD_ORDSTATUS 39
    pub leaves_qty : f32, // FIELD_LEAVESQTY 151
    pub cxl_qty : f32, // FIELD_CXLQTY 84
    pub avg_px : f32, // FIELD_AVGPX 6
    pub ord_rej_reason : Option<FieldOrdRejReasonEnum>, // FIELD_ORDREJREASON 103
    pub text : Option<String>, // FIELD_TEXT 58
    pub encoded_text_len : Option<usize>, // FIELD_ENCODEDTEXTLEN 354
    pub encoded_text : Option<String>, // FIELD_ENCODEDTEXT 355

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoQuoteEntries19Fields {
    pub quote_entry_id : Option<String>, // FIELD_QUOTEENTRYID 299
    pub symbol : Option<String>, // FIELD_SYMBOL 55
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
    pub quote_entry_reject_reason : Option<FieldQuoteEntryRejectReasonEnum>, // FIELD_QUOTEENTRYREJECTREASON 368

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoQuoteEntries20Fields {
    pub quote_entry_id : String, // FIELD_QUOTEENTRYID 299
    pub symbol : Option<String>, // FIELD_SYMBOL 55
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
    pub ord_type : Option<FieldOrdTypeEnum>, // FIELD_ORDTYPE 40
    pub fut_sett_date2 : Option<UtcDateTime>, // FIELD_FUTSETTDATE2 193
    pub order_qty2 : Option<f32>, // FIELD_ORDERQTY2 192
    pub currency : Option<f32>, // FIELD_CURRENCY 15

}



#[derive(PartialEq,Debug,Default,Serialize,Deserialize)]
pub struct NoMDEntries10Fields {
    pub mdupdate_action : FieldMDUpdateActionEnum, // FIELD_MDUPDATEACTION 279
    pub delete_reason : Option<FieldDeleteReasonEnum>, // FIELD_DELETEREASON 285
    pub mdentry_type : Option<FieldMDEntryTypeEnum>, // FIELD_MDENTRYTYPE 269
    pub mdentry_id : Option<String>, // FIELD_MDENTRYID 278
    pub mdentry_ref_id : Option<String>, // FIELD_MDENTRYREFID 280
    pub symbol : Option<String>, // FIELD_SYMBOL 55
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
    pub financial_status : Option<FieldFinancialStatusEnum>, // FIELD_FINANCIALSTATUS 291
    pub corporate_action : Option<FieldCorporateActionEnum>, // FIELD_CORPORATEACTION 292
    pub mdentry_px : Option<f32>, // FIELD_MDENTRYPX 270
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub mdentry_size : Option<f32>, // FIELD_MDENTRYSIZE 271
    pub mdentry_date : Option<UtcDate>, // FIELD_MDENTRYDATE 272
    pub mdentry_time : Option<UtcTime>, // FIELD_MDENTRYTIME 273
    pub tick_direction : Option<FieldTickDirectionEnum>, // FIELD_TICKDIRECTION 274
    pub mdmkt : Option<String>, // FIELD_MDMKT 275
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub quote_condition : Option<FieldQuoteConditionEnum>, // FIELD_QUOTECONDITION 276
    pub trade_condition : Option<FieldTradeConditionEnum>, // FIELD_TRADECONDITION 277
    pub mdentry_originator : Option<String>, // FIELD_MDENTRYORIGINATOR 282
    pub location_id : Option<String>, // FIELD_LOCATIONID 283
    pub desk_id : Option<String>, // FIELD_DESKID 284
    pub open_close_settle_flag : Option<FieldOpenCloseSettleFlagEnum>, // FIELD_OPENCLOSESETTLEFLAG 286
    pub time_in_force : Option<FieldTimeInForceEnum>, // FIELD_TIMEINFORCE 59
    pub expire_date : Option<UtcDateTime>, // FIELD_EXPIREDATE 432
    pub expire_time : Option<UtcDateTime>, // FIELD_EXPIRETIME 126
    pub min_qty : Option<f32>, // FIELD_MINQTY 110
    pub exec_inst : Option<FieldExecInstEnum>, // FIELD_EXECINST 18
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
pub struct NoMDEntries11Fields {
    pub mdentry_type : FieldMDEntryTypeEnum, // FIELD_MDENTRYTYPE 269
    pub mdentry_px : f32, // FIELD_MDENTRYPX 270
    pub currency : Option<f32>, // FIELD_CURRENCY 15
    pub mdentry_size : Option<f32>, // FIELD_MDENTRYSIZE 271
    pub mdentry_date : Option<UtcDate>, // FIELD_MDENTRYDATE 272
    pub mdentry_time : Option<UtcTime>, // FIELD_MDENTRYTIME 273
    pub tick_direction : Option<FieldTickDirectionEnum>, // FIELD_TICKDIRECTION 274
    pub mdmkt : Option<String>, // FIELD_MDMKT 275
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub quote_condition : Option<FieldQuoteConditionEnum>, // FIELD_QUOTECONDITION 276
    pub trade_condition : Option<FieldTradeConditionEnum>, // FIELD_TRADECONDITION 277
    pub mdentry_originator : Option<String>, // FIELD_MDENTRYORIGINATOR 282
    pub location_id : Option<String>, // FIELD_LOCATIONID 283
    pub desk_id : Option<String>, // FIELD_DESKID 284
    pub open_close_settle_flag : Option<FieldOpenCloseSettleFlagEnum>, // FIELD_OPENCLOSESETTLEFLAG 286
    pub time_in_force : Option<FieldTimeInForceEnum>, // FIELD_TIMEINFORCE 59
    pub expire_date : Option<UtcDateTime>, // FIELD_EXPIREDATE 432
    pub expire_time : Option<UtcDateTime>, // FIELD_EXPIRETIME 126
    pub min_qty : Option<f32>, // FIELD_MINQTY 110
    pub exec_inst : Option<FieldExecInstEnum>, // FIELD_EXECINST 18
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
pub struct NoRelatedSym24Fields {
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
    pub quote_request_type : Option<FieldQuoteRequestTypeEnum>, // FIELD_QUOTEREQUESTTYPE 303
    pub trading_session_id : Option<String>, // FIELD_TRADINGSESSIONID 336
    pub side : Option<FieldSideEnum>, // FIELD_SIDE 54
    pub order_qty : Option<f32>, // FIELD_ORDERQTY 38
    pub fut_sett_date : Option<UtcDateTime>, // FIELD_FUTSETTDATE 64
    pub ord_type : Option<FieldOrdTypeEnum>, // FIELD_ORDTYPE 40
    pub fut_sett_date2 : Option<UtcDateTime>, // FIELD_FUTSETTDATE2 193
    pub order_qty2 : Option<f32>, // FIELD_ORDERQTY2 192
    pub expire_time : Option<UtcDateTime>, // FIELD_EXPIRETIME 126
    pub transact_time : Option<UtcDateTime>, // FIELD_TRANSACTTIME 60
    pub currency : Option<f32>, // FIELD_CURRENCY 15

}








// Fields Constants / enums

const FIELD_ACCOUNT : u32 = 1; // STRING

const FIELD_ADVID : u32 = 2; // STRING

const FIELD_ADVREFID : u32 = 3; // STRING

const FIELD_ADVSIDE : u32 = 4; // CHAR
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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
#[derive(PartialEq,Debug,Serialize,Deserialize)]
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


fn parse_message_ioifields( consumer : &mut FixConsumer  ) -> IOIFields {
    // fields:
    let mut ioiid : Option<String> = None;
    let mut ioitrans_type : Option<FieldIOITransTypeEnum> = None;
    let mut ioiref_id : Option<String> = None;
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
    let mut ioishares : Option<FieldIOISharesEnum> = None;
    let mut price : Option<f32> = None;
    let mut currency : Option<f32> = None;
    let mut valid_until_time : Option<UtcDateTime> = None;
    let mut ioiqlty_ind : Option<FieldIOIQltyIndEnum> = None;
    let mut ioinatural_flag : Option<bool> = None;
    let mut no_ioiqualifiers : Option<Vec<NoIOIQualifiers9Fields>> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;
    let mut transact_time : Option<UtcDateTime> = None;
    let mut urllink : Option<String> = None;
    let mut no_routing_ids : Option<Vec<NoRoutingIDs27Fields>> = None;
    let mut spread_to_benchmark : Option<f32> = None;
    let mut benchmark : Option<FieldBenchmarkEnum> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_IOIID, val: v } => {

                ioiid = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_IOITRANSTYPE, val: v } => {

                ioitrans_type = Some( FieldIOITransTypeEnum::from_str(v).unwrap() );
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
            &FieldVal { id: FIELD_IOISHARES, val: v } => {

                ioishares = Some( FieldIOISharesEnum::from_str(v).unwrap() );
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

                ioiqlty_ind = Some( FieldIOIQltyIndEnum::from_str(v).unwrap() );
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

                benchmark = Some( FieldBenchmarkEnum::from_str(v).unwrap() );
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
    let mut no_contra_brokers : Option<Vec<NoContraBrokers7Fields>> = None;
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
    let mut no_msg_types : Option<Vec<NoMsgTypes14Fields>> = None;

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
    let mut urgency : Option<FieldUrgencyEnum> = None;
    let mut headline : Option<String> = None;
    let mut encoded_headline_len : Option<usize> = None;
    let mut encoded_headline : Option<String> = None;
    let mut no_routing_ids : Option<Vec<NoRoutingIDs27Fields>> = None;
    let mut no_related_sym : Option<Vec<NoRelatedSym25Fields>> = None;
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

                urgency = Some( FieldUrgencyEnum::from_str(v).unwrap() );
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
                let subgroup = build_group_no_related_sym25_fields(consumer, size);
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
    let mut email_type : Option<FieldEmailTypeEnum> = None;
    let mut orig_time : Option<UtcDateTime> = None;
    let mut subject : Option<String> = None;
    let mut encoded_subject_len : Option<usize> = None;
    let mut encoded_subject : Option<String> = None;
    let mut no_routing_ids : Option<Vec<NoRoutingIDs27Fields>> = None;
    let mut no_related_sym : Option<Vec<NoRelatedSym25Fields>> = None;
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

                email_type = Some( FieldEmailTypeEnum::from_str(v).unwrap() );
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
                let subgroup = build_group_no_related_sym25_fields(consumer, size);
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
    let mut no_allocs : Option<Vec<NoAllocs2Fields>> = None;
    let mut settlmnt_typ : Option<FieldSettlmntTypEnum> = None;
    let mut fut_sett_date : Option<UtcDateTime> = None;
    let mut handl_inst : Option<FieldHandlInstEnum> = None;
    let mut exec_inst : Option<FieldExecInstEnum> = None;
    let mut min_qty : Option<f32> = None;
    let mut max_floor : Option<f32> = None;
    let mut ex_destination : Option<String> = None;
    let mut no_trading_sessions : Option<Vec<NoTradingSessions29Fields>> = None;
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
                let subgroup = build_group_no_allocs2_fields(consumer, size);
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
                let subgroup = build_group_no_trading_sessions29_fields(consumer, size);
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
    let mut no_allocs : Option<Vec<NoAllocs2Fields>> = None;
    let mut settlmnt_typ : Option<FieldSettlmntTypEnum> = None;
    let mut fut_sett_date : Option<UtcDateTime> = None;
    let mut handl_inst : Option<FieldHandlInstEnum> = None;
    let mut exec_inst : Option<FieldExecInstEnum> = None;
    let mut min_qty : Option<f32> = None;
    let mut max_floor : Option<f32> = None;
    let mut ex_destination : Option<String> = None;
    let mut no_trading_sessions : Option<Vec<NoTradingSessions29Fields>> = None;
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
                let subgroup = build_group_no_allocs2_fields(consumer, size);
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


fn parse_message_allocation_fields( consumer : &mut FixConsumer  ) -> AllocationFields {
    // fields:
    let mut alloc_id : Option<String> = None;
    let mut alloc_trans_type : Option<FieldAllocTransTypeEnum> = None;
    let mut ref_alloc_id : Option<String> = None;
    let mut alloc_link_id : Option<String> = None;
    let mut alloc_link_type : Option<FieldAllocLinkTypeEnum> = None;
    let mut no_orders : Option<Vec<NoOrders17Fields>> = None;
    let mut no_execs : Option<Vec<NoExecs8Fields>> = None;
    let mut side : Option<FieldSideEnum> = None;
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
    let mut shares : Option<f32> = None;
    let mut last_mkt : Option<String> = None;
    let mut trading_session_id : Option<String> = None;
    let mut avg_px : Option<f32> = None;
    let mut currency : Option<f32> = None;
    let mut avg_prx_precision : Option<i32> = None;
    let mut trade_date : Option<UtcDateTime> = None;
    let mut transact_time : Option<UtcDateTime> = None;
    let mut settlmnt_typ : Option<FieldSettlmntTypEnum> = None;
    let mut fut_sett_date : Option<UtcDateTime> = None;
    let mut gross_trade_amt : Option<f32> = None;
    let mut net_money : Option<f32> = None;
    let mut open_close : Option<FieldOpenCloseEnum> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;
    let mut num_days_interest : Option<i32> = None;
    let mut accrued_interest_rate : Option<f32> = None;
    let mut no_allocs : Option<Vec<NoAllocs3Fields>> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_ALLOCID, val: v } => {

                alloc_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ALLOCTRANSTYPE, val: v } => {

                alloc_trans_type = Some( FieldAllocTransTypeEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_REFALLOCID, val: v } => {

                ref_alloc_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ALLOCLINKID, val: v } => {

                alloc_link_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_ALLOCLINKTYPE, val: v } => {

                alloc_link_type = Some( FieldAllocLinkTypeEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_NOORDERS, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_orders17_fields(consumer, size);
                no_orders = Some(subgroup);
            },
            &FieldVal { id: FIELD_NOEXECS, val: v } => {

                // group
                let size = usize::from_str(v).unwrap();
                let subgroup = build_group_no_execs8_fields(consumer, size);
                no_execs = Some(subgroup);
            },
            &FieldVal { id: FIELD_SIDE, val: v } => {

                side = Some( FieldSideEnum::from_str(v).unwrap() );
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

                settlmnt_typ = Some( FieldSettlmntTypEnum::from_str(v).unwrap() );
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

                open_close = Some( FieldOpenCloseEnum::from_str(v).unwrap() );
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
                let subgroup = build_group_no_allocs3_fields(consumer, size);
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
    let mut no_orders : Option<Vec<NoOrders16Fields>> = None;

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
                let subgroup = build_group_no_orders16_fields(consumer, size);
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
    let mut alloc_status : Option<FieldAllocStatusEnum> = None;
    let mut alloc_rej_code : Option<FieldAllocRejCodeEnum> = None;
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

                alloc_status = Some( FieldAllocStatusEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ALLOCREJCODE, val: v } => {

                alloc_rej_code = Some( FieldAllocRejCodeEnum::from_str(v).unwrap() );
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
    let mut dkreason : Option<FieldDKReasonEnum> = None;
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

                dkreason = Some( FieldDKReasonEnum::from_str(v).unwrap() );
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
    let mut no_related_sym : Option<Vec<NoRelatedSym24Fields>> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_QUOTEREQID, val: v } => {

                quote_req_id = Some( v.to_string() );
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
    QuoteRequestFields {
        quote_req_id: quote_req_id.unwrap() /* better error hdl? */ ,
        no_related_sym: no_related_sym.unwrap() /* better error hdl? */ ,
    }
}


fn parse_message_quote_fields( consumer : &mut FixConsumer  ) -> QuoteFields {
    // fields:
    let mut quote_req_id : Option<String> = None;
    let mut quote_id : Option<String> = None;
    let mut quote_response_level : Option<FieldQuoteResponseLevelEnum> = None;
    let mut trading_session_id : Option<String> = None;
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
    let mut ord_type : Option<FieldOrdTypeEnum> = None;
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

                quote_response_level = Some( FieldQuoteResponseLevelEnum::from_str(v).unwrap() );
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

                ord_type = Some( FieldOrdTypeEnum::from_str(v).unwrap() );
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
    let mut settl_inst_trans_type : Option<FieldSettlInstTransTypeEnum> = None;
    let mut settl_inst_ref_id : Option<String> = None;
    let mut settl_inst_mode : Option<FieldSettlInstModeEnum> = None;
    let mut settl_inst_source : Option<FieldSettlInstSourceEnum> = None;
    let mut alloc_account : Option<String> = None;
    let mut settl_location : Option<FieldSettlLocationEnum> = None;
    let mut trade_date : Option<UtcDateTime> = None;
    let mut alloc_id : Option<String> = None;
    let mut last_mkt : Option<String> = None;
    let mut trading_session_id : Option<String> = None;
    let mut side : Option<FieldSideEnum> = None;
    let mut security_type : Option<FieldSecurityTypeEnum> = None;
    let mut effective_time : Option<UtcDateTime> = None;
    let mut transact_time : Option<UtcDateTime> = None;
    let mut client_id : Option<String> = None;
    let mut exec_broker : Option<String> = None;
    let mut stand_inst_db_type : Option<FieldStandInstDbTypeEnum> = None;
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

                settl_inst_trans_type = Some( FieldSettlInstTransTypeEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SETTLINSTREFID, val: v } => {

                settl_inst_ref_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SETTLINSTMODE, val: v } => {

                settl_inst_mode = Some( FieldSettlInstModeEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SETTLINSTSOURCE, val: v } => {

                settl_inst_source = Some( FieldSettlInstSourceEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_ALLOCACCOUNT, val: v } => {

                alloc_account = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SETTLLOCATION, val: v } => {

                settl_location = Some( FieldSettlLocationEnum::from_str(v).unwrap() );
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

                side = Some( FieldSideEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SECURITYTYPE, val: v } => {

                security_type = Some( FieldSecurityTypeEnum::from_str(v).unwrap() );
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

                stand_inst_db_type = Some( FieldStandInstDbTypeEnum::from_str(v).unwrap() );
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
    let mut subscription_request_type : Option<FieldSubscriptionRequestTypeEnum> = None;
    let mut market_depth : Option<i32> = None;
    let mut mdupdate_type : Option<FieldMDUpdateTypeEnum> = None;
    let mut aggregated_book : Option<bool> = None;
    let mut no_mdentry_types : Option<Vec<NoMDEntryTypes12Fields>> = None;
    let mut no_related_sym : Option<Vec<NoRelatedSym23Fields>> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_MDREQID, val: v } => {

                mdreq_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SUBSCRIPTIONREQUESTTYPE, val: v } => {

                subscription_request_type = Some( FieldSubscriptionRequestTypeEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MARKETDEPTH, val: v } => {

                market_depth = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_MDUPDATETYPE, val: v } => {

                mdupdate_type = Some( FieldMDUpdateTypeEnum::from_str(v).unwrap() );
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
                let subgroup = build_group_no_related_sym23_fields(consumer, size);
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
    let mut financial_status : Option<FieldFinancialStatusEnum> = None;
    let mut corporate_action : Option<FieldCorporateActionEnum> = None;
    let mut total_volume_traded : Option<f32> = None;
    let mut no_mdentries : Option<Vec<NoMDEntries11Fields>> = None;

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
            &FieldVal { id: FIELD_FINANCIALSTATUS, val: v } => {

                financial_status = Some( FieldFinancialStatusEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CORPORATEACTION, val: v } => {

                corporate_action = Some( FieldCorporateActionEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TOTALVOLUMETRADED, val: v } => {

                total_volume_traded = Some( f32::from_str(v).unwrap() );
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
    let mut no_mdentries : Option<Vec<NoMDEntries10Fields>> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_MDREQID, val: v } => {

                mdreq_id = Some( v.to_string() );
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
    MarketDataIncrementalRefreshFields {
        mdreq_id: mdreq_id,
        no_mdentries: no_mdentries.unwrap() /* better error hdl? */ ,
    }
}


fn parse_message_market_data_request_reject_fields( consumer : &mut FixConsumer  ) -> MarketDataRequestRejectFields {
    // fields:
    let mut mdreq_id : Option<String> = None;
    let mut mdreq_rej_reason : Option<FieldMDReqRejReasonEnum> = None;
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

                mdreq_rej_reason = Some( FieldMDReqRejReasonEnum::from_str(v).unwrap() );
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
    let mut quote_cancel_type : Option<FieldQuoteCancelTypeEnum> = None;
    let mut quote_response_level : Option<FieldQuoteResponseLevelEnum> = None;
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

                quote_cancel_type = Some( FieldQuoteCancelTypeEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_QUOTERESPONSELEVEL, val: v } => {

                quote_response_level = Some( FieldQuoteResponseLevelEnum::from_str(v).unwrap() );
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
    let mut quote_ack_status : Option<FieldQuoteAckStatusEnum> = None;
    let mut quote_reject_reason : Option<FieldQuoteRejectReasonEnum> = None;
    let mut quote_response_level : Option<FieldQuoteResponseLevelEnum> = None;
    let mut trading_session_id : Option<String> = None;
    let mut text : Option<String> = None;
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
            &FieldVal { id: FIELD_QUOTEACKSTATUS, val: v } => {

                quote_ack_status = Some( FieldQuoteAckStatusEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_QUOTEREJECTREASON, val: v } => {

                quote_reject_reason = Some( FieldQuoteRejectReasonEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_QUOTERESPONSELEVEL, val: v } => {

                quote_response_level = Some( FieldQuoteResponseLevelEnum::from_str(v).unwrap() );
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
                let subgroup = build_group_no_quote_sets22_fields(consumer, size);
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
    let mut security_request_type : Option<FieldSecurityRequestTypeEnum> = None;
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
    let mut currency : Option<f32> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;
    let mut trading_session_id : Option<String> = None;
    let mut no_related_sym : Option<Vec<NoRelatedSym26Fields>> = None;

    // loop
    while let Some(fld) = consumer.next() {
        match fld {
            &FieldVal { id: FIELD_SECURITYREQID, val: v } => {

                security_req_id = Some( v.to_string() );
            },
            &FieldVal { id: FIELD_SECURITYREQUESTTYPE, val: v } => {

                security_request_type = Some( FieldSecurityRequestTypeEnum::from_str(v).unwrap() );
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
                let subgroup = build_group_no_related_sym26_fields(consumer, size);
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
    let mut security_response_type : Option<FieldSecurityResponseTypeEnum> = None;
    let mut total_num_securities : Option<i32> = None;
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
    let mut currency : Option<f32> = None;
    let mut trading_session_id : Option<String> = None;
    let mut text : Option<String> = None;
    let mut encoded_text_len : Option<usize> = None;
    let mut encoded_text : Option<String> = None;
    let mut no_related_sym : Option<Vec<NoRelatedSym26Fields>> = None;

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

                security_response_type = Some( FieldSecurityResponseTypeEnum::from_str(v).unwrap() );
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
                let subgroup = build_group_no_related_sym26_fields(consumer, size);
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
    let mut currency : Option<f32> = None;
    let mut subscription_request_type : Option<FieldSubscriptionRequestTypeEnum> = None;
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
            &FieldVal { id: FIELD_CURRENCY, val: v } => {

                currency = Some( f32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SUBSCRIPTIONREQUESTTYPE, val: v } => {

                subscription_request_type = Some( FieldSubscriptionRequestTypeEnum::from_str(v).unwrap() );
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
    let mut currency : Option<f32> = None;
    let mut trading_session_id : Option<String> = None;
    let mut unsolicited_indicator : Option<bool> = None;
    let mut security_trading_status : Option<FieldSecurityTradingStatusEnum> = None;
    let mut financial_status : Option<FieldFinancialStatusEnum> = None;
    let mut corporate_action : Option<FieldCorporateActionEnum> = None;
    let mut halt_reason_char : Option<FieldHaltReasonCharEnum> = None;
    let mut in_view_of_common : Option<bool> = None;
    let mut due_to_related : Option<bool> = None;
    let mut buy_volume : Option<f32> = None;
    let mut sell_volume : Option<f32> = None;
    let mut high_px : Option<f32> = None;
    let mut low_px : Option<f32> = None;
    let mut last_px : Option<f32> = None;
    let mut transact_time : Option<UtcDateTime> = None;
    let mut adjustment : Option<FieldAdjustmentEnum> = None;

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

                security_trading_status = Some( FieldSecurityTradingStatusEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_FINANCIALSTATUS, val: v } => {

                financial_status = Some( FieldFinancialStatusEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_CORPORATEACTION, val: v } => {

                corporate_action = Some( FieldCorporateActionEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_HALTREASONCHAR, val: v } => {

                halt_reason_char = Some( FieldHaltReasonCharEnum::from_str(v).unwrap() );
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

                adjustment = Some( FieldAdjustmentEnum::from_str(v).unwrap() );
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
    let mut trad_ses_method : Option<FieldTradSesMethodEnum> = None;
    let mut trad_ses_mode : Option<FieldTradSesModeEnum> = None;
    let mut subscription_request_type : Option<FieldSubscriptionRequestTypeEnum> = None;

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

                trad_ses_method = Some( FieldTradSesMethodEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRADSESMODE, val: v } => {

                trad_ses_mode = Some( FieldTradSesModeEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_SUBSCRIPTIONREQUESTTYPE, val: v } => {

                subscription_request_type = Some( FieldSubscriptionRequestTypeEnum::from_str(v).unwrap() );
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
    let mut trad_ses_method : Option<FieldTradSesMethodEnum> = None;
    let mut trad_ses_mode : Option<FieldTradSesModeEnum> = None;
    let mut unsolicited_indicator : Option<bool> = None;
    let mut trad_ses_status : Option<FieldTradSesStatusEnum> = None;
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

                trad_ses_method = Some( FieldTradSesMethodEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_TRADSESMODE, val: v } => {

                trad_ses_mode = Some( FieldTradSesModeEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_UNSOLICITEDINDICATOR, val: v } => {

                unsolicited_indicator = Some( boolconv(v) );
            },
            &FieldVal { id: FIELD_TRADSESSTATUS, val: v } => {

                trad_ses_status = Some( FieldTradSesStatusEnum::from_str(v).unwrap() );
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
    let mut quote_response_level : Option<FieldQuoteResponseLevelEnum> = None;
    let mut def_bid_size : Option<f32> = None;
    let mut def_offer_size : Option<f32> = None;
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
            &FieldVal { id: FIELD_QUOTERESPONSELEVEL, val: v } => {

                quote_response_level = Some( FieldQuoteResponseLevelEnum::from_str(v).unwrap() );
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
                let subgroup = build_group_no_quote_sets21_fields(consumer, size);
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
    let mut business_reject_reason : Option<FieldBusinessRejectReasonEnum> = None;
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

                business_reject_reason = Some( FieldBusinessRejectReasonEnum::from_str(v).unwrap() );
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
    let mut bid_request_trans_type : Option<FieldBidRequestTransTypeEnum> = None;
    let mut list_name : Option<String> = None;
    let mut total_num_securities : Option<i32> = None;
    let mut bid_type : Option<i32> = None;
    let mut num_tickets : Option<i32> = None;
    let mut currency : Option<f32> = None;
    let mut side_value1 : Option<f32> = None;
    let mut side_value2 : Option<f32> = None;
    let mut no_bid_descriptors : Option<Vec<NoBidDescriptors6Fields>> = None;
    let mut no_bid_components : Option<Vec<NoBidComponents5Fields>> = None;
    let mut liquidity_ind_type : Option<FieldLiquidityIndTypeEnum> = None;
    let mut wt_average_liquidity : Option<f32> = None;
    let mut exchange_for_physical : Option<bool> = None;
    let mut out_main_cntry_uindex : Option<f32> = None;
    let mut cross_percent : Option<f32> = None;
    let mut prog_rpt_reqs : Option<FieldProgRptReqsEnum> = None;
    let mut prog_period_interval : Option<i32> = None;
    let mut inc_tax_ind : Option<FieldIncTaxIndEnum> = None;
    let mut forex_req : Option<bool> = None;
    let mut num_bidders : Option<i32> = None;
    let mut trade_date : Option<UtcDateTime> = None;
    let mut trade_type : Option<FieldTradeTypeEnum> = None;
    let mut basis_px_type : Option<FieldBasisPxTypeEnum> = None;
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

                bid_request_trans_type = Some( FieldBidRequestTransTypeEnum::from_str(v).unwrap() );
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
                let subgroup = build_group_no_bid_components5_fields(consumer, size);
                no_bid_components = Some(subgroup);
            },
            &FieldVal { id: FIELD_LIQUIDITYINDTYPE, val: v } => {

                liquidity_ind_type = Some( FieldLiquidityIndTypeEnum::from_str(v).unwrap() );
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

                prog_rpt_reqs = Some( FieldProgRptReqsEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_PROGPERIODINTERVAL, val: v } => {

                prog_period_interval = Some( i32::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_INCTAXIND, val: v } => {

                inc_tax_ind = Some( FieldIncTaxIndEnum::from_str(v).unwrap() );
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

                trade_type = Some( FieldTradeTypeEnum::from_str(v).unwrap() );
            },
            &FieldVal { id: FIELD_BASISPXTYPE, val: v } => {

                basis_px_type = Some( FieldBasisPxTypeEnum::from_str(v).unwrap() );
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
    let mut no_bid_components : Option<Vec<NoBidComponents4Fields>> = None;

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
                let subgroup = build_group_no_bid_components4_fields(consumer, size);
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
    NoAllocs2Fields {
        alloc_account: alloc_account,
        alloc_shares: alloc_shares,
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
    NoRelatedSym23Fields {
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
    let mut cl_ord_id : Option<String> = None;
    let mut side : Option<FieldSideEnum> = None;
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

            &FieldVal { id: FIELD_CLORDID, val: v } => {

                if cl_ord_id.is_some() { break; }

                cl_ord_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_SIDE, val: v } => {

                if side.is_some() { break; }

                side = Some( FieldSideEnum::from_str(v).unwrap() );
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
    let mut misc_fee_type : Option<FieldMiscFeeTypeEnum> = None;

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

                misc_fee_type = Some( FieldMiscFeeTypeEnum::from_str(v).unwrap() );
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
    let mut relatd_sym : Option<String> = None;
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


            _ => { break; }
        };
        // consume only if recognized
        consumer.next();
    }

    // construction
    NoRelatedSym25Fields {
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
    let mut commission : Option<f32> = None;
    let mut comm_type : Option<FieldCommTypeEnum> = None;
    let mut list_id : Option<String> = None;
    let mut country : Option<String> = None;
    let mut side : Option<FieldSideEnum> = None;
    let mut price : Option<f32> = None;
    let mut price_type : Option<FieldPriceTypeEnum> = None;
    let mut fair_value : Option<f32> = None;
    let mut net_gross_ind : Option<FieldNetGrossIndEnum> = None;
    let mut settlmnt_typ : Option<FieldSettlmntTypEnum> = None;
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

                comm_type = Some( FieldCommTypeEnum::from_str(v).unwrap() );
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

                side = Some( FieldSideEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_PRICE, val: v } => {

                if price.is_some() { break; }

                price = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_PRICETYPE, val: v } => {

                if price_type.is_some() { break; }

                price_type = Some( FieldPriceTypeEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_FAIRVALUE, val: v } => {

                if fair_value.is_some() { break; }

                fair_value = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_NETGROSSIND, val: v } => {

                if net_gross_ind.is_some() { break; }

                net_gross_ind = Some( FieldNetGrossIndEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SETTLMNTTYP, val: v } => {

                if settlmnt_typ.is_some() { break; }

                settlmnt_typ = Some( FieldSettlmntTypEnum::from_str(v).unwrap() );
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
    NoBidComponents4Fields {
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
    let mut last_capacity : Option<FieldLastCapacityEnum> = None;

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

                last_capacity = Some( FieldLastCapacityEnum::from_str(v).unwrap() );
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
    let mut alloc_price : Option<f32> = None;
    let mut alloc_shares : Option<f32> = None;
    let mut process_code : Option<FieldProcessCodeEnum> = None;
    let mut broker_of_credit : Option<String> = None;
    let mut notify_broker_of_credit : Option<bool> = None;
    let mut alloc_handl_inst : Option<FieldAllocHandlInstEnum> = None;
    let mut alloc_text : Option<String> = None;
    let mut encoded_alloc_text_len : Option<usize> = None;
    let mut encoded_alloc_text : Option<String> = None;
    let mut exec_broker : Option<String> = None;
    let mut client_id : Option<String> = None;
    let mut commission : Option<f32> = None;
    let mut comm_type : Option<FieldCommTypeEnum> = None;
    let mut alloc_avg_px : Option<f32> = None;
    let mut alloc_net_money : Option<f32> = None;
    let mut settl_curr_amt : Option<f32> = None;
    let mut settl_currency : Option<f32> = None;
    let mut settl_curr_fx_rate : Option<f32> = None;
    let mut settl_curr_fx_rate_calc : Option<FieldSettlCurrFxRateCalcEnum> = None;
    let mut accrued_interest_amt : Option<f32> = None;
    let mut settl_inst_mode : Option<FieldSettlInstModeEnum> = None;
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

                process_code = Some( FieldProcessCodeEnum::from_str(v).unwrap() );
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

                alloc_handl_inst = Some( FieldAllocHandlInstEnum::from_str(v).unwrap() );
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

                comm_type = Some( FieldCommTypeEnum::from_str(v).unwrap() );
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

                settl_curr_fx_rate_calc = Some( FieldSettlCurrFxRateCalcEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_ACCRUEDINTERESTAMT, val: v } => {

                if accrued_interest_amt.is_some() { break; }

                accrued_interest_amt = Some( f32::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SETTLINSTMODE, val: v } => {

                if settl_inst_mode.is_some() { break; }

                settl_inst_mode = Some( FieldSettlInstModeEnum::from_str(v).unwrap() );
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
    NoAllocs3Fields {
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
    let mut ioiqualifier : Option<FieldIOIQualifierEnum> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_IOIQUALIFIER, val: v } => {

                if ioiqualifier.is_some() { break; }

                ioiqualifier = Some( FieldIOIQualifierEnum::from_str(v).unwrap() );
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
    let mut side : Option<FieldSideEnum> = None;
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

                side = Some( FieldSideEnum::from_str(v).unwrap() );
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
    NoRelatedSym26Fields {
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
    NoQuoteSets22Fields {
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
    NoOrders17Fields {
        cl_ord_id: cl_ord_id,
        order_id: order_id,
        secondary_order_id: secondary_order_id,
        list_id: list_id,
        wave_no: wave_no,
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
    let mut settl_inst_mode : Option<FieldSettlInstModeEnum> = None;
    let mut client_id : Option<String> = None;
    let mut exec_broker : Option<String> = None;
    let mut account : Option<String> = None;
    let mut no_allocs : Option<Vec<NoAllocs2Fields>> = None;
    let mut settlmnt_typ : Option<FieldSettlmntTypEnum> = None;
    let mut fut_sett_date : Option<UtcDateTime> = None;
    let mut handl_inst : Option<FieldHandlInstEnum> = None;
    let mut exec_inst : Option<FieldExecInstEnum> = None;
    let mut min_qty : Option<f32> = None;
    let mut max_floor : Option<f32> = None;
    let mut ex_destination : Option<String> = None;
    let mut no_trading_sessions : Option<Vec<NoTradingSessions29Fields>> = None;
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
                let items = build_group_no_allocs2_fields(consumer, size);
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
                let items = build_group_no_trading_sessions29_fields(consumer, size);
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
    NoMsgTypes14Fields {
        ref_msg_type: ref_msg_type,
        msg_direction: msg_direction,
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
    let mut list_id : Option<String> = None;
    let mut side : Option<FieldSideEnum> = None;
    let mut trading_session_id : Option<String> = None;
    let mut net_gross_ind : Option<FieldNetGrossIndEnum> = None;
    let mut settlmnt_typ : Option<FieldSettlmntTypEnum> = None;
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

                side = Some( FieldSideEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_TRADINGSESSIONID, val: v } => {

                if trading_session_id.is_some() { break; }

                trading_session_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_NETGROSSIND, val: v } => {

                if net_gross_ind.is_some() { break; }

                net_gross_ind = Some( FieldNetGrossIndEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_SETTLMNTTYP, val: v } => {

                if settlmnt_typ.is_some() { break; }

                settlmnt_typ = Some( FieldSettlmntTypEnum::from_str(v).unwrap() );
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
    NoBidComponents5Fields {
        list_id: list_id,
        side: side,
        trading_session_id: trading_session_id,
        net_gross_ind: net_gross_ind,
        settlmnt_typ: settlmnt_typ,
        fut_sett_date: fut_sett_date,
        account: account,
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
    let mut mdentry_type : Option<FieldMDEntryTypeEnum> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_MDENTRYTYPE, val: v } => {

                if mdentry_type.is_some() { break; }

                mdentry_type = Some( FieldMDEntryTypeEnum::from_str(v).unwrap() );
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
    NoQuoteSets21Fields {
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
    let mut routing_type : Option<FieldRoutingTypeEnum> = None;
    let mut routing_id : Option<String> = None;

    // loop
    while let Some(fld) = consumer.peek() {
        match fld {
            &FieldVal { id: FIELD_ROUTINGTYPE, val: v } => {

                if routing_type.is_some() { break; }

                routing_type = Some( FieldRoutingTypeEnum::from_str(v).unwrap() );
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
    let mut cum_qty : Option<f32> = None;
    let mut ord_status : Option<FieldOrdStatusEnum> = None;
    let mut leaves_qty : Option<f32> = None;
    let mut cxl_qty : Option<f32> = None;
    let mut avg_px : Option<f32> = None;
    let mut ord_rej_reason : Option<FieldOrdRejReasonEnum> = None;
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

                ord_status = Some( FieldOrdStatusEnum::from_str(v).unwrap() );
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

                ord_rej_reason = Some( FieldOrdRejReasonEnum::from_str(v).unwrap() );
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
    NoOrders16Fields {
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
    let mut quote_entry_reject_reason : Option<FieldQuoteEntryRejectReasonEnum> = None;

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

            &FieldVal { id: FIELD_QUOTEENTRYREJECTREASON, val: v } => {

                if quote_entry_reject_reason.is_some() { break; }

                quote_entry_reject_reason = Some( FieldQuoteEntryRejectReasonEnum::from_str(v).unwrap() );
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
    let mut ord_type : Option<FieldOrdTypeEnum> = None;
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

                ord_type = Some( FieldOrdTypeEnum::from_str(v).unwrap() );
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
    let mut mdupdate_action : Option<FieldMDUpdateActionEnum> = None;
    let mut delete_reason : Option<FieldDeleteReasonEnum> = None;
    let mut mdentry_type : Option<FieldMDEntryTypeEnum> = None;
    let mut mdentry_id : Option<String> = None;
    let mut mdentry_ref_id : Option<String> = None;
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
    let mut financial_status : Option<FieldFinancialStatusEnum> = None;
    let mut corporate_action : Option<FieldCorporateActionEnum> = None;
    let mut mdentry_px : Option<f32> = None;
    let mut currency : Option<f32> = None;
    let mut mdentry_size : Option<f32> = None;
    let mut mdentry_date : Option<UtcDate> = None;
    let mut mdentry_time : Option<UtcTime> = None;
    let mut tick_direction : Option<FieldTickDirectionEnum> = None;
    let mut mdmkt : Option<String> = None;
    let mut trading_session_id : Option<String> = None;
    let mut quote_condition : Option<FieldQuoteConditionEnum> = None;
    let mut trade_condition : Option<FieldTradeConditionEnum> = None;
    let mut mdentry_originator : Option<String> = None;
    let mut location_id : Option<String> = None;
    let mut desk_id : Option<String> = None;
    let mut open_close_settle_flag : Option<FieldOpenCloseSettleFlagEnum> = None;
    let mut time_in_force : Option<FieldTimeInForceEnum> = None;
    let mut expire_date : Option<UtcDateTime> = None;
    let mut expire_time : Option<UtcDateTime> = None;
    let mut min_qty : Option<f32> = None;
    let mut exec_inst : Option<FieldExecInstEnum> = None;
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

                mdupdate_action = Some( FieldMDUpdateActionEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_DELETEREASON, val: v } => {

                if delete_reason.is_some() { break; }

                delete_reason = Some( FieldDeleteReasonEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_MDENTRYTYPE, val: v } => {

                if mdentry_type.is_some() { break; }

                mdentry_type = Some( FieldMDEntryTypeEnum::from_str(v).unwrap() );
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

            &FieldVal { id: FIELD_FINANCIALSTATUS, val: v } => {

                if financial_status.is_some() { break; }

                financial_status = Some( FieldFinancialStatusEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_CORPORATEACTION, val: v } => {

                if corporate_action.is_some() { break; }

                corporate_action = Some( FieldCorporateActionEnum::from_str(v).unwrap() );
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

                tick_direction = Some( FieldTickDirectionEnum::from_str(v).unwrap() );
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

                quote_condition = Some( FieldQuoteConditionEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_TRADECONDITION, val: v } => {

                if trade_condition.is_some() { break; }

                trade_condition = Some( FieldTradeConditionEnum::from_str(v).unwrap() );
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

                open_close_settle_flag = Some( FieldOpenCloseSettleFlagEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_TIMEINFORCE, val: v } => {

                if time_in_force.is_some() { break; }

                time_in_force = Some( FieldTimeInForceEnum::from_str(v).unwrap() );
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

                exec_inst = Some( FieldExecInstEnum::from_str(v).unwrap() );
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
    NoMDEntries10Fields {
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
    let mut mdentry_type : Option<FieldMDEntryTypeEnum> = None;
    let mut mdentry_px : Option<f32> = None;
    let mut currency : Option<f32> = None;
    let mut mdentry_size : Option<f32> = None;
    let mut mdentry_date : Option<UtcDate> = None;
    let mut mdentry_time : Option<UtcTime> = None;
    let mut tick_direction : Option<FieldTickDirectionEnum> = None;
    let mut mdmkt : Option<String> = None;
    let mut trading_session_id : Option<String> = None;
    let mut quote_condition : Option<FieldQuoteConditionEnum> = None;
    let mut trade_condition : Option<FieldTradeConditionEnum> = None;
    let mut mdentry_originator : Option<String> = None;
    let mut location_id : Option<String> = None;
    let mut desk_id : Option<String> = None;
    let mut open_close_settle_flag : Option<FieldOpenCloseSettleFlagEnum> = None;
    let mut time_in_force : Option<FieldTimeInForceEnum> = None;
    let mut expire_date : Option<UtcDateTime> = None;
    let mut expire_time : Option<UtcDateTime> = None;
    let mut min_qty : Option<f32> = None;
    let mut exec_inst : Option<FieldExecInstEnum> = None;
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

                mdentry_type = Some( FieldMDEntryTypeEnum::from_str(v).unwrap() );
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

                tick_direction = Some( FieldTickDirectionEnum::from_str(v).unwrap() );
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

                quote_condition = Some( FieldQuoteConditionEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_TRADECONDITION, val: v } => {

                if trade_condition.is_some() { break; }

                trade_condition = Some( FieldTradeConditionEnum::from_str(v).unwrap() );
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

                open_close_settle_flag = Some( FieldOpenCloseSettleFlagEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_TIMEINFORCE, val: v } => {

                if time_in_force.is_some() { break; }

                time_in_force = Some( FieldTimeInForceEnum::from_str(v).unwrap() );
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

                exec_inst = Some( FieldExecInstEnum::from_str(v).unwrap() );
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
    NoMDEntries11Fields {
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
    let mut quote_request_type : Option<FieldQuoteRequestTypeEnum> = None;
    let mut trading_session_id : Option<String> = None;
    let mut side : Option<FieldSideEnum> = None;
    let mut order_qty : Option<f32> = None;
    let mut fut_sett_date : Option<UtcDateTime> = None;
    let mut ord_type : Option<FieldOrdTypeEnum> = None;
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

            &FieldVal { id: FIELD_QUOTEREQUESTTYPE, val: v } => {

                if quote_request_type.is_some() { break; }

                quote_request_type = Some( FieldQuoteRequestTypeEnum::from_str(v).unwrap() );
            },

            &FieldVal { id: FIELD_TRADINGSESSIONID, val: v } => {

                if trading_session_id.is_some() { break; }

                trading_session_id = Some( v.to_string() );
            },

            &FieldVal { id: FIELD_SIDE, val: v } => {

                if side.is_some() { break; }

                side = Some( FieldSideEnum::from_str(v).unwrap() );
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

                ord_type = Some( FieldOrdTypeEnum::from_str(v).unwrap() );
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


use bytes::{BytesMut};
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



pub fn write_fix_message(msg: &FixMessage, ts: &UtcDateTime, seq: u32, sender: &str, target: &str, buf: &mut BytesMut) -> Result<(), io::Error> {

    match msg {

        // type: 0
        &FixMessage::Heartbeat(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=0\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_heartbeat_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: 1
        &FixMessage::TestRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=1\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_test_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: 2
        &FixMessage::ResendRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=2\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_resend_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: 3
        &FixMessage::Reject(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=3\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_reject_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: 4
        &FixMessage::SequenceReset(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=4\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_sequence_reset_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: 5
        &FixMessage::Logout(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=5\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_logout_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: 6
        &FixMessage::IOI(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=6\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_ioifields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: 7
        &FixMessage::Advertisement(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=7\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_advertisement_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: 8
        &FixMessage::ExecutionReport(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=8\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_execution_report_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: 9
        &FixMessage::OrderCancelReject(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=9\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_order_cancel_reject_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: A
        &FixMessage::Logon(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=A\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_logon_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: B
        &FixMessage::News(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=B\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_news_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: C
        &FixMessage::Email(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=C\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_email_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: D
        &FixMessage::NewOrderSingle(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=D\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_new_order_single_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: E
        &FixMessage::NewOrderList(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=E\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_new_order_list_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: F
        &FixMessage::OrderCancelRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=F\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_order_cancel_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: G
        &FixMessage::OrderCancelReplaceRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=G\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_order_cancel_replace_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: H
        &FixMessage::OrderStatusRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=H\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_order_status_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: J
        &FixMessage::Allocation(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=J\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_allocation_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: K
        &FixMessage::ListCancelRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=K\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_list_cancel_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: L
        &FixMessage::ListExecute(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=L\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_list_execute_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: M
        &FixMessage::ListStatusRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=M\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_list_status_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: N
        &FixMessage::ListStatus(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=N\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_list_status_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: P
        &FixMessage::AllocationInstructionAck(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=P\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_allocation_instruction_ack_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: Q
        &FixMessage::DontKnowTrade(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=Q\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_dont_know_trade_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: R
        &FixMessage::QuoteRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=R\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_quote_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: S
        &FixMessage::Quote(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=S\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_quote_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: T
        &FixMessage::SettlementInstructions(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=T\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_settlement_instructions_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: V
        &FixMessage::MarketDataRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=V\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_market_data_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: W
        &FixMessage::MarketDataSnapshotFullRefresh(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=W\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_market_data_snapshot_full_refresh_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: X
        &FixMessage::MarketDataIncrementalRefresh(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=X\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_market_data_incremental_refresh_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: Y
        &FixMessage::MarketDataRequestReject(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=Y\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_market_data_request_reject_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: Z
        &FixMessage::QuoteCancel(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=Z\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_quote_cancel_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: a
        &FixMessage::QuoteStatusRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=a\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_quote_status_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: b
        &FixMessage::QuoteAcknowledgement(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=b\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_quote_acknowledgement_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: c
        &FixMessage::SecurityDefinitionRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=c\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_security_definition_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: d
        &FixMessage::SecurityDefinition(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=d\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_security_definition_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: e
        &FixMessage::SecurityStatusRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=e\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_security_status_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: f
        &FixMessage::SecurityStatus(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=f\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_security_status_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: g
        &FixMessage::TradingSessionStatusRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=g\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_trading_session_status_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: h
        &FixMessage::TradingSessionStatus(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=h\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_trading_session_status_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: i
        &FixMessage::MassQuote(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=i\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_mass_quote_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: j
        &FixMessage::BusinessMessageReject(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=j\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_business_message_reject_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: k
        &FixMessage::BidRequest(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=k\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_bid_request_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: l
        &FixMessage::BidResponse(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=l\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
            write_bid_response_fields(box_flds, &mut writer_wrapper)?;
            Ok ( () )
        },
        // type: m
        &FixMessage::ListStrikePrice(ref box_flds) => {
            let mut writer_wrapper = WriteWrapper { buf: buf };
            write!(writer_wrapper, "35=m\u{1}34={seq}\u{1}49={sender}\u{1}52={ts}\u{1}56={target}\u{1}",
                   ts= ts, seq= seq, sender= sender, target= target )?;
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

        write_group_no_related_sym25_fields( val, output )?;
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

        write_group_no_related_sym25_fields( val, output )?;
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

        write_group_no_allocs2_fields( val, output )?;
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

        write_group_no_allocs2_fields( val, output )?;
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

        write_group_no_orders17_fields( val, output )?;
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

        write_group_no_allocs3_fields( val, output )?;
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

        write_group_no_orders16_fields( val, output )?;
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

        write_group_no_related_sym24_fields( val, output )?;
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

        write_group_no_related_sym23_fields( val, output )?;
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

        write_group_no_mdentries11_fields( val, output )?;
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

        write_group_no_mdentries10_fields( val, output )?;
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

        write_group_no_quote_sets22_fields( val, output )?;
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

        write_group_no_related_sym26_fields( val, output )?;
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

        write_group_no_related_sym26_fields( val, output )?;
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

        write_group_no_quote_sets21_fields( val, output )?;
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

        write_group_no_bid_components5_fields( val, output )?;
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

        write_group_no_bid_components4_fields( val, output )?;
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
    if flds.alloc_shares.is_some() {
        let val = flds.alloc_shares.as_ref().unwrap();

        write!(output, "80={}\u{01}", val )?; // FIELD_ALLOCSHARES
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



fn write_group_no_related_sym25_fields( group: &Vec<NoRelatedSym25Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NORELATEDSYM, len )?;

    for g in group {
        write_group_no_related_sym25_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_related_sym25_fields_line( flds: &NoRelatedSym25Fields, output: &mut Write) -> Result<(), io::Error> {

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



fn write_group_no_bid_components4_fields( group: &Vec<NoBidComponents4Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOBIDCOMPONENTS, len )?;

    for g in group {
        write_group_no_bid_components4_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_bid_components4_fields_line( flds: &NoBidComponents4Fields, output: &mut Write) -> Result<(), io::Error> {

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



fn write_group_no_related_sym26_fields( group: &Vec<NoRelatedSym26Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NORELATEDSYM, len )?;

    for g in group {
        write_group_no_related_sym26_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_related_sym26_fields_line( flds: &NoRelatedSym26Fields, output: &mut Write) -> Result<(), io::Error> {

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



fn write_group_no_quote_sets22_fields( group: &Vec<NoQuoteSets22Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOQUOTESETS, len )?;

    for g in group {
        write_group_no_quote_sets22_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_quote_sets22_fields_line( flds: &NoQuoteSets22Fields, output: &mut Write) -> Result<(), io::Error> {

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



fn write_group_no_orders17_fields( group: &Vec<NoOrders17Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOORDERS, len )?;

    for g in group {
        write_group_no_orders17_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_orders17_fields_line( flds: &NoOrders17Fields, output: &mut Write) -> Result<(), io::Error> {

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

        write_group_no_allocs2_fields( val, output )?;
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



fn write_group_no_bid_components5_fields( group: &Vec<NoBidComponents5Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOBIDCOMPONENTS, len )?;

    for g in group {
        write_group_no_bid_components5_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_bid_components5_fields_line( flds: &NoBidComponents5Fields, output: &mut Write) -> Result<(), io::Error> {

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



fn write_group_no_quote_sets21_fields( group: &Vec<NoQuoteSets21Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOQUOTESETS, len )?;

    for g in group {
        write_group_no_quote_sets21_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_quote_sets21_fields_line( flds: &NoQuoteSets21Fields, output: &mut Write) -> Result<(), io::Error> {

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



fn write_group_no_orders16_fields( group: &Vec<NoOrders16Fields>, output: &mut Write ) -> Result<(), io::Error> {
    let len = group.len();
    write!(output, "{}={}\u{01}", FIELD_NOORDERS, len )?;

    for g in group {
        write_group_no_orders16_fields_line( g, output )?;
    }

    Ok( () )
}

fn write_group_no_orders16_fields_line( flds: &NoOrders16Fields, output: &mut Write) -> Result<(), io::Error> {

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










