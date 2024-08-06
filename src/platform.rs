pub mod platform { 
    use std::{
        fmt,
        rc::Rc,
    };
    use thiserror::Error;

    use crate::trader::trader::{Trader, ClientError};
    use crate::platforms::platform_binance::PlatformBinance;
    use crate::platforms::platform_binance;

    #[derive(Error, Debug)]
    pub enum PlatformError {
        InvalidPlatformError(String),
    }

    impl fmt::Display for PlatformError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                PlatformError::InvalidPlatformError(s) => write!(f, "Invalid Platform {}", *s),
                _ => write!(f, "Platform Error Unknonwn"),
            }
        }
    }

    pub fn create_platform(name: String) -> Result<impl Platform, PlatformError>{
        match name.as_str() {
            platform_binance::NAME => Ok(PlatformBinance::new()),
            _ => Err(PlatformError::InvalidPlatformError(name)),
        }
    }

    pub trait Platform {
        fn get_name(&self) -> Rc<str>;
        fn subscribe<'a>(&'a self, trader: &'a mut Trader) -> Result<&mut Trader, ClientError>;
        fn read_stream<'a>(&'a self, trader: &'a mut Trader) -> ();
    }

}



