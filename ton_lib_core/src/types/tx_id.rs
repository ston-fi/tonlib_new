use crate::cell::TonHash;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TxIdLTHash {
    pub lt: i64,
    pub hash: TonHash,
}

impl TxIdLTHash {
    pub const ZERO: Self = Self::new(0, TonHash::ZERO);
    pub const fn new(lt: i64, hash: TonHash) -> Self { Self { lt, hash } }
}
