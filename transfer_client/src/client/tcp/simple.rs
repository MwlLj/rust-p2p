use crate::structs::{request, response};
use crate::consts;
use crate::client;
use crate::decode;
use crate::encode;
use crate::wraps::defer;

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
use std::collections::HashMap;

// type DataAckCb = FnMut<&mut request::CAck>;
// type DataRecvCb = Box<dyn Fn(&response::CResponse, &CSimple) -> bool + Send + Sync>;
type DataRecvCb = Box<dyn Fn(&response::CResponse) -> bool + Send + Sync>;
type DataAckCb = Box<dyn Fn(&response::CResponse) -> Option<request::CAck> + Send + Sync>;
type SyncSenders = HashMap<String, Arc<Mutex<mpsc::Sender<response::CPeerAck>>>>;

pub struct CSimple {
    stream: TcpStream,
    ackSend: Arc<Mutex<mpsc::Sender<response::CAck>>>,
    ackRecv: Arc<Mutex<mpsc::Receiver<response::CAck>>>,
    // dataRecvCb: Arc<DataRecvCb>,
    dataSyncSenders: Arc<Mutex<SyncSenders>>
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
        let serverUuid = String::new();
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
                println!("ackRecv recv ack error, err: {}", err);
                return Err("ackRecv recv ack error");
            }
        };
        if ack.result != 0 {
            println!("server response error, result: {}", ack.result);
            return Err("server response error");
        }
        Ok(())
    }

    // pub fn sendAckToPeerAsync(&self, ack: &mut request::CAck) -> Result<(), &str> {
    //     let v = encode::request::req::encodeAck(ack);
    //     let mut writer = BufWriter::new(&self.stream);
    //     if let Err(err) = writer.write_all(&v) {
    //         return Err("sendAck write all error");
    //     };
    //     if let Err(err) = writer.flush() {
    //         return Err("sendAck flush error");
    //     };
    //     Ok(())
    // }

    pub fn sendDataUtilPeerAck<F>(&mut self, data: &mut request::CData
        , f: F, timeoutS: u64) -> Result<(), &str>
            where F: Fn(&str) -> bool {
        let v = encode::request::req::encodeData(data);
        {
            let mut writer = BufWriter::new(&self.stream);
            if let Err(err) = writer.write_all(&v) {
                return Err("write all error");
            };
            if let Err(err) = writer.flush() {
                return Err("flush error");
            };
        }
        let (s, r) = mpsc::channel();
        // store sends
        {
            let mut dataSyncSenders = match self.dataSyncSenders.lock() {
                Ok(d) => d,
                Err(err) => {
                    println!("dataSyncSenders lock error, err: {}", err);
                    return Err("dataSyncSenders lock error");
                }
            };
            dataSyncSenders.insert(data.dataUuid.clone(), Arc::new(Mutex::new(s)));
        }
        let _d = defer::defer(|| {
            println!("defer");
            let mut dataSyncSenders = match self.dataSyncSenders.lock() {
                Ok(d) => d,
                Err(err) => {
                    println!("dataSyncSenders lock error, err: {}", err);
                    return;
                }
            };
            dataSyncSenders.remove(&data.dataUuid);
        });
        // wait
        let ack = match r.recv_timeout(time::Duration::from_secs(timeoutS)) {
            Ok(ack) => ack,
            Err(err) => {
                println!("recv ack timeout error, err: {}", err);
                return Err("recv ack timeout error");
            }
        };
        if !f(&ack.peerResult) {
            println!("peer response error, result: {}", ack.peerResult);
            return Err("peer response error");
        }
        Ok(())
    }
}

