use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Debug;
use ton_lib_core::cell::TonHash;
use ton_lib_core::error::TLCoreError;

use crate::tep::metadata::Metadata;
use crate::tep::metadata::*;
use crate::tep::snake_data::SnakeData;

#[derive(Serialize, PartialEq, Eq, Deserialize, Debug, Clone)]
pub struct NFTCollectionMetadata {
    pub image: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub social_links: Option<Value>,
    pub marketplace: Option<String>,
}

impl Metadata for NFTCollectionMetadata {
    fn from_data(dict: &HashMap<TonHash, SnakeData>, json: Option<&str>) -> Result<Self, TLCoreError> {
        let mut external_meta: Option<NFTCollectionMetadata> =
            json.map(serde_json::from_str).transpose().map_err(|_| TLCoreError::MetadataParseError)?;

        Ok(NFTCollectionMetadata {
            image: MetadataField::IMAGE.use_string_or(external_meta.as_mut().and_then(|x| x.image.take()), dict),
            name: MetadataField::NAME.use_string_or(external_meta.as_mut().and_then(|x| x.name.take()), dict),
            description: MetadataField::DESCRIPTION
                .use_string_or(external_meta.as_mut().and_then(|x| x.description.take()), dict),
            social_links: MetadataField::SOCIAL_LINKS
                .use_value_or(external_meta.as_mut().and_then(|x| x.social_links.take()), dict),
            marketplace: MetadataField::MARKETPLACE
                .use_string_or(external_meta.as_mut().and_then(|x| x.marketplace.take()), dict),
        })
    }
}
