use crate::bc_constants::{TON_MASTERCHAIN_ID, TON_SHARD_FULL};
use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::errors::TonlibError;
use crate::types::tlb::tlb_type::{TLBPrefix, TLBType};

// TLBType implementation is quite tricky, it doesn't keep shard as is
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct ShardIdent {
    pub workchain: i32,
    pub shard: u64,
}

impl ShardIdent {
    pub fn new(workchain: i32, shard: u64) -> Self { Self { workchain, shard } }
    pub fn new_mc() -> Self { Self::new(TON_MASTERCHAIN_ID, TON_SHARD_FULL) }
}

impl TLBType for ShardIdent {
    const PREFIX: TLBPrefix = TLBPrefix::new(0b00, 2);

    fn read_definition(parser: &mut CellParser) -> Result<Self, TonlibError> {
        let prefix_len_bits: u32 = parser.read_num(6)?;
        if prefix_len_bits > 60 {
            return Err(TonlibError::TLBWrongData(format!("expecting prefix_len <= 60, got {prefix_len_bits}")));
        }
        let workchain = parser.read_num(32)?;
        let shard_prefix: u64 = parser.read_num(64)?;
        let tag = 1u64 << (63 - prefix_len_bits);
        let shard = (shard_prefix & (!tag).wrapping_add(1)) | tag;
        Ok(Self { workchain, shard })
    }

    fn write_definition(&self, builder: &mut CellBuilder) -> Result<(), TonlibError> {
        if self.shard == 0 {
            return Err(TonlibError::TLBWrongData("shard can't be 0".to_string()));
        }
        let prefix_trailing_zeros = self.shard.trailing_zeros();
        let prefix_len = 63u32 - prefix_trailing_zeros;
        builder.write_num(&prefix_len, 6)?;
        self.workchain.write(builder)?;
        let prefix = self.shard - (self.shard & (!self.shard).wrapping_add(1));
        builder.write_num(&prefix, 64)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bc_constants::{TON_MASTERCHAIN_ID, TON_SHARD_FULL};
    use crate::cell::ton_cell::TonCell;
    use crate::types::tlb::block_tlb::block::ShardIdent;

    #[test]
    fn test_block_tlb_shard_ident_master() -> anyhow::Result<()> {
        // master_block: shard:(shard_ident shard_pfx_bits:0 workchain_id:-1 shard_prefix:0)
        // https://explorer.toncoin.org/viewblock?workchain=-1&shard=8000000000000000&seqno=46991999&roothash=CBEBAA6AC4270C987C90C5ED930FF37F9B73C705999585D6D8C1C5E9FA3DD6E3&filehash=A660FB144506617B0C0132B92CF41E436A6A87E924FD97C769C5D6E36320327B
        let mut builder = TonCell::builder();
        builder.write_num(&0u32, 2)?; // ShardIdent prefix
        builder.write_num(&0u32, 6)?; // shard_pfx_bits
        builder.write_num(&TON_MASTERCHAIN_ID, 32)?; // workchain
        builder.write_num(&0u64, 64)?; // shard_prefix
        let mc_shard_ident_cell = builder.build()?;
        let mc_shard_ident_parsed = ShardIdent::from_cell(&mc_shard_ident_cell)?;
        assert_eq!(mc_shard_ident_parsed.workchain, TON_MASTERCHAIN_ID);
        assert_eq!(mc_shard_ident_parsed.shard, TON_SHARD_FULL);
        let mc_shard_ident_cell_serial = mc_shard_ident_parsed.to_cell()?;
        assert_eq!(mc_shard_ident_cell, mc_shard_ident_cell_serial);
        Ok(())
    }

    #[test]
    fn test_block_tlb_shard_ident_shard() -> anyhow::Result<()> {
        // shard_block: shard:(shard_ident shard_pfx_bits:2 workchain_id:0 shard_prefix:4611686018427387904)
        // https://explorer.toncoin.org/viewblock?workchain=0&shard=6000000000000000&seqno=52111590&roothash=D350895E85FFD081F564E5D138F374A9B52B53AEE0035B07CE5A5D6388B73B45&filehash=16944A89B7DC24BCF46AC434D5E29717B0905FDBCB523FACAFE40547BF2E7DB9
        let mut builder = TonCell::builder();
        builder.write_num(&0u32, 2)?; // ShardIdent prefix
        builder.write_num(&2u32, 6)?; // shard_pfx_bits
        builder.write_num(&0i32, 32)?; // workchain
        builder.write_num(&4611686018427387904u64, 64)?; // shard_prefix
        let shard_shard_ident_cell = builder.build()?;
        let mc_shard_ident_parsed = ShardIdent::from_cell(&shard_shard_ident_cell)?;
        assert_eq!(mc_shard_ident_parsed.workchain, 0);
        assert_eq!(mc_shard_ident_parsed.shard, 0x6000000000000000);
        let mc_shard_ident_cell_serial = mc_shard_ident_parsed.to_cell()?;
        assert_eq!(shard_shard_ident_cell, mc_shard_ident_cell_serial);
        Ok(())
    }
}
