use crate::shared;
use crate::decode;
use crate::encode;

use redis;
use redis::Client;

const group_name: &str = "server:";

pub struct CRedis {
    client: Client
}

impl CRedis {
    pub fn addServer(&self, id: &str, server: &shared::server::CServerInfo) -> Result<(), &str> {
        let mut conn = match self.client.get_connection() {
            Ok(conn) => conn,
            Err(_) => return Err("get connect error")
        };
        let serverEncode = encode::shared::server::encodeServerInfo(&server);
        // let ttl = ttlMs / 1000;
        // .arg("ex").arg(&ttl.to_string());
        if let Ok(()) = redis::cmd("set").arg(&self.joinKey(id)).arg(&serverEncode).query(&mut conn) {
            Ok(())
        } else {
            Err("set error")
        }
    }

    pub fn delServer(&self, id: &str) -> Result<(), &str> {
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

    pub fn server(&self, id: &str) -> Option<shared::server::CServerInfo> {
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
            let server = match decode::shared::server::decodeServerInfo(v.as_bytes()) {
                Ok(server) => server,
                Err(_) => return None
            };
            Some(server)
        } else {
            None
        }
    }

    pub fn servers(&self) -> Option<Vec<String>> {
        let mut conn = match self.client.get_connection() {
            Ok(conn) => conn,
            Err(_) => return None
        };
        if let Ok(value) = redis::cmd("keys").arg(group_name).query(&mut conn) {
            value
        } else {
            None
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
