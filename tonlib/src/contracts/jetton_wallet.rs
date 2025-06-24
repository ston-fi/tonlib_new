use crate::contracts::methods::get_wallet_data::GetWalletData;
use crate::contracts::traits::ContractCtx;
use crate::contracts::traits::ContractTrait;
use ton_lib_core::ton_contract;

#[ton_contract]
pub struct JettonWallet;
impl GetWalletData for JettonWallet {}
