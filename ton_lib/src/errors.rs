use hex::FromHexError;
use num_bigint::{BigInt, BigUint};
use std::env::VarError;
use std::time::Duration;
use thiserror::Error;
use ton_liteapi::tl::request::Request;
use ton_liteapi::types::LiteError;

#[derive(Error, Debug)]
pub enum TonlibError {
    // ton_hash
    #[error("TonHashError: Expecting {exp} bytes, got {given}")]
    TonHashWrongLen { exp: usize, given: usize },

    // cell_parser
    #[error("ParserError: Requested {req} bits, but only {left} left")]
    ParserDataUnderflow { req: usize, left: usize },
    #[error("ParserError: New position is {new_pos}, but data_bits_len is {bits_len}")]
    ParserBadPosition { new_pos: i32, bits_len: usize },
    #[error("ParserError: No ref with index={req}")]
    ParserRefsUnderflow { req: usize },
    #[error("ParserError: Cell is not empty: {bits_left} bits left, {refs_left} refs left")]
    ParserCellNotEmpty { bits_left: usize, refs_left: usize },

    // cell_builder
    #[error("BuilderError: Can't write {req} bits: only {left} free bits available")]
    BuilderDataOverflow { req: usize, left: usize },
    #[error("BuilderError: Can't write ref - 4 refs are written already")]
    BuilderRefsOverflow,
    #[error("BuilderError: Can't extract {required_bits} bits from {given} bytes")]
    BuilderNotEnoughData { required_bits: usize, given: usize },
    #[error("BuilderError: Can't write number {number} as {bits} bits")]
    BuilderNumberBitsMismatch { number: String, bits: usize },
    #[error("BuilderError: Cell validation error: {0}")]
    BuilderMeta(String),

    // boc
    #[error("BOCEmpty: can't parse BOC from empty slice")]
    BOCEmpty,
    #[error("CellType: Unexpected CellType tag: {0}")]
    BOCWrongTypeTag(u8),
    #[error("BOCError: Expected 1 root, got {0}")]
    BOCSingleRoot(usize),
    #[error("BOCError: Unexpected magic: {0}")]
    BOCWrongMagic(u32),
    #[error("BOCError: {0}")]
    BOCCustom(String),

    // tlb
    #[error("TLBWrongPrefix: Expecting {exp} bytes, got {given}, exp_bits={bits_exp}, left_bits={bits_left}")]
    TLBWrongPrefix {
        exp: u128,
        given: u128,
        bits_exp: usize,
        bits_left: usize,
    },
    #[error("TLBEnum: Out of options")]
    TLBEnumOutOfOptions, // TODO collect errors from all options
    #[error("TLBObject: No internal value found (method: {0})")]
    TLBObjectNoValue(String),
    #[error("TLBSnakeFormat: Unsupported bits_len ({0})")]
    TLBSnakeFormatUnsupportedBitsLen(u32),
    #[error("TLBDictWrongKeyLen: Wrong key_bits_len: exp={exp}, got={got} for key={key}")]
    TLBDictWrongKeyLen { exp: usize, got: usize, key: BigUint },
    #[error("TLBDictEmpty: empty dict can't be written")]
    TLBDictEmpty,

    #[error("TonAddressParseError: address={0}, err: {1}")]
    TonAddressParseError(String, String),

    #[error("NetRequestTimeout: {msg}, timeout={timeout:?}")]
    NetRequestTimeout { msg: String, timeout: Duration },

    // LiteClient
    #[error("LiteClientWrongResponse: expected {0}, got {1}")]
    TonLiteClientWrongResponse(String, String),
    #[error("LiteClientLiteError: {0}")]
    LiteClientLiteError(#[from] LiteError),
    #[error("LiteClientConnTimeout: {0:?}")]
    LiteClientConnTimeout(Duration),
    #[error("LiteClientReqTimeout: {0:?}")]
    LiteClientReqTimeout(Box<(Request, Duration)>),

    // TonlibClient
    #[error("TLClientCreationFailed: tonlib_client_json_create returns null")]
    TLClientCreationFailed,
    #[error("TLClientWrongResult: expected type: {0}, got: {1}")]
    TLClientWrongResponse(String, String),
    #[error("TLInvalidArguments: {0}")]
    TLInvalidArgs(String),
    #[error("TLSendError: fail to send request: {0}")]
    TLSendError(String),
    #[error("TLInvalidResponse: method: {method}, code: {code}, message: {message}")]
    TLExecError { method: String, code: i32, message: String },
    #[error("TLWrongExecImplUsage: {0}")]
    TLWrongUsage(String),

    // TVM
    #[error("TVMEmulatorCreationFailed: tvm_emulator_create returns null")]
    TVMEmulatorCreationFailed,
    #[error("TVMEmulatorSetParamFailed: fail to set param: {0}")]
    TVMEmulatorSetFailed(&'static str),
    #[error("TVMEmulatorError: {0}")]
    TVMEmulatorError(String),
    #[error("TVMEmulatorResponseParseError: {0}")]
    TVMEmulatorResponseParseError(String),
    #[error("TVMRunMethodError: vm_exit_code: {vm_exit_code:?}, response_raw: {response_raw}")]
    TVMRunMethodError {
        vm_exit_code: Option<i32>,
        response_raw: String,
    },

    // TVMStack
    #[error("TVMStackError: fail to pop specified type. expected: {0}, got: {1}")]
    TVMStackWrongType(String, String),
    #[error("TVMStackError: stack is empty")]
    TVMStackEmpty,

    // TonActiveContract
    #[error("TonContractNotActive: caching is not active")]
    TonContractNotActive,
    #[error("TonContractUnexpectedValue: expected: {expected}, actual: {actual}")]
    TonContractUnexpectedValue { expected: String, actual: String },
    #[error("CustomError: {0}")]
    CustomError(String),
    #[error("UnexpectedError: {0}")]
    UnexpectedError(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),
    // handling external errors
    #[error("{0}")]
    IO(#[from] std::io::Error),
    #[error("{0}")]
    FromHex(#[from] FromHexError),
    #[error("{0}")]
    B64Error(#[from] base64::DecodeError),
    #[error("{0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("{0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),
    #[error("{0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("{0}")]
    NulError(#[from] std::ffi::NulError),
    #[error("{0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("{0}")]
    ElapsedError(#[from] tokio::time::error::Elapsed),
    #[error("{0}")]
    AdnlError(#[from] adnl::AdnlError),
    #[error("{0}")]
    ParseBigIntError(#[from] num_bigint::ParseBigIntError),
    #[error("{0}")]
    RecvError(#[from] tokio::sync::oneshot::error::RecvError),
    #[error("{0}")]
    VarError(#[from] VarError),
}

impl<T> From<TonlibError> for Result<T, TonlibError> {
    fn from(val: TonlibError) -> Self { Err(val) }
}
