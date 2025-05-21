use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::cell::ton_cell::TonCellRef;
use crate::errors::TonlibError;
use crate::types::tlb::block_tlb::coins::CurrencyCollection;
use crate::types::tlb::block_tlb::config::config_params::ConfigParams;
use crate::types::tlb::{TLBPrefix, TLB};
use ton_lib_macros::TLBDerive;

#[derive(Debug, Clone)]
pub struct MCBlockExtra {
    pub key_block: bool,
    pub shard_hashes: Option<TonCellRef>,
    pub shard_fees: ShardFees,
    // https://github.com/ton-blockchain/ton/blob/6f745c04daf8861bb1791cffce6edb1beec62204/crypto/block/block.tlb#L597
    pub ref_data: TonCellRef,         // TODO ¯\_(ツ)_/¯
    pub config: Option<ConfigParams>, // TODO
}

#[derive(Debug, Clone, TLBDerive)]
pub struct ShardFeesCreated {
    pub fees: CurrencyCollection,
    pub create: CurrencyCollection,
}

#[derive(Debug, Clone, TLBDerive)]
pub struct ShardFees {
    pub hashmap_root: Option<TonCellRef>, // TODO
    pub extra: ShardFeesCreated,
}

impl TLB for MCBlockExtra {
    const PREFIX: TLBPrefix = TLBPrefix::new(0xcca5, 8);
    fn read_definition(parser: &mut CellParser) -> Result<Self, TonlibError> {
        let key_block = TLB::read(parser)?;
        let shard_hashes = TLB::read(parser)?;
        let shard_fees = TLB::read(parser)?;
        let ref_data = TLB::read(parser)?;

        let config = match key_block {
            true => Some(TLB::read(parser)?),
            false => None,
        };
        Ok(Self {
            key_block,
            shard_hashes,
            shard_fees,
            ref_data,
            config,
        })
    }

    fn write_definition(&self, builder: &mut CellBuilder) -> Result<(), TonlibError> {
        self.key_block.write(builder)?;
        self.shard_hashes.write(builder)?;
        self.shard_fees.write(builder)?;
        self.ref_data.write(builder)?;
        if self.key_block {
            self.config.write(builder)?;
        }
        Ok(())
    }
}
