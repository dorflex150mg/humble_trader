pub mod trader {

    use websocket::{ClientBuilder, Message};
    use std::net::TcpStream;
    use std::collections::HashMap;
    use std::fs;
    use std::str::from_utf8;
    use thiserror::Error;
    use websocket::client::ParseError;
    use websocket::result::WebSocketResult;
    use websocket::sync::Client;
    use websocket::WebSocketError;

    #[derive(Error, Debug, derive_more::From, derive_more::Display)]
    pub enum ClientError {
        UrlParseError(ParseError),
        SocketConnectError(WebSocketError),
    }

    pub struct TraderBuilder {
        endpoints: HashMap<String, Vec<String>>,
    }

    pub struct Trader {
        endpoint: String,
        subs_endpoint: String,
        client: Client<TcpStream>,
    }

    impl TraderBuilder {
        pub fn new(endpoint_file_path: &str, 
            subs_endpoint_file_path: &str) -> Result<Self, Box<dyn std::error::Error + 'static>>  { 
            let ep_data = fs::read(&endpoint_file_path)?;
            let endpoint_str = match from_utf8(&ep_data) { 
                Ok(s) => s,
                Err(e) => panic!("File {} is corrupted. Failed with error: {}", endpoint_file_path, e),
            };
            let endpoints_slc: Vec<&str> = endpoint_str.split("\n").collect();
            let endpoint_lines: Vec<String> = endpoints_slc.iter().map(|slc| {
                    slc.to_string()
                }).collect();
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
            match self.endpoints.get(key) {
                Some(ep) => Some(Trader::new(ep[0], ep[1])),
                None => None,
           }
        }
    }

    impl Trader {
        pub fn new(endpoint: String, subs_endpoint: String) -> Result<Self, ClientError> {
            let client = ClientBuilder::new(endpoint)?
                .connect()?;
            Ok(Trader {
                endpoint,
                client,
            })
        }

        pub fn subscribe(&self) -> Result<(), ClientError>{
            let msg = self.subscribe_msg(1);
            self.client.send_message(&Message::text(&msg))?
        }

        
        fn subscribe_msg(&self, id: u32) -> String { 
            let raw = format!(r#"{{"method": "SUBSCRIBE",
                          "params": ["{}"], "id": {}}}"#, self.subs_endpoint, id);
            println!("raw: {}", &raw);
            raw
        }

    }
}







