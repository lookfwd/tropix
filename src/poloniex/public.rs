
use iron::prelude::*;
use iron::status;
use url::form_urlencoded;

use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::digest::Digest;
use crypto::sha2::Sha512;

use std::io;
use std::io::Read;
use std::thread::sleep;
use std::time::Duration;
use std::io::{BufRead};
use std::ptr::null;

use rustc_serialize::hex::ToHex;
use rustc_serialize::{Decodable, Decoder};
use rustc_serialize::json::{self, ToJson, Json};

use hyper::Client;
use hyper::header::Connection;
use hyper::header::Headers;

use poloniex::connection::apiConnect;


//There are six public methods, all of which take HTTP GET requests and return output in JSON format:

pub fn returnLowestAsk(pair: String) -> String {
	let client = Client::new();
		//this_g.head(url)
    let mut results = client.get("https://poloniex.com/public?command=returnTicker")
    .header(Connection::close())
    .send().unwrap();

    let mut payload = String::new();
   	results.read_to_string(&mut payload).unwrap();

   	let json = Json::from_str(&payload).unwrap();

    let ticker_result = json.find_path(&[&pair, "lowestAsk"]).unwrap();
  let ticker_result_string: String = json::encode(&ticker_result).unwrap();
  let lowestAsk: String = json::decode(&ticker_result_string).unwrap();

	lowestAsk
}

pub fn returnHighestBid(pair: String) -> String {
	let client = Client::new();
		//this_g.head(url)
  let mut results = client.get("https://poloniex.com/public?command=returnTicker")
    .header(Connection::close())
    .send().unwrap();

  let mut payload = String::new();
  results.read_to_string(&mut payload).unwrap();

  let json = Json::from_str(&payload).unwrap();

  let ticker_result = json.find_path(&[&pair, "highestBid"]).unwrap();
  let ticker_result_string: String = json::encode(&ticker_result).unwrap();
  let highestBid: String = json::decode(&ticker_result_string).unwrap();

	highestBid
}

/*    Returns the ticker for all markets. Sample output:

    {"BTC_LTC":{"last":"0.0251","lowestAsk":"0.02589999","highestBid":"0.0251","percentChange":"0.02390438",
    "baseVolume":"6.16485315","quoteVolume":"245.82513926"},"BTC_NXT":{"last":"0.00005730","lowestAsk":"0.00005710",
    "highestBid":"0.00004903","percentChange":"0.16701570","baseVolume":"0.45347489","quoteVolume":"9094"}, ... }

    Call: https://poloniex.com/public?command=returnTicker */

pub fn return24Volume() {
	
}

/*    Returns the 24-hour volume for all markets, plus totals for primary currencies. Sample output:

    {"BTC_LTC":{"BTC":"2.23248854","LTC":"87.10381314"},"BTC_NXT":{"BTC":"0.981616","NXT":"14145"}, ... "totalBTC":"81.89657704","totalLTC":"78.52083806"}

    Call: https://poloniex.com/public?command=return24hVolume */

pub fn returnOrderBook(pair: String) -> String {

  let url = "http://poloniex.com/public?command=returnOrderBook&currencyPair=".to_string() + &pair + "&depth=250";

  let client = Client::new();
    //this_g.head(url)
  let mut results = client.get(&url)
    .header(Connection::close())
    .send().unwrap();

  let mut payload = String::new();

  results.read_to_string(&mut payload).unwrap();

  let json = Json::from_str(&payload).unwrap();

  let order_book_string: String = json::encode(&json).unwrap();

  order_book_string
}

/*    Returns the order book for a given market. You may also specify "all" to get the orderbooks of all markets. Sample output:

    {"asks":[[0.00007600,1164],[0.00007620,1300], ... "bids":[[0.00006901,200],[0.00006900,408], ... }

    Or, for all markets:

    {"BTC_NXT":{"asks":[[0.00007600,1164],[0.00007620,1300], ... "bids":[[0.00006901,200],[0.00006900,408], ... },"BTC_XMR":...}

    Call: http://poloniex.com/public?command=returnOrderBook&currencyPair=BTC_NXT&depth=50 */

pub fn returnPublicTradeHistory() {

}

 /*   Returns the past 200 trades for a given market, or all of the trades between a range specified in UNIX timestamps by the "start" and "end" GET parameters. Sample output:

    [{"date":"2014-02-10 04:23:23","type":"buy","rate":"0.00007600","amount":"140","total":"0.01064"},{"date":"2014-02-10 01:19:37","type":"buy","rate":"0.00007600","amount":"655","total":"0.04978"}, ... ]

    Call: https://poloniex.com/public?command=returnTradeHistory&currencyPair=BTC_NXT&start=1410158341&end=1410499372 */

pub fn returnChartData() {

}

/*    Returns candlestick chart data. Required GET parameters are "currencyPair", "period" (candlestick period in seconds; valid values are 300, 900, 1800, 7200, 14400, and 86400), "start", and "end". "Start" and "end" are given in UNIX timestamp format and used to specify the date range for the data returned. Sample output:

    [{"date":1405699200,"high":0.0045388,"low":0.00403001,"open":0.00404545,"close":0.00427592,"volume":44.11655644,
    "quoteVolume":10259.29079097,"weightedAverage":0.00430015}, ...]

    Call: https://poloniex.com/public?command=returnChartData&currencyPair=BTC_XMR&start=1405699200&end=9999999999&period=14400 */

pub fn returnCurrencies() {

}

/*    Returns information about currencies. Sample output:

    {"1CR":{"maxDailyWithdrawal":10000,"txFee":0.01,"minConf":3,"disabled":0},"ABY":{"maxDailyWithdrawal":10000000,"txFee":0.01,"minConf":8,"disabled":0}, ... }

    Call: https://poloniex.com/public?command=returnCurrencies */

pub fn returnLoanOrders() {

}

 /*   Returns the list of loan offers and demands for a given currency, specified by the "currency" GET parameter. Sample output:

    {"offers":[{"rate":"0.00200000","amount":"64.66305732","rangeMin":2,"rangeMax":8}, ... ],"demands":[{"rate":"0.00170000","amount":"26.54848841","rangeMin":2,"rangeMax":2}, ... ]}

    Call: https://poloniex.com/public?command=returnLoanOrders&currency=BTC */
