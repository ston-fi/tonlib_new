use crate::bc_constants::{TON_MASTERCHAIN_ID, TON_SHARD_FULL};
use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::cell::ton_hash::TonHash;
use crate::errors::TonlibError;
use crate::types::tlb::tlb_type::{TLBPrefix, TLBType};
use ton_lib_macros::TLBDerive;

// TLBType implementation is quite tricky, it doesn't keep shard as is
#[derive(Debug, Clone, PartialEq)]
pub struct ShardIdent {
    pub workchain: i32,
    pub shard: u64,
}

#[derive(Debug, Clone, PartialEq, TLBDerive)]
pub struct BlockIdExt {
    pub shard_id: ShardIdent,
    pub seqno: u32,
    pub root_hash: TonHash,
    pub file_hash: TonHash,
}

impl ShardIdent {
    pub fn new(workchain: i32, shard: u64) -> Self { Self { workchain, shard } }
    pub fn new_mc() -> Self { Self::new(TON_MASTERCHAIN_ID, TON_SHARD_FULL) }
}

impl TLBType for ShardIdent {
    const PREFIX: TLBPrefix = TLBPrefix {
        value: 0b00,
        bits_len: 2,
    };

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
