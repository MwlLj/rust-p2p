#[derive(Default, Clone, Debug)]
pub struct CNet {
    pub ip: String,
    pub port: u32
}

#[derive(Default, Clone, Debug)]
pub struct CServerInfo {
    pub net: CNet
}

pub mod redis;
