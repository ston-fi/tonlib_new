use crate::contracts::{
    methods::get_collection_data::GetCollectionData, methods::get_nft_address_by_index::GetNftAddressByIndex,
    methods::get_nft_content::GetNftContent, ton_contract::ContractCtx,
};
use ton_lib_core::ton_contract;

#[ton_contract]
pub struct NftCollectionContract;
impl GetNftContent for NftCollectionContract {}
impl GetCollectionData for NftCollectionContract {}
impl GetNftAddressByIndex for NftCollectionContract {}
