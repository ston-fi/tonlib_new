use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::cell::ton_cell::TonCellRef;
use crate::errors::TonlibError;
use crate::types::tlb::block_tlb::coins::CurrencyCollection;
use crate::types::tlb::block_tlb::config::config_params::ConfigParams;
use crate::types::tlb::{TLBPrefix, TLB};
use ton_lib_macros::TLBDerive;

// https://github.com/ton-blockchain/ton/blame/6f745c04daf8861bb1791cffce6edb1beec62204/crypto/block/block.tlb#L593
#[derive(Debug, Clone, PartialEq)]
pub struct MCBlockExtra {
    pub key_block: bool,
    pub shard_hashes: Option<TonCellRef>,
    pub shard_fees: Option<TonCellRef>,
    shard_fees_crated: ShardFeesCreated, // this is a mock to read/write cell properly while we don't support fail HashmapAugE
    // https://github.com/ton-blockchain/ton/blob/6f745c04daf8861bb1791cffce6edb1beec62204/crypto/block/block.tlb#L597
    pub ref_data: TonCellRef,
    pub config: Option<ConfigParams>,
}

#[derive(Debug, Clone, PartialEq, TLBDerive)]
pub struct ShardFeesCreated {
    pub fees: CurrencyCollection,
    pub create: CurrencyCollection,
}

impl TLB for MCBlockExtra {
    const PREFIX: TLBPrefix = TLBPrefix::new(0xcca5, 16);
    fn read_definition(parser: &mut CellParser) -> Result<Self, TonlibError> {
        let key_block = TLB::read(parser)?;
        let shard_hashes = TLB::read(parser)?;
        let shard_fees = TLB::read(parser)?;
        let shard_fees_crated = TLB::read(parser)?;
        let ref_data = TLB::read(parser)?;

        let config = match key_block {
            true => Some(TLB::read(parser)?),
            false => None,
        };
        Ok(Self {
            key_block,
            shard_hashes,
            shard_fees,
            shard_fees_crated,
            ref_data,
            config,
        })
    }

    fn write_definition(&self, builder: &mut CellBuilder) -> Result<(), TonlibError> {
        self.key_block.write(builder)?;
        self.shard_hashes.write(builder)?;
        self.shard_fees.write(builder)?;
        self.shard_fees_crated.write(builder)?;
        self.ref_data.write(builder)?;
        if self.key_block {
            match &self.config {
                Some(config) => config.write(builder)?,
                None => {
                    let err_msg = "MCBlockExtra has key_block=true, but config is None".to_string();
                    return Err(TonlibError::TLBWrongData(err_msg));
                }
            }
        }
        Ok(())
    }
}
