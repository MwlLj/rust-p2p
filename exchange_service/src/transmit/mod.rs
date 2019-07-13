use std::net::SocketAddr;

pub trait IFinder {
    fn transmitService(&self) -> SocketAddr;
}

pub mod config_finder;
