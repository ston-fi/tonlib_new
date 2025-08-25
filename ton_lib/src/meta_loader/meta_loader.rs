use reqwest::header;
use reqwest::header::HeaderValue;
use reqwest::Client;
use reqwest::StatusCode;
use std::num::ParseIntError;
use thiserror::Error;
use ton_lib_core::error::TLCoreError;
use ton_lib_core::traits::metadata::Metadata;

use crate::meta_loader::IpfsLoader;
use crate::meta_loader::IpfsLoaderError;
use crate::tep::metadata::metadata_content::MetadataContent;
use crate::tep::metadata::metadata_content::MetadataExternal;
use crate::tep::metadata::metadata_content::MetadataInternal;
use crate::tep::metadata::metadata_fields::META_URI;

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
        let http_client = match self.http_client {
            Some(client) => client,
            None => {
                let mut headers = header::HeaderMap::new();
                headers.insert("user-agent", HeaderValue::from_static("TonlibMetaLoader/0.x"));
                headers.insert("accept", HeaderValue::from_static("*/*"));
                Client::builder().default_headers(headers).build().unwrap()
            }
        };

        let ipfs_loader = match self.ipfs_loader {
            Some(ipfs_loader) => ipfs_loader,
            None => IpfsLoader::builder().with_client(http_client.clone()).build(),
        };

        MetaLoader {
            http_client,
            ipfs_loader,
        }
    }
}

impl MetaLoader {
    pub fn builder() -> MetaLoaderBuilder {
        MetaLoaderBuilder::new()
    }

    pub async fn load_external_meta(&self, uri: &str) -> Result<String, MetaLoaderError> {
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

    pub async fn load<T: Metadata>(&self, content: &MetadataContent) -> Result<T, MetaLoaderError> {
        match content {
            MetadataContent::External(MetadataExternal { uri }) => {
                let json = self.load_external_meta(&uri.as_str()).await?;
                Ok(T::from_json(&json)?)
            }
            MetadataContent::Internal(MetadataInternal { data: dict }) => {
                if dict.contains_key(&*META_URI) {
                    let uri = String::from_utf8_lossy(dict.get(&*META_URI).unwrap().as_slice()).to_string();
                    match self.load_external_meta(uri.as_str()).await {
                        Ok(json) => Ok(T::from_data(&dict, Some(&json))?),
                        Err(_) => Ok(T::from_dict(&dict)?),
                    }
                } else {
                    Ok(T::from_dict(&dict)?)
                }
            }
            content => Err(MetaLoaderError::ContentLayoutUnsupported(content.clone())),
        }
    }
}
