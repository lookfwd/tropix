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


fn main() {
	let mut pair_vec = Vec::new();
	pair_vec.push("BTC_LTC".to_string());
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
	//api Key

    println!("This robot will automatically sell a pair you will select on Poloniex\nYou will enter a maxmimum sell amount per interval\nand a frequency to buy, \n
    	every x seconds the robot will sell up to the maximum quantity");
    println!("Enter Your Poloniex Api Key");
    let mut input1 = String::new();
    let stdin1 = io::stdin();
    stdin1.lock().read_line(&mut input1).unwrap();

    let next_string = &input1.trim_right_matches("\n");
	
	let the_api_key = &next_string;

	let the_api_keyclone = the_api_key.clone();
//secret Key
	println!("Enter Your Poloniex Secret Key");
    let mut input2 = String::new();
    let stdin2 = io::stdin();
    stdin2.lock().read_line(&mut input2).unwrap();

    let the_secret_trimmed = input2.trim_right_matches("\n");

    let mut intd = 0;
    for pair in &pair_vec {
    	println!("{:?}, : {:?}", intd, pair);
    	intd += 1;
    }
    println!("choose a pair to trade by its index., enter the number before the pair you want to trade");
    let mut input5 = String::new();
    let stdin5 = io::stdin();
    stdin5.lock().read_line(&mut input5).unwrap();

    let pair_ind = input5.trim_right_matches("\n");

    let pair_ind2: usize = pair_ind.parse().ok().expect("pair index is not a number");

    println!("Enter a maximum position limit in the alt coin quantity \n the bot will sell only up to this much at a time");
    let mut input3 = String::new();
    let stdin3 = io::stdin();
    stdin3.lock().read_line(&mut input3).unwrap();

    let maxposition = input3.trim_right_matches("\n").to_string();
    let postion_clone = maxposition.clone();


    println!("Enter a buy frequency in seconds a value of 60 will mean 1 minute");
    let mut input4 = String::new();
    let stdin4 = io::stdin();
    stdin4.lock().read_line(&mut input4).unwrap();

    let frequency = input4.trim_right_matches("\n").to_string();

    let frequency: u64 = frequency.parse().ok().expect("frequency is not a number");
    

    println!("for margin trading enter 1 \n for non margin trading enter 0");
    let mut input6 = String::new();
    let stdin6 = io::stdin();
    stdin6.lock().read_line(&mut input6).unwrap();

    let margin_ind = input6.trim_right_matches("\n").to_string();

    let margin_ind: u64 = margin_ind.parse().ok().expect("frequency is not a number");

	let mut xyz = 0;
	while xyz == 0 {

		let all_orders = returnOrderBook(pair_vec[pair_ind2].to_string());
		let json = Json::from_str(&all_orders).unwrap();

   		let ticker_result = json.find_path(&[&"bids"]).unwrap();

		let ticker_result_string: String = json::encode(&ticker_result).unwrap();

		let bids_results: Vec<(f64, f64)> = json::decode(&ticker_result_string).unwrap();


		let timespec = time::get_time();
		let mut mills = timespec.sec * 100;

		if bids_results[0].1 < postion_clone.parse().ok().expect("max position turned out to not be a number either") {
            if margin_ind == 0 {
                sell(the_api_keyclone.to_string(), the_secret_trimmed, pair_vec[pair_ind2].to_string(), bids_results[0].0.to_string(), bids_results[0].1.to_string(), mills.to_string());
            } else {
                marginSell(the_api_keyclone.to_string(), the_secret_trimmed, pair_vec[pair_ind2].to_string(), bids_results[0].0.to_string(), bids_results[0].1.to_string(), mills.to_string());
            }
		}	else {
            if margin_ind == 0 {
                sell(the_api_keyclone.to_string(), the_secret_trimmed, pair_vec[pair_ind2].to_string(), bids_results[0].0.to_string(), maxposition.to_string(), mills.to_string());
            } else {
                marginSell(the_api_keyclone.to_string(), the_secret_trimmed, pair_vec[pair_ind2].to_string(), bids_results[0].0.to_string(), maxposition.to_string(), mills.to_string());
            }
		}

		

		let the_seconds = Duration::new(frequency, 0);
		sleep(the_seconds);

	}
}