use crate::shared;

use rust_parse;

pub fn decodeNodeSelf(buf: &[u8]) -> Result<shared::node::CCommunicateNode, &str> {
    let mut node: shared::node::CCommunicateNode = shared::node::CCommunicateNode::default();
    rust_parse::string::u8_parse::u8ArrSplit(buf, '|' as u8, &mut |index: &u8, field: &str| {
        if *index == 0 {
            node.streamFd = field.parse().expect("stream fd field parse error");
        }
    });
    Ok(node)
}
