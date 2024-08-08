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
    let client = &mut trader
        .subscribe() 
        .unwrap();
    //println!("reading: {}", subs_trader.read_stream());
    let _ = &mut trader.platform.read_stream(&mut trader.client);
}

