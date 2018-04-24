
#![allow(unused_parens)]
#![allow(unused_mut)]

extern crate hyper;
extern crate hyper_native_tls;
extern crate serde_json;
extern crate num;
extern crate sha2;
extern crate hmac;

mod books;
mod arbitrager;
mod utility;

use arbitrager::*;

fn main () {

  let target_symbol = "ETH";

  let ssl = hyper_native_tls::NativeTlsClient::new().unwrap();
  let connector = hyper::net::HttpsConnector::new(ssl);
  let client = hyper::Client::with_connector(connector);

  let starting_books : books::OrderBooks = books::poloniex::request(
    &client
  );

  let links : Links = derive_links(&starting_books);

  // potential arbitrage paths beginning and ending with the target symbol
  let paths = derive_paths(

    // wow, 2017-08-27, 9 was the max... nothing with 10..
    6, 

    target_symbol,

    links

  );
  
  println!(
    "{:#?}", 
    paths
  )

}
