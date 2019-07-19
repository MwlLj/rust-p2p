use std::net::TcpListener;

pub struct CCreateParam {
    pub nodeStorageDial: String,
    pub listenIp: String,
    pub listenPort: u32,
    pub threadMax: u32
}

pub mod tcp;
