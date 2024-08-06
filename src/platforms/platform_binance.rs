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

pub struct PlatformBinance {
    name: Rc<str>,
}

impl PlatformBinance {
    fn subscribe_msg(&self, trader: &mut Trader, id: u32) -> String { 
        let raw = format!(r#"{{"method": "SUBSCRIBE",
                      "params": ["{}"], "id": {}}}"#, trader.subs_endpoint, id);
        println!("raw: {}", &raw);
        raw
    }
}

impl Platform for PlatformBinance {

    fn get_name(&self) -> Rc<str>{
        self.name.clone()
    }

    fn subscribe<'a>(&'a self, trader: &'a mut Trader) -> Result<&mut Trader, ClientError>{
        let msg = self.subscribe_msg(trader, 50);
        trader.client.send_message(&Message::text(&msg))?;
        Ok(trader)
    }

    fn read_stream<'a> (&'a self, trader: &'a mut Trader) { 
        for message in trader.client.incoming_messages() {
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
