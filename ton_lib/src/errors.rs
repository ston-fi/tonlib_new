use hex::FromHexError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TonLibError {
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
    #[error("ParserError: Cell is not empty: {bits_left} bits left")]
    ParserCellNotEmpty { bits_left: u32 },

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
    #[error("TLBWrongPrefix: Expecting {exp} bytes, got {given}")]
    TLBWrongPrefix { exp: u128, given: u128 },

    #[error("TonAddressParseError: address={0}, err: {1}")]
    TonAddressParseError(String, String),

    // handling external errors
    #[error("{0}")]
    IO(#[from] std::io::Error),
    #[error("{0}")]
    FromHex(#[from] FromHexError),
    #[error("{0}")]
    B64Error(#[from] base64::DecodeError),
    #[error("{0}")]
    ParseInt(#[from] std::num::ParseIntError),
}
