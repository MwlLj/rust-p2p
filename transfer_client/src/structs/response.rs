/*
full response:
    responseMode
    serverUuid
    result
    selfCommunicateUuid
    peerCommunicateUuid
    dataUuid
    objectUuid
    peerResult
    packageIndex
    packageTotal
    data
    extraData
*/

#[derive(Default, Debug)]
pub struct CResponse {
    pub responseMode: String,
    pub serverUuid: String,
    pub result: u8,
    pub selfCommunicateUuid: String,
    pub peerCommunicateUuid: String,
    pub dataUuid: String,
    pub objectUuid: String,
    pub peerResult: String,
    pub u64Field1: u64,
    pub u64Field2: u64,
    pub data: Vec<u8>,
    pub extraData: Vec<u8>
}

#[derive(Default, Debug)]
pub struct CAck {
    pub serverUuid: String,
    pub result: u8
}

#[derive(Default, Debug)]
pub struct CPeerAck {
    pub dataUuid: String,
    pub objectUuid: String,
    pub peerResult: String,
    pub u64Field1: u64,
    pub u64Field2: u64,
    pub data: Vec<u8>,
    pub extraData: Vec<u8>
}
