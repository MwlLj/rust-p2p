use number_conv::array::u8arr;

#[derive(Default)]
pub struct CRequest {
    pub requestMode: String,
    pub selfCommunicateUuid: String,
    pub peerCommunicateUuid: String,
    pub serverUuid: String,
    pub packageIndex: u64,
    pub packageTotal: u64,
    pub data: Vec<u8>
}

