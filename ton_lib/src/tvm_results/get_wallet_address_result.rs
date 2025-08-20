use crate::block_tlb::TVMStack;
use std::ops::Deref;
use ton_lib_core::error::TLCoreError;
use ton_lib_core::traits::tlb::TLB;
use ton_lib_core::traits::tvm_result::TVMResult;
use ton_lib_core::types::TonAddress;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetWalletAddressResult {
    pub address: TonAddress,
}

impl TVMResult for GetWalletAddressResult {
    fn from_boc(boc: &[u8]) -> Result<Self, TLCoreError> {
        let mut stack = TVMStack::from_boc(boc)?;
        let address = TonAddress::from_cell(stack.pop_cell()?.deref())?;
        Ok(Self { address })
    }
}
