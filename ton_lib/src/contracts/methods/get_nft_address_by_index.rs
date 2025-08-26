use crate::block_tlb::TVMStack;
use crate::contracts::ton_contract::TonContract;
use crate::error::TLError;
use crate::tvm_results::GetNftAddressByIndexResult;
use async_trait::async_trait;
use ton_lib_core::traits::tvm_result::TVMResult;

#[async_trait]
pub trait GetNftAddressByIndex: TonContract {
    async fn get_nft_address_by_index(&self, index: i64) -> Result<GetNftAddressByIndexResult, TLError> {
        let mut stack = TVMStack::default();
        stack.push_tiny_int(index);

        let stack_boc = self.emulate_get_method("get_nft_address_by_index", &TVMStack::EMPTY).await?;
        Ok(GetNftAddressByIndexResult::from_boc(&stack_boc)?)
    }
}
