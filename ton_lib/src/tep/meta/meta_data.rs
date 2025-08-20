use crate::tep::snake_data::SnakeData;
use crate::tlb_adapters::DictKeyAdapterTonHash;
use crate::tlb_adapters::DictValAdapterTLBRef;
use crate::tlb_adapters::TLBHashMapE;
use serde::Deserialize;
use serde::Serialize;
use serde_aux::prelude::*;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::LazyLock;
use ton_lib_core::cell::{TonCell, TonHash};
use ton_lib_core::TLBDerive;

pub struct MetaDataField(TonHash);

impl Deref for MetaDataField {
    type Target = TonHash;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl MetaDataField {
    fn new(name: &str) -> MetaDataField {
        let mut hasher = Sha256::new();
        hasher.update(name);
        let slice = &hasher.finalize()[..];
        let key = TonHash::from_slice(slice).unwrap_or(TonHash::ZERO);
        MetaDataField(key)
    }

    pub fn use_string_or(&self, src: Option<String>, dict: &HashMap<TonHash, impl AsRef<[u8]>>) -> Option<String> {
        src.or(dict.get(&self.0).and_then(|data| String::from_utf8(data.as_ref().to_vec()).ok()))
    }

    pub fn use_value_or(&self, src: Option<Value>, dict: &HashMap<TonHash, impl AsRef<[u8]>>) -> Option<Value> {
        src.or_else(|| {
            dict.get(&self.0)
                .map(|attr_str| {
                    Some(Value::Array(vec![Value::String(
                        String::from_utf8_lossy(attr_str.as_ref()).to_string().clone(),
                    )]))
                })
                .unwrap_or_default()
        })
    }
}
pub const META_NAME: LazyLock<MetaDataField> = LazyLock::new(|| MetaDataField::new("name"));
pub const META_DESCRIPTION: LazyLock<MetaDataField> = LazyLock::new(|| MetaDataField::new("description"));
pub const META_IMAGE: LazyLock<MetaDataField> = LazyLock::new(|| MetaDataField::new("image"));
pub const META_SYMBOL: LazyLock<MetaDataField> = LazyLock::new(|| MetaDataField::new("symbol"));
pub const META_IMAGE_DATA: LazyLock<MetaDataField> = LazyLock::new(|| MetaDataField::new("image_data"));
pub const META_DECIMALS: LazyLock<MetaDataField> = LazyLock::new(|| MetaDataField::new("decimals"));
pub const META_URI: LazyLock<MetaDataField> = LazyLock::new(|| MetaDataField::new("uri"));
pub const META_CONTENT_URL: LazyLock<MetaDataField> = LazyLock::new(|| MetaDataField::new("content_url"));
pub const META_ATTRIBUTES: LazyLock<MetaDataField> = LazyLock::new(|| MetaDataField::new("attributes"));
pub const META_SOCIAL_LINKS: LazyLock<MetaDataField> = LazyLock::new(|| MetaDataField::new("social_links"));
pub const META_MARKETPLACE: LazyLock<MetaDataField> = LazyLock::new(|| MetaDataField::new("marketplace"));

#[derive(PartialEq, Eq, Debug, Clone, TLBDerive)]
pub enum MetaDataContent {
    Internal(MetaDataInternal),
    External(MetaDataExternal),
    Unsupported(MetaDataUnsupported),
}

#[derive(PartialEq, Eq, Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0x0, bits_len = 8)]
pub struct MetaDataInternal {
    #[tlb_derive(adapter = "TLBHashMapE::<DictKeyAdapterTonHash, DictValAdapterTLBRef, _, _>::new(256)")]
    pub data: HashMap<TonHash, SnakeData<true>>,
}

#[derive(PartialEq, Eq, Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0x1, bits_len = 8)]
pub struct MetaDataExternal {
    pub uri: SnakeData<false>,
}

#[derive(PartialEq, Eq, Debug, Clone, TLBDerive)]
pub struct MetaDataUnsupported {
    pub cell: TonCell,
}

// ------------------------------------JETTON META DATA----------------------------------------
#[derive(Serialize, PartialEq, Eq, Deserialize, Debug, Clone)]
pub struct JettonMetaData {
    ///Optional. UTF8 string. The name of the token - e.g. "Example Coin".
    pub name: Option<String>,
    ///Optional. Used by "Semi-chain content layout". ASCII string. A URI pointing to JSON document with metadata.
    pub uri: Option<String>,
    ///Optional. UTF8 string. The symbol of the token - e.g. "XMPL". Used in the form "You received 99 XMPL".
    pub symbol: Option<String>,
    ///Optional. UTF8 string. Describes the token - e.g. "This is an example jetton for the TON network".
    pub description: Option<String>,
    ///Optional. ASCII string. A URI pointing to a jetton icon with mime type image.
    pub image: Option<String>,
    ///Optional. Either binary representation of the image for onchain layout or base64 for offchain layout.
    pub image_data: Option<Vec<u8>>,
    ///Optional. If not specified, 9 is used by default. UTF8 encoded string with number from 0 to 255.
    ///The number of decimals the token uses - e.g. 8, means to divide the token amount by 100000000
    ///to get its user representation, while 0 means that tokens are indivisible:
    ///user representation of token number should correspond to token amount in wallet-contract storage.
    ///
    ///In case you specify decimals, it is highly recommended that you specify this parameter
    ///on-chain and that the smart contract code ensures that this parameter is immutable.
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub decimals: Option<u8>,
}

// --------------------------------------------------------------------------------------------

// ----------------------------------NFT METADATA----------------------------------------------

#[derive(Serialize, PartialEq, Eq, Deserialize, Debug, Clone)]
pub struct NftItemMetaData {
    ///  Optional. UTF8 string. Identifies the asset.
    pub name: Option<String>,
    /// Optional. UTF8 string. Describes the asset.
    pub description: Option<String>,
    /// Optional. ASCII string. A URI pointing to a resource with mime type image.
    pub image: Option<String>,
    /// Optional. No description in TEP64 yet
    pub content_url: Option<String>,
    /// Optional. No description in TEP64 yet
    pub attributes: Option<Value>,
}
// --------------------------------------------------------------------------------------------

// ------------------------------------NFT COLLECTION METADATA---------------------------------
#[derive(Serialize, PartialEq, Eq, Deserialize, Debug, Clone)]
pub struct NftCollectionMetaData {
    /// Optional. ASCII string. A URI pointing to a resource with mime type image.
    pub image: Option<String>,
    /// Optional. UTF8 string. Identifies the asset.
    pub name: Option<String>,
    /// Optional. UTF8 string. Describes the asset.
    pub description: Option<String>,
    /// Optional. No description in TEP64 yet
    pub social_links: Option<Value>,
    /// Optional. No description in TEP64 yet
    pub marketplace: Option<String>,
}
// --------------------------------------------------------------------------------------------
