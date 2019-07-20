use number_conv::array::u8arr;

#[derive(Default)]
pub struct CConnect {
    pub selfCommunicateUuid: String
}

#[derive(Default)]
pub struct CData {
    pub selfCommunicateUuid: String,
    pub peerCommunicateUuid: String,
    pub serverUuid: String,
    pub objectUuid: String,
    pub packageIndex: u64,
    pub packageTotal: u64,
    pub data: Vec<u8>
}

#[derive(Default)]
pub struct CAck {
    pub selfCommunicateUuid: String,
    pub peerCommunicateUuid: String,
    pub serverUuid: String,
    pub objectUuid: String,
    pub result: String
}

