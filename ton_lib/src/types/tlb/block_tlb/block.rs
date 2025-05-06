use crate::bc_constants::{TON_MASTERCHAIN_ID, TON_SHARD_FULL};
use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::cell::ton_cell::TonCellRef;
use crate::cell::ton_hash::TonHash;
use crate::errors::TonlibError;
use crate::types::tlb::adapters::TLBRef;
use crate::types::tlb::tlb_type::{TLBPrefix, TLBType};
use ton_lib_macros::TLBDerive;

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0x11ef55aa, bits_len = 32)]
pub struct Block {
    pub global_id: i32,
    #[tlb_derive(adapter = "TLBRef")]
    pub info: BlockInfo,
    pub value_flow: TonCellRef,   // TODO
    pub state_update: TonCellRef, // TODO
    #[tlb_derive(adapter = "TLBRef")]
    pub extra: BlockExtra,
}

const _GEN_SOFTWARE_EXISTS_FLAG: u8 = 1;

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0x9bc7a987, bits_len = 32)]
pub struct BlockInfo {
    pub version: u32,
    pub not_master: bool,
    pub after_merge: bool,
    pub before_split: bool,
    pub after_split: bool,
    pub want_split: bool,
    pub want_merge: bool,
    pub key_block: bool,
    pub vert_seqno_incr: u32,
    pub flags: u8,
    pub seqno: u32,
    pub vert_seqno: u32,
    pub shard: ShardIdent,
    pub gen_utime: u32,
    pub start_lt: u64,
    pub end_lt: u64,
    pub gen_validator_list_has_short: u32,
    pub get_catchain_seqno: u32,
    pub min_ref_mc_seqno: u32,
    pub prev_key_block_seqno: u32,
    // pub gen_software // TODO
}

#[derive(Debug, Clone, TLBDerive)]
pub struct BlockExtra {
    pub in_msg_descr: TonCellRef,
    pub out_msg_descr: TonCellRef,
    pub account_blocks: TonCellRef,
    pub rand_seed: TonHash,
    pub created_by: TonHash,
    #[tlb_derive(adapter = "TLBRef")]
    pub mc_block_extra: Option<MCBlockExtra>,
}

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0xcca5, bits_len = 8)]
pub struct MCBlockExtra {
    pub key_block: bool,
    // pub shard_hashes: ShardHashes // TODO
    // pub shard_fees: ShardFees, // TODO
    pub prev_block_sign: TonCellRef,
    // pub config: ConfigParams, // TODO
}

// TLBType implementation is quite tricky, it doesn't keep shard as is
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct ShardIdent {
    pub workchain: i32,
    pub shard: u64,
}

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

impl ShardIdent {
    pub fn new(workchain: i32, shard: u64) -> Self { Self { workchain, shard } }
    pub fn new_mc() -> Self { Self::new(TON_MASTERCHAIN_ID, TON_SHARD_FULL) }
}

impl TLBType for ShardIdent {
    const PREFIX: TLBPrefix = TLBPrefix::new(0b00, 2);

    fn read_definition(parser: &mut CellParser) -> Result<Self, TonlibError> {
        let prefix_len: u32 = parser.read_num(6)?;
        let workchain: i32 = parser.read_num(32)?;
        let shard_prefix: u64 = parser.read_num(64)?;
        let shard = (!shard_prefix).wrapping_add(1) | 1 << (64 - prefix_len);
        Ok(Self { workchain, shard })
    }

    fn write_definition(&self, builder: &mut CellBuilder) -> Result<(), TonlibError> {
        let prefix_len = match self.shard {
            0 => 64,
            prefix => 63 - prefix.trailing_zeros() as u8,
        };
        let prefix = self.shard - (!self.shard).wrapping_add(1);
        builder.write_num(&prefix_len, 6)?;
        self.workchain.write(builder)?;
        builder.write_num(&prefix, 64)?;
        Ok(())
    }
}
