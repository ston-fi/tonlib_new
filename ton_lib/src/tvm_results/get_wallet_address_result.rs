use crate::block_tlb::TVMStack;
use crate::error::TLError;
use std::ops::Deref;
use ton_lib_core::traits::tlb::TLB;
use ton_lib_core::types::TonAddress;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetWalletAddressResult {
    pub address: TonAddress,
}

impl GetWalletAddressResult {
    pub fn from_stack(mut stack: TVMStack) -> Result<Self, TLError> {
        let address = TonAddress::from_cell(stack.pop_cell()?.deref())?;
        Ok(Self { address })
    }
}
