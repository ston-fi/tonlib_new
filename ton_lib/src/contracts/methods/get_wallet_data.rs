use crate::block_tlb::TVMStack;
use crate::contracts::ton_contract::TonContract;
use crate::error::TLError;
use crate::tvm_results::GetWalletDataResult;
use async_trait::async_trait;

#[async_trait]
pub trait GetWalletData: TonContract {
    async fn get_wallet_data(&self) -> Result<GetWalletDataResult, TLError> {
        let rsp_stack = self.emulate_get_method("get_wallet_data", &TVMStack::EMPTY).await?;
        GetWalletDataResult::from_stack(rsp_stack)
    }
}
