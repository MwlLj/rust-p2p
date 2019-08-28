use crate::consts;
use crate::structs;
use crate::file;
use crate::wraps;
use super::storage;

use transfer_client::client::tcp::simple;
use transfer_client::structs::{request, response};

use std::sync::{Arc, Mutex};
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::cell::RefCell;

// type RecordStorage = storage::memory::CMemory;
type RecordStorage = storage::file::CFile;

pub struct CRecv {
    recorder: Arc<Mutex<RecordStorage>>,
    paths: Arc<Mutex<HashMap<String, String>>>
}

impl CRecv {
    fn start(&self, param: &structs::input::CStartParam) -> Result<(), &str> {
        if !Path::new(&param.downloadRoot).exists() {
            if let Err(err) = fs::DirBuilder::new().recursive(true).create(&param.downloadRoot) {
                println!("create dirs error, err: {}", err);
                return Err("create dirs error");
            };
        }
        let writeFileMode = param.writeFileMode.to_string();
        let downloadRoot = param.downloadRoot.to_string();
        let recorder = self.recorder.clone();
        let paths = self.paths.clone();
        let mut cli = match simple::CSimple::new(&param.server, |data: &response::CResponse| -> bool {
            return true;
        }, move |data: &response::CResponse| -> Option<request::CAck> {
            let resultCell = RefCell::new(String::new());
            *resultCell.borrow_mut() = consts::errors::success.to_string();
            let mut ack = request::CAck{
                selfCommunicateUuid: data.selfCommunicateUuid.clone(),
                peerCommunicateUuid: data.peerCommunicateUuid.clone(),
                serverUuid: data.serverUuid.clone(),
                dataUuid: data.dataUuid.clone(),
                objectUuid: data.objectUuid.clone(),
                result: consts::errors::success.to_string(),
                u64Field1: data.u64Field1,
                u64Field2: data.u64Field2,
                data: data.data.clone(),
                extraData: data.extraData.clone()
            };
            let extraData = match String::from_utf8(data.extraData.clone()) {
                Ok(p) => p,
                Err(err) => {
                    println!("extraData is not valid file path, err: {}", err);
                    ack.result = consts::errors::error.to_string();
                    return Some(ack);
                }
            };
            let mut path = match CRecv::joinPath(paths.clone(), &data.objectUuid, &writeFileMode, &downloadRoot, &extraData, data.u64Field2) {
                Some(p) => p,
                None => {
                    println!("joinPath error");
                    *resultCell.borrow_mut() = consts::errors::error.to_string();
                    ack.result = consts::errors::error.to_string();
                    return Some(ack);
                }
            };
            let s1 = data.u64Field1;
            let mut s2 = 0;
            {
                let mut recorder = match recorder.lock() {
                    Ok(r) => r,
                    Err(err) => {
                        ack.result = consts::errors::error.to_string();
                        return Some(ack);
                    }
                };
                let (_s2, _path) = recorder.readPosWithPath(&data.objectUuid, &path);
                s2 = _s2;
                path = _path;
            }
            let _d = wraps::defer::defer(|| {
                if data.u64Field2 == s1
                || *resultCell.borrow() == consts::errors::error {
                    // file total == next pos
                    // file send finish
                    println!("dataUuid destory, dataUuid: {}", &data.dataUuid);
                    let mut paths = match paths.lock() {
                        Ok(p) => p,
                        Err(err) => {
                            println!("paths lock error, err: {}", err);
                            return;
                        }
                    };
                    paths.remove(&data.objectUuid);
                    {
                        let mut recorder = match recorder.lock() {
                            Ok(r) => r,
                            Err(err) => {
                                return;
                            }
                        };
                        recorder.del(&data.objectUuid);
                    }
                }
            });
            let mut writer = match file::full::write::CWrite::new(&path) {
                Some(w) => w,
                None => {
                    *resultCell.borrow_mut() = consts::errors::error.to_string();
                    ack.result = consts::errors::error.to_string();
                    println!("new write error, path: {}", &path);
                    return Some(ack);
                }
            };
            if s2 == 0 {
                // writer file
                println!("write file, next pos: {}", s1);
                if let Err(err) = writer.write(s1, &data.data) {
                    *resultCell.borrow_mut() = consts::errors::error.to_string();
                    ack.result = consts::errors::error.to_string();
                };
                // record pos
                {
                    let mut recorder = match recorder.lock() {
                        Ok(r) => r,
                        Err(err) => {
                            ack.result = consts::errors::error.to_string();
                            return Some(ack);
                        }
                    };
                    recorder.writePosWithPath(s1, &data.objectUuid, &path);
                }
            } else if s1 == (s2 + data.data.len() as u64) {
                // writer file
                println!("write file, next pos: {}", s1);
                if let Err(err) = writer.write(s1, &data.data) {
                    *resultCell.borrow_mut() = consts::errors::error.to_string();
                    ack.result = consts::errors::error.to_string();
                };
                // record pos
                {
                    let mut recorder = match recorder.lock() {
                        Ok(r) => r,
                        Err(err) => {
                            ack.result = consts::errors::error.to_string();
                            return Some(ack);
                        }
                    };
                    recorder.writePosWithPath(s1, &data.objectUuid, &path);
                }
            } else if s1 > s2 {
                ack.u64Field1 = s2;
                ack.result = consts::errors::pos_error.to_string();
            } else if s1 < s2 {
                ack.u64Field1 = s2;
                ack.result = consts::errors::pos_error.to_string();
            } else {
            }
            Some(ack)
        }) {
            Ok(cli) => cli,
            Err(err) => {
                println!("new error, err: {}", err);
                return Err("new error");
            }
        };
        let serverUuid = match cli.connect(&mut request::CConnect{
            selfCommunicateUuid: param.selfUuid.clone()
        }, param.connectTimeoutS) {
            Ok(serverUuid) => serverUuid,
            Err(err) => {
                println!("connect error, err: {}", err);
                return Err("connect error");
            }
        };
        Ok(())
    }
}

