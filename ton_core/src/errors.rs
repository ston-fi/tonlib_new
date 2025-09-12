use hex::FromHexError;
use std::env::VarError;
use std::sync::Arc;
use thiserror::Error;

#[macro_export]
macro_rules! bail_ton_core {
    ($($arg:tt)*) => {
        return Err(TonCoreError::Custom(format!($($arg)*)))
    };
}

#[derive(Error, Debug)]
pub enum TonCoreError {
    #[error("DataError: [{0}] {1}")]
    DataError(String, String),

    // tlb
    #[error("TLBWrongData: {0}")]
    TLBWrongData(String),
    #[error("TLBWrongPrefix: expected={exp}, given={given}, exp_bits={bits_exp}, left_bits={bits_left}")]
    TLBWrongPrefix {
        exp: usize,
        given: usize,
        bits_exp: usize,
        bits_left: usize,
    },
    #[error("TLBEnumOutOfOptions: data doesn't match any variant of {0}")]
    TLBEnumOutOfOptions(String),
    #[error("TLBObjectNoValue: No internal value found (method: {0})")]
    TLBObjectNoValue(String),

    // contracts
    #[error("ContractError: {0}")]
    ContractError(String),

    // General errors
    #[error("Custom: {0}")]
    Custom(String),
    #[error("UnexpectedValue: expected: {expected}, actual: {actual}")]
    UnexpectedValue { expected: String, actual: String },

    // handling external errors
    #[error("{0}")]
    IO(#[from] std::io::Error),
    #[error("{0}")]
    FromHex(#[from] FromHexError),
    #[error("{0}")]
    Base64Error(#[from] base64::DecodeError),
    #[error("{0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("{0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),
    #[error("{0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("{0}")]
    NulError(#[from] std::ffi::NulError),
    #[error("{0}")]
    SystemTimeError(#[from] std::time::SystemTimeError),

    #[error("{0}")]
    ParseBigIntError(#[from] num_bigint::ParseBigIntError),
    #[error("{0}")]
    VarError(#[from] VarError),
    #[error("{0}")]
    BoxedError(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("{0}")]
    ArcError(#[from] Arc<dyn std::error::Error + Send + Sync>),
    #[error("{0}")]
    ArcSelf(#[from] Arc<TonCoreError>),
}

impl TonCoreError {
    pub fn data<P: Into<String>, M: Into<String>>(producer: P, msg: M) -> Self {
        Self::DataError(producer.into(), msg.into())
    }
}
