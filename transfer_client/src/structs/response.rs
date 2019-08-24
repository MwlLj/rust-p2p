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
    extraData
*/

#[derive(Default)]
pub struct CResponse {
    pub responseMode: String,
    pub serverUuid: String,
    pub result: u8,
    pub selfCommunicateUuid: String,
    pub peerCommunicateUuid: String,
    pub objectUuid: String,
    pub peerResult: String,
    pub packageIndex: u64,
    pub packageTotal: u64,
    pub data: Vec<u8>,
    pub extraData: Vec<u8>
}

#[derive(Default)]
pub struct CAck {
    pub serverUuid: String,
    pub result: u8
}
