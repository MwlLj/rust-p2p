#[derive(Default)]
pub struct CResponse {
    pub responseMode: String,
    pub serverUuid: String,
    pub result: u8,
    pub selfCommunicateUuid: String,
    pub peerCommunicateUuid: String,
    pub packageIndex: u64,
    pub packageTotal: u64,
    pub data: Vec<u8>
}