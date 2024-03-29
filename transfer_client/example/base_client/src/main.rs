use transfer_client::client::tcp::simple;
use transfer_client::structs::{request, response};

use rust_parse::cmd::CCmd;
use uuid;

use std::thread;
use std::time;
use std::sync::Arc;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut cmdHandler = CCmd::new();
    let server = cmdHandler.register("-server", "127.0.0.1:20001");
    let selfUuid = cmdHandler.register("-self", "123456");
    let peerUuid = cmdHandler.register("-peer", "654321");
    let objectUuid = cmdHandler.register("-obj", "123");
    let data = cmdHandler.register("-data", "hello");
    let extraData = cmdHandler.register("-extraData", "");
    cmdHandler.parse();

    let server = server.borrow().to_string();
    let selfUuid = selfUuid.borrow().to_string();
    let peerUuid = peerUuid.borrow().to_string();
    let objectUuid = objectUuid.borrow().to_string();
    let data = data.borrow().to_string();
    let extraData = extraData.borrow().to_string();

    println!("self: {}, peer: {}", selfUuid, peerUuid);

    // let mut cli = match simple::CSimple::new(&(*server), |data: &response::CResponse, s: &simple::CSimple| -> bool {
    //     let mut file = OpenOptions::new().append(true).create(true).open("test.txt").expect("open file error");
    //     file.write_all(data.data.as_slice());
    //     // println!("recv data: {}", String::from_utf8(data.data).unwrap());
    //     s.sendAckToPeerAsync(&mut request::CAck{
    //         selfCommunicateUuid: data.selfCommunicateUuid.clone(),
    //         peerCommunicateUuid: data.peerCommunicateUuid.clone(),
    //         serverUuid: data.serverUuid.clone(),
    //         objectUuid: data.objectUuid.clone(),
    //         result: "success".to_string()
    //     });
    //     return true;
    // }) {
    //     Ok(cli) => cli,
    //     Err(err) => {
    //         println!("new error, err: {}", err);
    //         return;
    //     }
    // };
    let mut cli = match simple::CSimple::new(&(*server), |data: &response::CResponse| -> bool {
        let mut file = OpenOptions::new().append(true).create(true).open("test.txt").expect("open file error");
        file.write_all(data.data.as_slice());
        return true;
    }, |data: &response::CResponse| -> Option<request::CAck> {
        println!("send ack, dataUuid: {}", &data.dataUuid);
        Some(request::CAck{
            selfCommunicateUuid: data.selfCommunicateUuid.clone(),
            peerCommunicateUuid: data.peerCommunicateUuid.clone(),
            serverUuid: data.serverUuid.clone(),
            dataUuid: data.dataUuid.clone(),
            objectUuid: data.objectUuid.clone(),
            result: "success".to_string(),
            u64Field1: 0,
            u64Field2: 0,
            data: data.data.clone(),
            extraData: data.extraData.clone()
        })
    }) {
        Ok(cli) => cli,
        Err(err) => {
            println!("new error, err: {}", err);
            return;
        }
    };
    let serverUuid = match cli.connect(&mut request::CConnect{
        selfCommunicateUuid: (*selfUuid).to_string()
    }, 3) {
        Ok(serverUuid) => serverUuid,
        Err(err) => {
            println!("connect error, err: {}", err);
            return;
        }
    };
    println!("serverUuid: {}", &serverUuid);

    // sync send to peer
    loop {
        let uid = uuid::Uuid::new_v4().to_string();
        println!("send uuid: {}", &uid);
        cli.sendDataUtilPeerAck(&mut request::CData{
            selfCommunicateUuid: (*selfUuid).to_string(),
            peerCommunicateUuid: (*peerUuid).to_string(),
            serverUuid: serverUuid.clone(),
            dataUuid: uid,
            objectUuid: (*objectUuid).to_string(),
            u64Field1: 0,
            u64Field2: 0,
            data: (*data).as_bytes().to_vec(),
            extraData: (*extraData).as_bytes().to_vec()
        }, |peerResult: &str| -> bool {
            if peerResult == "success" {
                return true;
            }
            false
        }, 30);
        thread::sleep(time::Duration::from_secs(3));
    }
    /*
    */

    /*
    // async send to peer
    thread::spawn(move || {
        let cli = Arc::new(cli);
        loop {
            let cli = cli.clone();
            if let Err(err) = cli.sendAsync(&mut request::CData{
                selfCommunicateUuid: (*selfUuid).to_string(),
                peerCommunicateUuid: (*peerUuid).to_string(),
                serverUuid: serverUuid.clone(),
                objectUuid: (*objectUuid).to_string(),
                packageIndex: 0,
                packageTotal: 0,
                data: (*data).as_bytes().to_vec(),
                extraData: (*extraData).as_bytes().to_vec()
            }) {
                println!("sendAsyn error, err: {}", err);
                return;
            };
            thread::sleep(time::Duration::from_millis(50));
        }
    });
    */

    // let mut cli = simple::CSimple::new("").unwrap();
    // <simple::CSimple as client::IClient>::dataRecv(&mut cli, move |_data: &response::CResponse| -> bool {
    //     // <simple::CSimple as client::IClient>::send(&c, &request::CData::default());
    //     return true;
    // });

    // let mut cli = simple::CSimple::new("").unwrap();
    // let mut c = Rc::new(&cli);
    // // let mut cli = Rc::clone(&mut c);
    // cli.dataRecv(move |_data: &response::CResponse| -> bool {
    //     c.send(&request::CData::default());
    //     return true;
    // });

    loop {
        thread::sleep(time::Duration::from_secs(10));
    }
}
