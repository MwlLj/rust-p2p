use crate::transfer;
use crate::consts;
use crate::shared;
use crate::structs::{request, response};
use crate::encode;
use crate::decode;

use uuid::Uuid;
use net2::{TcpBuilder};
use rust_parse::stream::tcp_block;
use number_conv::array::u8arr;
use socket::fd::tcp;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::Mutex;
use std::io::BufWriter;
use std::io::prelude::*;
use std::mem;

type NodeSharedStorage = shared::node::redis::CRedis;
type ServerSharedStorage = shared::server::redis::CRedis;
type Client = transfer::tcp::simple_client::CClient;

struct CSimple {
    serverUuid: String,
    nodeStorage: Arc<Mutex<NodeSharedStorage>>,
    serverStorage: Arc<Mutex<ServerSharedStorage>>,
    client: Arc<Mutex<Client>>
}

impl CSimple {
    fn handleClient(&self, stream: TcpStream) -> Result<(), &str> {
        let nodeStorage = self.nodeStorage.clone();
        let serverStorage = self.serverStorage.clone();
        let serverUuid = self.serverUuid.clone();
        let client = self.client.clone();
        thread::spawn(move || {
            let mut req = request::CRequest::default();
            loop {
                let s = match stream.try_clone() {
                    Ok(s) => s,
                    Err(err) => {
                        println!("stream clone error");
                        break;
                    }
                };
                let mut block = tcp_block::CStreamBlockParse::new(s);
                block.lines(1, &mut req, &mut |index: u64, data: Vec<u8>, request: &mut request::CRequest| -> (bool, u64) {
                    decode_request!(index, data, request);
                }, &mut |req: &mut request::CRequest| -> bool {
                    let stream = match stream.try_clone() {
                        Ok(s) => s,
                        Err(err) => {
                            println!("stream clone error");
                            return false;
                        }
                    };
                    let nodeStorage = nodeStorage.clone();
                    let serverStorage = serverStorage.clone();
                    let serverUuid = serverUuid.clone();
                    let client = client.clone();
                    let mut request = req.clone();
                    thread::spawn(move || {
                        CSimple::handleRequest(request, stream, nodeStorage, serverStorage, serverUuid, client);
                    });
                    return true;
                });
                break;
            }
            /*
            disconnect:
                1. delete from shared
            */
            let nodeStorage = match nodeStorage.lock() {
                Ok(s) => s,
                Err(err) => {
                    return;
                }
            };
            nodeStorage.delNode(&req.selfCommunicateUuid);
        });
        Ok(())
    }

    fn handleRequest(mut request: request::CRequest, stream: TcpStream, nodeStorage: Arc<Mutex<NodeSharedStorage>>, serverStorage: Arc<Mutex<ServerSharedStorage>>, serverUuid: String, client: Arc<Mutex<Client>>) {
        let mut result: u8 = 0;
        loop {
            let s = match stream.try_clone() {
                Ok(s) => s,
                Err(err) => {
                    println!("stream clone error");
                    // return false;
                    return;
                }
            };
            if request.requestMode == consts::proto::request_mode_connect {
                // handleConnect
                if let Err(err) = CSimple::handleConnect(nodeStorage.clone(), s, &request.selfCommunicateUuid, &serverUuid) {
                    println!("handle connect error, error: {}", err);
                    result = 1;
                    break;
                }
            } else if request.requestMode == consts::proto::request_mode_data {
                // handleDataTransfer
                if let Err(err) = CSimple::handleTransfer(&serverUuid, client.clone(), serverStorage.clone(), nodeStorage.clone(), s, &mut request) {
                    println!("handle data transfer error, err: {}", err);
                    result = 1;
                    break;
                }
            } else if request.requestMode == consts::proto::request_mode_ack {
                // handleAckTransfer
                if let Err(err) = CSimple::handleTransfer(&serverUuid, client.clone(), serverStorage.clone(), nodeStorage.clone(), s, &mut request) {
                    println!("handle ack transfer error, err: {}", err);
                    result = 1;
                    break;
                }
            }
            break;
        }
        let s = match stream.try_clone() {
            Ok(s) => s,
            Err(err) => {
                println!("stream clone error");
                // return false;
                return;
            }
        };
        if let Err(err) = CSimple::sendResponse(s, &mut response::CAck{
            serverUuid: serverUuid.to_string(),
            result: result
        }) {
            println!("send response error");
            // return false;
            return;
        };
        // return true;
    }

