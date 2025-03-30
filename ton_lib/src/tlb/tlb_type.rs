use crate::boc::boc::BOC;
use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::cell::ton_cell::TonCell;
use crate::cell::ton_hash::TonHash;
use crate::errors::TonLibError;
use std::ops::Deref;

pub trait TLBType: Sized {
    // read-write definition
    // https://docs.ton.org/v3/documentation/data-formats/tlb/tl-b-language#overview
    // must be implemented by all TLB objects
    // doesn't include prefix handling
    fn read_def(parser: &mut CellParser) -> Result<Self, TonLibError>;
    fn write_def(&self, builder: &mut CellBuilder) -> Result<(), TonLibError>;

    // interface
    fn read(parser: &mut CellParser) -> Result<Self, TonLibError> {
        Self::verify_prefix(parser)?;
        Self::read_def(parser)
    }

    fn write(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> {
        Self::write_prefix(builder)?;
        self.write_def(builder)
    }

    fn prefix() -> &'static TLBPrefix { &TLBPrefix::NULL }

    // Utilities
    fn cell_hash(&self) -> Result<TonHash, TonLibError> { Ok(self.to_cell()?.hash().clone()) }

    /// Parsing
    ///
    fn from_cell(cell: &TonCell) -> Result<Self, TonLibError> { Self::read(&mut CellParser::new(cell)) }

    fn from_boc(boc: &[u8]) -> Result<Self, TonLibError> {
        Self::from_cell(BOC::from_bytes(boc)?.single_root()?.deref())
    }

    fn from_boc_hex(boc_hex: &str) -> Result<Self, TonLibError> {
        Self::from_cell(BOC::from_hex(boc_hex)?.single_root()?.deref())
    }

    /// Serialization
    ///
    fn to_cell(&self) -> Result<TonCell, TonLibError> {
        let mut builder = CellBuilder::new();
        self.write(&mut builder)?;
        builder.build()
    }

    fn to_boc(&self, add_crc32: bool) -> Result<Vec<u8>, TonLibError> {
        let mut builder = CellBuilder::new();
        self.write(&mut builder)?;
        BOC::new(builder.build()?).to_bytes(add_crc32)
    }

    fn to_boc_hex(&self, add_crc32: bool) -> Result<String, TonLibError> { Ok(hex::encode(self.to_boc(add_crc32)?)) }

    /// Helpers - for internal use
    ///
    fn verify_prefix(reader: &mut CellParser) -> Result<(), TonLibError> {
        let expected_prefix = Self::prefix();
        if expected_prefix == &TLBPrefix::NULL {
            return Ok(());
        }
        let actual_value = reader.read_num(expected_prefix.bit_len)?;
        if actual_value != expected_prefix.value {
            return Err(TonLibError::TLBWrongPrefix {
                exp: expected_prefix.value,
                given: actual_value,
            });
        }
        Ok(())
    }

    fn write_prefix(builder: &mut CellBuilder) -> Result<(), TonLibError> {
        let prefix = Self::prefix();
        if prefix != &TLBPrefix::NULL {
            builder.write_num(prefix.value, prefix.bit_len)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TLBPrefix {
    pub value: u128,
    pub bit_len: u32,
}

impl TLBPrefix {
    pub const NULL: TLBPrefix = TLBPrefix { bit_len: 0, value: 0 };
    pub const fn new(value: u128, bit_len: u32) -> Self { Self { bit_len, value } }
}
