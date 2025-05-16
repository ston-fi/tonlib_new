use crate::cell::ton_hash::TonHash;
use crate::clients::client_types::{BlockId, MasterchainInfo, ZeroStateIdExt};
use crate::types::tlb::block_tlb::block::{BlockIdExt, ShardIdent};
use ton_liteapi::tl::common::Int256;

impl From<Int256> for TonHash {
    fn from(value: Int256) -> Self { TonHash::from_slice(&value.0) }
}

impl From<TonHash> for Int256 {
    fn from(value: TonHash) -> Self { Int256(*value.as_slice_sized()) }
}

impl From<ton_liteapi::tl::common::BlockId> for BlockId {
    fn from(value: ton_liteapi::tl::common::BlockId) -> Self {
        BlockId {
            workchain: value.workchain,
            shard: value.shard,
            seqno: value.seqno,
        }
    }
}

impl From<BlockId> for ton_liteapi::tl::common::BlockId {
    fn from(value: BlockId) -> Self {
        ton_liteapi::tl::common::BlockId {
            workchain: value.workchain,
            shard: value.shard,
            seqno: value.seqno,
        }
    }
}

impl From<ton_liteapi::tl::common::BlockIdExt> for BlockIdExt {
    fn from(value: ton_liteapi::tl::common::BlockIdExt) -> Self {
        BlockIdExt {
            shard_id: ShardIdent::new(value.workchain, value.shard),
            seqno: value.seqno,
            root_hash: value.root_hash.into(),
            file_hash: value.file_hash.into(),
        }
    }
}

impl From<BlockIdExt> for ton_liteapi::tl::common::BlockIdExt {
    fn from(value: BlockIdExt) -> Self {
        ton_liteapi::tl::common::BlockIdExt {
            workchain: value.shard_id.workchain,
            shard: value.shard_id.shard,
            seqno: value.seqno,
            root_hash: value.root_hash.into(),
            file_hash: value.file_hash.into(),
        }
    }
}

impl From<ton_liteapi::tl::common::ZeroStateIdExt> for ZeroStateIdExt {
    fn from(value: ton_liteapi::tl::common::ZeroStateIdExt) -> Self {
        ZeroStateIdExt {
            workchain: value.workchain,
            root_hash: value.root_hash.into(),
            file_hash: value.file_hash.into(),
        }
    }
}

impl From<ZeroStateIdExt> for ton_liteapi::tl::common::ZeroStateIdExt {
    fn from(value: ZeroStateIdExt) -> Self {
        ton_liteapi::tl::common::ZeroStateIdExt {
            workchain: value.workchain,
            root_hash: value.root_hash.into(),
            file_hash: value.file_hash.into(),
        }
    }
}

impl From<ton_liteapi::tl::response::MasterchainInfo> for MasterchainInfo {
    fn from(value: ton_liteapi::tl::response::MasterchainInfo) -> Self {
        MasterchainInfo {
            last: value.last.into(),
            state_root_hash: value.state_root_hash.into(),
            init: value.init.into(),
        }
    }
}

impl From<MasterchainInfo> for ton_liteapi::tl::response::MasterchainInfo {
    fn from(value: MasterchainInfo) -> Self {
        ton_liteapi::tl::response::MasterchainInfo {
            last: value.last.into(),
            state_root_hash: value.state_root_hash.into(),
            init: value.init.into(),
        }
    }
}
