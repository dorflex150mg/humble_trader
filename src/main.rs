use crate::trader::trader::Trader;

mod trader;

#[tokio::main]
async fn main() {
    let trader = match Trader::new("./endpoints.dat") {
        Ok(t) => t,
        Err(e) => panic!("Cannot create trader: {}", e),
    };
}

