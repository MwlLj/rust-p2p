use crate::shared;
use crate::decode;
use crate::encode;

use redis;
use redis::Client;

const group_name: &str = "node:";

pub struct CRedis {
    client: Client
}

impl CRedis {
    pub fn communicateNode(&self, id: &str) -> Option<shared::node::CCommunicateNode> {
        let mut conn = match self.client.get_connection() {
            Ok(conn) => conn,
            Err(_) => return None
        };
        if let Ok(value) = redis::cmd("get").arg(&self.joinKey(id)).query(&mut conn) {
            let va: Option<String> = value;
            let v = match va {
                Some(v) => v,
                None => return None,
            };
            let node = match decode::shared::node::decodeNodeSelf(v.as_bytes()) {
                Ok(node) => node,
                Err(_) => return None
            };
            Some(node)
        } else {
            None
        }
    }

    pub fn addCommunicateNode(&self, id: &str, node: &shared::node::CCommunicateNode) -> Result<(), &str> {
        let mut conn = match self.client.get_connection() {
            Ok(conn) => conn,
            Err(_) => return Err("get connect error")
        };
        let nodeEncode = encode::shared::node::encodeNodeSelf(&node);
        // let ttl = ttlMs / 1000;
        // .arg("ex").arg(&ttl.to_string());
        if let Ok(()) = redis::cmd("set").arg(&self.joinKey(id)).arg(&nodeEncode).query(&mut conn) {
            Ok(())
        } else {
            Err("set error")
        }
    }

    pub fn delNode(&self, id: &str) -> Result<(), &str> {
        let mut conn = match self.client.get_connection() {
            Ok(conn) => conn,
            Err(_) => return Err("get connect error")
        };
        if let Ok(()) = redis::cmd("del").arg(&self.joinKey(id)).query(&mut conn) {
            Ok(())
        } else {
            Err("del error")
        }
    }
}

impl CRedis {
    fn joinKey(&self, id: &str) -> String {
        let mut key = String::new();
        key.push_str(group_name);
        key.push_str(id);
        key
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
