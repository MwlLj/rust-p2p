use std::io::Error;

use crate::structs::req_res;

pub fn encodeCheckResponse(res: &req_res::CCheckResponse) -> String {
    let mut s = String::new();
    s.push_str(res.getSelfUuid());
    s
}
