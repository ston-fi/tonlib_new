use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::LazyLock;
use ton_lib_core::cell::TonHash;

use crate::tep::snake_data::SnakeData;

pub struct MetadataField(TonHash);

impl Deref for MetadataField {
    type Target = TonHash;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl MetadataField {
    pub const NAME: LazyLock<MetadataField> = LazyLock::new(|| MetadataField::new("name"));
    pub const DESCRIPTION: LazyLock<MetadataField> = LazyLock::new(|| MetadataField::new("description"));
    pub const IMAGE: LazyLock<MetadataField> = LazyLock::new(|| MetadataField::new("image"));
    pub const SYMBOL: LazyLock<MetadataField> = LazyLock::new(|| MetadataField::new("symbol"));
    pub const IMAGE_DATA: LazyLock<MetadataField> = LazyLock::new(|| MetadataField::new("image_data"));
    pub const DECIMALS: LazyLock<MetadataField> = LazyLock::new(|| MetadataField::new("decimals"));
    pub const URI: LazyLock<MetadataField> = LazyLock::new(|| MetadataField::new("uri"));
    pub const CONTENT_URL: LazyLock<MetadataField> = LazyLock::new(|| MetadataField::new("content_url"));
    pub const ATTRIBUTES: LazyLock<MetadataField> = LazyLock::new(|| MetadataField::new("attributes"));
    pub const SOCIAL_LINKS: LazyLock<MetadataField> = LazyLock::new(|| MetadataField::new("social_links"));
    pub const MARKETPLACE: LazyLock<MetadataField> = LazyLock::new(|| MetadataField::new("marketplace"));

    fn new(name: &str) -> MetadataField {
        let mut hasher = Sha256::new();
        hasher.update(name);
        let slice = &hasher.finalize()[..];
        let key = TonHash::from_slice(slice).unwrap_or(TonHash::ZERO);
        MetadataField(key)
    }

    pub fn use_string_or(&self, src: Option<String>, dict: &HashMap<TonHash, SnakeData>) -> Option<String> {
        if src.is_some() {
            return src;
        };

        match dict.get(&self.0) {
            None => None,
            Some(slice) => String::from_str(slice.as_str().deref()).ok(),
        }
    }

    pub fn use_value_or(&self, src: Option<Value>, dict: &HashMap<TonHash, SnakeData>) -> Option<Value> {
        src.or_else(|| {
            dict.get(&self.0)
                .map(|attr_str| {
                    Some(Value::Array(vec![Value::String(
                        String::from_utf8_lossy(attr_str.as_str().as_bytes()).to_string().clone(),
                    )]))
                })
                .unwrap_or_default()
        })
    }
}
