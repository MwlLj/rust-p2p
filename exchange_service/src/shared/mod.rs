struct CNet {
	ip: String,
	port: u32
}

struct CSelf {
	lanNet: CNet,
	wanNet: CNet
}

struct CNode {
	lanNet: CNet,
	wanNet: CNet,
	natType: String
}

pub trait IShared {
	fn selfExist(&self, id: &str) -> Option<CSelf>;
	fn peerExist(&self, id: &str) -> Option<CNode>;
	fn addSelf(&self, id: &str, obj: CSelf, ttlMs: u32) -> Result<(), &str>;
	fn addPeer(&self, id: &str, obj: CNode, ttlMs: u32) -> Result<(), &str>;
	fn delNode(&self, id: &str) -> Result<(), &str>;
}

pub mod redis;