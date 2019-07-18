#[derive(Default)]
pub struct CData {
    pub selfCommunicateUuid: String,
    pub peerCommunicateUuid: String,
    pub packageIndex: u64,
    pub packageTotal: u64,
    pub data: Vec<u8>
}
