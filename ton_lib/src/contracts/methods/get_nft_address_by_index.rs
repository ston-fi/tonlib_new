use crate::block_tlb::TVMStack;
use crate::contracts::ton_contract::TonContract;
use crate::error::TLError;
use crate::tvm_results::GetNFTAddressByIndexResult;
use async_trait::async_trait;
use num_bigint::BigInt;
use ton_lib_core::traits::tvm_result::TVMResult;

#[async_trait]
pub trait GetNFTAddressByIndex: TonContract {
    async fn get_nft_address_by_index(&self, index: BigInt) -> Result<GetNFTAddressByIndexResult, TLError> {
        let mut stack = TVMStack::default();
        stack.push_int(index);

        let stack_boc = self.emulate_get_method("get_nft_address_by_index", &stack).await?;
        Ok(GetNFTAddressByIndexResult::from_boc(&stack_boc)?)
    }
}
