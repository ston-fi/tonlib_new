use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Debug;
use ton_lib_core::cell::TonHash;
use ton_lib_core::error::TLCoreError;
use ton_lib_core::traits::metadata::Metadata;

use crate::tep::metadata::metadata_fields::*;

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
    fn from_data(dict: &HashMap<TonHash, impl AsRef<[u8]>>, json: Option<&str>) -> Result<Self, TLCoreError> {
        let mut external_meta: Option<NftCollectionMetadata> =
            json.map(serde_json::from_str).transpose().map_err(|_| TLCoreError::MetadataParseError)?;

        Ok(NftCollectionMetadata {
            image: META_IMAGE.use_string_or(external_meta.as_mut().and_then(|x| x.image.take()), dict),
            name: META_NAME.use_string_or(external_meta.as_mut().and_then(|x| x.name.take()), dict),
            description: META_DESCRIPTION
                .use_string_or(external_meta.as_mut().and_then(|x| x.description.take()), dict),
            social_links: META_SOCIAL_LINKS
                .use_value_or(external_meta.as_mut().and_then(|x| x.social_links.take()), dict),
            marketplace: META_MARKETPLACE
                .use_string_or(external_meta.as_mut().and_then(|x| x.marketplace.take()), dict),
        })
    }
}
