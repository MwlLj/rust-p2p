/*
use crate::shared;

use socket::fd::tcp;

use std::collections::HashMap;
use std::net::TcpStream;

pub struct CClient {
    serverNets: HashMap<String, u64>
}

impl CClient {
    pub fn findStream(&self, serverUuid: &str) -> Option<u64> {
        let server = match self.serverNets.get(serverUuid) {
            Some(streamFd) => {
                return Some(*streamFd);
            },
            None => {
                return None;
            }
        };
        None
    }

    pub fn addServer(&mut self, serverUuid: &str, streamFd: u64) {
        self.serverNets.insert(serverUuid.to_string(), streamFd);
    }

    pub fn serverConnect(serverNet: &shared::server::CNet) -> Result<u64, &str> {
        let addr = CClient::joinAddr(serverNet);
        let stream = match TcpStream::connect(addr.as_str()) {
            Ok(s) => s,
            Err(err) => {
                println!("connect error, err: {}", err);
                return Err("connect server error");
            }
        };
        Ok(tcp::stream2fd(stream))
    }

    fn joinAddr(net: &shared::server::CNet) -> String {
        let mut addr = String::new();
        addr.push_str(&net.ip);
        addr.push_str(":");
        addr.push_str(&net.port.to_string());
        addr
    }
}

impl CClient {
    pub fn new() -> CClient {
        CClient{
            serverNets: HashMap::new()
        }
    }
}
*/

///*
use crate::shared;

use socket::fd::tcp;

use std::collections::HashMap;
use std::net::TcpStream;

pub struct CClient {
    serverNets: HashMap<String, TcpStream>
}

impl CClient {
    pub fn findStream(&self, serverUuid: &str) -> Option<TcpStream> {
        let server = match self.serverNets.get(serverUuid) {
            Some(stream) => {
                let s = match stream.try_clone() {
                    Ok(s) => s,
                    Err(err) => {
                        println!("stream try clone error");
                        return None;
                    }
                };
                return Some(s);
            },
            None => {
                return None;
            }
        };
        None
    }

    pub fn addServer(&mut self, serverUuid: &str, stream: TcpStream) {
        self.serverNets.insert(serverUuid.to_string(), stream);
    }

    pub fn serverConnect(serverNet: &shared::server::CNet) -> Result<TcpStream, &str> {
        let addr = CClient::joinAddr(serverNet);
        let stream = match TcpStream::connect(addr.as_str()) {
            Ok(s) => s,
            Err(err) => {
                println!("connect error, err: {}", err);
                return Err("connect server error");
            }
        };
        let s = match stream.try_clone() {
            Ok(s) => s,
            Err(err) => {
                println!("stream try_clone error");
                return Err("stream try_clone error");
            }
        };
        Ok(s)
    }

    fn joinAddr(net: &shared::server::CNet) -> String {
        let mut addr = String::new();
        addr.push_str(&net.ip);
        addr.push_str(":");
        addr.push_str(&net.port.to_string());
        addr
    }
}

impl CClient {
    pub fn new() -> CClient {
        CClient{
            serverNets: HashMap::new()
        }
    }
}
//*/

