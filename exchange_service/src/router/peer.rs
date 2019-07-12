use crate::shared;
use crate::enums;

pub struct CPeer {
}

impl CPeer {
    /*
        return: can make hole communicate
    */
    pub fn peerCheck(peer1: &shared::CNode, peer2: &shared::CNode) -> bool {
        /*
            1. peer1 natType == Nat4 && peer2 natType == Nat4 -> false
            2. (peer1 natType == Nat3 && peer2 natType == Nat4) || (peer1 natType == Nat4 && peer2 natType == Nat3) -> false
        */
        if (peer1.natType == enums::nat::Nat::Nat4 && peer2.natType == enums::nat::Nat::Nat4)
            || ((peer1.natType == enums::nat::Nat::Nat3 && peer2.natType == enums::nat::Nat::Nat4)
                || (peer1.natType == enums::nat::Nat::Nat4 && peer2.natType == enums::nat::Nat::Nat3)) {
            return false;
        }
        true
    }
}

impl CPeer {
    pub fn new() -> CPeer {
        CPeer{}
    }
}
