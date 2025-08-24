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
pub struct NftCollectionMetadata {
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

impl Metadata for NftCollectionMetadata {
    fn from_data(
        onchain: Option<&HashMap<TonHash, impl AsRef<[u8]>>>,
        offchain: Option<&str>,
    ) -> Result<Self, TLCoreError> {
        match (onchain, offchain) {
            (Some(dict), Some(json)) => {
                let external_meta: NftCollectionMetadata =
                    serde_json::from_str(json).map_err(|_| TLCoreError::MetadataParseError)?;
                Ok(NftCollectionMetadata {
                    image: META_IMAGE.use_string_or(external_meta.image, dict),
                    name: META_NAME.use_string_or(external_meta.name, dict),
                    description: META_DESCRIPTION.use_string_or(external_meta.description, dict),
                    social_links: META_SOCIAL_LINKS.use_value_or(external_meta.social_links, dict),
                    marketplace: META_MARKETPLACE.use_string_or(external_meta.marketplace, dict),
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
        Ok(NftCollectionMetadata {
            image: META_IMAGE.use_string_or(None, onchain),
            name: META_NAME.use_string_or(None, onchain),
            description: META_DESCRIPTION.use_string_or(None, onchain),
            social_links: META_SOCIAL_LINKS.use_value_or(None, onchain),
            marketplace: META_MARKETPLACE.use_string_or(None, onchain),
        })
    }
}
