use crate::block_tlb::TVMStack;
use crate::contracts::ton_contract::TonContract;
use crate::error::TLError;
use crate::tep::MetaDataContent;
use crate::tvm_results::GetJettonDataResult;
use async_trait::async_trait;
use ton_lib_core::cell::TonCellRef;
use ton_lib_core::traits::tlb::TLB;
use ton_lib_core::traits::tvm_result::TVMResult;
use ton_lib_core::types::TonAddress;

#[async_trait]
pub trait GetJettonData: TonContract {
    async fn get_jetton_data(&self) -> Result<GetJettonDataResult, TLError> {
        let stack_boc = self.emulate_get_method("get_jetton_data", &TVMStack::EMPTY).await?;
        Ok(GetJettonDataResult::from_boc(&stack_boc)?)
    }
}
