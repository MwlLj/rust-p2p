use crate::structs::{request, response};
use crate::consts;
use crate::client;
use crate::decode;
use crate::encode;

use number_conv::array::u8arr;
use rust_parse::stream::tcp_block;

use std::net::TcpStream;
use std::io::BufWriter;
use std::io::prelude::*;
use std::sync::mpsc;
use std::time;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

type DataRecvCb = Box<dyn Fn(&response::CResponse, &CSimple) -> bool + Send + Sync>;

pub struct CSimple {
    stream: TcpStream,
    ackSend: Arc<Mutex<mpsc::Sender<response::CAck>>>,
    ackRecv: Arc<Mutex<mpsc::Receiver<response::CAck>>>,
    dataRecvCb: Arc<DataRecvCb>
}

impl CSimple {
    pub fn connect(&self, conn: &mut request::CConnect, timeoutS: u64) -> Result<String, &str> {
        let v = encode::request::req::encodeConnect(conn);
        let mut writer = BufWriter::new(&self.stream);
        if let Err(err) = writer.write_all(&v) {
            return Err("write all error");
        };
        if let Err(err) = writer.flush() {
            return Err("flush error");
        };
        let mut serverUuid = String::new();
        let s = match self.stream.try_clone() {
            Ok(s) => s,
            Err(err) => {
                println!("stream try clone error, err: {}", err);
                return Err("stream try clone error");
            }
        };
        let ackRecv = match self.ackRecv.lock() {
            Ok(ackRecv) => ackRecv,
            Err(err) => {
                println!("ackRecv lock error, err: {}", err);
                return Err("ackRecv lock error");
            }
        };
        let ack = match ackRecv.recv_timeout(time::Duration::from_secs(timeoutS)) {
            Ok(ack) => ack,
            Err(err) => {
                println!("connect recv ack error, err: {}", err);
                return Err("connect recv ack error");
            }
        };
        if ack.result != 0 {
            println!("server response error, result: {}", ack.result);
            return Err("server response error");
        }
        Ok(ack.serverUuid.clone())
    }

    pub fn sendAsync(&self, data: &mut request::CData) -> Result<(), &str> {
        let v = encode::request::req::encodeData(data);
        let mut writer = BufWriter::new(&self.stream);
        if let Err(err) = writer.write_all(&v) {
            return Err("write all error");
        };
        if let Err(err) = writer.flush() {
            return Err("flush error");
        };
        Ok(())
    }

    pub fn sendSync(&self, data: &mut request::CData, timeoutS: u64) -> Result<(), &str> {
        let v = encode::request::req::encodeData(data);
        let mut writer = BufWriter::new(&self.stream);
        if let Err(err) = writer.write_all(&v) {
            return Err("write all error");
        };
        if let Err(err) = writer.flush() {
            return Err("flush error");
        };
        let ackRecv = match self.ackRecv.lock() {
            Ok(ackRecv) => ackRecv,
            Err(err) => {
                println!("ackRecv lock error, err: {}", err);
                return Err("ackRecv lock error");
            }
        };
        let ack = match ackRecv.recv_timeout(time::Duration::from_secs(timeoutS)) {
            Ok(ack) => ack,
            Err(err) => {
                println!("connect recv ack error, err: {}", err);
                return Err("connect recv ack error");
            }
        };
        if ack.result != 0 {
            println!("server response error, result: {}", ack.result);
            return Err("server response error");
        }
        Ok(())
    }
}

impl CSimple {
    fn startLoop(&self) {
        let s = match self.stream.try_clone() {
            Ok(s) => s,
            Err(err) => {
                println!("stream try clone error");
                return;
            }
        };
        let mut block = tcp_block::CStreamBlockParse::new(s);
        let mut res = response::CResponse::default();
        let simple = Arc::new(self);
        block.lines(1, &mut res, &mut |index: u64, data: Vec<u8>, response: &mut response::CResponse| -> (bool, u64) {
            decode_response!(index, data, response);
        }, &mut |response: &mut response::CResponse| -> bool {
            if response.responseMode == consts::proto::response_mode_ack {
                simple.handleAck(response);
            } else if response.responseMode == consts::proto::response_mode_data {
                simple.handleData(response);
            } else if response.responseMode == consts::proto::response_mode_peer_ack {
            }
            return true;
        });
    }

    fn handleData(&self, response: &response::CResponse) {
        (*self.dataRecvCb)(response, self);
    }

    fn handleAck(&self, response: &response::CResponse) {
        let ackSend = match self.ackSend.lock() {
            Ok(ackSend) => ackSend,
            Err(err) => {
                println!("handle ack error");
                return;
            }
        };
        ackSend.send(response::CAck{
            serverUuid: response.serverUuid.clone(),
            result: response.result
        });
    }
}

impl CSimple {
    pub fn new<F: 'static + Send + Sync>(server: &str, f: F) -> Result<Arc<CSimple>, &str>
        where F: Fn(&response::CResponse, &CSimple) -> bool {
        let stream = match TcpStream::connect(server) {
            Ok(s) => s,
            Err(err) => {
                println!("connect server error, err: {}", err);
                return Err("connect server error");
            }
        };
        let (s, r) = mpsc::channel();
        let simple = CSimple{
            stream: stream,
            ackSend: Arc::new(Mutex::new(s)),
            ackRecv: Arc::new(Mutex::new(r)),
            dataRecvCb: Arc::new(Box::new(f))
        };
        let simple = Arc::new(simple);
        {
            let s = simple.clone();
            thread::spawn(move || {
                s.startLoop();
            });
        }
        Ok(simple.clone())
    }
}
