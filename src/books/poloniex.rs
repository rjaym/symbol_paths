

use serde_json;
use hyper::Client;

use std::collections::{HashMap};
use std::vec::Vec;

use std::time::{SystemTime};
use std::io::{self, Write};
use utility;

pub fn request (
  client : & Client
  //filter_symbols : & Symbols
) -> super::OrderBooks {

  let started_at = SystemTime::now();

  // for now, don't nest these guys..
  let resp =
    client
    .get(
      "\
      https://poloniex.com/public?\
      command=returnOrderBook&\
      currencyPair=all&\
      depth=6\
      "
    )
    .send()
    .unwrap()
  ;

  let v : serde_json::Value =
    serde_json::from_reader(
      resp
    )
    .unwrap()
  ;

  let mut books =
    v
    .as_object()
    .unwrap()
  ;

  let normalized_books =
    normalize_books (
      &books
    )
  ;

  print!(
    "normalized poloniex books {}ms, ",
    utility::time_difference(&started_at)
  ); io::stdout().flush().unwrap();

  normalized_books
}


fn normalize_books (
  books : & serde_json::Map<String, serde_json::Value>
) -> super::OrderBooks {

  let mut normalized_books = HashMap::new();
  
  for ( key, value ) in books.iter() {
   
    let mut market = HashMap::new();
    
    // next time around, 
    // can I use a closure on the asks and bids?

    // asks
    let mut asks = Vec::new();
    let raw_asks = value["asks"].as_array().unwrap();
    
    for element in raw_asks {

      let inner_array = element.as_array().unwrap();

      let str_price = inner_array[0].as_str().unwrap();
      let price : f64 = str_price.parse().unwrap();
      
      let quantity = inner_array[1].as_f64().unwrap();
      
      asks.push(
        (
          price, 
          quantity
        )
      );

    }
    
    market.insert("asks".to_string(),asks);

    
    // bids
    let mut bids = Vec::new();
    let raw_bids = value["bids"].as_array().unwrap();

    for element in raw_bids {

      let inner_array = element.as_array().unwrap();

      let str_price = inner_array[0].as_str().unwrap();
      let price : f64 = str_price.parse().unwrap();

      let quantity = inner_array[1].as_f64().unwrap();

      bids.push(
        (
          price,
          quantity
        )
      );

    }

    market.insert("bids".to_string(),bids);

    normalized_books.insert(key.to_string(),market);

  }

  normalized_books
}

