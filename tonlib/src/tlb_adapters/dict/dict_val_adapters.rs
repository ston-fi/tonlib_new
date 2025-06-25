use ton_lib_core::cell::CellBuilder;
use ton_lib_core::cell::CellParser;
use ton_lib_core::cell::TonCellNum;
use ton_lib_core::error::TLCoreError;
use ton_lib_core::traits::tlb::TLB;

pub trait DictValAdapter<T> {
    fn write(builder: &mut CellBuilder, val: &T) -> Result<(), TLCoreError>;
    fn read(parser: &mut CellParser) -> Result<T, TLCoreError>;
}

pub struct DictValAdapterTLB;
pub struct DictValAdapterTLBRef;
pub struct DictValAdapterNum<const BITS_LEN: usize>;

impl<T: TLB> DictValAdapter<T> for DictValAdapterTLB {
    fn write(builder: &mut CellBuilder, val: &T) -> Result<(), TLCoreError> { val.write(builder) }
    fn read(parser: &mut CellParser) -> Result<T, TLCoreError> { T::read(parser) }
}

impl<T: TLB> DictValAdapter<T> for DictValAdapterTLBRef {
    fn write(builder: &mut CellBuilder, val: &T) -> Result<(), TLCoreError> { builder.write_ref(val.to_cell_ref()?) }
    fn read(parser: &mut CellParser) -> Result<T, TLCoreError> {
        let out_msg_cell = parser.read_next_ref()?;
        T::from_cell(out_msg_cell)
    }
}

impl<T: TonCellNum, const BITS_LEN: usize> DictValAdapter<T> for DictValAdapterNum<BITS_LEN> {
    fn write(builder: &mut CellBuilder, val: &T) -> Result<(), TLCoreError> { builder.write_num(val, BITS_LEN) }
    fn read(parser: &mut CellParser) -> Result<T, TLCoreError> { parser.read_num(BITS_LEN) }
}