impl CSimple {
    fn startLoop(stream: TcpStream
        , dataRecvCb: Arc<DataRecvCb>
        , dataAckCb: Arc<DataAckCb>
        , ackSend: Arc<Mutex<mpsc::Sender<response::CAck>>>
        , dataSyncSenders: Arc<Mutex<SyncSenders>>) {
        let s = match stream.try_clone() {
            Ok(s) => s,
            Err(err) => {
                println!("stream try clone error, err: {}", err);
                return;
            }
        };
        let mut block = tcp_block::CStreamBlockParse::new(s);
        let mut res = response::CResponse::default();
        block.lines(1, &mut res, &mut |index: u64, data: Vec<u8>, response: &mut response::CResponse| -> (bool, u64) {
            // println!("decode response");
            decode_response!(index, data, response);
        }, &mut |response: &mut response::CResponse| -> bool {
            println!("response mode: {:?}", &response.responseMode);
            if response.responseMode == consts::proto::response_mode_ack {
                CSimple::handleAck(ackSend.clone(), response);
            } else if response.responseMode == consts::proto::response_mode_data {
                CSimple::handleData(&stream, dataRecvCb.clone(), dataAckCb.clone(), response);
            } else if response.responseMode == consts::proto::request_mode_peer_ack {
                CSimple::handlePeerAck(dataSyncSenders.clone(), response);
            }
            return true;
        });
    }

    fn handleData(stream: &TcpStream, dataRecvCb: Arc<DataRecvCb>, dataAckCb: Arc<DataAckCb>, response: &response::CResponse) {
        (*dataRecvCb)(response);
        match (*dataAckCb)(response) {
            Some(mut ack) => {
                CSimple::sendAckToPeerAsync(stream, &mut ack);
            },
            None => {}
        }
    }

    fn handleAck(ackSend: Arc<Mutex<mpsc::Sender<response::CAck>>>, response: &response::CResponse) {
        let ackSend = match ackSend.lock() {
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

    fn handlePeerAck(dataSyncSenders: Arc<Mutex<SyncSenders>>, response: &response::CResponse) {
        println!("handlePeerAck, dataUuid: {}", &response.dataUuid);
        let mut dataSyncSenders = match dataSyncSenders.lock() {
            Ok(d) => d,
            Err(err) => {
                println!("dataSyncSenders lock error, err: {}", err);
                return;
            }
        };
        let sender = match dataSyncSenders.get(&response.dataUuid) {
            Some(s) => s,
            None => {
                println!("handle peer ack, not get dataUuid, map: {:?}, dataUuid: {}", &dataSyncSenders, &response.dataUuid);
                return;
            }
        };
        let sender = match sender.lock() {
            Ok(s) => s,
            Err(err) => {
                println!("sender lock error, err: {}", err);
                return;
            }
        };
        sender.send(response::CPeerAck{
            dataUuid: response.dataUuid.clone(),
            peerResult: response.peerResult.clone()
        });
    }

    fn sendAckToPeerAsync<'a>(stream: &'a TcpStream, ack: &mut request::CAck) -> Result<(), &'a str> {
        let v = encode::request::req::encodeAck(ack);
        // println!("send ack: {:?}", &v);
        let mut writer = BufWriter::new(stream);
        if let Err(err) = writer.write_all(&v) {
            return Err("sendAck write all error");
        };
        if let Err(err) = writer.flush() {
            return Err("sendAck flush error");
        };
        Ok(())
    }

    // fn startLoop(&self) {
    //     let s = match self.stream.try_clone() {
    //         Ok(s) => s,
    //         Err(err) => {
    //             println!("stream try clone error");
    //             return;
    //         }
    //     };
    //     let mut block = tcp_block::CStreamBlockParse::new(s);
    //     let mut res = response::CResponse::default();
    //     let mut simple = Arc::new(self);
    //     block.lines(1, &mut res, &mut |index: u64, data: Vec<u8>, response: &mut response::CResponse| -> (bool, u64) {
    //         println!("decode response");
    //         decode_response!(index, data, response);
    //     }, &mut |response: &mut response::CResponse| -> bool {
    //         println!("response mode: {:?}", &response.responseMode);
    //         if response.responseMode == consts::proto::response_mode_ack {
    //             simple.handleAck(response);
    //             // self.handleAck(response);
    //         } else if response.responseMode == consts::proto::response_mode_data {
    //             simple.handleData(response);
    //             // self.handleData(response);
    //         } else if response.responseMode == consts::proto::request_mode_peer_ack {
    //             simple.handlePeerAck(response);
    //             // self.handlePeerAck(response);
    //         }
    //         return true;
    //     });
    // }

