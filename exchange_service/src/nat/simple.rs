use crate::shared;
use crate::nat;
use crate::enums;

pub struct CSimple {
}

impl nat::INat for CSimple {
    fn natType(&self, conn1: &shared::CSelf, conn2: &shared::CSelf) -> enums::nat::Nat {
        /*
            1. pri-port == pub-port => must be Nat 4
            2. pub-ip-1 == pub-ip-2 && pub-port-1 == pub-port-2 => not Nat 4
        */
        /*
        if conn1.lanNet.port == conn1.wanNet.port || conn2.lanNet.port == conn2.wanNet.port {
            return enums::nat::Nat::Nat4;
        }
        */
        if conn1.wanNet.ip == conn2.wanNet.ip && conn1.wanNet.port == conn2.wanNet.port {
            return enums::nat::Nat::NotNat4;
        }
        enums::nat::Nat::Nat4
    }
}
