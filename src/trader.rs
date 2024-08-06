pub mod trader {

    use std::{fs,
              fmt,
              str::from_utf8,
              collections::HashMap,
              net::TcpStream,
    };
    use websocket::{ClientBuilder, 
                    Message,
                    WebSocketError, 
                    OwnedMessage,
                    client::ParseError,
                    result::WebSocketResult,
                    sync::Client,
                    stream::sync::NetworkStream,

    };
    use thiserror::Error;
    use chrono::{NaiveDate, NaiveDateTime};
    use json::JsonValue;


    #[derive(Error, Debug, derive_more::From, derive_more::Display)]
    pub enum ClientError {
        UrlParseError(ParseError),
        SocketConnectError(WebSocketError),
    }

    pub struct TraderBuilder {
        endpoints: HashMap<String, Vec<String>>,
    }

    pub struct PriceReading {
        pub timestamp: NaiveDateTime,
        pub price: f64,
    }

    impl fmt::Display for PriceReading {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Time: {}, Price: {}", self.timestamp.timestamp(), self.price)
        }
    }

    pub struct Trader {
        pub endpoint: String,
        pub subs_endpoint: String,
        pub client: Client<Box<dyn NetworkStream + std::marker::Send>>,
    }

    impl TraderBuilder {
        pub fn new(endpoint_file_path: &str) -> Result<Self, Box<dyn std::error::Error + 'static>>  { 
            let ep_data = fs::read(&endpoint_file_path)?;
            let endpoint_str = match from_utf8(&ep_data) { 
                Ok(s) => s,
                Err(e) => panic!("File {} is corrupted. Failed with error: {}", endpoint_file_path, e),
            };
            let mut endpoints_slc: Vec<&str> = endpoint_str.split("\n").collect();
            if endpoints_slc.iter().count() == 0 {
                panic!("File is empty");
            }
            let endpoint_lines: Vec<String> = endpoints_slc.iter()
                .filter(|&line| { *line != "" })
                .into_iter()
                .map(|slc| {slc.to_string()}) 
                .collect();

            let endpoints: HashMap<String, Vec<String>> = endpoint_lines
                .iter()
                .map(|line| {
                    let vals: Vec<&str> = line.split(",").collect();
                    (vals[0].to_string(), vec![vals[1].to_string(), vals[2].to_string()])
                }).collect();

            Ok(TraderBuilder {
                endpoints,
            })
        }


        pub fn build(&self, key: String) -> Option<Trader> {
            match self.endpoints.get(&key) {
                Some(ep) => Some(Trader::new(ep[0].clone(), ep[1].clone()).unwrap()),
                None => None,
           }
        }
    }

    impl Trader {
        pub fn new(endpoint: String, subs_endpoint: String) -> Result<Self, ClientError> {
            println!("creatin trader with endpoint: {}", &endpoint);
            let client = match ClientBuilder::new(&endpoint)?
                .connect(None) {
                    Ok(c) => c,
                    Err(e) => panic!("Client building failed with: {:?}", e),
                };

            Ok(Trader {
                endpoint,
                subs_endpoint,
                client,
            })
        }

        pub fn subscribe(&mut self) -> Result<&mut Self, ClientError>{
            let msg = self.subscribe_msg(50);
            self.client.send_message(&Message::text(&msg))?;
            Ok(self)
        }

        
        fn subscribe_msg(&self, id: u32) -> String { 
            let raw = format!(r#"{{"method": "SUBSCRIBE",
                          "params": ["{}"], "id": {}}}"#, self.subs_endpoint, id);
            println!("raw: {}", &raw);
            raw
        }

        pub fn read_stream(&mut self) { 
            for message in self.client.incoming_messages() {
                let inner = &message.unwrap();
                let json = match inner {
                    OwnedMessage::Text(string) => {
                        json::parse(string).unwrap()
                    }
                    _ => panic!("wrong message type"),
                };
                let content = match json["E"].is_null() || json["c"].is_null() { 
                    true => None,
                    false => { 
                        let unix_timestamp: &i64 = &json["E"].as_i64().unwrap();
                        let timestamp = NaiveDateTime::from_timestamp(*unix_timestamp, 0);
                        let closing_price: &f64 = &json["c"].as_str().unwrap().parse::<f64>().unwrap(); 
                        Some(PriceReading {
                            timestamp,
                            price: *closing_price,
                        })
                    }
                };
                match content {
                    Some(c) => println!("Message from stream: {}", c),
                    None => println!("Invalid price message received from stream"),
                }
            }
        } 
    }
}







