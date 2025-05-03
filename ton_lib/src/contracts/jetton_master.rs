use crate::contracts::methods::get_jetton_data::GetJettonData;
use crate::contracts::methods::get_wallet_address::GetWalletAddress;
use crate::contracts::ton_contract::ContractCtx;
use crate::contracts::ton_contract::TonContract;
use ton_lib_macros::ton_contract;

#[ton_contract]
pub struct JettonMaster {}
impl GetJettonData for JettonMaster {}
impl GetWalletAddress for JettonMaster {}
