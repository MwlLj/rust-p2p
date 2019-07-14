use crate::structs;

use rust_parse;

pub fn decodeCheckResponse(buf: &[u8]) -> Result<structs::req_res::CCheckResponse, &str> {
    let mut res: structs::req_res::CCheckResponse = structs::req_res::CCheckResponse::default();
    rust_parse::string::u8_parse::u8ArrSplit(buf, '|' as u8, &mut |index: &u8, field: &str| {
        if *index == 0 {
            res.selfUuid = field.to_string();
        }
    });
    Ok(res)
}

pub fn decodePeerNetResponse(buf: &[u8]) -> Result<structs::req_res::CPeerNetResponse, &str> {
    let mut res: structs::req_res::CPeerNetResponse = structs::req_res::CPeerNetResponse::default();
    rust_parse::string::u8_parse::u8ArrSplit(buf, '|' as u8, &mut |index: &u8, field: &str| {
        if *index == 0 {
            res.peerIp = field.to_string();
        } else if *index == 1 {
            res.peerPort = field.to_string();
        } else if *index == 2 {
            res.portInterval = field.parse().unwrap();
        }
    });
    Ok(res)
}

#[test]
// #[ignore]
fn testDecodeCheckResponse() {
    // 123456
    let v = [49, 50, 51, 52, 53, 54];
    if let Ok(res) = decodeCheckResponse(&v) {
        assert_eq!(res.selfUuid, "123456");
    } else {
        assert!(false);
    }
}
