use crate::shared;
use crate::enums;

pub trait INat {
    fn natType(&self, conn1: &shared::CSelf, conn2: &shared::CSelf) -> enums::nat::Nat;
}

pub mod simple;
