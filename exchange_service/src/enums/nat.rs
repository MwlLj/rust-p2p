use std::cmp::PartialEq;

#[derive(PartialEq, Debug, Clone)]
pub enum Nat {
    Nat1 = 1,
    Nat2 = 2,
    Nat3 = 3,
    Nat4 = 4,
    NotNat4 = 5
}

impl std::convert::From<u8> for Nat {
    fn from(item: u8) -> Self {
        match item {
            1 => Nat::Nat1,
            2 => Nat::Nat2,
            3 => Nat::Nat3,
            4 => Nat::Nat4,
            5 => Nat::NotNat4,
            _ => Nat::Nat1
        }
    }
}

// impl PartialEq for Nat {
//     fn eq(&self, other: &Nat) -> bool {
//         *self as u8 == *other as u8
//     }
// }
