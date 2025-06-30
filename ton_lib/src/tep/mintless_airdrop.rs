use crate::block_tlb::Coins;
use crate::tlb_adapters::ConstLen;
use crate::tlb_adapters::DictKeyAdapterAddress;
use crate::tlb_adapters::DictValAdapterTLB;
use crate::tlb_adapters::TLBHashMap;
use std::collections::HashMap;
use ton_lib_core::types::TonAddress;
use ton_lib_core::TLBDerive;

#[derive(Clone, Debug, TLBDerive)]
pub struct MintlessAirdropDict {
    #[tlb_derive(adapter = "TLBHashMap::<DictKeyAdapterAddress, DictValAdapterTLB, _, _>::new(267)")]
    pub data: HashMap<TonAddress, MintlessAirdropData>,
}

#[derive(Clone, Debug, PartialEq, TLBDerive)]
pub struct MintlessAirdropData {
    pub amount: Coins,
    #[tlb_derive(bits_len = 48)]
    pub start_from: u64,
    #[tlb_derive(bits_len = 48)]
    pub expired_at: u64,
}
