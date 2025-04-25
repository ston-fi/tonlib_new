use std::ffi::NulError;
use std::str::Utf8Error;

use crate::clients::tonlibjson::tl_api::stack::TLTvmStackEntry;
use crate::errors::TonlibError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TvmStackError {
    #[error("Unsupported conversion to string (TvmStackEntry: {e:?}, index: {index})")]
    StringConversion { e: TLTvmStackEntry, index: usize },

    #[error("Unsupported conversion to i32 (TvmStackEntry: {e:?}, index: {index})")]
    I32Conversion { e: TLTvmStackEntry, index: usize },

    #[error("Unsupported conversion to i64 (TvmStackEntry: {e:?}, index: {index})")]
    I64Conversion { e: TLTvmStackEntry, index: usize },

    #[error("Unsupported conversion to BigUint (TvmStackEntry: {e:?}, index: {index})")]
    BigUintConversion { e: TLTvmStackEntry, index: usize },

    #[error("Unsupported conversion to BigInt (TvmStackEntry: {e:?}, index: {index})")]
    BigIntConversion { e: TLTvmStackEntry, index: usize },

    #[error("Unsupported conversion to BagOfCells (TvmStackEntry: {e:?}, index: {index})")]
    BoCConversion { e: TLTvmStackEntry, index: usize },

    #[error("Invalid tvm stack index ( Index: {index}, total length {len})")]
    InvalidTvmStackIndex { index: usize, len: usize },

    #[error("TonCellError ({0})")]
    TonLibError(#[from] TonlibError),
}

#[derive(Error, Debug)]
pub enum TlError {
    #[error("Utf8 Error ({0})")]
    Utf8Error(#[from] Utf8Error),

    #[error("Serde_json Error ({0})")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("CString is null ({0})")]
    NulError(#[from] NulError),
}
