use exchange_service::shared::IShared;
use exchange_service::shared::redis::CRedis;
use exchange_service::change;
use exchange_service::change::ruled_out;
use exchange_service::decode;
use exchange_service::structs;
use exchange_service::consts;

use rust_parse::cmd::CCmd;

use std::thread;
use std::time;

fn main() {
    let mut cmdHandler = CCmd::new();
    let redisDial = cmdHandler.register("-dial", "127.0.0.1:6379");
    let port = cmdHandler.register("-port", "20001");
    cmdHandler.parse();

    let redisDial = redisDial.borrow();
    let port = port.borrow();

    let port = match port.parse::<u32>() {
        Ok(port) => port,
        Err(err) => {
            println!("port parse error: {}", err);
            return;
        }
    };

    ruled_out::CRuledOut::new(&change::CCreateParam{
        sharedMode: &consts::run::storage_mode_redis,
        transmitServiceFindMode: &consts::run::transmit_service_find_mode_config,
        dial: &redisDial,
        port: port,
        nat4IsTryMake: false,
        threadMax: 10
    });

    loop {
        thread::sleep(time::Duration::from_secs(10));
    }
}
