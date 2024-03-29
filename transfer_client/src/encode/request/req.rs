use crate::consts::proto;
use crate::structs::request;
use crate::structs::response;

use number_conv::array::u8arr;

pub fn encodeConnect(req: &mut request::CConnect) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    u8arr::u64AppendTou8arr(proto::request_mode_connect.len() as u64, 1, &mut buf);
    buf.append(&mut proto::request_mode_connect.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.selfCommunicateUuid.len() as u64, 1, &mut buf);
    buf.append(&mut req.selfCommunicateUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(0, 1, &mut buf);
    u8arr::u64AppendTou8arr(0, 1, &mut buf);
    u8arr::u64AppendTou8arr(0, 1, &mut buf);
    u8arr::u64AppendTou8arr(0, 1, &mut buf);
    u8arr::u64AppendTou8arr(0, 4, &mut buf);
    u8arr::u64AppendTou8arr(0, 1, &mut buf);
    u8arr::u64AppendTou8arr(0, 1, &mut buf);
    u8arr::u64AppendTou8arr(0, 4, &mut buf);
    u8arr::u64AppendTou8arr(0, 4, &mut buf);
    buf
}

pub fn encodeData(req: &mut request::CData) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    u8arr::u64AppendTou8arr(proto::request_mode_data.len() as u64, 1, &mut buf);
    buf.append(&mut proto::request_mode_data.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.selfCommunicateUuid.len() as u64, 1, &mut buf);
    buf.append(&mut req.selfCommunicateUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.peerCommunicateUuid.len() as u64, 1, &mut buf);
    buf.append(&mut req.peerCommunicateUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.serverUuid.len() as u64, 1, &mut buf);
    buf.append(&mut req.serverUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.dataUuid.len() as u64, 1, &mut buf);
    buf.append(&mut req.dataUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.objectUuid.len() as u64, 1, &mut buf);
    buf.append(&mut req.objectUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(0, 4, &mut buf);
    u8arr::u64NumberAppendTou8arr(req.u64Field1, &mut buf);
    u8arr::u64NumberAppendTou8arr(req.u64Field2, &mut buf);
    // println!("{:?}", &buf);
    u8arr::u64AppendTou8arr(req.data.len() as u64, 4, &mut buf);
    buf.append(&mut req.data);
    u8arr::u64AppendTou8arr(req.extraData.len() as u64, 4, &mut buf);
    buf.append(&mut req.extraData);
    buf
}

/*
    encode send to peer's ack
*/
pub fn encodeAck(req: &mut request::CAck) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    u8arr::u64AppendTou8arr(proto::request_mode_peer_ack.len() as u64, 1, &mut buf);
    buf.append(&mut proto::request_mode_peer_ack.as_bytes().to_vec());
    /********** be careful!!! **********/
    // peer ack, selfUuid and peerUuid change
    u8arr::u64AppendTou8arr(req.peerCommunicateUuid.len() as u64, 1, &mut buf);
    buf.append(&mut req.peerCommunicateUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.selfCommunicateUuid.len() as u64, 1, &mut buf);
    buf.append(&mut req.selfCommunicateUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.serverUuid.len() as u64, 1, &mut buf);
    buf.append(&mut req.serverUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.dataUuid.len() as u64, 1, &mut buf);
    buf.append(&mut req.dataUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.objectUuid.len() as u64, 1, &mut buf);
    buf.append(&mut req.objectUuid.as_bytes().to_vec());
    // u8arr::u64AppendTou8arr(0, 1, &mut buf);
    u8arr::u64AppendTou8arr(req.result.len() as u64, 4, &mut buf);
    buf.append(&mut req.result.as_bytes().to_vec());
    u8arr::u64NumberAppendTou8arr(req.u64Field1, &mut buf);
    u8arr::u64NumberAppendTou8arr(req.u64Field2, &mut buf);
    // println!("{:?}", &buf);
    u8arr::u64AppendTou8arr(req.data.len() as u64, 4, &mut buf);
    buf.append(&mut req.data);
    u8arr::u64AppendTou8arr(req.extraData.len() as u64, 4, &mut buf);
    buf.append(&mut req.extraData);
    // u8arr::u64AppendTou8arr(0, 1, &mut buf);
    // u8arr::u64AppendTou8arr(0, 1, &mut buf);
    // u8arr::u64AppendTou8arr(0, 4, &mut buf);
    // u8arr::u64AppendTou8arr(0, 4, &mut buf);
    // println!("{:?}", &buf);
    buf
}