use super::IStorage;

use std::path::Path;
use std::fs;
use std::fs::{OpenOptions, File, DirBuilder};

pub struct CFile {
    tmp: String
}

impl CFile {
    pub fn new() -> Option<Self> {
        let tmp = "tmp";
        if !Path::new(tmp).exists() {
            if let Err(err) = DirBuilder::new().recursive(true).create(tmp) {
                println!("create tmp dir error, err: {}", err);
                return None;
            }
        }
        Some(CFile{
            tmp: tmp.to_string()
        })
    }

    pub fn readPosWithPath(&mut self, objectUuid: &str, inPath: &str) -> (u64, String) {
        let path = self.joinPath(objectUuid);
        if !Path::new(&path).exists() {
            fs::remove_file(inPath);
            let mut s = String::new();
            s.push_str("0");
            s.push_str("|");
            s.push_str(inPath);
            fs::write(&path, s.as_bytes());
            return (0, inPath.to_string());
        }
        let data = match fs::read(&path) {
            Ok(d) => d,
            Err(err) => {
                println!("read file: {} error, err: {}", &path, err);
                return (0, inPath.to_string());
            }
        };
        let value = match String::from_utf8(data) {
            Ok(d) => d,
            Err(err) => {
                println!("from utf8 error,err: {}", err);
                return (0, inPath.to_string());
            }
        };
        let index = match value.find("|") {
            Some(i) => i,
            None => {
                println!("not found |");
                return (0, inPath.to_string());
            }
        };
        let pos = &value[0..index];
        let pos = match pos.parse::<u64>() {
            Ok(p) => p,
            Err(err) => {
                println!("string parse to u64 error, err: {}", err);
                return (0, inPath.to_string());
            }
        };
        return (pos, value[(index+1)..].to_string());
    }

    pub fn readPos(&mut self, objectUuid: &str) -> u64 {
        let path = self.joinPath(objectUuid);
        if !Path::new(&path).exists() {
            fs::write(&path, "0".as_bytes());
            return 0;
        }
        let data = match fs::read(&path) {
            Ok(d) => d,
            Err(err) => {
                println!("read file: {} error, err: {}", &path, err);
                return 0;
            }
        };
        let value = match String::from_utf8(data) {
            Ok(d) => d,
            Err(err) => {
                println!("from utf8 error,err: {}", err);
                return 0;
            }
        };
        let pos = match value.parse::<u64>() {
            Ok(p) => p,
            Err(err) => {
                println!("string parse to u64 error, err: {}", err);
                return 0;
            }
        };
        return pos;
    }

    pub fn writePos(&mut self, pos: u64, objectUuid: &str) -> Result<(), &str> {
        let path = self.joinPath(objectUuid);
        if let Err(err) = fs::write(&path, pos.to_string().as_bytes()) {
            println!("write error, err: {}", err);
            return Err("write error");
        };
        Ok(())
    }

    pub fn writePosWithPath(&mut self, pos: u64, objectUuid: &str, inPath: &str) -> Result<(), &str> {
        let path = self.joinPath(objectUuid);
        let mut s = String::new();
        s.push_str(&pos.to_string());
        s.push_str("|");
        s.push_str(inPath);
        if let Err(err) = fs::write(&path, s.as_bytes()) {
            println!("write error, err: {}", err);
            return Err("write error");
        };
        Ok(())
    }

    pub fn del(&mut self, objectUuid: &str) -> Result<(), &str> {
        let path = self.joinPath(objectUuid);
        fs::remove_file(path);
        Ok(())
    }
}

impl CFile {
    fn joinPath(&self, objectUuid: &str) -> String {
        let mut path = String::new();
        path.push_str(&self.tmp);
        path.push_str("/");
        path.push_str(objectUuid);
        path.push_str(".tmp");
        path
    }
}

