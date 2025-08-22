use crate::tep::snake_data::SnakeData;
use crate::tlb_adapters::DictKeyAdapterTonHash;
use crate::tlb_adapters::DictValAdapterTLBRef;
use crate::tlb_adapters::TLBHashMapE;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Debug;
use ton_lib_core::cell::{TonCell, TonHash};
use ton_lib_core::TLBDerive;

mod jetton_metadata;
mod nft_collection_metadata;
mod nft_item_metadata;

pub use jetton_metadata::*;
pub use nft_collection_metadata::*;
pub use nft_item_metadata::*;

#[derive(PartialEq, Eq, Debug, Clone, TLBDerive)]
pub enum MetadataContent {
    Internal(MetadataInternal),
    External(MetadataExternal),
    Unsupported(MetadataUnsupported),
}

#[derive(PartialEq, Eq, Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0x0, bits_len = 8)]
pub struct MetadataInternal {
    #[tlb_derive(adapter = "TLBHashMapE::<DictKeyAdapterTonHash, DictValAdapterTLBRef, _, _>::new(256)")]
    pub data: HashMap<TonHash, SnakeData<true>>,
}

#[derive(PartialEq, Eq, Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0x1, bits_len = 8)]
pub struct MetadataExternal {
    pub uri: SnakeData<false>,
}

#[derive(PartialEq, Eq, Debug, Clone, TLBDerive)]
pub struct MetadataUnsupported {
    pub cell: TonCell,
}

#[derive(Serialize, PartialEq, Eq, Deserialize, Debug, Clone)]
pub struct NftItemMetadata {
    pub name: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub content_url: Option<String>,
    pub attributes: Option<Value>,
}

#[derive(Serialize, PartialEq, Eq, Deserialize, Debug, Clone)]
pub struct NftCollectionMetadata {
    pub image: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub social_links: Option<Value>,
    pub marketplace: Option<String>,
}
