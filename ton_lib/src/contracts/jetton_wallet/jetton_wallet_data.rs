use crate::cell::ton_cell::TonCellRef;
use crate::errors::TonlibError;
use crate::types::tlb::block_tlb::coins::Coins;
use crate::types::tlb::block_tlb::tvm::VMStack;
use crate::types::tlb::tlb_type::TLBType;
use crate::types::ton_address::TonAddress;
use std::ops::Deref;

pub struct JettonWalletData {
    pub balance: Coins,
    pub owner: TonAddress,
    pub master: TonAddress,
    pub wallet_code: TonCellRef,
}

impl JettonWalletData {
    pub fn from_stack(mut stack: VMStack) -> Result<Self, TonlibError> {
        let balance_int = stack.pop_int()?;
        let balance = match balance_int.to_biguint() {
            Some(balance) => Coins::new(balance),
            None => return Err(TonlibError::TonContractUnexpectedValue(balance_int)),
        };
        let owner = TonAddress::from_cell(stack.pop_cell_slice()?.deref())?;
        let master = TonAddress::from_cell(stack.pop_cell_slice()?.deref())?;
        let wallet_code = stack.pop_cell_slice()?;

        Ok(Self {
            balance,
            owner,
            master,
            wallet_code,
        })
    }
}
