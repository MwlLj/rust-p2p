use crate::shared;

pub struct CRedis {
}

impl shared::IShared for CRedis {
    fn selfExist(&self, id: &str) -> Option<shared::CSelf>
    {
        None
    }

    fn peerExist(&self, id: &str) -> Option<shared::CNode>
    {
        None
    }

    fn addSelf(&self, id: &str, obj: shared::CSelf, ttlMs: u32) -> Result<(), &str>
    {
        Ok(())
    }

    fn addPeer(&self, id: &str, obj: shared::CNode, ttlMs: u32) -> Result<(), &str>
    {
        Ok(())
    }

    fn delNode(&self, id: &str) -> Result<(), &str>
    {
        Ok(())
    }
}

impl CRedis {
    pub fn new(addr: &str) -> CRedis {
        CRedis{}
    }
}
