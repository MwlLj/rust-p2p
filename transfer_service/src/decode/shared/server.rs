use crate::shared;

use rust_parse;

pub fn decodeServerInfo(buf: &[u8]) -> Result<shared::server::CServerInfo, &str> {
    let mut server: shared::server::CServerInfo = shared::server::CServerInfo::default();
    rust_parse::string::u8_parse::u8ArrSplit(buf, '|' as u8, &mut |index: &u8, field: &str| {
        if *index == 0 {
            server.net.ip = field.to_string();
        } else if *index == 1 {
            server.net.port = field.parse().expect("field parse error");
        }
    });
    Ok(server)
}
