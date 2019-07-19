use transfer_service::transfer;

use rust_parse::cmd::CCmd;

use std::thread;
use std::time;

fn main() {
    let mut cmdHandler = CCmd::new();
    let redisDial = cmdHandler.register("-dial", "127.0.0.1:6379");
    let port = cmdHandler.register("-port", "20001");
    let threadMax = cmdHandler.register("-thread-max", "10");
    cmdHandler.parse();

    let redisDial = redisDial.borrow();
    let port = port.borrow();
    let threadMax = threadMax.borrow();

    let port = match port.parse::<u32>() {
        Ok(port) => port,
        Err(err) => {
            println!("port parse error: {}", err);
            return;
        }
    };
    let threadMax = match threadMax.parse::<u32>() {
        Ok(threadMax) => threadMax,
        Err(err) => {
            println!("thread max error: {}", err);
            return;
        }
    };

    let server = transfer::tcp::simple::CServer::new("localhost:6379").unwrap();
    server.start(&transfer::CCreateParam{
        nodeStorageDial: "localhost:6379".to_string(),
        listenIp: "127.0.0.1".to_string(),
        listenPort: port,
        threadMax: threadMax
    });

    loop {
        thread::sleep(time::Duration::from_secs(10));
    }
}
