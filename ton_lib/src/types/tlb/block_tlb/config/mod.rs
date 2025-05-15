mod config_param_18;
mod config_param_8;
mod config_params;

pub use config_param_18::*;
pub use config_param_8::*;
pub use config_params::*;

use crate::types::tlb::adapters::dict_key_adapters::DictKeyAdapterInto;
use crate::types::tlb::adapters::dict_val_adapters::DictValAdapterTLB;
use crate::types::tlb::adapters::Dict;
use crate::types::tlb::adapters::TLBRef;
