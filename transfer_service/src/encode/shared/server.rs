use crate::shared;

pub fn encodeServerInfo(server: &shared::server::CServerInfo) -> String {
    let mut s = String::new();
    s.push_str(&server.net.ip);
    s.push_str("|");
    s.push_str(&server.net.port.to_string());
    s
}
