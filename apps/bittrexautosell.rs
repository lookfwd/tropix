extern crate tropix;
extern crate rustc_serialize;
extern crate time;

use tropix::bittrex::bittrex::*;

use rustc_serialize::{Decodable, Decoder};
use rustc_serialize::json::{self, ToJson, Json};

use std::io;
use std::io::Read;
use std::thread::sleep;
use std::time::Duration;
use std::io::{BufRead};

fn main() {

	let pair_vec = vec!["SAFEX", "ETH", "MAID", "ETC", "BCC"];

	println!("Enter Your Bittrex Api Key");
    let mut input1 = String::new();
    let stdin1 = io::stdin();
    stdin1.lock().read_line(&mut input1).unwrap();

    let next_string = &input1.trim_right_matches("\n");
	
	let the_api_key = &next_string;
//secret Key
	println!("Enter Your Bittrex Secret Key");
    let mut input2 = String::new();
    let stdin2 = io::stdin();
    stdin2.lock().read_line(&mut input2).unwrap();

    let the_secret_trimmed = input2.trim_right_matches("\n");

	let mut intd = 0;
     for pair in &pair_vec {
    	println!("{:?}, : {:?}", intd, pair);
    	intd += 1;
    }

    let firstcoin = "BTC".to_string();

    println!("choose a currency to trade by its index., enter the number before the currency you want to trade");
    let mut input5 = String::new();
    let stdin5 = io::stdin();
    stdin5.lock().read_line(&mut input5).unwrap();

    let pair_ind = input5.trim_right_matches("\n");

    let pair_ind2: usize = pair_ind.parse().ok().expect("currency index is not a number");

    let secondcoin = pair_vec[pair_ind2];

    println!("Enter a maximum position limit in the alt coin quantity \n the bot will sell only up to this much at a time");
    let mut input3 = String::new();
    let stdin3 = io::stdin();
    stdin3.lock().read_line(&mut input3).unwrap();

    let maxposition = input3.trim_right_matches("\n").to_string();
    let position_clone = maxposition.clone().parse().ok().expect("max position turned out to not be a number either");


    println!("Enter a sell frequency in seconds a value of 60 will mean 1 minute");
    let mut input4 = String::new();
    let stdin4 = io::stdin();
    stdin4.lock().read_line(&mut input4).unwrap();

    let frequency = input4.trim_right_matches("\n").to_string();

    let frequency: u64 = frequency.parse().ok().expect("frequency is not a number");

    println!("pair {} / {}", &firstcoin, &secondcoin);
    println!("max position {}", &maxposition);
    println!("frequency {} seconds", &frequency);
    let mut xyz = 0;
	while xyz == 0 {

		
    	let order_book = get_orderbook(&the_secret_trimmed, &firstcoin, &secondcoin, "500");
    	for buy in order_book.buy {

			if buy.Quantity > position_clone {
    			let the_trade = sell_limit(&the_api_key, &the_secret_trimmed, &firstcoin, &secondcoin, &position_clone.to_string(), &buy.Rate.to_string());
    			println!("bought {}, {}", position_clone, the_trade);
    			break;
			} else {
				let the_trade = sell_limit(&the_api_key, &the_secret_trimmed, &firstcoin, &secondcoin, &buy.Quantity.to_string(), &buy.Rate.to_string());
				println!("bought {}, {}", &buy.Quantity, the_trade);
				break;
			}    								
    	}

		let the_seconds = Duration::new(frequency, 0);
		sleep(the_seconds);

	}
}
