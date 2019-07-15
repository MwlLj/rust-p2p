use crate::shared;
use crate::enums;
use crate::structs;

use std::rc::Rc;

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

    pub fn peerChange(isMakeHole: bool, node1: &shared::CNode, peer1: &mut structs::req_res::CPeerNetResponse, node2: &shared::CNode, peer2: &mut structs::req_res::CPeerNetResponse) {
        /*
            1. node1 wanIp == node2 wanIp -> may be in the lan -> change theirs lan net info
            2. other -> change theirs wan net info
        */
        if node1.wanNet.ip.contains(&node2.wanNet.ip) {
            peer1.portInterval = 0;
            peer1.peerIp = node2.lanNet.ip.clone();
            peer1.peerPort = node2.lanNet.port.clone();

            peer2.portInterval = 0;
            peer2.peerIp = node1.lanNet.ip.clone();
            peer2.peerPort = node1.lanNet.port.clone();
        } else {
            let mut portInterval = node2.wanNet.port.parse::<i32>().unwrap() - node2.wanNet.port.parse::<i32>().unwrap();
            if portInterval < 0 {
                portInterval = -1 * portInterval;
            }
            peer1.portInterval = portInterval;
            peer1.peerIp = node2.wanNet.ip.clone();
            peer1.peerPort = node2.wanNet.port.clone();

            peer2.portInterval = portInterval;
            peer2.peerIp = node1.wanNet.ip.clone();
            peer2.peerPort = node1.wanNet.port.clone();
        }
    }
}

impl CPeer {
    pub fn new() -> CPeer {
        CPeer{}
    }
}
