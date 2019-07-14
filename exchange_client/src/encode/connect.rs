use crate::structs;

pub fn encodeRequest(req: &structs::req_res::CRequest) -> String {
    let mut s = String::new();
    s.push_str(&req.requestType);
    s.push_str("|");
    s.push_str(&req.selfUuid);
    s.push_str("|");
    s.push_str(&req.communicateUuid);
    s.push_str("|");
    s.push_str(&req.lanIp);
    s.push_str("|");
    s.push_str(&req.lanPort);
    s
}
