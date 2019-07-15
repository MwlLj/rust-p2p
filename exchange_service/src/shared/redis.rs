use crate::shared;
use crate::decode;
use crate::encode;

use redis;
use redis::Client;

pub struct CRedis {
    client: Client
}

impl shared::IShared for CRedis {
    fn selfExist(&self, id: &str) -> Option<shared::CSelf> {
        let mut conn = match self.client.get_connection() {
            Ok(conn) => conn,
            Err(_) => return None
        };
        if let Ok(value) = redis::cmd("get").arg(id).query(&mut conn) {
            let va: Option<String> = value;
            let v = match va {
                Some(v) => v,
                None => return None,
            };
            let node = match decode::shared::decodeSelf(v.as_bytes()) {
                Ok(node) => node,
                Err(_) => return None
            };
            Some(node)
        } else {
            None
        }
    }

    fn peerExist(&self, id: &str) -> Option<shared::CNode> {
        let mut conn = match self.client.get_connection() {
            Ok(conn) => conn,
            Err(_) => return None
        };
        if let Ok(value) = redis::cmd("get").arg(id).query(&mut conn) {
            let va: Option<String> = value;
            let v = match va {
                Some(v) => v,
                None => return None,
            };
            let node = match decode::shared::decodeNode(v.as_bytes()) {
                Ok(node) => node,
                Err(_) => return None
            };
            Some(node)
        } else {
            None
        }
    }

    fn addSelf(&self, id: &str, obj: shared::CSelf, ttlMs: u32) -> Result<(), &str> {
        let mut conn = match self.client.get_connection() {
            Ok(conn) => conn,
            Err(_) => return Err("get connect error")
        };
        let selfEncode = encode::shared::encodeSelf(&obj);
        let ttl = ttlMs / 1000;
        if let Ok(()) = redis::cmd("set").arg(id).arg(&selfEncode).arg("ex").arg(&ttl.to_string()).query(&mut conn) {
            Ok(())
        } else {
            Err("set error")
        }
    }

    fn addPeer(&self, id: &str, obj: shared::CNode, ttlMs: u32) -> Result<(), &str> {
        let mut conn = match self.client.get_connection() {
            Ok(conn) => conn,
            Err(_) => return Err("get connect error")
        };
        let nodeEncode = encode::shared::encodeNode(&obj);
        let ttl = ttlMs / 1000;
        if let Ok(()) = redis::cmd("set").arg(id).arg(&nodeEncode).arg("ex").arg(&ttl.to_string()).query(&mut conn) {
            Ok(())
        } else {
            Err("set error")
        }
    }

    fn delNode(&self, id: &str) -> Result<(), &str> {
        let mut conn = match self.client.get_connection() {
            Ok(conn) => conn,
            Err(_) => return Err("get connect error")
        };
        if let Ok(()) = redis::cmd("del").arg(id).query(&mut conn) {
            Ok(())
        } else {
            Err("del error")
        }
    }
}

impl CRedis {
    pub fn new(addr: &str) -> Result<CRedis, &str> {
        let mut connAddr = String::from("redis://");
        connAddr.push_str(addr);
        let client = match Client::open(connAddr.as_str()) {
            Ok(client) => client,
            Err(err) => {
                println!("err: {}", err);
                return Err("dial redis error");
            }
        };
        Ok(CRedis{
            client: client
        })
    }
}
