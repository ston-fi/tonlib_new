use std::collections::HashMap;

use ton_lib_core::{cell::TonHash, error::TLCoreError};

use crate::tep::metadata::snake_data::SnakeData;
pub trait Metadata: Sized {
    fn from_data(dict: &HashMap<TonHash, SnakeData>, json: Option<&str>) -> Result<Self, TLCoreError>;

    fn from_json(json: &str) -> Result<Self, TLCoreError> {
        Self::from_data(&HashMap::<TonHash, SnakeData>::new(), Some(json))
    }
    fn from_dict(dict: &HashMap<TonHash, SnakeData>) -> Result<Self, TLCoreError> { Self::from_data(dict, None) }
}
