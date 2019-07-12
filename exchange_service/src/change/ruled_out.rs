// direct find NAT 3
// ruled out NAT 3
use crate::shared;
use crate::nat;
use crate::consts::proto;
use crate::decode;
use crate::encode;
use crate::structs;
use crate::router;

use uuid::Uuid;

use std::net::UdpSocket;
use std::thread;
use std::time;

pub struct CRuledOut {
    sharedStorage: Box<dyn shared::IShared>,
    natCheck: Box<dyn nat::INat>
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
                Err(err) => continue
            };
            let selfUuid = request.getSelfUuid();
            if selfUuid == "" {
                // first -> assign uuid
                let uid = uuid::Uuid::new_v4().to_string();
                let mut response = structs::req_res::CCheckResponse::default();
                response.setSelfUuid(&uid);
                let responseStr = encode::check::encodeCheckResponse(&response);
                while let Err(_) = socket.send_to(responseStr.as_bytes(), src) {
                    thread::sleep(time::Duration::from_millis(500));
                }
                // save to shared
                // how to handle add failed ???
                self.sharedStorage.addSelf(&uid, shared::CSelf{
                    lanNet: shared::CNet{
                        ip: request.getLanIp().to_string(),
                        port: request.getLanPort().to_string()
                    },
                    wanNet: shared::CNet{
                        ip: src.ip().to_string(),
                        port: src.port().to_string()
                    }
                }, 60000);
            } else {
                /*
                second:
                    1. get self node info
                    2. judge node NAT type
                    3. delete selfUuid from shared
                    4. add communicate uuid to shared
                */
                // get node info
                let firstConnectInfo = match self.sharedStorage.selfExist(selfUuid) {
                    Some(info) => info,
                    None => continue
                };
                // judge NAT type
                let natType = self.natCheck.natType(&firstConnectInfo, &shared::CSelf{
                    lanNet: shared::CNet{
                        ip: request.getLanIp().to_string(),
                        port: request.getLanPort().to_string()
                    },
                    wanNet: shared::CNet{
                        ip: src.ip().to_string(),
                        port: src.port().to_string()
                    }
                });
                // delete selfUuid
                self.sharedStorage.delNode(selfUuid);
                if let Some(peerInfo) = self.sharedStorage.peerExist(&request.communicateUuid) {
                    // check
                    let isCommunicate = router::peer::CPeer::peerCheck(&peerInfo, &shared::CNode{
                        lanNet: shared::CNet{
                            ip: request.getLanIp().to_string(),
                            port: request.getLanPort().to_string()
                        },
                        wanNet: shared::CNet{
                            ip: src.ip().to_string(),
                            port: src.port().to_string()
                        },
                        natType: natType
                    });
                } else {
                    // add
                    // add communicateUuid
                    self.sharedStorage.addPeer(&request.communicateUuid, shared::CNode{
                        lanNet: shared::CNet{
                            ip: request.getLanIp().to_string(),
                            port: request.getLanPort().to_string()
                        },
                        wanNet: shared::CNet{
                            ip: src.ip().to_string(),
                            port: src.port().to_string()
                        },
                        natType: natType
                    }, 60000);
                }
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
                sharedStorage: Box::new(shared::redis::CRedis::new(dial)),
                natCheck: Box::new(nat::simple::CSimple{})
            });
        }
        return Err(String::from("shared mode is not found"));
    }
}
