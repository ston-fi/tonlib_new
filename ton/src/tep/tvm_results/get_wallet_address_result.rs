use crate::block_tlb::TVMStack;
use crate::tep::tvm_results::tvm_result::TVMResult;
use std::ops::Deref;
use ton_lib_core::error::TLCoreError;
use ton_lib_core::traits::tlb::TLB;
use ton_lib_core::types::TonAddress;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetWalletAddressResult {
    pub address: TonAddress,
}

impl TVMResult for GetWalletAddressResult {
    fn from_stack(stack: &mut TVMStack) -> Result<Self, TLCoreError> {
        let address = TonAddress::from_cell(stack.pop_cell()?.deref())?;
        Ok(Self { address })
    }
}
