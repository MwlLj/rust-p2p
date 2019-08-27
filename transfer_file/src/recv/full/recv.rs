use crate::consts;
use crate::structs;
use super::storage;

use transfer_client::client::tcp::simple;
use transfer_client::structs::{request, response};

type RecordStorage = storage::memory::CMemory;

pub struct CRecv {
    recorder: RecordStorage
}

impl CRecv {
    fn start(&self, param: &structs::input::CStartParam) -> Result<(), &str> {
        let mut cli = match simple::CSimple::new(&param.server, |data: &response::CResponse| -> bool {
            println!("startPos: {}, data: {:?}", data.u64Field1, String::from_utf8(data.data.clone()));
            return true;
        }, |data: &response::CResponse| -> Option<request::CAck> {
            let s1 = data.u64Field1;
            // let s2 = self.recorder.readPos();
            Some(request::CAck{
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
            })
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
    pub fn new(param: &structs::input::CStartParam) -> Option<CRecv> {
        let recorder = match RecordStorage::load(&param.objectUuid) {
            Some(r) => r,
            None => {
                return None;
            }
        };
        let mut r = CRecv{
            recorder: recorder
        };
        if let Err(err) = r.start(param) {
            return None;
        };
        Some(r)
    }
}
