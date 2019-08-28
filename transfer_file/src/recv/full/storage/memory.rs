use super::IStorage;

use std::collections::HashMap;

pub struct CMemory {
    positions: HashMap<String, u64>
}

impl CMemory {
    pub fn new() -> Option<Self> {
        Some(CMemory{
            positions: HashMap::new()
        })
    }

    pub fn readPos(&mut self, objectUuid: &str) -> u64 {
        match self.positions.get(objectUuid) {
            Some(p) => {
                return *p;
            },
            None => {
                self.positions.insert(objectUuid.to_string(), 0);
                return 0;
            }
        }
        0
    }

    pub fn writePos(&mut self, pos: u64, objectUuid: &str) -> Result<(), &str> {
        match self.positions.get_mut(objectUuid) {
            Some(p) => {
                *p = pos;
            },
            None => {
                self.positions.insert(objectUuid.to_string(), 0);
            }
        }
        Ok(())
    }

    pub fn del(&mut self, objectUuid: &str) -> Result<(), &str> {
        self.positions.remove(objectUuid);
        Ok(())
    }
}
