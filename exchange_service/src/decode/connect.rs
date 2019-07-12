use std::io::Error;

use crate::structs::req_res;

pub fn decodeConnectRequest(buf: &[u8]) -> Result<req_res::CRequest, Error> {
    let mut req: req_res::CRequest = req_res::CRequest::default();
    let mut index = 0;
    let mut s = String::new();
    for item in buf.iter() {
        if *item as char == '|' {
            if index == 0 {
                req.selfUuid = s.to_string();
            } else if index == 1 {
                req.communicateUuid = s.to_string();
            } else if index == 2 {
                req.lanIp = s.to_string();
            }
            index += 1;
            s.clear();
        } else {
            s.push(*item as char);
        }
    }
    if index == 3 {
        req.setLanPort(&s);
    }
    Ok(req)
}

#[test]
#[ignore]
fn testDecodeConnectRequest() {
    // ||192.168.9.15|123456
    let v = [124, 124, 49, 57, 50, 46, 49, 54, 56, 46, 57, 46, 49, 53, 124, 49, 50, 51, 52, 53, 54];
    if let Ok(req) = decodeConnectRequest(&v) {
        assert_eq!(req.getSelfUuid(), "");
        assert_eq!(req.communicateUuid, "");
        assert_eq!(req.getLanIp(), "192.168.9.15");
        assert_eq!(req.getLanPort(), "123456");
    } else {
    }
}
