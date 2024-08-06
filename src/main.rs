mod trader;
mod platform;
mod platforms;

use crate::trader::trader::{Trader, TraderBuilder};
use crate::platforms::platform_binance::PlatformBinance;
use crate::platform::platform::Platform;

#[tokio::main]
async fn main() {
    let mut trader = TraderBuilder::new("./endpoints.dat")
        .unwrap()
        .build("binance".to_string())
        .unwrap();
    let subs_trader = trader
        .subscribe() 
        .unwrap();
    //println!("reading: {}", subs_trader.read_stream());
    subs_trader.read_stream();
}

