use crate::trader::trader::{Trader, TraderBuilder};

mod trader;

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

