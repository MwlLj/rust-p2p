/*
1. provide handle result
*/

pub trait IReport {
    fn handlePeerNotExist(&self) -> Result<(), &str>;
}