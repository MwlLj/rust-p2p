// direct find NAT 3
// ruled out NAT 3
use crate::shared;
use crate::consts::proto;
use crate::decode;
use crate::encode;
use crate::structs;

use uuid::Uuid;

use std::net::UdpSocket;
use std::thread;
use std::time;

pub struct CRuledOut {
    sharedStorage: Box<dyn shared::IShared>
}

impl CRuledOut {
    pub fn start(&self, port: u32) -> Result<(), std::io::Error> {
        let addr = self.joinAddr(port);
        let mut socket = match UdpSocket::bind(addr) {
            Ok(socket) => socket,
            Err(err) => return Err(err)
        };
        // get self addr info
        loop {
            let mut buf = [0; 128];
            let (amt, src) = match socket.recv_from(&mut buf) {
                Ok((amt, src)) => (amt, src),
                Err(err) => return Err(err)
            };
            let request = match decode::connect::decodeConnectRequest(&buf) {
                Ok(request) => request,
                Err(err) => return Err(err)
            };
            if request.getSelfUuid() == "" {
                // first -> assign uuid
                let uid = uuid::Uuid::new_v4();
                let mut response = structs::req_res::CCheckResponse::default();
                response.setSelfUuid(&uid.to_string());
                let responseStr = encode::check::encodeCheckResponse(&response);
                while let Err(_) = socket.send_to(responseStr.as_bytes(), src) {
                    thread::sleep(time::Duration::from_millis(500));
                }
            } else {
                // second -> 1. delete selfUuid from shared; 2. add communicate uuid to shared
            }
        }
        Ok(())
    }
}

impl CRuledOut {
    fn joinAddr(&self, port: u32) -> String {
        let mut addr = String::from("0.0.0.0");
        addr.push_str(":");
        addr.push_str(&port.to_string());
        addr
    }
}

impl CRuledOut {
    fn new(sharedMode: &str, dial: &str) -> Result<CRuledOut, String> {
        if sharedMode == proto::storage_mode_redis {
            return Ok(CRuledOut{
                sharedStorage: Box::new(shared::redis::CRedis::new(dial))
            });
        }
        return Err(String::from("shared mode is not found"));
    }
}
