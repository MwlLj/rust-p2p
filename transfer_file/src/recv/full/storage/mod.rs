pub trait IStorage : Sized {
    fn new() -> Option<Self>;
    fn readPos(&self, objectUuid: &str) -> u64;
    fn writePos(&mut self, pos: u64, objectUuid: &str) -> Result<(), &str>;
    fn del(&mut self, objectUuid: &str) -> Result<(), &str>;
}

pub mod file;
pub mod memory;
