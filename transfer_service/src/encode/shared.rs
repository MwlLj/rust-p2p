use crate::shared;

pub fn encodeNodeSelf(node: &shared::node::CCommunicateNode) -> String {
    let mut s = String::new();
    s.push_str(&node.streamAddr.to_string());
    s
}
