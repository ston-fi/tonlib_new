use crate::cell::TonHash;
use crate::error::TLCoreError;
use std::collections::HashMap;
use std::fmt::Debug;
pub trait Metadata: Sized {
    fn from_data(dict: &HashMap<TonHash, impl AsRef<[u8]>>, json: Option<&str>) -> Result<Self, TLCoreError>;

    fn from_json(json: &str) -> Result<Self, TLCoreError> {
        Self::from_data(&HashMap::<TonHash, Vec<u8>>::new(), Some(json))
    }
    fn from_dict(dict: &HashMap<TonHash, impl AsRef<[u8]>>) -> Result<Self, TLCoreError> { Self::from_data(dict, None) }
}
