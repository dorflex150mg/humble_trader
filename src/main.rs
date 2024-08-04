use crate::trader::trader::{Trader, TraderBuilder};

mod trader;

#[tokio::main]
async fn main() {
    let trader = match TraderBuilder::new("./endpoints.dat") {
        Ok(t) => t,
        Err(e) => panic!("Cannot create trader: {}", e),
    };
}

