use crate::block_tlb::TVMStack;
use crate::contracts::ton_contract::TonContract;
use crate::error::TLError;
use crate::tvm_results::GetNftContentResult;
use async_trait::async_trait;
use ton_lib_core::cell::TonCellRef;
use ton_lib_core::traits::tvm_result::TVMResult;

#[async_trait]
pub trait GetNftContent: TonContract {
    async fn get_nft_content(
        &self,
        index: i64,
        individual_content: TonCellRef,
    ) -> Result<GetNftContentResult, TLError> {
        let mut stack = TVMStack::default();
        stack.push_tiny_int(index);
        stack.push_cell(individual_content);

        let stack_boc = self.emulate_get_method("get_nft_content", &TVMStack::EMPTY).await?;

        Ok(GetNftContentResult::from_boc(&stack_boc)?)
    }
}
