use crate::cell::ton_cell::TonCellRef;
use crate::cell::ton_hash::TonHash;
use ton_lib_macros::TLBDerive;

#[derive(Debug, Clone, PartialEq, TLBDerive)]
pub struct ConfigParams {
    pub config_addr: TonHash,
    // #[tlb_derive(adapter = "TLBRef")]
    pub config: TonCellRef,
}

// #[derive(Debug, Clone, PartialEq, TLBDerive)]
// pub struct Config {
//     #[tlb_derive(adapter = "Dict::<DictKeyAdapterInto, DictValAdapterTLB, _, _>::new(32)")]
//     pub data: HashMap<u32, TonCellRef>,
// }

impl ConfigParams {
    // pub fn storage_prices(&self) -> Result<Option<ConfigParam18>, TonlibError> { self.get_param::<ConfigParam18>(18) }
    // pub fn global_version(&self) -> Result<Option<GlobalVersion>, TonlibError> { self.get_param::<GlobalVersion>(8) }

    // pub fn get_param<T: TLB>(&self, index: u32) -> Result<Option<T>, TonlibError> {
    //     self.config.data.get(&index).map(|x| TLB::from_cell(x)).transpose()
    // }
}
