#[derive(Default)]
pub struct CRequest {
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

impl CCheckResponse {
    pub fn setSelfUuid(&mut self, selfUuid: &str) {
        self.selfUuid = selfUuid.to_string();
    }

    pub fn getSelfUuid(&self) -> &str {
        return &self.selfUuid;
    }
}

impl CRequest {
    pub fn setSelfUuid(&mut self, selfUuid: &str) {
        self.selfUuid = selfUuid.to_string();
    }

    pub fn getSelfUuid(&self) -> &str {
        return &self.selfUuid;
    }

    pub fn setLanIp(&mut self, lanIp: &str) {
        self.lanIp = lanIp.to_string();
    }

    pub fn getLanIp(&self) -> &str {
        return &self.lanIp;
    }

    pub fn setLanPort(&mut self, lanPort: &str) {
        self.lanPort = lanPort.to_string();
    }

    pub fn getLanPort(&self) -> &str {
        return &self.lanPort;
    }
}

