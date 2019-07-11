#[derive(Default)]
pub struct CRequest {
    selfUuid: String,
    lanIp: String,
    lanPort: String
}

#[derive(Default)]
pub struct CCheckResponse {
    selfUuid: String
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

