

use std::collections::{HashMap};
use std::vec::Vec;

pub type Orders = 
  Vec<
    (
      f64, 
      f64
    )
  >
;

pub type OrderBook = 
  HashMap<
    String, 
    Orders
  >
;

pub type OrderBooks = 
  HashMap<
    String, 
    OrderBook
  >
;

pub mod poloniex;

