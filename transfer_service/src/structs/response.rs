/*
full response:
    responseMode
    serverUuid
    result
    selfCommunicateUuid
    peerCommunicateUuid
    objectUuid
    peerResult
    packageIndex
    packageTotal
    data
*/

#[derive(Default)]
pub struct CAck {
    pub serverUuid: String,
    pub result: u8
}

/*
#[derive(Default)]
pub struct CDateTransfer {
    pub selfCommunicateUuid: String,
    pub peerCommunicateUuid: String,
    pub objectUuid: String,
    pub packageIndex: u64,
    pub packageTotal: u64,
    pub data: Vec<u8>
}

#[derive(Default)]
pub struct CAckTransfer {
    pub selfCommunicateUuid: String,
    pub peerCommunicateUuid: String,
    pub objectUuid: String,
    pub peerResult: String
}
*/