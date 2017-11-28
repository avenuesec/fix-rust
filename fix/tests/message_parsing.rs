

extern crate fix;

use fix::*;


#[test]
fn test_logon_message() {
    let line = "8=FIX.4.4|9=98|35=A|34=1|49=CCLRA301|52=20170627-14:23:04.660|56=OE101C|95=6|96=YWEKNJ|98=0|108=20|141=Y|35002=0|10=103|\r\n".replace("|", "\x01");
	let b = frame::parse(line.as_bytes());
    println!("{:?}", b);
}

#[test]
fn test_heartbeat_message() {
    let line = "8=FIX.4.4|9=57|35=0|49=OE101C|56=CCLRA301|34=2|52=20170627-14:23:54.802|10=065|\r\n".replace("|", "\x01");
	let b = frame::parse(line.as_bytes());
    println!("{:?}", b);
}

#[test]
fn test_new_order_single_message() {
    let line = "8=FIX.4.4|9=206|35=D|34=2|49=CCLRA301|52=20170627-19:32:13.105|56=OE101C|1=4004|11=30011_0|38=100|40=2|44=5|54=1|55=PETR4|59=0|60=20170627-16:32:13|453=3|448=CCLRA301|447=D|452=36|448=308|447=D|452=7|448=DMA1|447=D|452=31|10=207|\r\n".replace("|", "\x01");
	let b = frame::parse(line.as_bytes());
    // println!("{:?}", b);
    // let expected = FixFrame { 
    //     ts: 2017-06-27T19:32:13.105Z, 
    //     seq: 2, 
    //     sender_comp_id: "CCLRA301", 
    //     target_comp_id: "OE101C", 
    //     msg: NewOrderSingle(NewOrderSingleFields { 
    //             cl_ord_id: "30011_0", 
    //             parties: Some(PartiesFields { no_party_ids: 
                        // Some([NoPartyIDs4Fields { party_id: Some("CCLRA301"), party_idsource: Some(PROPCODE), party_role: Some(ENTERINGTRADER), no_party_sub_ids: None }, 
                        //       NoPartyIDs4Fields { party_id: Some("308"), party_idsource: Some(PROPCODE), party_role: Some(ENTERINGFIRM), no_party_sub_ids: None }, 
                        //       NoPartyIDs4Fields { party_id: Some("DMA1"), party_idsource: Some(PROPCODE), party_role: Some(SUBCUSTODIAN), no_party_sub_ids: None }]) })
    //             trade_origination_date: None, 
    //             trade_date: None, 
    //             account: Some("4004"), 
    //             instrument: InstrumentFields { symbol: Some("PETR4") }, 
    //             side: BUY, 
    //             transact_time: 2017-06-27 16:32:13 UTC, 
    //             order_qty_data: OrderQtyDataFields { order_qty: Some(100), cash_order_qty: None, order_percent: None, rounding_direction: None, rounding_modulus: None }, 
    //             ord_type: LIMIT, 
    //             price: Some(5), 
    //             time_in_force: Some(DAY) }) 
    //         };
}
