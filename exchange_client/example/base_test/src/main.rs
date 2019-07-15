use exchange_client;
use exchange_client::make;
use exchange_client::structs;
use rust_parse::cmd::CCmd;

fn main() {
    let mut cmdHandler = CCmd::new();
    let ip = cmdHandler.register("-ip", "192.168.9.15");
    let port = cmdHandler.register("-port", "30001");
    let server1Ip = cmdHandler.register("-s1ip", "192.168.9.15");
    let server1Port = cmdHandler.register("-s1port", "20001");
    let server2Ip = cmdHandler.register("-s2ip", "192.168.9.15");
    let server2Port = cmdHandler.register("-s2port", "20002");
    let communicateUuid = cmdHandler.register("-uuid", "123456");
    cmdHandler.parse();

    let ip = ip.borrow();
    let port = port.borrow();
    let server1Ip = server1Ip.borrow();
    let server1Port = server1Port.borrow();
    let server2Ip = server2Ip.borrow();
    let server2Port = server2Port.borrow();
    let communicateUuid = communicateUuid.borrow();

    let m = exchange_client::make::ruled_out::CRuledOut::new();
    match m.make(&make::CMakeParam::new(structs::net::CNet{
        ip: ip.clone(),
        port: port.clone()
    }, structs::net::CNet{
        ip: server1Ip.clone(),
        port: server1Port.clone()
    }, structs::net::CNet{
        ip: server2Ip.clone(),
        port: server2Port.clone()
    }, communicateUuid.clone())) {
        Ok(_) => println!("ok"),
        Err(err) => println!("err: {}", err)
    }
}
