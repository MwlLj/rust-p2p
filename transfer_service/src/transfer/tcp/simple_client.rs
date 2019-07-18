use crate::shared;

use std::collections::HashMap;
use std::net::TcpStream;

pub struct CClient {
    serverNets: HashMap<String, TcpStream>
}

impl CClient {
    pub fn connect(&self, serverUuid: &str, serverNet: &shared::server::CNet) -> Option<TcpStream> {
        let server = match self.serverNets.get(serverUuid) {
            Some(s) => {
                let stream = match s.try_clone() {
                    Ok(s) => s,
                    Err(err) => {
                        println!("err: {}", err);
                        return None;
                    }
                };
                return Some(stream);
            },
            None => {
                let stream = match self.serverConnect() {
                    Ok(s) => s,
                    Err(err) => {
                        println!("err: {}", err);
                        return None;
                    }
                };
                return Some(stream);
            }
        };
        None
    }

    fn serverConnect(&self) -> Result<TcpStream, &str> {
        Err("not impl")
    }
}

impl CClient {
    pub fn new() -> CClient {
        CClient{
            serverNets: HashMap::new()
        }
    }
}