    fn handleConnect<'a>(storage: Arc<Mutex<NodeSharedStorage>>, stream: TcpStream, selfCommunicateUuid: &'a str, serverUuid: &'a str) -> Result<(), &'a str> {
        /*
        1. add socket info to shared
        */
        let storage = match storage.lock() {
            Ok(s) => s,
            Err(err) => {
                println!("storage lock error");
                return Err("storage lock error");
            }
        };
        let addr = tcp::stream2fd(stream);
        if let Err(err) = storage.addCommunicateNode(selfCommunicateUuid, &shared::node::CCommunicateNode{
            streamFd: addr,
            serverUuid: serverUuid.to_string()
        }) {
            println!("addCommunicateNode error to shared erorr, {}", err);
            return Err("addCommunicateNode to shared error");
        };
        Ok(())
    }

    fn handleTransfer<'a>(serverUuid: &'a str, client: Arc<Mutex<Client>>, serverStorage: Arc<Mutex<ServerSharedStorage>>, nodeStorage: Arc<Mutex<NodeSharedStorage>>, stream: TcpStream, request: &'a mut request::CRequest) -> Result<(), &'a str> {
        /*
        1. serverUuid is self
            yes -> find socket, then transfer
            no -> find server info, then transfer this server
        */
        let mut node = shared::node::CCommunicateNode::default();
        {
            let nodeStorage = match nodeStorage.lock() {
                Ok(s) => s,
                Err(err) => {
                    println!("node storage lock error, err: {}", err);
                    return Err("node storage lock error");
                }
            };
            node = match nodeStorage.communicateNode(&request.peerCommunicateUuid) {
                Some(node) => node,
                None => {
                    // response error
                    println!("peer is not exist, peerUuid: {}", &request.peerCommunicateUuid);
                    return Err("peer is not exist");
                }
            };
        }
        if node.serverUuid == serverUuid {
            /*
            let mut streamFd: u64 = 0;
            {
                let nodeStorage = match nodeStorage.lock() {
                    Ok(s) => s,
                    Err(err) => {
                        println!("node storage lock error, err: {}", err);
                        return Err("node storage lock error");
                    }
                };
                let node = match nodeStorage.communicateNode(&request.peerCommunicateUuid) {
                    Some(node) => node,
                    None => {
                        // response error
                        println!("peer is not exist");
                        return Err("peer is not exist");
                    }
                };
                streamFd = node.streamFd;
            }
            */
            let peerStream = tcp::fd2stream(node.streamFd);
            match CSimple::sendToPeer(peerStream.try_clone().unwrap(), request) {
                Ok(()) => {
                    mem::forget(peerStream);
                },
                Err(err) => {
                    println!("send to peer error, err: {}", err);
                }
            };
        } else {
            let mut serverInfo = shared::server::CServerInfo::default();
            {
                let serverStorage = match serverStorage.lock() {
                    Ok(s) => s,
                    Err(err) => {
                        println!("server storage lock error, err: {}", err);
                        return Err("server storage lock error");
                    }
                };
                let info = match serverStorage.server(&node.serverUuid) {
                    Some(info) => info,
                    None => {
                        println!("server is not exist");
                        return Err("server is not exist");
                    }
                };
                serverInfo = info;
            }
            ///*
            let mut cli = match client.lock() {
                Ok(c) => c,
                Err(err) => {
                    println!("client lock error: {}", err);
                    return Err("client lock error");
                }
            };
            match cli.findStream(&node.serverUuid) {
                Some(peerStream) => {
                    CSimple::sendToServer(peerStream.try_clone().expect("send to peer try clone error"), request);
                },
                None => {
                    mem::drop(cli);
                    if let Ok(peerStream) = Client::serverConnect(&serverInfo.net) {
                        println!("connect other server success");
                        let mut cli = match client.lock() {
                            Ok(c) => c,
                            Err(err) => {
                                println!("client lock error");
                                return Err("client lock error");
                            }
                        };
                        cli.addServer(&node.serverUuid, peerStream.try_clone().expect("send to peer try clone error"));
                        mem::drop(cli);
                        match CSimple::sendToServer(peerStream, request) {
                            Ok(_) => {
                                println!("send to other server success");
                            },
                            Err(err) => {
                                println!("send to other server error, err: {}", err);
                            }
                        };
                    } else {
                        println!("connect other server error");
                        return Err("connect server error");
                    }
                }
            };
            //*/
            /*
            let mut streamFd = 0;
            let mut isFind = true;
            {
                let client = match client.lock() {
                    Ok(c) => c,
                    Err(err) => {
                        println!("client lock error: {}", err);
                        return Err("client lock error");
                    }
                };
                if let Some(fd) = client.findStream(&node.serverUuid) {
                    isFind = true;
                    streamFd = fd;
                } else {
                    isFind = false;
                }
            }
            if isFind == false {
                println!("server client not found");
                if let Ok(fd) = Client::serverConnect(&serverInfo.net) {
                    let mut client = match client.lock() {
                        Ok(c) => c,
                        Err(err) => {
                            println!("client lock error: {}", err);
                            return Err("client lock error");
                        }
                    };
                    client.addServer(&node.serverUuid, fd);
                    streamFd = fd;
                } else {
                    println!("connect server error");
                    return Err("connect server error");
                }
            }
            println!("other server streamFd: {}", streamFd);
            let peerStream = tcp::fd2stream(streamFd);
            CSimple::sendToPeer(peerStream.try_clone().expect("send to peer try clone error"), request);
            mem::forget(peerStream);
            */
        }
        Ok(())
    }

