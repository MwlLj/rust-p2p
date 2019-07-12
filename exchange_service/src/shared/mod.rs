use crate::enums;

pub struct CNet {
	pub ip: String,
	pub port: String
}

pub struct CSelf {
	pub lanNet: CNet,
	pub wanNet: CNet
}

pub struct CNode {
	pub lanNet: CNet,
	pub wanNet: CNet,
	pub natType: enums::nat::Nat
}

pub trait IShared {
	fn selfExist(&self, id: &str) -> Option<CSelf>;
	fn peerExist(&self, id: &str) -> Option<CNode>;
	fn addSelf(&self, id: &str, obj: CSelf, ttlMs: u32) -> Result<(), &str>;
	fn addPeer(&self, id: &str, obj: CNode, ttlMs: u32) -> Result<(), &str>;
	fn delNode(&self, id: &str) -> Result<(), &str>;
}

pub mod redis;