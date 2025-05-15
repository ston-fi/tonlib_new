use crate::cell::ton_hash::TonHash;
use crate::errors::TonlibError;
use crate::types::ton_address::TonAddress;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::ffi::CString;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct TVMEmulatorC7 {
    pub address: CString,
    pub unix_time: u32,
    pub balance: u64,
    pub rand_seed: TonHash,
    pub config: CString,
}

impl TVMEmulatorC7 {
    pub fn new(address: TonAddress, config_boc: &[u8]) -> Result<Self, TonlibError> {
        Ok(Self {
            address: CString::new(address.to_hex().as_bytes())?,
            unix_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32,
            balance: 0,
            rand_seed: TonHash::ZERO,
            config: CString::new(STANDARD.encode(&config_boc))?,
        })
    }
}
