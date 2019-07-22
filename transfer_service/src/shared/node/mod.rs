#[derive(Default)]
pub struct CCommunicateNode {
    pub streamFd: u64,
    pub serverUuid: String
}

pub mod redis;
