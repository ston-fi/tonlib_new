use crate::cell::TonHash;
use crate::error::TLCoreError;
use std::collections::HashMap;
pub trait Metadata: Sized {
    fn from_data(
        onchain: Option<&HashMap<TonHash, impl AsRef<[u8]>>>,
        offchain: Option<&str>,
    ) -> Result<Self, TLCoreError>;

    fn from_offchain(offchain: &str) -> Result<Self, TLCoreError>;
    fn from_onchain(onchain: &HashMap<TonHash, impl AsRef<[u8]>>) -> Result<Self, TLCoreError>;
}
