
use curl::http;

use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha512;

use std::io::Read;

use rustc_serialize::hex::ToHex;
use rustc_serialize::json::{self, Json};
use serde_json;

#[derive(Deserialize, Serialize)]
pub struct OrderBookLine {
    #[serde(rename="Quantity")]
	pub quantity: f64,
    #[serde(rename="Rate")]
	pub rate: f64,
}

#[derive(Deserialize, Serialize)]
pub struct OrderBook {
	pub buy: Vec<OrderBookLine>,
	pub sell: Vec<OrderBookLine>,
}

#[derive(Deserialize, Serialize)]
pub struct Balances {
	#[serde(rename="Currency")]
	pub currency: String,
	#[serde(rename="Balance")]
	pub balance: f64,
	#[serde(rename="Available")]
	pub available: f64,
	#[serde(rename="Pending")]
	pub pending: f64,
}

#[derive(Deserialize, Serialize)]
pub struct OpenOrder {
    #[serde(rename="Uuid")]
	pub uuid: Option<String>,
	#[serde(rename="OrderUuid")]
	pub order_uuid: String,
	#[serde(rename="Exchange")]
	pub exchange: String,
	#[serde(rename="OrderType")]
	pub order_type: String,
	#[serde(rename="Quantity")]
	pub quantity: f64,
	#[serde(rename="QuantityRemaining")]
	pub quantity_remaining: f64,
	#[serde(rename="Limit")]
	pub limit: f64,
	#[serde(rename="CommissionPaid")]
	pub commission_paid: f64,
	#[serde(rename="Price")]
	pub price: f64,
	#[serde(rename="PricePerUnit")]
	pub price_per_unit: Option<String>,
	#[serde(rename="Opened")]
	pub opened: String,
	#[serde(rename="Closed")]
	pub closed: Option<String>,
	#[serde(rename="CancelInitiated")]
	pub cancel_initiated: bool,
	#[serde(rename="ImmediateOrCancel")]
	pub immediate_or_cancel: bool,
	#[serde(rename="IsConditional")]
	pub is_conditional: bool,
	#[serde(rename="Condition")]
	pub condition: Option<String>,
	#[serde(rename="ConditionTarget")]
	pub condition_target: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct BidAskLast {
	#[serde(rename="Bid")]
	pub bid: f64,
	#[serde(rename="Ask")]
	pub ask: f64,
	#[serde(rename="Last")]
	pub last: f64,
}

fn sign(secretkey: String, url: String) -> String {
	let the_secret_bytes = secretkey[..].as_bytes();

	//hmac-sha512 signature of uri
	let the_sha = Sha512::new();
	let mut the_new_mac = Hmac::new(the_sha, the_secret_bytes);
	the_new_mac.input(url.as_bytes());
	the_new_mac.result().code().to_hex().to_string()
}

enum ArgumentsForRequest {
    Signed(String, String, String),
	Simple(String, String)
}

use self::ArgumentsForRequest::{Signed, Simple};

fn request(arg: ArgumentsForRequest) -> String {
    let resp = match arg {
        Simple(url, parameters) => http::handle()
                                   .post(url, &parameters)
                                   .exec().unwrap(),
        Signed(secretkey, url, parameters) => http::handle()
                                              .post(url.clone(), &parameters)
                                              .header("apisign", &sign(secretkey, url))
                                              .exec().unwrap()
    };
	let mut data = String::new();
    resp.get_body().read_to_string(&mut data).unwrap();
	data
}

fn get_result(data: String) -> String {
    let json = Json::from_str(&data).unwrap();
    let this_part = json.find("result");
    let this_data: String = json::encode(&this_part).unwrap();
	this_data
}

fn parse_balances(data: String) -> Vec<Balances> {
	let result: String = get_result(data);
    let the_balances: Vec<Balances> = serde_json::from_str(&result).unwrap();
    the_balances
}

pub fn get_balances(apikey: &str, secretkey: &str) -> Vec<Balances> {
	let api_keystring = "apikey=".to_string() + apikey;
	let api_nonce = "&nonce=1";
	let parameters = "".to_string() + &api_keystring + &api_nonce;
	let begin_url = "https://bittrex.com/api/v1.1/account/getbalances?".to_string() + &parameters;

	parse_balances(request(Signed(secretkey.to_string(), begin_url, parameters)))
}


pub fn buy_limit(apikey: &str, secretkey: &str, firstcoin: &str, secondcoin: &str, quantity: &str, rate: &str) -> String {

  //Buy
//secret Key
	let api_keystring = "apikey=".to_string() + apikey;
	let market_selection = "&market=".to_string() + firstcoin + &"-".to_string() + secondcoin;
	let market_quantity = "&quantity=".to_string() + quantity;
	let market_rate = "&rate=".to_string() + rate;
	let api_nonce = "&nonce=1";
	let parameters = "".to_string() + &api_keystring + &market_selection + &market_quantity + &market_rate + &api_nonce;

	let begin_url = "https://bittrex.com/api/v1.1/market/buylimit?".to_string() + &parameters;
	let the_url_clone = begin_url.clone();
    let resp_orderbook = http::handle()
		.post(the_url_clone, &parameters)
		.header("apisign", &sign(secretkey.to_string(), begin_url))
		.exec().unwrap();

	let mut data = String::new();
    resp_orderbook.get_body().read_to_string(&mut data).unwrap();

    let json = Json::from_str(&data).unwrap();

    let thetrade_string: String = json::encode(&json).unwrap();
    thetrade_string
}

pub fn sell_limit(apikey: &str, secretkey: &str, firstcoin: &str, secondcoin: &str, quantity: &str, rate: &str) -> String {

	let api_keystring = "apikey=".to_string() + apikey;
	let market_selection = "&market=".to_string() + firstcoin + &"-".to_string() + secondcoin;
	let market_quantity = "&quantity=".to_string() + quantity;
	let market_rate = "&rate=".to_string() + rate;
	let api_nonce = "&nonce=1";
	let parameters = "".to_string() + &api_keystring + &market_selection + &market_quantity + &market_rate + &api_nonce;

	let begin_url = "https://bittrex.com/api/v1.1/market/selllimit?".to_string() + &parameters;
	let the_url_clone = begin_url.clone();
    let resp_orderbook = http::handle()
		.post(the_url_clone, &parameters)
		.header("apisign", &sign(secretkey.to_string(), begin_url))
		.exec().unwrap();

	let mut data = String::new();
    resp_orderbook.get_body().read_to_string(&mut data).unwrap();

    let json = Json::from_str(&data).unwrap();

    let thetrade_string: String = json::encode(&json).unwrap();
    thetrade_string
}

fn parse_open_order(data: String) -> Vec<OpenOrder> {
    let result: String = get_result(data);
    let open_order: Vec<OpenOrder> = serde_json::from_str(&result).unwrap();
    open_order
}

pub fn get_openorders(apikey: &str, secretkey: &str, firstcoin: &str, secondcoin: &str) -> Vec<OpenOrder> {

	let api_keystring = "apikey=".to_string() + apikey;
	let market_selection = "&market=".to_string() + firstcoin + &"-".to_string() + secondcoin;
	let api_nonce = "&nonce=1";
	let parameters = "".to_string() + &api_keystring + &market_selection + &api_nonce;

	let begin_url = "https://bittrex.com/api/v1.1/market/getopenorders?".to_string() + &parameters;
	
	parse_open_order(request(Signed(secretkey.to_string(), begin_url, parameters)))
}

fn parse_orderbook(data: String) -> OrderBook {
    let result: String = get_result(data);
    let orderbook: OrderBook = serde_json::from_str(&result).unwrap();
    orderbook
}

pub fn get_orderbook(secretkey: &str, firstcoin: &str, secondcoin: &str, depth: &str) -> OrderBook {

	let market_selection = "market=".to_string() + firstcoin + &"-".to_string() + secondcoin;
	let market_type = "&type=both".to_string();
	let market_depth = "&depth=".to_string() + depth;
	let parameters = "".to_string() + &market_selection + &market_type + &market_depth;

	let begin_url = "https://bittrex.com/api/v1.1/public/getorderbook?".to_string() + &parameters;
	
	parse_orderbook(request(Signed(secretkey.to_string(), begin_url, parameters)))
}

pub fn cancel_order(apikey: &str, secretkey: &str, uuid: &str) -> String {
	let api_keystring = "apikey=".to_string() + apikey;
	let the_uuid = "&uuid=".to_string() + uuid;
	let api_nonce = "&nonce=1";
	let parameters = "".to_string() + &api_keystring + &the_uuid + &api_nonce;

	let begin_url = "https://bittrex.com/api/v1.1/market/cancel?".to_string() + &parameters;
	let the_url_clone = begin_url.clone();
	//let the_digestive = Digest::input(the_digestive, &the_uri_bytes);
    let resp_openorders = http::handle()
		.post(the_url_clone, &parameters)
		.header("apisign", &sign(secretkey.to_string(), begin_url))
		.exec().unwrap();

	let mut data = String::new();
    resp_openorders.get_body().read_to_string(&mut data).unwrap();
    data
}

fn parse_ticker(data: String) -> BidAskLast {
    let result: String = get_result(data);
    let bid_ask_last: BidAskLast = serde_json::from_str(&result).unwrap();
    bid_ask_last
}

pub fn get_ticker(firstcoin: &str, secondcoin: &str) -> BidAskLast {
	let market = "market=".to_string() + firstcoin + "-" + secondcoin;
	let parameters = "".to_string() + &market;
	let begin_url = "https://bittrex.com/api/v1.1/public/getticker?".to_string() + &parameters;
	
	parse_ticker(request(Simple(begin_url, parameters)))
}

#[test]
fn test_sign() {
	let secretkey: String = String::from("foobar");
	let url: String = String::from("https://bittrex.com/api/v1.1/account/getbalances");
	let signature: String = sign(secretkey, url);
	assert_eq!("e5689e844f911567ff84230eac7d0f78397b542967".to_owned() + 
	           "c09fee319045e147c164f7551253c031cc197fcb3a" + 
			   "09060f040103d2d826ee6bd40000f8178e6a09746c99", signature);
}

#[test]
fn test_parse_open_order() {
	let response = r#"
	{
		"success" : true,
		"message" : "",
		"result" : [{
				"Uuid" : null,
				"OrderUuid" : "09aa5bb6-8232-41aa-9b78-a5a1093e0211",
				"Exchange" : "BTC-LTC",
				"OrderType" : "LIMIT_SELL",
				"Quantity" : 5.00000000,
				"QuantityRemaining" : 5.00000000,
				"Limit" : 2.00000000,
				"CommissionPaid" : 0.00000000,
				"Price" : 0.00000000,
				"PricePerUnit" : null,
				"Opened" : "2014-07-09T03:55:48.77",
				"Closed" : null,
				"CancelInitiated" : false,
				"ImmediateOrCancel" : false,
				"IsConditional" : false,
				"Condition" : null,
				"ConditionTarget" : null
			}, {
				"Uuid" : null,
				"OrderUuid" : "8925d746-bc9f-4684-b1aa-e507467aaa99",
				"Exchange" : "BTC-LTC",
				"OrderType" : "LIMIT_BUY",
				"Quantity" : 100000.00000000,
				"QuantityRemaining" : 100000.00000000,
				"Limit" : 0.00000001,
				"CommissionPaid" : 0.00000000,
				"Price" : 0.00000000,
				"PricePerUnit" : null,
				"Opened" : "2014-07-09T03:55:48.583",
				"Closed" : null,
				"CancelInitiated" : false,
				"ImmediateOrCancel" : false,
				"IsConditional" : false,
				"Condition" : null,
				"ConditionTarget" : null
			}
		]
	}
	"#;
	
	let data = String::from(response);

	let decoded: Vec<OpenOrder> = parse_open_order(data);

	assert_eq!(2, decoded.len());
	assert_eq!("09aa5bb6-8232-41aa-9b78-a5a1093e0211", decoded[0].order_uuid);
	assert_eq!("8925d746-bc9f-4684-b1aa-e507467aaa99", decoded[1].order_uuid);
}

#[test]
fn test_parse_orderbook() {
	let response = r#"
		    {
			"success" : true,
			"message" : "",
			"result" : {
				"buy" : [{
						"Quantity" : 12.37000000,
						"Rate" : 0.02525000
					}
				],
				"sell" : [{
						"Quantity" : 32.55412402,
						"Rate" : 0.02540000
					}, {
						"Quantity" : 60.00000000,
						"Rate" : 0.02550000
					}, {
						"Quantity" : 60.00000000,
						"Rate" : 0.02575000
					}, {
						"Quantity" : 84.00000000,
						"Rate" : 0.02600000
					}
				]
			}
		}
	"#;
	
	let data = String::from(response);

	let decoded: OrderBook = parse_orderbook(data);
	
	assert_eq!(1, decoded.buy.len());
	assert_eq!(4, decoded.sell.len());
	
	assert!((12.37 - decoded.buy[0].quantity).abs() < 0.01);
	assert!((0.025 - decoded.buy[0].rate).abs() < 0.001);

	assert!((84.00 - decoded.sell[3].quantity).abs() < 0.01);
	assert!((0.026 - decoded.sell[3].rate).abs() < 0.001);
}

#[test]
fn test_parse_balances() {
	let response = r#"
		    {
			"success" : true,
			"message" : "",
			"result" : [{
					"Currency" : "DOGE",
					"Balance" : 0.00000000,
					"Available" : 0.00000000,
					"Pending" : 0.00000000,
					"CryptoAddress" : "DLxcEt3AatMyr2NTatzjsfHNoB9NT62HiF",
					"Requested" : false,
					"Uuid" : null

				}, {
					"Currency" : "BTC",
					"Balance" : 14.21549076,
					"Available" : 14.21549076,
					"Pending" : 0.00000000,
					"CryptoAddress" : "1Mrcdr6715hjda34pdXuLqXcju6qgwHA31",
					"Requested" : false,
					"Uuid" : null
				}
			]
		}
	"#;
	
	let data = String::from(response);

	let decoded: Vec<Balances> = parse_balances(data);
	
	assert_eq!(2, decoded.len());
	
	assert_eq!("DOGE", decoded[0].currency);
	assert_eq!("BTC", decoded[1].currency);
	
	assert!((0.00 - decoded[0].balance).abs() < 0.01);
	assert!((14.21 - decoded[1].balance).abs() < 0.01);
}

#[test]
fn test_parse_ticker() {
	let response = r#"
		{
			"success" : true,
			"message" : "",
			"result" : {
				"Bid" : 2.05670368,
				"Ask" : 3.35579531,
				"Last" : 3.35579531
			}
		}
	"#;
	
	let data = String::from(response);

	let decoded: BidAskLast = parse_ticker(data);
	
	assert!((2.05 - decoded.bid).abs() < 0.01);
	assert!((3.35 - decoded.ask).abs() < 0.01);
	assert!((3.35 - decoded.last).abs() < 0.01);
}
