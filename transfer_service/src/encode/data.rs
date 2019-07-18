use crate::structs::request;

use number_conv::array::u8arr;

pub fn encodeRequest2Data(req: &mut request::CRequest) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    u8arr::u64AppendTou8arr(req.peerCommunicateUuid.len() as u64, 1, &mut buf);
    buf.append(&mut req.peerCommunicateUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.selfCommunicateUuid.len() as u64, 1, &mut buf);
    buf.append(&mut req.selfCommunicateUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.packageIndex, 4, &mut buf);
    buf.append(&mut req.packageIndex.to_string().as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.packageTotal, 4, &mut buf);
    buf.append(&mut req.packageTotal.to_string().as_bytes().to_vec());
    u8arr::u64AppendTou8arr(req.data.len() as u64, 4, &mut buf);
    buf.append(&mut req.data);
    buf
}