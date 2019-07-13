use crate::transmit;

use std::net::SocketAddr;

pub struct CConfigFinder {
}

impl transmit::IFinder for CConfigFinder {
    fn transmitService(&self) -> SocketAddr {
        SocketAddr::new("127.0.0.1".parse().unwrap(), "21010".parse().unwrap())
    }
}

impl CConfigFinder {
    pub fn new() -> CConfigFinder {
        CConfigFinder{}
    }
}
