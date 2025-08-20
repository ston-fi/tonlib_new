use crate::block_tlb::TVMStack;
use crate::contracts::ton_contract::TonContract;
use crate::error::TLError;
use crate::tvm_results::GetJettonDataResult;
use async_trait::async_trait;

#[async_trait]
pub trait GetJettonData: TonContract {
    async fn get_jetton_data(&self) -> Result<GetJettonDataResult, TLError> {
        let rsp_stack = self.emulate_get_method("get_jetton_data", &TVMStack::EMPTY).await?;
        GetJettonDataResult::from_stack(rsp_stack)
    }
}
