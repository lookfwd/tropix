extern crate tropix;
extern crate rustc_serialize;
extern crate time;

use tropix::poloniex::public::*;
use tropix::poloniex::private::*;
use rustc_serialize::{Decodable, Decoder};
use rustc_serialize::json::{self, ToJson, Json};

use std::io;
use std::io::Read;
use std::thread::sleep;
use std::time::Duration;
use std::io::{BufRead};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct Trade {
	date: String,
	type_name: String,
	rate: f64,
	amount: f64,
	total: f64
}


fn main() {

	let mut pair_vec = Vec::new();
    pair_vec.push("BTC_ETC".to_string());
    pair_vec.push("BTC_AMP".to_string());
	pair_vec.push("BTC_LTC".to_string());
    pair_vec.push("BTC_DAO".to_string());
    pair_vec.push("BTC_LSK".to_string());
	pair_vec.push("BTC_MAID".to_string());
	pair_vec.push("BTC_DOGE".to_string());
	pair_vec.push("BTC_XMR".to_string());
	pair_vec.push("BTC_ETH".to_string());
	pair_vec.push("BTC_FCT".to_string());
	pair_vec.push("BTC_OMNI".to_string());
	pair_vec.push("BTC_QORA".to_string());
	pair_vec.push("BTC_DASH".to_string());
	pair_vec.push("BTC_SYS".to_string());
	pair_vec.push("BTC_XRP".to_string());
	pair_vec.push("BTC_DCR".to_string());
	pair_vec.push("BTC_ZEC".to_string());
	
	let mut intd = 0;
    for pair in &pair_vec {
    	println!("{:?}, : {:?}", intd, pair);
    	intd += 1;
    }

	println!("choose a pair to trade by its index., enter the number before the pair you want to trade");
    let mut input1 = String::new();
    let stdin1 = io::stdin();
    stdin1.lock().read_line(&mut input1).unwrap();

    let pair_ind = input1.trim_right_matches("\n");

    let pair_ind2: usize = pair_ind.parse().ok().expect("pair index is not a number");


    println!("How far back do you want to process?");
    println!("Enter the number of hours back you want to check");
    let mut input2 = String::new();
    let stdin2 = io::stdin();
    stdin2.lock().read_line(&mut input2).unwrap();

    let next_string = &input2.trim_right_matches("\n");
	
    let hours_number: i64 = next_string.parse().ok().expect("this was not a number input");

    let date_back = hours_number * 3600;

    let end = time::get_time().sec;

    let start = end - date_back;

    println!("{:?}", &pair_vec[pair_ind2]);

    let trade_data = returnPublicTradeHistory(pair_vec[pair_ind2].to_string(), start, end).to_string();
	let json = Json::from_str(&trade_data).unwrap();


	let trade_result_string: String = json::encode(&json).unwrap();

	let result = trade_result_string.replace("type", "type_name");

	let trade_json: Vec<Trade> = json::decode(&result).unwrap();

	let mut buys = 0f64;
	let mut sells = 0f64;
	for trade in trade_json {
		if trade.type_name == "buy" {
			buys += trade.amount;
		} else {
			sells += trade.amount;
		}
	}

	println!("buys {:?}", buys);
	println!("sells {:?}", sells);
}
