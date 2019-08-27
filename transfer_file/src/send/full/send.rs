use crate::consts;
use crate::structs;
use crate::file;
use super::storage;

use transfer_client::client::tcp::simple;
use transfer_client::structs::{request, response};

type RecordStorage = storage::memory::CMemory;

pub struct CSend {
    recorder: RecordStorage
}

impl CSend {
    fn start(&mut self, param: &structs::input::CStartParam) -> Result<(), &str> {
        let path = match String::from_utf8(param.extraData.clone()) {
            Ok(p) => p,
            Err(err) => {
                println!("extraData is not valid file path, err: {}", err);
                return Err("extraData is not valid file path");
            }
        };
        let mut cli = match simple::CSimple::new(&param.server, |data: &response::CResponse| -> bool {
            return true;
        }, |data: &response::CResponse| -> Option<request::CAck> {
            None
        }) {
            Ok(cli) => cli,
            Err(err) => {
                println!("new simple client error, err: {}", err);
                return Err("new simple client error");
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
        let read = match file::full::read::CRead::new(&path) {
            Some(r) => r,
            None => {
                println!("new CRead error");
                return Err("new CRead error");
            }
        };
        // get start position
        let startPos = self.recorder.readPos();
        read.read(startPos, param.onceMaxLen, &mut |start: &u64, once: &u64, data: Vec<u8>| -> file::full::read::ResultCode {
            let next = start + once;
            println!("next: {}", &next);
            let uid = uuid::Uuid::new_v4().to_string();
            match cli.sendDataUtilPeerAck(&mut request::CData{
                selfCommunicateUuid: param.selfUuid.clone(),
                peerCommunicateUuid: param.peerUuid.clone(),
                serverUuid: serverUuid.clone(),
                dataUuid: uid,
                objectUuid: param.objectUuid.clone(),
                u64Field1: next,
                u64Field2: 0,
                data: data,
                extraData: param.extraData.clone()
            }, |ack: &response::CPeerAck| -> Result<(), simple::ResultCode> {
                if ack.peerResult == consts::errors::success {
                    if let Err(err) = self.recorder.writePos(next) {
                        return Err(simple::ResultCode::WriteError);
                    };
                    return Ok(());
                } else if ack.peerResult == consts::errors::pos_error {
                    // return false, reread file
                    return Err(simple::ResultCode::PosError(ack.u64Field1));
                }
                Err(simple::ResultCode::Error)
            }, 30) {
                Ok(()) => {},
                Err(err) => {
                    match err {
                        simple::ResultCode::PosError(pos) => {
                            return file::full::read::ResultCode::Continue(pos);
                        },
                        _ => {
                            return file::full::read::ResultCode::Error;
                        }
                    }
                }
            }
            return file::full::read::ResultCode::Success;
        });
        Ok(())
    }
}

impl CSend {
    pub fn new(param: &structs::input::CStartParam) -> Option<CSend> {
        let recorder = match RecordStorage::load(&param.objectUuid) {
            Some(r) => r,
            None => {
                return None;
            }
        };
        let mut s = CSend{
            recorder: recorder
        };
        if let Err(err) = s.start(param) {
            return None;
        };
        Some(s)
    }
}
