extern crate transfer_file;

use transfer_file::send;
use transfer_file::recv;
use transfer_file::consts;
use transfer_file::structs;

use rust_parse::cmd::CCmd;
use uuid;

use std::thread;
use std::time;

fn start() {
    let mut cmdHandler = CCmd::new();
    let mode = cmdHandler.register("-mode", "");
    let server = cmdHandler.register("-server", "127.0.0.1:20001");
    let selfUuid = cmdHandler.register("-self", "123456");
    let peerUuid = cmdHandler.register("-peer", "654321");
    let objectUuid = cmdHandler.register("-obj", "");
    let extraData = cmdHandler.register("-extra-data", "");
    let onceMax = cmdHandler.register("-once-max", "256");
    let connectTimeoutS = cmdHandler.register("-conn-timeouts", "10");
    let sendTimeoutS = cmdHandler.register("-send-timeouts", "60");
    let downloadRoot = cmdHandler.register("-download-root", "./dst");
    let writeFileMode = cmdHandler.register("-write-file-mode", consts::input::file_write_mode_create);
    cmdHandler.parse();

    let mode = mode.borrow();
    if *mode == "" {
        println!("please input mode, if send, input: -mode send; if recv, input: -mode recv");
        return;
    }
    let server = server.borrow().to_string();
    let selfUuid = selfUuid.borrow().to_string();
    let peerUuid = peerUuid.borrow().to_string();
    let objectUuid = objectUuid.borrow();
    let extraData = extraData.borrow().to_string();
    let mut objUuid = objectUuid.to_string();
    if objUuid == "" {
        // objUuid = uuid::Uuid::new_v4().to_string();
        objUuid = extraData.clone();
    }
    let onceMax = match onceMax.borrow().parse::<u64>() {
        Ok(v) => v,
        Err(err) => {
            println!("onceMax is invalid, err: {}", err);
            return;
        }
    };
    let connectTimeoutS = match connectTimeoutS.borrow().parse::<u64>() {
        Ok(v) => v,
        Err(err) => {
            println!("connectTimeoutS is invalid, err: {}", err);
            return;
        }
    };
    let sendTimeoutS = match sendTimeoutS.borrow().parse::<u64>() {
        Ok(v) => v,
        Err(err) => {
            println!("sendTimeoutS is invalid, err: {}", err);
            return;
        }
    };
    let downloadRoot = downloadRoot.borrow().to_string();
    let writeFileMode = writeFileMode.borrow().to_string();

    if *mode == consts::input::mode_send {
        let send = match send::full::send::CSend::new(&structs::input::CStartParam{
            server: server,
            selfUuid: selfUuid,
            peerUuid: peerUuid,
            objectUuid: objUuid,
            extraData: Vec::from(extraData),
            onceMaxLen: onceMax,
            connectTimeoutS: connectTimeoutS,
            sendTimeoutS: sendTimeoutS,
            downloadRoot: downloadRoot,
            writeFileMode: writeFileMode
        }) {
            Some(s) => s,
            None => {
                println!("send new error");
                return;
            }
        };
    } else if *mode == consts::input::mode_recv {
        let recv = match recv::full::recv::CRecv::new(&structs::input::CStartParam{
            server: server,
            selfUuid: selfUuid,
            peerUuid: peerUuid,
            objectUuid: objUuid,
            extraData: Vec::from(extraData),
            onceMaxLen: onceMax,
            connectTimeoutS: connectTimeoutS,
            sendTimeoutS: sendTimeoutS,
            downloadRoot: downloadRoot,
            writeFileMode: writeFileMode
        }) {
            Some(r) => r,
            None => {
                println!("recv new error");
                return;
            }
        };
    } else {
        println!("mode is not support");
        return;
    }

    loop {
        thread::sleep(time::Duration::from_secs(10));
    }
}

fn main() {
    start();
}
