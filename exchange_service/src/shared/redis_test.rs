use crate::shared;
use crate::decode;
use super::redis;

const redis_addr: &str = "redis://localhost:6379";

fn redisConnect(addr: &str) -> Option<Box<dyn shared::IShared>> {
    let red = match redis::CRedis::new(addr) {
        Ok(r) => r,
        Err(err) => {
            println!("{:?}", err);
            assert!(false);
            return None;
        }
    };
    let red: Box<dyn shared::IShared> = Box::new(red);
    Some(red)
}

#[test]
#[ignore]
fn addSelfTest() {
    if let Some(red) = redisConnect(redis_addr) {
        if let Err(err) = red.addSelf("self1", shared::CSelf{
            lanNet: shared::CNet{
                ip: "192.168.9.15".to_string(),
                port: "30000".to_string()
            },
            wanNet: shared::CNet{
                ip: "180.167.210.2".to_string(),
                port: "12345".to_string()
            }
        }, 100000) {
            assert!(false);
        }
    } else {
        assert!(false);
    }
}

#[test]
// #[ignore]
fn selfExistTest() {
    if let Some(red) = redisConnect(redis_addr) {
        if let Some(s) = red.selfExist("self1") {
            println!("{:?}", &s);
        } else {
            assert!(false);
        }
    } else {
        assert!(false);
    }
}
