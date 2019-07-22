use crate::shared;

pub fn encodeNodeSelf(node: &shared::node::CCommunicateNode) -> String {
    let mut s = String::new();
    s.push_str(&node.streamFd.to_string());
    s.push_str("|");
    s.push_str(&node.serverUuid);
    s
}
