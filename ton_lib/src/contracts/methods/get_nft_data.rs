use crate::block_tlb::TVMStack;
use crate::contracts::ton_contract::TonContract;
use crate::error::TLError;
use crate::tvm_results::GetNFTDataResult;
use async_trait::async_trait;
use ton_lib_core::traits::tvm_result::TVMResult;

#[async_trait]
pub trait GetNFTData: TonContract {
    async fn get_nft_data(&self) -> Result<GetNFTDataResult, TLError> {
        let stack_boc = self.emulate_get_method("get_nft_data", &TVMStack::EMPTY).await?;
        Ok(GetNFTDataResult::from_boc(&stack_boc)?)
    }
}
