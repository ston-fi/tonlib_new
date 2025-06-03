use crate::contracts::ton_contract::TonContractTrait;
use crate::errors::TonlibError;
use crate::types::tlb::block_tlb::tvm::tvm_stack::TVMStack;
use crate::types::tlb::TLB;
use crate::types::ton_address::TonAddress;
use crate::{cell::ton_cell::TonCellRef, types::tlb::block_tlb::tvm::tvm_stack_value::TVMStackValue};
use async_trait::async_trait;
use num_bigint::BigInt;
use std::ops::Deref;

#[async_trait]
pub trait GetWalletData: TonContractTrait {
    async fn get_wallet_data(&self) -> Result<GetWalletDataResult, TonlibError> {
        let run_result = self.run_get_method("get_wallet_data", &TVMStack::default()).await?;
        GetWalletDataResult::from_stack(&mut run_result.stack_parsed()?)
    }
}

pub struct GetWalletDataResult {
    pub balance: BigInt,
    pub owner: TonAddress,
    pub master: TonAddress,
    pub wallet_code: TonCellRef,
}

impl GetWalletDataResult {
    pub fn from_stack(stack: &mut TVMStack) -> Result<Self, TonlibError> {
        let wallet_code = stack.pop_cell()?;
        let master = TonAddress::from_cell(stack.pop_cell()?.deref())?;
        let owner = TonAddress::from_cell(stack.pop_cell()?.deref())?;
        let balance = match stack.pop() {
            Some(TVMStackValue::Int(i)) => i.value,
            Some(TVMStackValue::TinyInt(i)) => BigInt::from(i.value),
            Some(t) => return Err(TonlibError::TVMStackWrongType("Int or TinyInt".to_string(), format!("{:?}", t))),
            None => return Err(TonlibError::TVMStackEmpty),
        };

        Ok(Self {
            balance,
            owner,
            master,
            wallet_code,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_get_wallet_data_result_from_stack() -> anyhow::Result<()> {
        let mut stack = TVMStack::from_boc_hex("b5ee9c7201010701009800020800000403010202090452e5602003040842028f452d7a4dfd74066b682365177259ed05734435be76b5fd4bd5d8af2b7c3d680209041014b020050400950701e31ccb92173980047f52f3231e3fce05b722017f087d0f8e02bd99a7348e43d36d7582e178099c7002c44ea652d4092859c67da44e4ca3add6565b0e2897d640a2c51bfb370d8877fa0112010001e31ccb921739060000")?;
        let result = GetWalletDataResult::from_stack(&mut stack)?;
        assert_eq!(result.owner, TonAddress::from_str("UQAj-peZGPH-cC25EAv4Q-h8cBXszTmkch6ba6wXC8BM40qt")?);
        assert_eq!(result.master, TonAddress::from_str("EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs")?);
        Ok(())
    }
}
