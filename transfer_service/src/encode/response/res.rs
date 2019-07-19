use crate::structs::response;

use number_conv::array::u8arr;

pub fn encodeResponse(res: &mut response::CResponse) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    u8arr::u64AppendTou8arr(res.serverUuid.len() as u64, 1, &mut buf);
    buf.append(&mut res.serverUuid.as_bytes().to_vec());
    u8arr::u64AppendTou8arr(res.result as u64, 1, &mut buf);
    buf.append(&mut res.result.to_string().as_bytes().to_vec());
    buf
}