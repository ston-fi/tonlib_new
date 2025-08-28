use crate::block_tlb::TVMStack;
use crate::contracts::ton_contract::TonContract;
use crate::error::TLError;
use crate::tvm_results::GetNftDataResult;
use async_trait::async_trait;
use ton_lib_core::traits::tvm_result::TVMResult;

#[async_trait]
pub trait GetNftData: TonContract {
    async fn get_nft_data(&self) -> Result<GetNftDataResult, TLError> {
        let stack_boc = self.emulate_get_method("get_nft_data", &TVMStack::EMPTY).await?;
        Ok(GetNftDataResult::from_boc(&stack_boc)?)
    }
}
