use crate::tep::meta::metadata_field::*;
use serde::Deserialize;
use serde::Serialize;
use serde_aux::prelude::*;
use std::collections::HashMap;
use std::fmt::Debug;
use ton_lib_core::cell::TonHash;
use ton_lib_core::error::TLCoreError;
use ton_lib_core::traits::metadata::Metadata;

#[derive(Serialize, PartialEq, Eq, Deserialize, Debug, Clone)]
pub struct JettonMetadata {
    pub name: Option<String>,
    pub uri: Option<String>,
    pub symbol: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub image_data: Option<Vec<u8>>,
    ///Optional. If not specified, 9 is used by default. UTF8 encoded string with number from 0 to 255.
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub decimals: Option<u8>,
}

impl Metadata for JettonMetadata {
    fn from_data(
        onchain: Option<&HashMap<TonHash, impl AsRef<[u8]>>>,
        offchain: Option<&str>,
    ) -> Result<Self, TLCoreError> {
        match (onchain, offchain) {
            (Some(dict), Some(json)) => {
                let external_meta: JettonMetadata =
                    serde_json::from_str(json).map_err(|_| TLCoreError::MetadataParseError)?;
                Ok(JettonMetadata {
                    name: META_NAME.use_string_or(external_meta.name, dict),
                    uri: META_URI.use_string_or(external_meta.uri, dict),
                    symbol: META_SYMBOL.use_string_or(external_meta.symbol, dict),
                    description: META_DESCRIPTION.use_string_or(external_meta.description, dict),
                    image: META_IMAGE.use_string_or(external_meta.image, dict),
                    image_data: external_meta
                        .image_data
                        .or(dict.get(&*META_IMAGE_DATA).map(|elem| elem.as_ref().to_vec())),
                    decimals: META_DECIMALS.use_string_or(None, dict).map(|v| v.parse::<u8>().unwrap()),
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
        let decimals = META_DECIMALS
            .use_string_or(None, onchain)
            .map(|v| v.parse::<u8>())
            .transpose()
            .map_err(|_| TLCoreError::MetadataParseError)?;

        Ok(JettonMetadata {
            name: META_NAME.use_string_or(None, onchain),
            uri: META_URI.use_string_or(None, onchain),
            symbol: META_SYMBOL.use_string_or(None, onchain),
            description: META_DESCRIPTION.use_string_or(None, onchain),
            image: META_IMAGE.use_string_or(None, onchain),
            image_data: onchain.get(&META_IMAGE_DATA).map(|elem| elem.as_ref().to_vec()),
            decimals,
        })
    }
}
