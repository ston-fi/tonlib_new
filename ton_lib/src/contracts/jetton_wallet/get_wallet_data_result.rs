use crate::cell::ton_cell::TonCellRef;
use crate::errors::TonlibError;
use crate::types::tlb::block_tlb::coins::Coins;
use crate::types::tlb::block_tlb::tvm::VMStack;
use crate::types::tlb::tlb_type::TLBType;
use crate::types::ton_address::TonAddress;
use num_bigint::ToBigUint;
use std::ops::Deref;

pub struct GetWalletDataResult {
    pub balance: Coins,
    pub owner: TonAddress,
    pub master: TonAddress,
    pub wallet_code: TonCellRef,
}

impl GetWalletDataResult {
    pub fn from_stack(mut stack: VMStack) -> Result<Self, TonlibError> {
        let balance_int = stack.pop_tiny_int()?;
        let balance = match balance_int.to_biguint() {
            Some(balance) => Coins::new(balance),
            None => {
                return Err(TonlibError::TonContractUnexpectedValue {
                    expected: "positive int".to_string(),
                    actual: format!("{balance_int}"),
                })
            }
        };
        let owner = TonAddress::from_cell(stack.pop_cell()?.deref())?;
        let master = TonAddress::from_cell(stack.pop_cell()?.deref())?;
        let wallet_code = stack.pop_cell()?;

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
        let stack = VMStack::from_boc_hex("b5ee9c7201010701009800020800000403010202090452e5602003040842028f452d7a4dfd74066b682365177259ed05734435be76b5fd4bd5d8af2b7c3d680209041014b020050400950701e31ccb92173980047f52f3231e3fce05b722017f087d0f8e02bd99a7348e43d36d7582e178099c7002c44ea652d4092859c67da44e4ca3add6565b0e2897d640a2c51bfb370d8877fa0112010001e31ccb921739060000")?;
        println!("stack: {}", stack);
        let result = GetWalletDataResult::from_stack(stack)?;
        assert_eq!(result.owner, TonAddress::from_str("UQAj-peZGPH-cC25EAv4Q-h8cBXszTmkch6ba6wXC8BM40qt")?);
        assert_eq!(result.master, TonAddress::from_str("EQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id_sDs")?);
        Ok(())
    }
}
