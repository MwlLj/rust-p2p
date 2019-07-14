use crate::shared;
use crate::enums;

use rust_parse;

pub fn decodeSelf(buf: &[u8]) -> Result<shared::CSelf, &str> {
    let mut node = shared::CSelf::default();
    rust_parse::string::u8_parse::u8ArrSplit(buf, '|' as u8, &mut |index: &u8, field: &str| {
        if *index == 0 {
            node.lanNet.ip = field.to_string();
        } else if *index == 1 {
            node.lanNet.port = field.to_string();
        } else if *index == 2 {
            node.wanNet.ip = field.to_string();
        } else if *index == 3 {
            node.wanNet.port = field.to_string();
        }
    });
    Ok(node)
}

pub fn decodeNode(buf: &[u8]) -> Result<shared::CNode, &str> {
    let mut node = shared::CNode::default();
    rust_parse::string::u8_parse::u8ArrSplit(buf, '|' as u8, &mut |index: &u8, field: &str| {
        if *index == 0 {
            node.lanNet.ip = field.to_string();
        } else if *index == 1 {
            node.lanNet.port = field.to_string();
        } else if *index == 2 {
            node.wanNet.ip = field.to_string();
        } else if *index == 3 {
            node.wanNet.port = field.to_string();
        } else if *index == 4 {
            let natType = field.parse::<u8>().unwrap();
            node.natType = u8::into(natType);
        }
    });
    Ok(node)
}

#[test]
#[ignore]
fn testDecodeSelf() {
    // 192.168.9.15|123456|192.168.9.15|123456
    let v = [49, 57, 50, 46, 49, 54, 56, 46, 57, 46, 49, 53, 124, 49, 50, 51, 52, 53, 54, 124, 49, 57, 50, 46, 49, 54, 56, 46, 57, 46, 49, 53, 124, 49, 50, 51, 52, 53, 54];
    if let Ok(node) = decodeSelf(&v) {
        assert_eq!(node.lanNet.ip, "192.168.9.15");
        assert_eq!(node.lanNet.port, "123456");
        assert_eq!(node.wanNet.ip, "192.168.9.15");
        assert_eq!(node.wanNet.port, "123456");
    } else {
    }
}

#[test]
#[ignore]
fn testDecodeNode() {
    // 192.168.9.15|123456|192.168.9.15|123456|4
    let v = [49, 57, 50, 46, 49, 54, 56, 46, 57, 46, 49, 53, 124, 49, 50, 51, 52, 53, 54, 124, 49, 57, 50, 46, 49, 54, 56, 46, 57, 46, 49, 53, 124, 49, 50, 51, 52, 53, 54, 124, 52];
    if let Ok(node) = decodeNode(&v) {
        assert_eq!(node.lanNet.ip, "192.168.9.15");
        assert_eq!(node.lanNet.port, "123456");
        assert_eq!(node.wanNet.ip, "192.168.9.15");
        assert_eq!(node.wanNet.port, "123456");
        assert_eq!(node.natType, enums::nat::Nat::Nat4);
    } else {
    }
}
