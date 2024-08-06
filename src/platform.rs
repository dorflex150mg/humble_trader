pub mod platform { 
    use crate::trader::trader::{Trader, ClientError};
    use std::rc::Rc;

    pub trait Platform {
        fn get_name(&self) -> Rc<str>;
        fn subscribe<'a>(&'a self, trader: &'a mut Trader) -> Result<&mut Trader, ClientError>;
        fn read_stream<'a>(&'a self, trader: &'a mut Trader) -> ();
    }
}



