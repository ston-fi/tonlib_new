use crate::contracts::ton_contract::TonContractTrait;
use crate::errors::TonlibError;
use crate::types::tlb::block_tlb::tvm::VMStack;
use crate::types::tlb::tlb_type::TLBType;
use crate::types::ton_address::TonAddress;
use async_trait::async_trait;
use std::ops::Deref;

#[async_trait]
pub trait GetWalletAddress: TonContractTrait {
    async fn get_wallet_address(&self, owner: &TonAddress) -> Result<TonAddress, TonlibError> {
        let mut stack = VMStack::default();
        stack.push_cell_slice(owner.to_cell_ref()?);
        let mut run_result = self.run_method("get_wallet_address", &stack, None).await?;
        TonAddress::from_cell(run_result.stack.pop_cell()?.deref())
    }
}
