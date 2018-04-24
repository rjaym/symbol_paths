

use std::collections::{HashMap};

use serde_json;
use hyper::Client;

use utility;
use std::time::{SystemTime};

pub type Prices = 
  HashMap<
    String, 
    f64
  >
;

pub fn request (
  client : & Client
) -> Prices {
  
  let started_at = SystemTime::now();

  let v : serde_json::Value =
    serde_json::from_reader(
      client
      .get("https://api.coinmarketcap.com/v1/ticker/")
      .send()
      .unwrap()
    )
    .unwrap()
  ;
  
  let prices = 
    v.as_array()
    .unwrap()
  ;
  
  let mut r : Prices = HashMap::new();

  for market in prices.iter() {

    let symbol =
      market["symbol"]
      .as_str()
      .unwrap()
    ;

    // something was returning None.. need to handle it --2017-11-03
    match market["price_usd"].as_str() {
      Some(s) => {
        r.insert(
          String::from(symbol),
          s.parse().unwrap()
        );
      },
      None => {} // pft..
    }

  }

  println!(
    "## returning coinmarketcap prices, took {}ms",
    utility::time_difference(&started_at)
  );

  r
}

