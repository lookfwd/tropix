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
	
	//api Key

    println!("This robot will automatically buy MAID on Poloniex\nYou will enter a maxmimum buy amount per interval\nand a frequency to buy, \n
    	every x seconds the robot will buy up to the maximum quantity");
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

    println!("Enter a maximum position limit in MAID quantities");
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

	let mut xyz = 0;
	while xyz == 0 {

		let all_orders = returnOrderBook("BTC_MAID".to_string());
		let json = Json::from_str(&all_orders).unwrap();

   		let ticker_result = json.find_path(&[&"asks"]).unwrap();

		let ticker_result_string: String = json::encode(&ticker_result).unwrap();

		let ask_results: Vec<(f64, f64)> = json::decode(&ticker_result_string).unwrap();


		let timespec = time::get_time();
		let mut mills = timespec.sec * 100;

		if ask_results[0].1 < postion_clone.parse().ok().expect("max position turned out to not be a number either") {
			buy(the_api_keyclone.to_string(), the_secret_trimmed, "BTC_MAID".to_string(), ask_results[0].0.to_string(), ask_results[0].1.to_string(), mills.to_string());
		}	else {
			buy(the_api_keyclone.to_string(), the_secret_trimmed, "BTC_MAID".to_string(), ask_results[0].0.to_string(), maxposition.to_string(), mills.to_string());
		}

		

		let the_seconds = Duration::new(frequency, 0);
		sleep(the_seconds);

	}
}