    fn sendToPeer<'a>(stream: TcpStream, request: &mut request::CRequest) -> Result<(), &'a str> {
        let mut writer = BufWriter::new(&stream);
        let mut buf: Vec<u8> = Vec::new();
        if request.requestMode == consts::proto::response_mode_peer_ack {
            buf = encode::response::res::encodeAckTransfer(request);
        } else if request.requestMode == consts::proto::response_mode_data {
            buf = encode::response::res::encodeDataTransfer(request);
        } else {
            println!("request mode is not support, mode: {}", &request.requestMode);
            return Err("request mode is not support");
        }
        if let Err(err) = writer.write_all(&buf) {
            println!("write all error, err: {}", err);
            return Err("write all error");
        };
        if let Err(err) = writer.flush() {
            println!("flush error, err: {}", err);
            return Err("flush error");
        };
        Ok(())
    }

    fn sendToServer<'a>(stream: TcpStream, request: &mut request::CRequest) -> Result<(), &'a str> {
        let mut writer = BufWriter::new(&stream);
        let mut buf: Vec<u8> = Vec::new();
        if request.requestMode == consts::proto::response_mode_peer_ack {
            buf = encode::response::res::encodeOtherServerAckRequest(request);
        } else if request.requestMode == consts::proto::response_mode_data {
            buf = encode::response::res::encodeOtherServerDataRequest(request);
        } else {
            println!("request mode is not support, mode: {}", &request.requestMode);
            return Err("request mode is not support");
        }
        if let Err(err) = writer.write_all(&buf) {
            println!("write all error, err: {}", err);
            return Err("write all error");
        };
        if let Err(err) = writer.flush() {
            println!("flush error, err: {}", err);
            return Err("flush error");
        };
        Ok(())
    }

    fn sendResponse<'a>(stream: TcpStream, response: &mut response::CAck) -> Result<(), &'a str> {
        let mut writer = BufWriter::new(&stream);
        let buf = encode::response::res::encodeAck(response);
        if let Err(err) = writer.write_all(&buf) {
            return Err("write all error");
        };
        if let Err(err) = writer.flush() {
            return Err("flush error");
        };
        Ok(())
    }

    fn handleListen(&self, listener: TcpListener) -> Result<(), &str> {
        for stream in listener.incoming() {
            println!("new connect");
            let stream = match stream {
                Ok(s) => s,
                Err(err) => {
                    println!("listen connect error, err: {}", err);
                    return Err("listen connect error");
                }
            };
            let stream = match stream.try_clone() {
                Ok(s) => s,
                Err(err) => {
                    println!("stream clone error, err: {}", err);
                    return Err("stream clone error");
                }
            };
            if let Err(err) = self.handleClient(stream) {
                continue;
            }
        }
        Ok(())
    }

