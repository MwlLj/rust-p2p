use crate::structs::{request, response};

pub trait IClient {
    fn connect(&mut self, conn: &mut request::CConnect, timeoutS: u64) -> Result<(), &str>;
    fn sendAsync(&self, data: &mut request::CData) -> Result<(), &str>;
    fn sendSync(&self, data: &mut request::CData, timeoutS: u64) -> Result<(), &str>;
    fn dataRecv<F: 'static>(&mut self, f: F) -> Result<(), &str>
        where F: Fn(&response::CResponse) -> bool;
}

pub mod tcp;
pub mod client;
