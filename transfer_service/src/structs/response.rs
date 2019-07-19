/*
full response:
    responseMode
    serverUuid
    result
    selfCommunicateUuid
    peerCommunicateUuid
    packageIndex
    packageTotal
    data
*/

#[derive(Default)]
pub struct CAck {
    pub serverUuid: String,
    pub result: u8
}

#[derive(Default)]
pub struct CTransfer {
    pub selfCommunicateUuid: String,
    pub peerCommunicateUuid: String,
    pub packageIndex: u64,
    pub packageTotal: u64,
    pub data: Vec<u8>
}