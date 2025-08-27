use crate::tep::metadata::metadata::Metadata;
use crate::tep::metadata::metadata_fields::*;
use crate::tep::metadata::snake_data::SnakeData;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Debug;
use ton_lib_core::cell::TonHash;
use ton_lib_core::error::TLCoreError;

#[derive(Serialize, PartialEq, Eq, Deserialize, Debug, Clone)]
pub struct NftItemMetadata {
    pub name: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub content_url: Option<String>,
    pub attributes: Option<Value>,
}

impl Metadata for NftItemMetadata {
    fn from_data(dict: &HashMap<TonHash, SnakeData>, json: Option<&str>) -> Result<Self, TLCoreError> {
        let mut external_meta: Option<NftItemMetadata> =
            json.map(serde_json::from_str).transpose().map_err(|_| TLCoreError::MetadataParseError)?;
        Ok(NftItemMetadata {
            name: META_NAME.use_string_or(external_meta.as_mut().and_then(|x| x.name.take()), dict),
            description: META_DESCRIPTION
                .use_string_or(external_meta.as_mut().and_then(|x| x.description.take()), dict),
            content_url: META_URI.use_string_or(external_meta.as_mut().and_then(|x| x.content_url.take()), dict),
            image: META_IMAGE.use_string_or(external_meta.as_mut().and_then(|x| x.image.take()), dict),
            attributes: META_ATTRIBUTES.use_value_or(external_meta.as_mut().and_then(|x| x.attributes.take()), dict),
        })
    }
}
