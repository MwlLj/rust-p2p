use super::IStorage;

pub struct CMemory {
    position: u64
}

impl CMemory {
    pub fn load(objectUuid: &str) -> Option<Self> {
        Some(CMemory{
            position: 0
        })
    }

    pub fn readPos(&self) -> u64 {
        self.position
    }

    pub fn writePos(&mut self, pos: u64) -> Result<(), &str> {
        self.position = pos;
        Ok(())
    }
}
