use ton_lib_core::{
    cell::{CellBuilder, CellParser},
    error::TLCoreError,
    traits::tlb::{TLBPrefix, TLB},
    TLBDerive,
};

#[derive(Debug, Clone, PartialEq)]

pub enum SnakeData {
    Head(Box<SnakeDataCons>),
    Tail(SnakeDataTail),
}
#[derive(Debug, Clone, PartialEq)]
pub struct SnakeDataTail {
    bn: usize,
    b: Vec<u8>,
}

impl TLB for SnakeDataTail {
    const PREFIX: TLBPrefix = TLBPrefix { value: 1, bits_len: 8 };
    fn read_definition(parser: &mut CellParser) -> Result<Self, TLCoreError> {
        let bn = TLB::read(parser)?;
        let b = parser.read_bits(bn)?;
        Ok(Self { bn, b })
    }

    fn write_definition(&self, builder: &mut CellBuilder) -> Result<(), TLCoreError> { todo!() }
}

#[derive(Debug, Clone, PartialEq, TLBDerive)]
pub struct SnakeDataCons {
    data: SnakeDataTail,
    next: Box<SnakeData>,
}

impl TLB for SnakeData {
    fn read_definition(parser: &mut CellParser) -> Result<Self, TLCoreError> {
        let ref_count = parser.next_ref_pos;
        if ref_count == 0 {
            let tail = TLB::read(parser)?;
            Ok(Self::Tail(tail))
        } else if ref_count == 1 {
            let data = TLB::read(parser)?;
            let next = TLB::read(&mut parser.read_next_ref()?.parser())?;
            Ok(Self::Head(Box::new(SnakeDataCons { data, next })))
        } else {
            Err(TLCoreError::TLBWrongData("Snake format data can not have more than 1 reference".to_string()))
        }
    }

    fn write_definition(&self, builder: &mut CellBuilder) -> Result<(), TLCoreError> { todo!() }
}

/// tail#_ {bn:#} b:(bits bn) = SnakeData ~0;
/// cons#_ {bn:#} {n:#} b:(bits bn) next:^(SnakeData ~n) = SnakeData ~(n + 1);

#[cfg(test)]
mod tests {
    

    use ton_lib_core::cell::TonCell;

    use super::*;

    #[test]
    fn test_meta_snake_format_in_ref_cell() -> anyhow::Result<()> {
        let cell = TonCell::from_boc_hex("b5ee9c7201010201003d0001020101006e68747470733a2f2f676966746966792d6170702e70616c657474652e66696e616e63652f746f79626561722d6a6574746f6e2e6a736f6e")?;

        let parser = &mut cell.parser();
        let a: SnakeData = TLB::read(parser)?;
        println!("{a:?}");

        Ok(())
    }
}
