use crate::contracts::{methods::get_nft_data::GetNftData, ton_contract::ContractCtx};
use ton_lib_core::ton_contract;

#[ton_contract]
pub struct NftItemContract;
impl GetNftData for NftItemContract {}