impl CRecv {
    fn joinPath(paths: Arc<Mutex<HashMap<String, String>>>, objectUuid: &str, writeFileMode: &str, downloadRoot: &str, extraData: &str, sourceTotal: u64) -> Option<String> {
        {
            let paths = match paths.lock() {
                Ok(p) => p,
                Err(err) => {
                    println!("paths lock error, err: {}", err);
                    return None;
                }
            };
            if let Some(p) = paths.get(objectUuid) {
                return Some(p.to_string());
            };
        }
        let mut path = String::new();
        path.push_str(downloadRoot);
        path.push_str("/");
        if writeFileMode == consts::input::file_write_mode_cover {
            path.push_str(extraData);
            /*
            let meta = match fs::metadata(&path) {
                Ok(m) => m,
                Err(err) => {
                    println!("metadata error, err: {}", err);
                    return None;
                }
            };
            let total = meta.len();
            if total == sourceTotal && total != 0 {
                // delete file
                if Path::new(&path).exists() {
                    if let Err(err) = fs::remove_file(&path) {
                        println!("remove file error, err:");
                        return None;
                    };
                }
            }
            */
        } else if writeFileMode == consts::input::file_write_mode_create {
            let mut t = String::new();
            t.push_str(&path);
            t.push_str(extraData);
            if !Path::new(&t).exists() {
                path.push_str(extraData);
            } else {
                let mut fileName = String::new();
                let mut ext = String::new();
                match extraData.rfind(".") {
                    Some(i) => {
                        fileName = extraData[0..i].to_string();
                        ext = extraData[(i+1)..].to_string();
                    },
                    None => {
                        fileName = extraData.to_string();
                    }
                }
                let mut index = 1;
                loop {
                    let mut tmp = String::new();
                    tmp.push_str(&fileName);
                    tmp.push_str(".");
                    tmp.push_str(&index.to_string());
                    tmp.push_str(".");
                    tmp.push_str(&ext);
                    let mut tt = String::new();
                    tt.push_str(&path);
                    tt.push_str(&tmp);
                    if !Path::new(&tt).exists() {
                        path.push_str(&tmp);
                        break;
                    }
                    index += 1;
                }
            }
        }
        {
            let mut paths = match paths.lock() {
                Ok(p) => p,
                Err(err) => {
                    println!("paths lock error, err: {}", err);
                    return None;
                }
            };
            paths.insert(objectUuid.to_string(), path.clone());
        }
        Some(path)
    }
}

impl CRecv {
    pub fn new(param: &structs::input::CStartParam) -> Option<CRecv> {
        let recorder = match RecordStorage::new() {
            Some(r) => r,
            None => {
                return None;
            }
        };
        let mut r = CRecv{
            recorder: Arc::new(Mutex::new(recorder)),
            paths: Arc::new(Mutex::new(HashMap::new()))
        };
        if let Err(err) = r.start(param) {
            return None;
        };
        Some(r)
    }
}

#[test]
fn joinPathTest() {
    let path = CRecv::joinPath(consts::input::file_write_mode_create, "./dst", "test.txt");
    println!("path: {:?}", path);
}
