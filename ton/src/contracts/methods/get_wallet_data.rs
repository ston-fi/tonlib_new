use crate::block_tlb::TVMStack;
use crate::contracts::ton_contract::TonContract;
use crate::error::TLError;
use crate::tep::tvm_results::GetWalletDataResult;
use async_trait::async_trait;
use ton_lib_core::traits::tvm_result::TVMResult;

#[async_trait]
pub trait GetWalletData: TonContract {
    async fn get_wallet_data(&self) -> Result<GetWalletDataResult, TLError> {
        let stack_boc = self.emulate_get_method("get_wallet_data", &TVMStack::EMPTY).await?;
        Ok(GetWalletDataResult::from_boc(&stack_boc)?)
    }
}
