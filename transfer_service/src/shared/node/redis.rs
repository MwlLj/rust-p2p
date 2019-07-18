use crate::shared;

pub struct CRedis {
}

impl CRedis {
    pub fn communicateNode(&self, id: &str) -> Option<shared::node::CCommunicateNode> {
        None
    }

    pub fn addCommunicateNode(&self, id: &str, node: &shared::node::CCommunicateNode) -> Result<(), &str> {
        Ok(())
    }

    pub fn delNode(&self, id: &str) -> Result<(), &str> {
        Ok(())
    }
}
