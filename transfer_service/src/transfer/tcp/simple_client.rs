use crate::shared;

use socket::fd::tcp;

use std::collections::HashMap;
use std::net::TcpStream;

pub struct CClient {
    serverNets: HashMap<String, TcpStream>
}

impl CClient {
    pub fn findStream(&self, serverUuid: &str) -> Option<u64> {
        let server = match self.serverNets.get(serverUuid) {
            Some(s) => {
                let stream = match s.try_clone() {
                    Ok(s) => s,
                    Err(err) => {
                        println!("err: {}", err);
                        return None;
                    }
                };
                let addr = tcp::stream2fd(stream);
                return Some(addr);
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
        Ok(stream)
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