    fn portReuselisten(&self, port: u32) -> Result<(), &str> {
        // port reuse
        let tcpBuilder = match TcpBuilder::new_v4() {
            Ok(builder) => builder,
            Err(err) => {
                println!("create builder err: {}", err);
                return Err("create builder error");
            }
        };
        // reuse
        if let Err(err) = tcpBuilder.reuse_address(true) {
            println!("reuse address err: {}", err);
            return Err("reuse address error");
        };
        let mut addr = "0.0.0.0:".to_string();
        addr.push_str(&port.to_string());
        // bind
        if let Err(err) = tcpBuilder.bind(&addr) {
            println!("bind err: {}", err);
            return Err("bind error");
        };
        let listener = match tcpBuilder.listen(10) {
            Ok(li) => li,
            Err(err) => {
                println!("listen error: {}", err);
                return Err("listen error");
            }
        };
        if let Err(err) = self.handleListen(listener) {
            return Err(err);
        }
        Ok(())
    }
}

pub struct CServer {
    serverUuid: String,
    serverStorage: Arc<Mutex<ServerSharedStorage>>
}

impl CServer {
    pub fn new(serverStorageDial: &str) -> Result<CServer, &str> {
        let serverStorage = match ServerSharedStorage::new(serverStorageDial) {
            Ok(s) => s,
            Err(err) => {
                println!("serverStorage error, err: {}", err);
                return Err("server storage error");
            }
        };
        let mut serverUuid = uuid::Uuid::new_v4().to_string();
        Ok(CServer{
            serverUuid: serverUuid,
            serverStorage: Arc::new(Mutex::new(serverStorage))
        })
    }

    pub fn start(&self, param: &transfer::CCreateParam) -> Result<(), &str> {
        loop {
            let serverStorage = self.serverStorage.lock().unwrap();
            let server = match serverStorage.server(&self.serverUuid) {
                Some(s) => s,
                None => {
                    if let Err(err) = serverStorage.addServer(&self.serverUuid, &shared::server::CServerInfo{
                        net: shared::server::CNet{
                            ip: param.listenIp.to_string(),
                            port: param.listenPort
                        }
                    }) {
                        return Err("addServer error");
                    }
                    break;
                }
            };
            break;
        }
        // let mut addr = param.listenIp.to_string();
        let mut addr = "0.0.0.0".to_string();
        addr.push_str(":");
        addr.push_str(&param.listenPort.to_string());
        // bind
        let listener = match TcpListener::bind(&addr) {
            Ok(l) => l,
            Err(err) => {
                println!("bind err: {}", err);
                return Err("bind error");
            }
        };
        let node = match NodeSharedStorage::new(&param.nodeStorageDial) {
            Ok(n) => n,
            Err(err) => {
                println!("node shared storage new error, err: {}", err);
                return Err("node shared storage new error");
            }
        };
        let nodeStorage = Arc::new(Mutex::new(node));
        let client = Arc::new(Mutex::new(Client::new()));
        // let serverStorage = self.serverStorage.clone();
        for i in 0..param.threadMax {
            let nodeStorage = nodeStorage.clone();
            let serverStorage = self.serverStorage.clone();
            let client = client.clone();
            let listen = listener.try_clone().unwrap();
            let serverUuid = self.serverUuid.clone();
            thread::spawn(move || {
                let obj = CSimple{
                    serverUuid: serverUuid.clone(),
                    nodeStorage: nodeStorage,
                    serverStorage: serverStorage,
                    client: client
                };
                obj.handleListen(listen);
            });
        }
        Ok(())
    }
}

impl Drop for CServer {
    fn drop(&mut self) {
        let storage = match self.serverStorage.lock() {
            Ok(s) => {
                println!("drop ok");
                s
            },
            Err(err) => {
                println!("err: {}", err);
                return;
            }
        };
        storage.delServer(&self.serverUuid);
    }
}
