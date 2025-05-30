use crate::cell::ton_hash::TonHash;
use crate::errors::TonlibError;
use crate::types::ton_address::TonAddress;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::ffi::CString;
use std::ops::Deref;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct TVMEmulatorC7 {
    pub address: TonAddress,
    pub unix_time: u32,
    pub balance: u64,
    pub rand_seed: TonHash,
    pub config: EmulatorBCConfig,
}

impl TVMEmulatorC7 {
    pub fn new(address: TonAddress, config: EmulatorBCConfig) -> Result<Self, TonlibError> {
        Ok(Self {
            address,
            unix_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32,
            balance: 0,
            rand_seed: TonHash::ZERO,
            config,
        })
    }
}

#[derive(Clone, Debug)]
pub struct EmulatorBCConfig(Arc<CString>);

impl Deref for EmulatorBCConfig {
    type Target = CString;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl From<Arc<CString>> for EmulatorBCConfig {
    fn from(config: Arc<CString>) -> Self { Self(config) }
}

impl EmulatorBCConfig {
    pub fn from_boc(config_boc: &[u8]) -> Result<Self, TonlibError> {
        Self::from_boc_base64(&STANDARD.encode(config_boc))
    }
    pub fn from_boc_hex(config_boc_hex: &str) -> Result<Self, TonlibError> {
        Self::from_boc_base64(&STANDARD.encode(hex::decode(config_boc_hex)?))
    }
    pub fn from_boc_base64(config_boc_base64: &str) -> Result<Self, TonlibError> {
        Ok(Self(Arc::new(CString::new(config_boc_base64)?)))
    }
}
