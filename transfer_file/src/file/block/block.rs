#[derive(Default)]
pub struct Block {
    startPos: u64,
    endPos: u64
}

#[derive(Default)]
pub struct CBlock {
    offset: u64,
    startPos: u64,
    endPos: u64,
    length: u64
}

impl CBlock {
    pub fn sub(&mut self, size: u64) -> Option<Block> {
        if self.offset >= self.length {
            return None;
        }
        let block = Block{
            startPos: self.startPos + self.offset,
            endPos: self.startPos + self.offset + size
        };
        self.offset += size;
        Some(block)
    }
}

impl CBlock {
    pub fn new(startPos: u64, endPos: u64) -> CBlock {
        CBlock{
            offset: 0,
            startPos: startPos,
            endPos: endPos,
            length: endPos - startPos
        }
    }
}
