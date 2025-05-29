use crate::cell::ton_cell::TonCellRef;
use crate::cell::ton_hash::TonHash;
use crate::errors::TonlibError;
use crate::types::tlb::adapters::dict_key_adapters::DictKeyAdapterInto;
use crate::types::tlb::adapters::dict_val_adapters::DictValAdapterTLB;
use crate::types::tlb::adapters::Dict;
use crate::types::tlb::adapters::TLBRef;
use crate::types::tlb::block_tlb::config::config_param_18::ConfigParam18;
use crate::types::tlb::block_tlb::config::config_param_8::GlobalVersion;
use crate::types::tlb::TLB;
use std::collections::HashMap;
use ton_lib_macros::TLBDerive;

// https://github.com/ton-blockchain/ton/blame/6f745c04daf8861bb1791cffce6edb1beec62204/crypto/block/block.tlb#L543
#[derive(Debug, Clone, PartialEq, TLBDerive)]
pub struct ConfigParams {
    pub config_addr: TonHash,
    #[tlb_derive(adapter = "TLBRef")]
    pub config: Config,
}

#[derive(Debug, Clone, PartialEq, TLBDerive)]
pub struct Config {
    #[tlb_derive(adapter = "Dict::<DictKeyAdapterInto, DictValAdapterTLB, _, _>::new(32)")]
    pub data: HashMap<u32, TonCellRef>,
}

impl ConfigParams {
    pub fn storage_prices(&self) -> Result<ConfigParam18, TonlibError> { self.get_param::<ConfigParam18>(18) }
    pub fn global_version(&self) -> Result<GlobalVersion, TonlibError> { self.get_param::<GlobalVersion>(8) }

    pub fn get_param<T: TLB>(&self, index: u32) -> Result<T, TonlibError> {
        self.config
            .data
            .get(&index)
            .map(|x| TLB::from_cell(x))
            .transpose()?
            .ok_or_else(|| TonlibError::TLBWrongData(format!("Config param with index {} not found", index)))
    }
}

#[cfg(test)]
mod tests {
    use crate::types::tlb::block_tlb::config::config_param_18::{ConfigParam18, StoragePrices};
    use crate::types::tlb::block_tlb::config::config_param_8::GlobalVersion;
    use crate::types::tlb::block_tlb::config::config_params::ConfigParams;
    use crate::types::tlb::block_tlb::test_block_data::CONFIG_BOC_HEX;
    use crate::types::tlb::TLB;
    use std::collections::HashMap;

    #[test]
    fn test_config_params() -> anyhow::Result<()> {
        let config_params = ConfigParams::from_boc_hex(CONFIG_BOC_HEX)?;
        let serialized = config_params.to_boc()?;
        let parsed_back = ConfigParams::from_boc(&serialized)?;
        assert_eq!(config_params, parsed_back);
        Ok(())
    }

    #[test]
    fn test_config_param_8() -> anyhow::Result<()> {
        let parsed_param = ConfigParams::from_boc_hex(CONFIG_BOC_HEX)?.global_version()?;
        let expected_param = GlobalVersion {
            version: 9,
            capabilities: 494,
        };
        assert_eq!(parsed_param, expected_param);
        Ok(())
    }

    #[test]
    fn test_config_param_18() -> anyhow::Result<()> {
        let parsed_param = ConfigParams::from_boc_hex(CONFIG_BOC_HEX)?.storage_prices()?;
        let expected_prices = StoragePrices {
            utime_since: 0,
            bit_price_ps: 1,
            cell_price_ps: 500,
            mc_bit_price_ps: 1000,
            mc_cell_price_ps: 500000,
        };
        let expected_param = ConfigParam18 {
            storage_prices: HashMap::from([(0, expected_prices.clone())]),
        };
        assert_eq!(expected_param, parsed_param);
        assert_eq!(&expected_prices, parsed_param.get_first_prices()?);

        Ok(())
    }
}
