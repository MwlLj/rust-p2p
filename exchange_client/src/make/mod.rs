use crate::structs;

#[derive(Default)]
pub struct CMakeParam {
    selfNet: structs::net::CNet,
    server1Net: structs::net::CNet,
    server2Net: structs::net::CNet,
    communicateUuid: String
}

pub mod ruled_out;
