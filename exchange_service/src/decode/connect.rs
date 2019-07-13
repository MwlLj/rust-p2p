use crate::structs::req_res;

use rust_parse;

pub fn decodeConnectRequest(buf: &[u8]) -> Result<req_res::CRequest, std::io::Error> {
    let mut req: req_res::CRequest = req_res::CRequest::default();
    rust_parse::string::u8_parse::u8ArrSplit(buf, '|' as u8, &mut |index: &u8, field: &str| {
        if *index == 0 {
            req.requestType = field.to_string();
        } else if *index == 1 {
            req.selfUuid = field.to_string();
        } else if *index == 2 {
            req.communicateUuid = field.to_string();
        } else if *index == 3 {
            req.lanIp = field.to_string();
        } else if *index == 4 {
            req.lanPort = field.to_string();
        }
    });
    Ok(req)
}

#[test]
#[ignore]
fn testDecodeConnectRequest() {
    // connect|||192.168.9.15|123456
    let v = [0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 124, 124, 124, 49, 57, 50, 46, 49, 54, 56, 46, 57, 46, 49, 53, 124, 49, 50, 51, 52, 53, 54];
    if let Ok(req) = decodeConnectRequest(&v) {
        assert_eq!(req.requestType, "connect");
        assert_eq!(req.getSelfUuid(), "");
        assert_eq!(req.communicateUuid, "");
        assert_eq!(req.getLanIp(), "192.168.9.15");
        assert_eq!(req.getLanPort(), "123456");
    } else {
    }
}
