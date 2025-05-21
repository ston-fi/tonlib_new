use crate::bc_constants::{TON_MASTERCHAIN_ID, TON_SHARD_FULL};
use crate::cell::ton_hash::TonHash;
use crate::types::tlb::block_tlb::block::shard_ident::ShardIdent;
use ton_lib_macros::TLBDerive;

#[derive(Debug, Clone, PartialEq, Eq, Hash, TLBDerive)]
pub struct BlockIdExt {
    pub shard_id: ShardIdent,
    pub seqno: u32,
    pub root_hash: TonHash,
    pub file_hash: TonHash,
}

impl BlockIdExt {
    pub const ZERO_BLOCK_ID: BlockIdExt = BlockIdExt {
        shard_id: ShardIdent {
            workchain: TON_MASTERCHAIN_ID,
            shard: TON_SHARD_FULL,
        },
        seqno: 0,
        root_hash: TonHash::from_slice(&[
            23u8, 163, 169, 41, 146, 170, 190, 167, 133, 167, 160, 144, 152, 90, 38, 92, 211, 31, 50, 61, 132, 157,
            165, 18, 57, 115, 126, 50, 31, 176, 85, 105,
        ]),
        file_hash: TonHash::from_slice(&[
            94, 153, 79, 207, 77, 66, 92, 10, 108, 230, 167, 146, 89, 75, 113, 115, 32, 95, 116, 10, 57, 205, 86, 245,
            55, 222, 253, 40, 180, 138, 15, 110,
        ]),
    };
}
