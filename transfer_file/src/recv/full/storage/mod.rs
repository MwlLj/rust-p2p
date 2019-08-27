pub trait IStorage : Sized {
    fn load(objectUuid: &str) -> Option<Self>;
    fn readPos(&self) -> u64;
    fn writePos(&mut self, pos: u64) -> Result<(), &str>;
}

pub mod file;
pub mod memory;
