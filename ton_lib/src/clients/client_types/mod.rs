mod liteapi_serde;

use serde::{Deserialize, Serialize};
use crate::cell::ton_hash::TonHash;
use crate::types::tlb::block_tlb::block::BlockIdExt;
use crate::types::ton_address::TonAddress;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockId {
    pub workchain: i32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub shard: u64,
    pub seqno: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ZeroStateIdExt {
    pub workchain: i32,
    pub root_hash: TonHash,
    pub file_hash: TonHash,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MasterchainInfo {
    pub last: BlockIdExt,
    pub state_root_hash: TonHash,
    pub init: ZeroStateIdExt,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TxId {
    LTHash { lt: i64, hash: TonHash},
    LTAddress{ lt: i64, address: TonAddress},
    ExtInMsgHash { hash: TonHash },
    ExtInMsgHashNorm { hash: TonHash },
}

impl TxId {
    pub const ZERO: Self = Self { lt: 0, hash: TonHash::ZERO };
}
