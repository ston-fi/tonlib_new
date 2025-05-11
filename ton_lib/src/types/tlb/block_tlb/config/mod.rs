mod config_param_18;
mod config_param_8;

pub use config_param_8::*;
pub use config_param_18::*;

use crate::types::tlb::adapters::TLBRef;
use crate::types::tlb::adapters::Dict;
use crate::types::tlb::adapters::dict_val_adapters::DictValAdapterTLB;
use crate::types::tlb::adapters::dict_key_adapters::DictKeyAdapterInto;
use std::collections::HashMap;
use ton_lib_macros::TLBDerive;
use crate::cell::ton_cell::TonCellRef;
use crate::cell::ton_hash::TonHash;
use crate::errors::TonlibError;
use crate::types::tlb::tlb_type::TLBType;

#[derive(Debug, Clone, TLBDerive)]
pub struct ConfigParams {
    pub config_addr: TonHash,
    #[tlb_derive(adapter = "TLBRef")]
    pub config: Config,
}

#[derive(Debug, Clone, TLBDerive)]
pub struct Config {
    #[tlb_derive(adapter = "Dict::<DictKeyAdapterInto, DictValAdapterTLB, _, _>::new(32)")]
    pub data: HashMap<u32, TonCellRef>,
}

impl ConfigParams {
    pub fn storage_prices(&self) -> Result<Option<ConfigParam18>, TonlibError> { self.get_param::<ConfigParam18>(18) }
    pub fn global_version(&self) -> Result<Option<GlobalVersion>, TonlibError> { self.get_param::<GlobalVersion>(8) }
    
    pub fn get_param<T: TLBType>(&self, index: u32) -> Result<Option<T>, TonlibError> {
        self.config.data.get(&index).map(|x| TLBType::from_cell(x)).transpose()
    }
}

