pub struct CStartParam {
    pub server: String,
    pub selfUuid: String,
    pub peerUuid: String,
    pub objectUuid: String,
    pub filePath: String,
    pub fileName: String,
    pub onceMaxLen: u64,
    pub connectTimeoutS: u64,
    pub sendTimeoutS: u64,
    pub downloadRoot: String,
    pub writeFileMode: String
}
