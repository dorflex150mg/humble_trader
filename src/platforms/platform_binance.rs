use crate::{Trader, Platform};
use crate::trader::trader::{PriceReading, ClientError};
use std::{fs,
          fmt,
          str::from_utf8,
          collections::HashMap,
          net::TcpStream,
          rc::Rc,
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

pub const NAME: &str = "binance";

pub struct PlatformBinance {
    name: Rc<str>,
}

impl PlatformBinance {

    pub fn new() -> Self {
        PlatformBinance {
            name: NAME.into(),
        }
    }

    fn subscribe_msg(&self, subs_endpoint: &str, id: u32) -> String { 
        let raw = format!(r#"{{"method": "SUBSCRIBE",
                      "params": ["{}"], "id": {}}}"#, subs_endpoint, id);
        println!("raw: {}", &raw);
        raw
    }
}

impl Platform for PlatformBinance {

    fn get_name(&self) -> Rc<str>{
        self.name.clone()
    }

    fn subscribe<'a>(&'a self, client: &'a mut Client<Box<dyn NetworkStream + std::marker::Send>>, 
            subs_endpoint: &str) -> 
                Result<&mut Client<Box<dyn NetworkStream + std::marker::Send>>, ClientError>{
        let msg = self.subscribe_msg(subs_endpoint, 50);
        client.send_message(&Message::text(&msg))?;
        Ok(client)
    }

    fn read_stream<'a> (&'a self, client: &'a mut 
            Client<Box<dyn NetworkStream + std::marker::Send>>) { 
        for message in client.incoming_messages() {
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
