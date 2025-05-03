use crate::contracts::jetton_master::GetJettonDataResult;
use crate::contracts::ton_contract::ContractCtx;
use crate::contracts::ton_contract::TonContract;
use crate::errors::TonlibError;
use crate::types::tlb::block_tlb::tvm::VMStack;
use crate::types::tlb::tlb_type::TLBType;
use crate::types::ton_address::TonAddress;
use std::ops::Deref;
use ton_lib_macros::ton_contract;

#[ton_contract]
pub struct JettonMasterContract {}

impl JettonMasterContract {
    pub async fn get_wallet_address(&self, owner: &TonAddress) -> Result<TonAddress, TonlibError> {
        let mut stack = VMStack::default();
        stack.push_cell_slice(owner.to_cell_ref()?);
        let mut run_result = self.run_method("get_wallet_address", &stack).await?;
        TonAddress::from_cell(run_result.stack.pop_cell()?.deref())
    }

    pub async fn get_jetton_data(&self) -> Result<GetJettonDataResult, TonlibError> {
        let mut run_result = self.run_method("get_jetton_data", &VMStack::default()).await?;
        GetJettonDataResult::from_stack(&mut run_result.stack)
    }
}
