use crate::cell::TonHash;
use crate::error::TLCoreError;
use std::collections::HashMap;
pub trait Metadata: Sized {
    fn from_data(dict: Option<&HashMap<TonHash, impl AsRef<[u8]>>>, json: Option<&str>) -> Result<Self, TLCoreError>;

    fn from_json(json: &str) -> Result<Self, TLCoreError>;
    fn from_dict(dict: &HashMap<TonHash, impl AsRef<[u8]>>) -> Result<Self, TLCoreError>;
}
