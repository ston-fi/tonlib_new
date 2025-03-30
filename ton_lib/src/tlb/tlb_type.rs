use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::cell::cell_owned::CellOwned;
use crate::cell::ton_cell::TonCell;
use crate::cell::ton_hash::TonHash;
use crate::errors::TonLibError;

pub trait TLBType: Sized {
    // read-write definition
    // https://docs.ton.org/v3/documentation/data-formats/tlb/tl-b-language#overview
    // must be implemented by all TLB objects
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
    fn from_cell(cell: &dyn TonCell) -> Result<Self, TonLibError> { Self::read(&mut CellParser::new(cell)) }

    // fn from_boc(boc: &[u8]) -> Ton<Self> {
    //     unimplemented!()
    // }
    //
    // fn from_boc_hex(boc_hex: &str) -> TonLibResult<Self> {
    //     unimplemented!()
    // }
    //
    // fn from_boc_b64(boc_b64: &str) -> TonLibResult<Self> {
    //     unimplemented!()
    // }

    /// Serialization
    ///
    fn to_cell(&self) -> Result<CellOwned, TonLibError> {
        let mut builder = CellBuilder::new();
        self.write(&mut builder)?;
        builder.build()
    }

    // fn to_boc(&self, add_crc32: bool) -> Result<Vec<u8>, TonCellError> {
    //     unimplemented!()
    // }
    //
    // fn to_boc_hex(&self, add_crc32: bool) -> Result<String, TonCellError> {
    //     unimplemented!()
    // }
    //
    // fn to_boc_b64(&self, add_crc32: bool) -> Result<String, TonCellError> {
    //     unimplemented!()
    // }

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
