use crate::consts::proto;
use crate::structs::request;
use crate::structs::response;

use number_conv::array::u8arr;

pub fn encodeAck(res: &mut response::CAck) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    u8arr::u64AppendTou8arr(proto::response_mode_ack.len() as u64, 1, &mut buf);
    buf.append(&mut proto::response_mode_ack.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(res.serverUuid.len() as u64, 1, &mut buf);
    buf.append(&mut res.serverUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(1, 1, &mut buf);
    buf.append(&mut res.result.to_string().as_bytes().to_vec());
    u8arr::u64AppendTou8arr(0, 1, &mut buf);
    u8arr::u64AppendTou8arr(0, 1, &mut buf);
    u8arr::u64AppendTou8arr(0, 1, &mut buf);
    u8arr::u64AppendTou8arr(0, 4, &mut buf);
    u8arr::u64AppendTou8arr(0, 4, &mut buf);
    u8arr::u64AppendTou8arr(0, 4, &mut buf);
    u8arr::u64AppendTou8arr(0, 4, &mut buf);
    buf
}

pub fn encodeDataTransfer(req: &mut request::CRequest) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    u8arr::u64AppendTou8arr(proto::response_mode_data.len() as u64, 1, &mut buf);
    buf.append(&mut proto::response_mode_data.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.selfCommunicateUuid.len() as u64, 1, &mut buf);
    buf.append(&mut req.selfCommunicateUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(0, 1, &mut buf);
    u8arr::u64AppendTou8arr(req.peerCommunicateUuid.len() as u64, 1, &mut buf);
    buf.append(&mut req.peerCommunicateUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.objectUuid.len() as u64, 1, &mut buf);
    buf.append(&mut req.objectUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.peerResult.len() as u64, 4, &mut buf);
    buf.append(&mut req.peerResult.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.packageIndex, 4, &mut buf);
    buf.append(&mut req.packageIndex.to_string().as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.packageTotal, 4, &mut buf);
    buf.append(&mut req.packageTotal.to_string().as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.data.len() as u64, 4, &mut buf);
    buf.append(&mut req.data);
    buf
}

pub fn encodeAckTransfer(req: &mut request::CRequest) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    u8arr::u64AppendTou8arr(proto::response_mode_data.len() as u64, 1, &mut buf);
    buf.append(&mut proto::response_mode_data.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.selfCommunicateUuid.len() as u64, 1, &mut buf);
    buf.append(&mut req.selfCommunicateUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(0, 1, &mut buf);
    u8arr::u64AppendTou8arr(req.peerCommunicateUuid.len() as u64, 1, &mut buf);
    buf.append(&mut req.peerCommunicateUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.objectUuid.len() as u64, 1, &mut buf);
    buf.append(&mut req.objectUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.peerResult.len() as u64, 4, &mut buf);
    buf.append(&mut req.peerResult.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(0, 4, &mut buf);
    u8arr::u64AppendTou8arr(0, 4, &mut buf);
    u8arr::u64AppendTou8arr(0, 4, &mut buf);
    buf
}