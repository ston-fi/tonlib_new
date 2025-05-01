use crate::contracts::jetton_wallet::jetton_wallet_data::JettonWalletData;
use crate::contracts::ton_contract::{ContractCtx, TonContract};
use crate::errors::TonlibError;
use crate::types::tlb::block_tlb::tvm::VMStack;
use ton_lib_macros::ton_contract;

#[ton_contract]
pub struct JettonWalletContract {}

impl JettonWalletContract {
    pub async fn get_wallet_data(&self) -> Result<JettonWalletData, TonlibError> {
        let run_result = self.run_method("get_wallet_data", &VMStack::default()).await?;
        JettonWalletData::from_stack(run_result.stack)
    }
}
