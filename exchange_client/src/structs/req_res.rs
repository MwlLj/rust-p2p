#[derive(Default)]
pub struct CRequest {
    pub requestType: String,
    pub selfUuid: String,
    pub communicateUuid: String,
    pub lanIp: String,
    pub lanPort: String
}

#[derive(Default)]
pub struct CCheckResponse {
    pub selfUuid: String
}

#[derive(Default)]
pub struct CPeerNetResponse {
    pub peerIp: String,
    pub peerPort: String,
    pub portInterval: i32
}

