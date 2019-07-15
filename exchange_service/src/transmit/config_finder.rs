use crate::transmit;

use std::net::SocketAddr;

pub struct CConfigFinder {
}

impl transmit::IFinder for CConfigFinder {
    fn transmitService(&self) -> SocketAddr {
        SocketAddr::new("119.3.72.228".parse().unwrap(), "31001".parse().unwrap())
    }
}

impl CConfigFinder {
    pub fn new() -> CConfigFinder {
        CConfigFinder{}
    }
}
