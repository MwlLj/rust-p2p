use std::cmp::PartialEq;

#[derive(PartialEq)]
pub enum Nat {
    Nat1 = 1,
    Nat2 = 2,
    Nat3 = 3,
    Nat4 = 4,
    NotNat4 = 5
}

// impl PartialEq for Nat {
//     fn eq(&self, other: &Nat) -> bool {
//         *self as u8 == *other as u8
//     }
// }