    // fn handleData(&self, response: &response::CResponse) {
    //     (*self.dataRecvCb)(response, self);
    // }

    // fn handleAck(&self, response: &response::CResponse) {
    //     let ackSend = match self.ackSend.lock() {
    //         Ok(ackSend) => ackSend,
    //         Err(err) => {
    //             println!("handle ack error");
    //             return;
    //         }
    //     };
    //     ackSend.send(response::CAck{
    //         serverUuid: response.serverUuid.clone(),
    //         result: response.result
    //     });
    // }

    // fn handlePeerAck(&self, response: &response::CResponse) {
    //     let sender = match self.dataSyncSenders.get(&response.objectUuid) {
    //         Some(s) => s,
    //         None => {
    //             println!("handle peer ack");
    //             return;
    //         }
    //     };
    //     let sender = match sender.lock() {
    //         Ok(s) => s,
    //         Err(err) => {
    //             println!("sender lock error, err: {}", err);
    //             return;
    //         }
    //     };
    //     sender.send(response::CPeerAck{
    //         objectUuid: response.objectUuid.clone(),
    //         peerResult: response.peerResult.clone()
    //     });
    // }
}

impl CSimple {
    pub fn new<F: 'static + Send + Sync, AckF: 'static + Send + Sync>(server: &str, f: F, ackF: AckF) -> Result<CSimple, &str>
        // where F: Fn(&response::CResponse, &CSimple) -> bool {
        where F: Fn(&response::CResponse) -> bool
        , AckF: Fn(&response::CResponse) -> Option<request::CAck> {
        let stream = match TcpStream::connect(server) {
            Ok(s) => s,
            Err(err) => {
                println!("connect server error, err: {}", err);
                return Err("connect server error");
            }
        };
        let (s, r) = mpsc::channel();
        // let dataRecvCb = Arc::new(Box::new(f));
        let ackSend = Arc::new(Mutex::new(s));
        let dataSyncSenders = Arc::new(Mutex::new(HashMap::new()));
        let simple = CSimple{
            stream: stream.try_clone().unwrap(),
            ackSend: ackSend.clone(),
            ackRecv: Arc::new(Mutex::new(r)),
            // dataRecvCb: dataRecvCb.clone(),
            dataSyncSenders: dataSyncSenders.clone()
        };
        thread::spawn(move || {
            CSimple::startLoop(stream, Arc::new(Box::new(f)), Arc::new(Box::new(ackF)), ackSend, dataSyncSenders);
        });
        Ok(simple)
    }

    // pub fn new<F: 'static + Send + Sync>(server: &str, f: F) -> Result<Arc<CSimple>, &str>
    //     where F: Fn(&response::CResponse, &CSimple) -> bool {
    //     let stream = match TcpStream::connect(server) {
    //         Ok(s) => s,
    //         Err(err) => {
    //             println!("connect server error, err: {}", err);
    //             return Err("connect server error");
    //         }
    //     };
    //     let (s, r) = mpsc::channel();
    //     let simple = CSimple{
    //         stream: stream,
    //         ackSend: Arc::new(Mutex::new(s)),
    //         ackRecv: Arc::new(Mutex::new(r)),
    //         dataRecvCb: Arc::new(Box::new(f)),
    //         dataSyncSenders: HashMap::new()
    //     };
    //     let simple = Arc::new(simple);
    //     {
    //         let s = simple.clone();
    //         thread::spawn(move || {
    //             s.startLoop();
    //         });
    //     }
    //     Ok(simple.clone())
    // }
}

/*
impl std::ops::Deref for CSimple {
    type Target = Self;

    fn deref(&self) -> &Self::Target {
        &self
    }
}

impl std::ops::DerefMut for CSimple {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self
    }
}
*/
