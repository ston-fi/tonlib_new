use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::cell::ton_cell_num::TonCellNum;
use crate::errors::TonlibError;
use crate::types::tlb::tlb_type::TLBType;

pub trait DictValAdapter<T> {
    fn write(builder: &mut CellBuilder, val: &T) -> Result<(), TonlibError>;
    fn read(parser: &mut CellParser) -> Result<T, TonlibError>;
}

pub struct DictValAdapterTLB;
pub struct DictValAdapterTLBRef;
pub struct DictValAdapterNum<const BITS_LEN: usize>;

impl<T: TLBType> DictValAdapter<T> for DictValAdapterTLB {
    fn write(builder: &mut CellBuilder, val: &T) -> Result<(), TonlibError> { val.write(builder) }
    fn read(parser: &mut CellParser) -> Result<T, TonlibError> { T::read(parser) }
}

impl<T: TLBType> DictValAdapter<T> for DictValAdapterTLBRef {
    fn write(builder: &mut CellBuilder, val: &T) -> Result<(), TonlibError> { builder.write_ref(val.to_cell_ref()?) }
    fn read(parser: &mut CellParser) -> Result<T, TonlibError> {
        let out_msg_cell = parser.read_next_ref()?;
        T::from_cell(out_msg_cell)
    }
}

impl<T: TonCellNum, const BITS_LEN: usize> DictValAdapter<T> for DictValAdapterNum<BITS_LEN> {
    fn write(builder: &mut CellBuilder, val: &T) -> Result<(), TonlibError> { builder.write_num(val, BITS_LEN) }
    fn read(parser: &mut CellParser) -> Result<T, TonlibError> { parser.read_num(BITS_LEN) }
}
