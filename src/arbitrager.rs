

use std::collections::{HashMap,HashSet};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use books::*;

use std::time::{SystemTime};

use utility;


// a starting symbol can have various endpoints (related-symbols)
// for example, zec -> btc, or zec -> usdt
pub type Links = 
  HashMap<
    String, // starting symbol (btc)
    HashSet<
      String // ending symbol (zec)
    >
  >
;

// given an order book, derive all possible links 
// within the given markets (in the order book)
pub fn derive_links (

  // the books come from the per-exchange adapters (poloniex.rs)
  books : &OrderBooks

) -> Links {

  let started_at = SystemTime::now();

  let mut links = HashMap::new();

  // key here is in the form "symbolA_symbolB"
  for (key,_) in books.iter() {

    let split = key.split("_");
    let symbols = split.collect::<Vec<&str>>();

    {
      // forward, from the first symbol to the second
      let mut entry_a = links
        .entry(String::from(symbols[0])) // link start
        .or_insert( // no duplicates
          HashSet::new()
        )
      ;
      
      entry_a.insert(
        String::from(symbols[1]) // link end
      );
    }


    {
      // backward, from the second symbol to the first
      let mut entry_b = links
        .entry(String::from(symbols[1])) // reverse the above (link end)
        .or_insert( // no duplicates
          HashSet::new()
        )
      ;
      
      entry_b.insert(
        String::from(symbols[0]) // link start
      );
    }

  }
  
  println!(
    "## returning links, took {}ms",
    utility::time_difference(&started_at)
  );

  links
}

// used below, took it from rust documentation
fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub type Paths = HashMap<

    u64, // hash of the path to uniquely identify/reference it later.. (? hm)

    // the path itself is a series of symbols like ETH,GNO,BTC,ETH
    Vec<
      String
    >

  >
;

pub type Symbols = HashSet < String >;

// paths are a series of symbols which can be traded for one-another
pub fn derive_paths (

  // the number of consecutive symbols that should constitute the final path
  desired_length : usize, // minimum is 4 in this context..
  
  // that paths we want start and end with the given target symbol
  target : &str,

  // using these links 
  // (derived via derive_links() above ↑)
  known_links : Links

) -> /*(*/
  Paths
  /*Symbols
)*/ {
  
  let started_at = SystemTime::now();
  
  if desired_length < 3 {
    panic!("need at least 3 tiers");
  }
  
  let mut useless_paths = HashSet::<u64>::new();
  let mut useful_paths = HashMap::new(); // ultimately returned..
  let mut useful_symbols = HashSet::new();

  fn recurse (
    path : &mut Vec<String>,
    useful_paths : &mut Paths,
    useless_paths : &mut HashSet<u64>,
    useful_symbols : &mut Symbols,
    desired_length : usize,
    target : &str, 
    known_links : &Links
  ) {

    // calculate the unique hash for the given path
    let h = hash(&path);
    
    // if we haven't determined it's a useless path yet..
    if !useless_paths.contains(&h) {
      
      // such that we don't process this particular path again..
      useless_paths.insert(h);
      
      // pull out the last symbol in the path 
      let last : String = path.last().unwrap().clone();
      
      // to determine which symbols we can go forward with from here..
      for _symbol in known_links[&last].iter() {
        
        // need a copy of what's being referenced; will .push() this later..
        let symbol = _symbol.to_string();
        
        // len compared multiple times below; store it here for now..
        let len = path.len();
        
        if (
          // so the symbol which we're about to .push() is actually the 
          // same as the target? 
          symbol == target &&
          // .. and after we .push() it, our path will be the desired length?
          len == desired_length - 1
        ) {
          // store this somewhere

          path.push(symbol);

          // also throw in each useful symbol somewhere 
          // for pruning further book calls
          for useful_symbol in path.iter() {
            useful_symbols.insert(useful_symbol.clone());
          }

          // will ultimately be returned below..
          useful_paths.insert(
            hash(&path),
            path.to_owned()
          );

          continue; // and keep moving
          
        } else if (
          // the path will become circular if we add a symbol that's already
          // in the path; the exception to this (and how to 'complete the path')
          // is in the conditional above ↑
          path.contains(&symbol)
        ) {
          
          // just keep moving..
          continue;
          
        } else if (
          // at this point, 
          // for the current symbol we've pulled from known_links, we know: 
          // - we can't complete the path
          // - the symbol doesn't yet exist in the given path
          // so.. we'll just .push() this symbol, and keep recursing
          
          // but also put this here for safety; don't want to inadvertently 
          // .push() stuff forever..
          len < desired_length
        ) {
          
          // copy the current path to continue recursing
          // each time we recurse, we want the function to have its own 
          // path to work with... the un-cloned/original path passed will 
          // be used/cloned again as we continue to go through this for{}
          let mut clone = path.clone();
          
          clone.push(symbol);
          
          recurse (
            &mut clone,
            useful_paths,
            useless_paths,
            useful_symbols,
            desired_length,
            target,
            known_links
          );
          
        }

      }
      
    }
    
  }
  
  // to start, go through the symbols related to the target
  for symbol in known_links[target].iter() {
    
    let mut path = Vec::new();
    
    path.push(String::from(target));
    path.push(symbol.to_string());

    recurse (
      &mut path,
      &mut useful_paths,
      &mut useless_paths,
      &mut useful_symbols,
      desired_length,
      target,
      &known_links
    );
    
  }
  
  println!(
    "## returning paths, took {}ms",
    utility::time_difference(&started_at)
  );

  useful_paths
}

