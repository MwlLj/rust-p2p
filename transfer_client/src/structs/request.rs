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
    pub dataUuid: String,
    pub objectUuid: String,
    pub packageIndex: u64,
    pub packageTotal: u64,
    pub data: Vec<u8>,
    pub extraData: Vec<u8>
}

#[derive(Default, Debug)]
pub struct CAck {
    pub selfCommunicateUuid: String,
    pub peerCommunicateUuid: String,
    pub serverUuid: String,
    pub dataUuid: String,
    pub result: String
}

// #[derive(Default)]
// pub struct CPeerAck {
//     pub selfCommunicateUuid: String,
//     pub peerCommunicateUuid: String,
//     pub objectUuid: String,
//     pub peerResult: String
// }

