# symbol_paths

Using data from CoinMarketCap and Poloniex, derive the feasible/unique arbitrage "paths" that begin and end with the given symbol

Usage: 
- git clone git@github.com:rjaym/symbol_paths.git
- cd ./symbols
- vim ./src/main.rs (Edit the depth & desired symbol you would like to see paths for)
- cargo build
- ./target/debug/symbol_paths

Note: This is part of an arbitrage program I wrote not too long ago. Nothing too serious, just me learning some rust.  
Note: Yes the code quality is rough here, I just wanted to put something in my GitHub account...

