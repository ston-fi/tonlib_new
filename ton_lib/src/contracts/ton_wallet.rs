use crate::cell::ton_hash::TonHash;
use crate::contracts::ton_contract::ContractCtx;
use crate::contracts::ton_contract::TonContractTrait;
use crate::errors::TonlibError;
use crate::types::tlb::block_tlb::tvm::VMStack;
use ton_lib_macros::ton_contract;

#[ton_contract]
pub struct WalletContract {}

impl WalletContract {
    pub async fn seqno(&self) -> Result<u32, TonlibError> {
        let mut result = self.run_method("seqno", &VMStack::default(), None).await?;
        let seqno_int = result.stack.pop_tiny_int()?;
        if seqno_int < 0 {
            return Err(TonlibError::UnexpectedValue {
                expected: "non-negative integer".to_string(),
                actual: seqno_int.to_string(),
            });
        }
        Ok(seqno_int as u32)
    }

    pub async fn get_public_key(&self) -> Result<TonHash, TonlibError> {
        let mut result = self.run_method("get_public_key", &VMStack::default(), None).await?;
        TonHash::from_num(&result.stack.pop_int()?)
    }
}
