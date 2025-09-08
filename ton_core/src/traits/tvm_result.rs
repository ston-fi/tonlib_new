use crate::error::TLCoreError;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;

pub trait TVMResult: Sized {
    fn from_boc(boc: &[u8]) -> Result<Self, TLCoreError>;
    fn from_boc_hex(boc: &str) -> Result<Self, TLCoreError> { Self::from_boc(&hex::decode(boc)?) }
    fn from_boc_b64(boc: &str) -> Result<Self, TLCoreError> { Self::from_boc(&BASE64_STANDARD.decode(boc)?) }
}
