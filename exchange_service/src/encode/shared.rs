use crate::shared;

pub fn encodeSelf(node: &shared::CSelf) -> String {
    let mut s = String::new();
    s.push_str(&node.lanNet.ip);
    s.push_str("|");
    s.push_str(&node.lanNet.port);
    s.push_str("|");
    s.push_str(&node.wanNet.ip);
    s.push_str("|");
    s.push_str(&node.wanNet.port);
    s
}

pub fn encodeNode(node: &shared::CNode) -> String {
    let mut s = String::new();
    s.push_str(&node.lanNet.ip);
    s.push_str("|");
    s.push_str(&node.lanNet.port);
    s.push_str("|");
    s.push_str(&node.wanNet.ip);
    s.push_str("|");
    s.push_str(&node.wanNet.port);
    s.push_str("|");
    s.push_str(&(node.natType.clone() as u8).to_string());
    s
}
