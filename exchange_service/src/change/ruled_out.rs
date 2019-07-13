// direct find NAT 3
// ruled out NAT 3
use crate::shared;
use crate::nat;
use crate::consts;
use crate::decode;
use crate::encode;
use crate::structs;
use crate::router;
use crate::transmit;
use crate::enums;

use uuid::Uuid;

use std::net::{UdpSocket, SocketAddr, IpAddr};
use std::thread;
use std::time;

pub struct CRuledOut {
    sharedStorage: Box<dyn shared::IShared + 'static + std::marker::Send + std::marker::Sync>,
    natCheck: Box<dyn nat::INat + 'static + std::marker::Send + std::marker::Sync>,
    transmitServiceFinder: Box<dyn transmit::IFinder + 'static + std::marker::Send + std::marker::Sync>
}

impl CRuledOut {
    pub fn start(&self, port: u32, nat4IsTryMake: bool) -> Result<(), std::io::Error> {
        let addr = self.joinAddr(port);
        let mut socket = match UdpSocket::bind(addr) {
            Ok(socket) => socket,
            Err(err) => return Err(err)
        };
        loop {
            self.listen(socket.try_clone().unwrap(), nat4IsTryMake);
        }
        Ok(())
    }

    fn listen(&self, socket: UdpSocket, nat4IsTryMake: bool) -> Result<(), std::io::Error> {
        // get self addr info
        let mut buf = [0; 128];
        let (amt, src) = match socket.recv_from(&mut buf) {
            Ok((amt, src)) => (amt, src),
            Err(err) => return Err(err)
        };
        let request = match decode::connect::decodeConnectRequest(&buf) {
            Ok(request) => request,
            Err(err) => return Err(err)
        };
        if request.requestType == consts::proto::request_type_connect {
            self.handleConnect(socket.try_clone().unwrap(), src, request, nat4IsTryMake);
        } else if request.requestType == consts::proto::request_type_make_falied {
            self.handleMakeFailed(socket.try_clone().unwrap(), src, request);
        }
        Ok(())
    }

    fn handleConnect(&self, socket: UdpSocket, src: SocketAddr, request: structs::req_res::CRequest, nat4IsTryMake: bool) -> Result<(), std::io::Error> {
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
                None => return Err(std::io::Error::new(std::io::ErrorKind::NotFound, ""))
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
                let node = shared::CNode{
                    lanNet: shared::CNet{
                        ip: request.getLanIp().to_string(),
                        port: request.getLanPort().to_string()
                    },
                    wanNet: shared::CNet{
                        ip: src.ip().to_string(),
                        port: src.port().to_string()
                    },
                    natType: natType
                };
                // check
                let isCommunicate = router::peer::CPeer::peerCheck(&peerInfo, &node);
                /*
                    1. if isCommunicate == false -> try make hole
                    2. if isCommunicate == true -> make hole
                    ->
                    is make hole success, client judge
                */
                if !nat4IsTryMake && !isCommunicate {
                    // nat4 wantn't try make hole
                    // middle transmit
                    let middleAddr = self.transmitServiceFinder.transmitService();
                    let middle = structs::req_res::CPeerNetResponse{
                        peerIp: middleAddr.ip().to_string(),
                        peerPort: middleAddr.port().to_string(),
                        portInterval: 0
                    };
                    let middleEncode = encode::check::encodePeerNetResponse(&middle);
                    // send to peer1
                    self.sendToNode(socket.try_clone().unwrap(), middleEncode.as_bytes(), &peerInfo.wanNet);
                    // send to peer2
                    self.sendToNode(socket.try_clone().unwrap(), middleEncode.as_bytes(), &node.wanNet);
                } else {
                    // make hole
                    let mut peer1 = structs::req_res::CPeerNetResponse::default();
                    let mut peer2 = structs::req_res::CPeerNetResponse::default();
                    router::peer::CPeer::peerChange(isCommunicate, &peerInfo, &mut peer1, &node, &mut peer2);
                    let peer1Encode = encode::check::encodePeerNetResponse(&peer1);
                    let peer2Encode = encode::check::encodePeerNetResponse(&peer2);
                    // send to peer1
                    self.sendToNode(socket.try_clone().unwrap(), peer2Encode.as_bytes(), &peerInfo.wanNet);
                    // send to peer2
                    self.sendToNode(socket.try_clone().unwrap(), peer1Encode.as_bytes(), &node.wanNet);
                }
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
        Ok(())
    }

    fn handleMakeFailed(&self, socket: UdpSocket, src: SocketAddr, request: structs::req_res::CRequest) -> Result<(), std::io::Error> {
        // middle transmit
        let middleAddr = self.transmitServiceFinder.transmitService();
        let middle = structs::req_res::CPeerNetResponse{
            peerIp: middleAddr.ip().to_string(),
            peerPort: middleAddr.port().to_string(),
            portInterval: 0
        };
        let middleEncode = encode::check::encodePeerNetResponse(&middle);
        // send to client
        let node = shared::CNet{
            ip: request.getLanIp().to_string(),
            port: request.getLanPort().to_string()
        };
        self.sendToNode(socket.try_clone().unwrap(), middleEncode.as_bytes(), &node);
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

    fn sendToNode(&self, socket: UdpSocket, data: &[u8], dst: &shared::CNet) {
        socket.send_to(data, SocketAddr::new(dst.ip.parse().expect("ip parse error"), dst.port.parse().expect("port parse error")));
    }
}

impl CRuledOut {
    fn new(sharedMode: &str, transmitServiceFindMode: &str, dial: &str) -> Result<CRuledOut, String> {
        if sharedMode == consts::run::storage_mode_redis {
            if transmitServiceFindMode == consts::run::transmit_service_find_mode_config {
                return Ok(CRuledOut{
                    sharedStorage: Box::new(shared::redis::CRedis::new(dial)),
                    natCheck: Box::new(nat::simple::CSimple{}),
                    transmitServiceFinder: Box::new(transmit::config_finder::CConfigFinder::new())
                });
            }
        }
        return Err(String::from("shared mode is not found"));
    }
}
