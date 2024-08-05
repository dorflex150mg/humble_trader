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
    use websocket::OwnedMessage;
    use websocket::stream::sync::NetworkStream;

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
        //client: Client<TcpStream>,
        client: Client<Box<dyn NetworkStream + std::marker::Send>>,
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

        pub fn read_message(&mut self) -> String {
            match self.client.recv_message()
                .unwrap() {
                    OwnedMessage::Text(string) => string,
                    _ => panic!("wrong message type"),
                }
        }

        pub fn read_stream(&mut self) { 
            for message in self.client.incoming_messages() {
                let inner = &message.unwrap();
                let content = match inner {
                    OwnedMessage::Text(string) => string,
                    _ => panic!("wrong message type"),
                };
                println!("Message from stream: {}", content);
            }
        } 
    }
}







