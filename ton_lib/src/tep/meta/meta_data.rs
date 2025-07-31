use crate::tep::snake_data::SnakeData;
use crate::tlb_adapters::DictKeyAdapterTonHash;
use crate::tlb_adapters::DictValAdapterTLBRef;
use crate::tlb_adapters::TLBHashMap;
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

    pub fn use_string_or(&self, src: Option<String>, dict: &HashMap<TonHash, Vec<u8>>) -> Option<String> {
        src.or(dict.get(&self.0).cloned().and_then(|vec| String::from_utf8(vec).ok()))
    }

    pub fn use_value_or(&self, src: Option<Value>, dict: &HashMap<TonHash, Vec<u8>>) -> Option<Value> {
        src.or_else(|| {
            dict.get(&self.0)
                .map(|attr_str| {
                    Some(Value::Array(vec![Value::String(String::from_utf8_lossy(attr_str).to_string().clone())]))
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

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum MetaDataContent {
    Internal(MetaDataInternal),
    External(MetaDataExternal),
    Unsupported(MetaDataUnsupported),
}

#[derive(PartialEq, Eq, Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0x0, bits_len = 8)]
pub struct MetaDataInternal {
    #[tlb_derive(adapter = "TLBHashMap::<DictKeyAdapterTonHash, DictValAdapterTLBRef, _, _>::new(256)")]
    data: HashMap<TonHash, SnakeData<false>>,
}

#[derive(PartialEq, Eq, Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0x1, bits_len = 8)]
pub struct MetaDataExternal {
    uri: SnakeData<false>,
}

#[derive(PartialEq, Eq, Debug, Clone, TLBDerive)]
pub struct MetaDataUnsupported {
    cell: TonCell,
}
