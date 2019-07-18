#[derive(Default)]
pub struct CNet {
    pub ip: String,
    pub port: u32
}

#[derive(Default)]
pub struct CServerInfo {
    pub net: CNet
}

pub mod redis;
