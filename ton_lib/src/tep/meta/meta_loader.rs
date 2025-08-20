use std::collections::HashMap;
use std::num::ParseIntError;

use crate::tep::meta::ipfs_loader::IpfsLoader;
use crate::tep::meta::MetaDataContent;
use crate::tep::meta::*;
use crate::tep::IpfsLoaderConfig;
use crate::tep::IpfsLoaderError;
use crate::tep::JettonMetaData;
use crate::tep::SnakeData;
use async_trait::async_trait;
use reqwest::Client;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use thiserror::Error;
use ton_lib_core::cell::TonHash;

#[derive(Debug, Error)]
pub enum MetaLoaderError {
    #[error("Unsupported content layout (Metadata content: {0:?})")]
    ContentLayoutUnsupported(MetaDataContent),

    #[error("Failed to load jetton metadata (URI: {uri}, response status code: {status})")]
    LoadMetaDataFailed { uri: String, status: StatusCode },

    #[error("Serde_json Error ({0})")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("IpfsLoaderError ({0})")]
    IpfsLoaderError(#[from] IpfsLoaderError),

    #[error("Transport error ({0})")]
    TransportError(#[from] reqwest::Error),

    #[error("Internal error ({0})")]
    InternalError(String),

    #[error("Parse int error ({0})")]
    ParseIntError(#[from] ParseIntError),
}

pub struct MetaLoader<MetaData>
where
    MetaData: DeserializeOwned,
{
    http_client: reqwest::Client,
    meta_data_marker: std::marker::PhantomData<MetaData>,
    ipfs_loader: IpfsLoader,
}

impl<MetaData> Default for MetaLoader<MetaData>
where
    MetaData: DeserializeOwned,
{
    fn default() -> Self { MetaLoaderBuilder::new().build() }
}

pub struct MetaLoaderBuilder {
    http_client: reqwest::Client,
    ipfs_loader: IpfsLoader,
}

impl MetaLoaderBuilder {
    pub fn new() -> Self {
        MetaLoaderBuilder {
            http_client: Client::builder().build().unwrap(),
            ipfs_loader: IpfsLoader::new(&IpfsLoaderConfig::default()).unwrap(),
        }
    }

    pub fn http_client(&mut self, http_client: reqwest::Client) -> &mut Self {
        self.http_client = http_client;
        self
    }
    pub fn ipfs_loader(&mut self, ipfs_loader: IpfsLoader) -> &mut Self {
        self.ipfs_loader = ipfs_loader;
        self
    }

    pub fn build<MetaData: DeserializeOwned>(self) -> MetaLoader<MetaData> {
        MetaLoader {
            http_client: self.http_client,
            meta_data_marker: std::marker::PhantomData::default(),
            ipfs_loader: self.ipfs_loader,
        }
    }
}

#[async_trait]
pub trait LoadMeta<MetaData>
where
    MetaData: DeserializeOwned,
{
    async fn load(&self, content: &MetaDataContent) -> Result<MetaData, MetaLoaderError>;
}

impl<MetaData> MetaLoader<MetaData>
where
    MetaData: DeserializeOwned,
{
    pub fn new() -> MetaLoaderBuilder { MetaLoaderBuilder::new() }

    pub async fn load_meta_from_uri(&self, uri: &str) -> Result<MetaData, MetaLoaderError> {
        log::trace!("Downloading metadata from {}", uri);
        let meta_str: String = if uri.starts_with("ipfs://") {
            let path: String = uri.chars().skip(7).collect();
            self.ipfs_loader.load_utf8_lossy(path.as_str()).await?
        } else {
            let resp = self.http_client.get(uri).send().await?;
            if resp.status().is_success() {
                resp.text().await?
            } else {
                return Err(MetaLoaderError::LoadMetaDataFailed {
                    uri: uri.to_string(),
                    status: resp.status(),
                });
            }
        };

        // Deserialize using the original meta_str
        let meta: MetaData = serde_json::from_str(&meta_str)?;

        Ok(meta)
    }
}

// ------------------------------------JETTON META DATA----------------------------------------
#[async_trait]
impl LoadMeta<JettonMetaData> for MetaLoader<JettonMetaData> {
    async fn load(&self, content: &MetaDataContent) -> Result<JettonMetaData, MetaLoaderError> {
        match content {
            MetaDataContent::External(MetaDataExternal { uri }) => self.load_meta_from_uri(&uri.as_str()).await,
            MetaDataContent::Internal(MetaDataInternal { data: dict }) => {
                if dict.contains_key(&*META_URI) {
                    let uri = String::from_utf8_lossy(dict.get(&*META_URI).unwrap().as_slice()).to_string();
                    let result = self.load_meta_from_uri(uri.as_str()).await;

                    match result {
                        Ok(external_meta) => Ok(JettonMetaData {
                            name: META_NAME.use_string_or(external_meta.name, dict),
                            uri: META_URI.use_string_or(external_meta.uri, dict),
                            symbol: META_SYMBOL.use_string_or(external_meta.symbol, dict),
                            description: META_DESCRIPTION.use_string_or(external_meta.description, dict),
                            image: META_IMAGE.use_string_or(external_meta.image, dict),
                            image_data: external_meta
                                .image_data
                                .or(dict.get(&*META_IMAGE_DATA).map(|elem| elem.data.to_vec())),
                            decimals: META_DECIMALS.use_string_or(None, dict).map(|v| v.parse::<u8>().unwrap()),
                        }),
                        Err(_) => Ok(dict.try_into()?),
                    }
                } else {
                    dict.try_into()
                }
            }

            content => Err(MetaLoaderError::ContentLayoutUnsupported(content.clone())),
        }
    }
}

impl<const HAS_PREFIX: bool> TryFrom<&HashMap<TonHash, SnakeData<HAS_PREFIX>>> for JettonMetaData {
    type Error = MetaLoaderError;

    fn try_from(dict: &HashMap<TonHash, SnakeData<HAS_PREFIX>>) -> Result<Self, Self::Error> {
        let decimals = META_DECIMALS.use_string_or(None, dict).map(|v| v.parse::<u8>()).transpose()?;

        Ok(JettonMetaData {
            name: META_NAME.use_string_or(None, dict),
            uri: META_URI.use_string_or(None, dict),
            symbol: META_SYMBOL.use_string_or(None, dict),
            description: META_DESCRIPTION.use_string_or(None, dict),
            image: META_IMAGE.use_string_or(None, dict),
            image_data: dict.get(&META_IMAGE_DATA).map(|elem| elem.data.clone()),
            decimals,
        })
    }
}

// --------------------------------------------------------------------------------------------

// ----------------------------------NFT METADATA----------------------------------------------
#[async_trait]
impl LoadMeta<NftItemMetaData> for MetaLoader<NftItemMetaData> {
    async fn load(&self, content: &MetaDataContent) -> Result<NftItemMetaData, MetaLoaderError> {
        match content {
            MetaDataContent::External(MetaDataExternal { uri }) => self.load_meta_from_uri(&uri.as_str()).await,
            MetaDataContent::Internal(MetaDataInternal { data: dict }) => {
                if dict.contains_key(&META_URI) {
                    let uri = String::from_utf8_lossy(dict.get(&META_URI).unwrap().as_ref()).to_string();
                    let external_meta = self.load_meta_from_uri(uri.as_str()).await?;
                    Ok(NftItemMetaData {
                        name: META_NAME.use_string_or(external_meta.name, dict),
                        content_url: META_URI.use_string_or(external_meta.content_url, dict),
                        description: META_DESCRIPTION.use_string_or(external_meta.description, dict),
                        image: META_IMAGE.use_string_or(external_meta.image, dict),
                        attributes: META_ATTRIBUTES.use_value_or(external_meta.attributes, dict),
                    })
                } else {
                    Ok(NftItemMetaData {
                        name: META_NAME.use_string_or(None, dict),
                        content_url: META_URI.use_string_or(None, dict),
                        description: META_DESCRIPTION.use_string_or(None, dict),
                        image: META_IMAGE.use_string_or(None, dict),
                        attributes: META_ATTRIBUTES.use_value_or(None, dict),
                    })
                }
            }
            content => Err(MetaLoaderError::ContentLayoutUnsupported(content.clone())),
        }
    }
}
// --------------------------------------------------------------------------------------------

// ------------------------------------NFT COLLECTION METADATA---------------------------------
#[async_trait]
impl LoadMeta<NftCollectionMetaData> for MetaLoader<NftCollectionMetaData> {
    async fn load(&self, content: &MetaDataContent) -> Result<NftCollectionMetaData, MetaLoaderError> {
        match content {
            MetaDataContent::External(MetaDataExternal { uri }) => self.load_meta_from_uri(&uri.as_str()).await,
            MetaDataContent::Internal(MetaDataInternal { data: dict }) => {
                if dict.contains_key(&META_URI) {
                    let uri = String::from_utf8_lossy(dict.get(&META_URI).unwrap().as_ref()).to_string();
                    let external_meta = self.load_meta_from_uri(uri.as_str()).await?;
                    Ok(NftCollectionMetaData {
                        image: META_IMAGE.use_string_or(external_meta.image, dict),
                        name: META_NAME.use_string_or(external_meta.name, dict),
                        description: META_DESCRIPTION.use_string_or(external_meta.description, dict),
                        social_links: META_SOCIAL_LINKS.use_value_or(external_meta.social_links, dict),
                        marketplace: META_MARKETPLACE.use_string_or(external_meta.marketplace, dict),
                    })
                } else {
                    Ok(NftCollectionMetaData {
                        image: META_IMAGE.use_string_or(None, dict),
                        name: META_NAME.use_string_or(None, dict),
                        description: META_DESCRIPTION.use_string_or(None, dict),
                        social_links: META_SOCIAL_LINKS.use_value_or(None, dict),
                        marketplace: META_MARKETPLACE.use_string_or(None, dict),
                    })
                }
            }
            content => Err(MetaLoaderError::ContentLayoutUnsupported(content.clone())),
        }
    }
}

// --------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use crate::tep::meta::meta_loader::{MetaLoader, NftItemMetaData};

    #[tokio::test]
    async fn test_meta_data_load_ordinal_https() -> anyhow::Result<()> {
        let loader = MetaLoader::<NftItemMetaData>::default();

        let metadata =
            loader.load_meta_from_uri("https://s.getgems.io/nft/b/c/62fba50217c3fe3cbaad9e7f/95/meta.json").await?;

        let expected_meta = NftItemMetaData {
            name: Some(String::from("TON Smart Challenge #2 Winners Trophy")),
            description: Some(String::from("TON Smart Challenge #2 Winners Trophy 93 place out of 181")),
            image: Some(String::from("https://s.getgems.io/nft/b/c/62fba50217c3fe3cbaad9e7f/images/943e994f91227c3fdbccbc6d8635bfaab256fbb4")),
            content_url: Some(String::from("https://s.getgems.io/nft/b/c/62fba50217c3fe3cbaad9e7f/content/84f7f698b337de3bfd1bc4a8118cdfd8226bbadf")),
            attributes: Some(serde_json::Value::Array(vec![]))
        };
        assert_eq!(expected_meta, metadata);
        Ok(())
    }
}
