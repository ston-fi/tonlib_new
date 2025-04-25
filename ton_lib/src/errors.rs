use hex::FromHexError;
use num_bigint::BigUint;
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
    ParserDataUnderflow { req: u32, left: u32 },
    #[error("ParserError: New position is {new_pos}, but data_bits_len is {bits_len}")]
    ParserBadPosition { new_pos: i32, bits_len: u32 },
    #[error("ParserError: No ref with index={req}")]
    ParserRefsUnderflow { req: usize },
    #[error("ParserError: Cell is not empty: {bits_left} bits left, {refs_left} refs left")]
    ParserCellNotEmpty { bits_left: u32, refs_left: usize },

    // cell_builder
    #[error("BuilderError: Can't write {req} bits: only {left} free bits available")]
    BuilderDataOverflow { req: u32, left: u32 },
    #[error("BuilderError: Can't write ref - 4 refs are written already")]
    BuilderRefsOverflow,
    #[error("BuilderError: Can't extract {required_bits} bits from {given} bytes")]
    BuilderNotEnoughData { required_bits: u32, given: u32 },
    #[error("BuilderError: Can't write number {number} as {bits} bits")]
    BuilderNumberBitsMismatch { number: String, bits: u32 },
    #[error("BuilderError: Cell validation error: {0}")]
    BuilderMeta(String),

    // boc
    #[error("CellType: Unexpected CellType tag: {0}")]
    CellTypeTag(u8),
    #[error("BOCError: Expected 1 root, got {0}")]
    BocSingleRoot(usize),
    #[error("BOCError: Unexpected magic: {0}")]
    BocWrongMagic(u32),
    #[error("BOCError: {0}")]
    BocCustom(String),

    // tlb
    #[error("TLBWrongPrefix: Expecting {exp} bytes, got {given}, exp_bits={bits_exp}, left_bits={bits_left}")]
    TLBWrongPrefix {
        exp: u128,
        given: u128,
        bits_exp: u32,
        bits_left: u32,
    },
    #[error("TLBEnum: Out of options")]
    TLBEnumOutOfOptions, // TODO collect errors from all options
    #[error("TLBObject: No internal value found (method: {0})")]
    TLBObjectNoValue(String),
    #[error("TLBSnakeFormat: Unsupported bits_len ({0})")]
    TLBSnakeFormatUnsupportedBitsLen(u32),
    #[error("TLBDictWrongKeyLen: Wrong key_bits_len: exp={exp}, got={got} for key={key}")]
    TLBDictWrongKeyLen { exp: usize, got: usize, key: BigUint },

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
    #[error("TLJClientWrongResult: expected type: {0}, got: {1}")]
    TLJClientWrongResponse(String, String),
    #[error("TLJInvalidArguments: {0}")]
    TLJInvalidArgs(String),
    #[error("TLJSendError: fail to send request: {0}")]
    TLJSendError(String),
    #[error("TLJInvalidResponse: method: {method}, code: {code}, message: {message}")]
    TLJExecError { method: String, code: i32, message: String },
    #[error("TLJWrongExecImplUsage: {0}")]
    TLJWrongUsage(String),

    // TVM
    #[error("TvmStackError: {0}")]
    TvmStackError(String),

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
}

impl<T> From<TonlibError> for Result<T, TonlibError> {
    fn from(val: TonlibError) -> Self { Err(val) }
}
