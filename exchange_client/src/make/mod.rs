use crate::structs;

#[derive(Default)]
pub struct CMakeParam {
    selfNet: structs::net::CNet,
    server1Net: structs::net::CNet,
    server2Net: structs::net::CNet,
    communicateUuid: String
}

impl CMakeParam {
    pub fn new(selfNet: structs::net::CNet, server1Net: structs::net::CNet, server2Net: structs::net::CNet, communicateUuid: String) -> CMakeParam {
        CMakeParam{
            selfNet: selfNet,
            server1Net: server1Net,
            server2Net: server2Net,
            communicateUuid: communicateUuid
        }
    }
}

pub mod ruled_out;
