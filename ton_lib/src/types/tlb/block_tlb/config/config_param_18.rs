use crate::types::tlb::block_tlb::config::Dict;
use crate::types::tlb::block_tlb::config::DictValAdapterTLB;
use crate::types::tlb::block_tlb::config::DictKeyAdapterInto;
use std::collections::HashMap;
use ton_lib_macros::TLBDerive;
use crate::cell::ton_cell::TonCellRef;

#[derive(Debug, Clone, PartialEq, TLBDerive)]
#[tlb_derive(prefix = 0xcc, bits_len = 8)]
pub struct StoragePrices {
    pub utime_since: u32,
    pub bit_price_ps: u64,
    pub cell_price_ps: u64,
    pub mc_bit_price_ps: u64,
    pub mc_cell_price_ps: u64,
}

#[derive(Debug, Clone, PartialEq, TLBDerive)]
pub struct ConfigParam18 {
    #[tlb_derive(adapter = "Dict::<DictKeyAdapterInto, DictValAdapterTLB, _, _>::new(32)")]
    pub storage_prices: HashMap<u32, TonCellRef>,
}
