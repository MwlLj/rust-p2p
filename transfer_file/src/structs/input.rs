pub struct CStartParam {
    pub server: String,
    pub selfUuid: String,
    pub peerUuid: String,
    pub objectUuid: String,
    pub extraData: Vec<u8>,
    pub onceMaxLen: u64,
    pub connectTimeoutS: u64
}
