use crate::tep::meta::metadata_field::*;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Debug;
use ton_lib_core::cell::TonHash;
use ton_lib_core::error::TLCoreError;
use ton_lib_core::traits::metadata::Metadata;

#[derive(Serialize, PartialEq, Eq, Deserialize, Debug, Clone)]
pub struct NftItemMetadata {
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

impl Metadata for NftItemMetadata {
    fn from_data(
        onchain: Option<&HashMap<TonHash, impl AsRef<[u8]>>>,
        offchain: Option<&str>,
    ) -> Result<Self, TLCoreError> {
        match (onchain, offchain) {
            (Some(dict), Some(json)) => {
                let external_meta: NftItemMetadata =
                    serde_json::from_str(json).map_err(|_| TLCoreError::MetadataParseError)?;
                Ok(NftItemMetadata {
                    name: META_NAME.use_string_or(external_meta.name, dict),
                    content_url: META_URI.use_string_or(external_meta.content_url, dict),
                    description: META_DESCRIPTION.use_string_or(external_meta.description, dict),
                    image: META_IMAGE.use_string_or(external_meta.image, dict),
                    attributes: META_ATTRIBUTES.use_value_or(external_meta.attributes, dict),
                })
            }
            (Some(dict), None) => Self::from_dict(dict),
            (None, Some(external_meta)) => Self::from_json(external_meta),
            (None, None) => Err(TLCoreError::MetadataParseError),
        }
    }

    fn from_json(offchain: &str) -> Result<Self, TLCoreError> {
        serde_json::from_str(&offchain).map_err(|_| TLCoreError::MetadataParseError)
    }

    fn from_dict(onchain: &HashMap<TonHash, impl AsRef<[u8]>>) -> Result<Self, TLCoreError> {
        Ok(NftItemMetadata {
            name: META_NAME.use_string_or(None, onchain),
            content_url: META_URI.use_string_or(None, onchain),
            description: META_DESCRIPTION.use_string_or(None, onchain),
            image: META_IMAGE.use_string_or(None, onchain),
            attributes: META_ATTRIBUTES.use_value_or(None, onchain),
        })
    }
}
