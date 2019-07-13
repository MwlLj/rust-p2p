use std::io::Error;

use crate::structs;

pub fn encodeCheckResponse(res: &structs::req_res::CCheckResponse) -> String {
    let mut s = String::new();
    s.push_str(res.getSelfUuid());
    s
}

pub fn encodePeerNetResponse(res: &structs::req_res::CPeerNetResponse) -> String {
    let mut s = String::new();
    s.push_str(&res.peerIp);
    s.push_str(":");
    s.push_str(&res.peerPort);
    s.push_str(":");
    s.push_str(&res.portInterval.to_string());
    s
}
