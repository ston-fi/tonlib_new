use std::num::ParseIntError;

use crate::tep::meta::ipfs_loader::IpfsLoader;
use crate::tep::meta::metadata_field::META_URI;
use crate::tep::meta::MetadataContent;
use crate::tep::meta::*;
use crate::tep::IpfsLoaderConfig;
use crate::tep::IpfsLoaderError;
use reqwest::header;
use reqwest::header::HeaderValue;
use reqwest::Client;
use reqwest::StatusCode;
use thiserror::Error;
use ton_lib_core::error::TLCoreError;
use ton_lib_core::traits::metadata::Metadata;

#[derive(Debug, Error)]
pub enum MetaLoaderError {
    #[error("Unsupported content layout (Metadata content: {0:?})")]
    ContentLayoutUnsupported(MetadataContent),

    #[error("Failed to load jetton metadata (URI: {uri}, response status code: {status})")]
    LoadMetadataFailed { uri: String, status: StatusCode },

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

    #[error(transparent)]
    MetadataParseError(#[from] TLCoreError),
}

pub struct MetaLoader {
    http_client: reqwest::Client,
    ipfs_loader: IpfsLoader,
}

impl Default for MetaLoader {
    fn default() -> Self {
        MetaLoaderBuilder::new().build()
    }
}

pub struct MetaLoaderBuilder {
    http_client: Option<reqwest::Client>,
    ipfs_loader: Option<IpfsLoader>,
}

impl MetaLoaderBuilder {
    fn new() -> Self {
        MetaLoaderBuilder {
            http_client: None,
            ipfs_loader: None,
        }
    }

    pub fn with_http_client(mut self, http_client: reqwest::Client) -> Self {
        self.http_client = Some(http_client);
        self
    }
    pub fn with_ipfs_loader(mut self, ipfs_loader: IpfsLoader) -> Self {
        self.ipfs_loader = Some(ipfs_loader);
        self
    }

    pub fn build(self) -> MetaLoader {
        let mut headers = header::HeaderMap::new();
        headers.insert("user-agent", HeaderValue::from_static("TonlibMetaLoader/0.x"));
        headers.insert("accept", HeaderValue::from_static("*/*"));

        MetaLoader {
            http_client: self.http_client.or(Client::builder().default_headers(headers).build().ok()).unwrap(),
            ipfs_loader: self.ipfs_loader.or(IpfsLoader::new(&IpfsLoaderConfig::default()).ok()).unwrap(),
        }
    }
}

impl MetaLoader {
    pub fn builder() -> MetaLoaderBuilder {
        MetaLoaderBuilder::new()
    }

    pub async fn load_json_meta_from_uri(&self, uri: &str) -> Result<String, MetaLoaderError> {
        log::trace!("Downloading metadata from {}", uri);
        let meta_str: String = if uri.starts_with("ipfs://") {
            let path: String = uri.chars().skip(7).collect();
            self.ipfs_loader.load_utf8_lossy(path.as_str()).await?
        } else {
            let resp = self.http_client.get(uri).send().await?;
            if resp.status().is_success() {
                resp.text().await?
            } else {
                return Err(MetaLoaderError::LoadMetadataFailed {
                    uri: uri.to_string(),
                    status: resp.status(),
                });
            }
        };

        Ok(meta_str)
    }
}

// ------------------------------------JETTON META DATA----------------------------------------
impl MetaLoader {
    pub async fn load<T: Metadata>(&self, content: &MetadataContent) -> Result<T, MetaLoaderError> {
        match content {
            MetadataContent::External(MetadataExternal { uri }) => {
                let json = self.load_json_meta_from_uri(&uri.as_str()).await?;
                Ok(T::from_offchain(&json)?)
            }
            MetadataContent::Internal(MetadataInternal { data: dict }) => {
                if dict.contains_key(&*META_URI) {
                    let uri = String::from_utf8_lossy(dict.get(&*META_URI).unwrap().as_slice()).to_string();
                    match self.load_json_meta_from_uri(uri.as_str()).await {
                        Ok(json) => Ok(T::from_data(Some(&dict), Some(&json))?),
                        Err(_) => Ok(T::from_onchain(&dict)?),
                    }
                } else {
                    Ok(T::from_onchain(&dict)?)
                }
            }
            content => Err(MetaLoaderError::ContentLayoutUnsupported(content.clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use crate::tep::meta::meta_loader::{MetaLoader, NftItemMetadata};

    #[tokio::test]
    async fn test_meta_data_load_ordinal_https() -> anyhow::Result<()> {
        let loader = MetaLoader::default();

        let metadata = loader
            .load_json_meta_from_uri("https://s.getgems.io/nft/b/c/62fba50217c3fe3cbaad9e7f/95/meta.json")
            .await?;

        let expected_meta = NftItemMetadata {
            name: Some(String::from("TON Smart Challenge #2 Winners Trophy")),
            description: Some(String::from("TON Smart Challenge #2 Winners Trophy 93 place out of 181")),
            image: Some(String::from("https://s.getgems.io/nft/b/c/62fba50217c3fe3cbaad9e7f/images/943e994f91227c3fdbccbc6d8635bfaab256fbb4")),
            content_url: Some(String::from("https://s.getgems.io/nft/b/c/62fba50217c3fe3cbaad9e7f/content/84f7f698b337de3bfd1bc4a8118cdfd8226bbadf")),
            attributes: Some(serde_json::Value::Array(vec![]))
        };
        assert_eq!(expected_meta, serde_json::from_str(&metadata)?);
        Ok(())
    }
}
