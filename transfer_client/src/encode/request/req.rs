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
    u8arr::u64AppendTou8arr(0, 4, &mut buf);
    u8arr::u64AppendTou8arr(0, 4, &mut buf);
    u8arr::u64AppendTou8arr(0, 4, &mut buf);
    u8arr::u64AppendTou8arr(0, 4, &mut buf);
    println!("{:?}", &buf);
    buf
}

pub fn encodeData(req: &mut request::CData) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    u8arr::u64AppendTou8arr(proto::request_mode_data.len() as u64, 1, &mut buf);
    buf.append(&mut proto::request_mode_data.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.selfCommunicateUuid.len() as u64, 1, &mut buf);
    buf.append(&mut req.peerCommunicateUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.peerCommunicateUuid.len() as u64, 1, &mut buf);
    buf.append(&mut req.selfCommunicateUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.serverUuid.len() as u64, 1, &mut buf);
    buf.append(&mut req.serverUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.objectUuid.len() as u64, 1, &mut buf);
    buf.append(&mut req.objectUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(0, 4, &mut buf);
    u8arr::u64AppendTou8arr(req.packageIndex, 4, &mut buf);
    if req.packageIndex > 0 {
        buf.append(&mut req.packageIndex.to_string().as_bytes().to_vec());
    }
    u8arr::u64AppendTou8arr(req.packageTotal, 4, &mut buf);
    if req.packageTotal > 0 {
        buf.append(&mut req.packageTotal.to_string().as_bytes().to_vec());
    }
    u8arr::u64AppendTou8arr(req.data.len() as u64, 4, &mut buf);
    buf.append(&mut req.data);
    println!("{:?}", &buf);
    buf
}

pub fn encodeAck(req: &mut request::CAck) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    u8arr::u64AppendTou8arr(proto::request_mode_ack.len() as u64, 1, &mut buf);
    buf.append(&mut proto::request_mode_ack.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.selfCommunicateUuid.len() as u64, 1, &mut buf);
    buf.append(&mut req.peerCommunicateUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.peerCommunicateUuid.len() as u64, 1, &mut buf);
    buf.append(&mut req.selfCommunicateUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.serverUuid.len() as u64, 1, &mut buf);
    buf.append(&mut req.serverUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.objectUuid.len() as u64, 1, &mut buf);
    buf.append(&mut req.objectUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.result.len() as u64, 4, &mut buf);
    buf.append(&mut req.result.to_string().as_bytes().to_vec());
    u8arr::u64AppendTou8arr(0, 4, &mut buf);
    u8arr::u64AppendTou8arr(0, 4, &mut buf);
    u8arr::u64AppendTou8arr(0, 4, &mut buf);
    buf
}