use crate::block_tlb::TVMStack;
use crate::contracts::ton_contract::ContractCtx;
use crate::contracts::ton_contract::TonContract;
use crate::error::TLError;
use ton_lib_core::cell::TonHash;
use ton_lib_core::ton_contract;
use ton_lib_core::traits::tlb::TLB;

#[ton_contract]
pub struct TonWalletContract;

impl TonWalletContract {
    pub async fn seqno(&self) -> Result<u32, TLError> {
        let stack_boc = self.emulate_get_method("seqno", &TVMStack::EMPTY).await?;
        let seqno_int = TVMStack::from_boc(&stack_boc)?.pop_tiny_int()?;
        if seqno_int < 0 {
            return Err(TLError::UnexpectedValue {
                expected: "non-negative integer".to_string(),
                actual: seqno_int.to_string(),
            });
        }
        Ok(seqno_int as u32)
    }

    pub async fn get_public_key(&self) -> Result<TonHash, TLError> {
        let stack_boc = self.emulate_get_method("get_public_key", &TVMStack::EMPTY).await?;
        Ok(TonHash::from_num(&TVMStack::from_boc(&stack_boc)?.pop_int()?)?)
    }
}
