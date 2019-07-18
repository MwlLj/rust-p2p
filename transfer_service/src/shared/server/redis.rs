use crate::shared;

pub struct CRedis {
}

impl CRedis {
    pub fn addServer(&self, id: &str, info: &shared::server::CServerInfo) -> Result<(), &str> {
        Ok(())
    }

    pub fn delServer(&self, id: &str) -> Result<(), &str> {
        Ok(())
    }

    pub fn server(&self, id: &str) -> Option<shared::server::CServerInfo> {
        None
    }
}
