use std::fs::{File, OpenOptions};
use std::io::prelude::*;

pub struct CWrite {
    file: File
}

impl CWrite {
    pub fn write(&mut self, startPos: u64, data: &[u8]) -> Result<(), &str>{
        match self.file.seek(std::io::SeekFrom::Start(startPos)) {
            Ok(pos) => {},
            Err(err) => {
                println!("seek error, err: {}", err);
                return Err("seek error");
            }
        }
        if let Err(err) = self.file.write(data) {
            println!("write error, err: {}", err);
            return Err("write error");
        };
        if let Err(err) = self.file.flush() {
            println!("flush error, err: {}", err);
            return Err("flush error");
        };
        // if let Err(err) = self.file.sync_all() {
        //     println!("sync_all error, err: {}", err);
        //     return Err("sync_all error");
        // };
        Ok(())
    }
}

impl CWrite {
    pub fn new(path: &str) -> Option<CWrite> {
        let file = OpenOptions::new().write(true).create(true).append(true).open(path);
        let file = match file {
            Ok(f) => f,
            Err(err) => {
                println!("open file error, err: {}",  err);
                return None;
            }
        };
        Some(CWrite{
            file: file
        })
    }
}
