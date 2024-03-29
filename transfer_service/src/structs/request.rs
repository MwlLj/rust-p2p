use number_conv::array::u8arr;

#[derive(Default, Clone, Debug)]
pub struct CRequest {
    pub requestMode: String,
    pub selfCommunicateUuid: String,
    pub peerCommunicateUuid: String,
    pub serverUuid: String,
    pub dataUuid: String,
    pub objectUuid: String,
    pub peerResult: String,
    pub u64Field1: u64,
    pub u64Field2: u64,
    pub data: Vec<u8>,
    pub extraData: Vec<u8>
}

