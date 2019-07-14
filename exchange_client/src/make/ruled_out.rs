use crate::make;
use crate::structs;
use crate::decode;
use crate::encode;
use crate::consts;

use std::net::{UdpSocket, SocketAddr, TcpStream};
use std::time;
use std::thread;

pub struct CRuledOut {
}

impl CRuledOut {
    pub fn make(&self, param: &make::CMakeParam) -> Result<(), &str> {
        let socket = match UdpSocket::bind(CRuledOut::joinAddr(&param.selfNet)) {
            Ok(socket) => socket,
            Err(_) => return Err("bind socket error")
        };
        // set read timeout
        if let Err(_) = socket.set_read_timeout(Some(time::Duration::from_secs(30))) {
            return Err("set read timeout error");
        }
        // send info to server1
        let resVec = match self.syncSendToNode(socket.try_clone().unwrap(), &consts::proto::request_type_connect, "", &param, &param.server1Net) {
            Ok(v) => v,
            Err(_) => return Err("send to server1 error")
        };
        let checkRes = match decode::connect::decodeCheckResponse(resVec.as_slice()) {
            Ok(res) => res,
            Err(_) => return Err("decode check response error")
        };
        // send info to server2
        let resVec = match self.syncSendToNode(socket.try_clone().unwrap(), &consts::proto::request_type_connect, &checkRes.selfUuid, &param, &param.server2Net) {
            Ok(v) => v,
            Err(_) => return Err("send to server2 error")
        };
        let peerNetRes = match decode::connect::decodePeerNetResponse(resVec.as_slice()) {
            Ok(res) => res,
            Err(_) => return Err("decode peer net error")
        };
        // start make hole
        let isSuccess = self.tryMakeHole(socket.try_clone().unwrap(), &peerNetRes, &param);
        if isSuccess {
            // return peer net info -> peer communicate
        } else {
        }
        Ok(())
    }
}

impl CRuledOut {
    fn tryMakeHole(&self, socket: UdpSocket, peer: &structs::req_res::CPeerNetResponse, param: &make::CMakeParam) -> bool {
        for i in 0..10 {
            self.sendToNode(socket.try_clone().unwrap(), "".as_bytes(), &structs::net::CNet{
                ip: peer.peerIp.clone(),
                port: peer.peerPort.clone()
            });
            thread::sleep(time::Duration::from_millis(100));
        }
        if let Err(_) = self.syncSendToNode(socket.try_clone().unwrap(), consts::proto::request_type_try_make_finish, "", param, &param.server1Net) {
            return false;
        };
        // build tcp connect
        if let Err(_) = TcpStream::connect(CRuledOut::joinAddr(&structs::net::CNet{
            ip: peer.peerIp.clone(),
            port: peer.peerPort.clone()
        })) {
            return false;
        }
        true
    }

    fn syncSendToNode(&self, socket: UdpSocket, requestType: &str, selfUuid: &str, param: &make::CMakeParam, dstNet: &structs::net::CNet) -> Result<Vec<u8>, &str> {
        let reqEncode = encode::connect::encodeRequest(&structs::req_res::CRequest{
            requestType: requestType.to_string(),
            selfUuid: selfUuid.to_string(),
            communicateUuid: param.communicateUuid.clone(),
            lanIp: param.selfNet.ip.clone(),
            lanPort: param.selfNet.port.clone()
        });
        if let Err(_) = self.sendToNode(socket.try_clone().unwrap(), reqEncode.as_bytes(), dstNet) {
            return Err("send to server1 error");
        }
        let mut buf = [0; 128];
        let (length, src) = match socket.recv_from(&mut buf) {
            Ok((l, s)) => (l, s),
            Err(_) => return Err("recv error")
        };
        Ok(buf[..length].to_vec())
    }

    fn sendToNode(&self, socket: UdpSocket, data: &[u8], dst: &structs::net::CNet) -> Result<(), &str> {
        if let Err(_) = socket.send_to(data, SocketAddr::new(dst.ip.parse().expect("ip parse error"), dst.port.parse().expect("port parse error"))) {
            return Err("send to error");
        }
        Ok(())
    }
}

impl CRuledOut {
    fn joinAddr(addr: &structs::net::CNet) -> String {
        let mut s = String::new();
        s.push_str(&addr.ip);
        s.push_str(":");
        s.push_str(&addr.port);
        s
    }
}

impl CRuledOut {
    pub fn new() -> CRuledOut {
        CRuledOut{}
    }
}

/*
        {
            let s = socket.try_clone().unwrap();
            thread::spawn(move || {
                for i in 0..20 {
                    let mut buf = [0; 128];
                    if let Err(_) = s.send("".as_bytes()) {
                        break;
                    }
                    thread::sleep(time::Duration::from_secs(1));
                }
            });
        }
        let mut failedTimes = 0;
        loop {
            let mut b = [0; 128];
            if let Err(_) = socket.recv(&mut b) {
                failedTimes += 1;
                if failedTimes > 3 {
                    return Ok(false);
                }
            } else {
                return Ok(true);
            }
        }
*/
