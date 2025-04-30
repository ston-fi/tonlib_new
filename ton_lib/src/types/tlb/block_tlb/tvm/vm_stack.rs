use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::cell::ton_cell::{TonCell, TonCellRef};
use crate::errors::TonlibError;
use crate::types::tlb::block_tlb::tvm::{VMCell, VMCellSlice, VMInt, VMStackValue, VMTinyInt};
use crate::types::tlb::tlb_type::TLBType;
use num_bigint::BigInt;
use std::ops::{Deref, DerefMut};

macro_rules! extract_stack_val {
    ($maybe_result:expr, $variant:ident) => {
        match $maybe_result {
            None => Err(TonlibError::TVMStackEmpty),
            Some(VMStackValue::$variant(val)) => Ok(val.value),
            Some(rest) => Err(TonlibError::TVMStackWrongType(stringify!($variant).to_string(), format!("{rest:?}"))),
        }
    };
}

// https://github.com/ton-blockchain/ton/blob/ed4682066978f69ffa38dd98912ca77d4f660f66/crypto/block/block.tlb#L864
// Doesn't implement tlb schema directly for convenience purposes
#[derive(Debug, Clone, Default)]
pub struct VMStack(Vec<VMStackValue>);

impl Deref for VMStack {
    type Target = Vec<VMStackValue>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for VMStack {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl VMStack {
    pub fn new(items: Vec<VMStackValue>) -> Self { Self(items) }

    pub fn push_tiny_int(&mut self, value: i64) { self.push(VMStackValue::TinyInt(VMTinyInt { value })); }
    pub fn push_int(&mut self, value: BigInt) { self.push(VMStackValue::Int(VMInt { value })); }
    pub fn push_cell(&mut self, value: TonCellRef) { self.push(VMStackValue::Cell(VMCell { value })); }
    pub fn push_cell_slice(&mut self, cell: TonCellRef) {
        self.push(VMStackValue::CellSlice(VMCellSlice::from_cell(cell)));
    }

    pub fn pop_tiny_int(&mut self) -> Result<i64, TonlibError> { extract_stack_val!(self.pop(), TinyInt) }
    pub fn pop_int(&mut self) -> Result<BigInt, TonlibError> { extract_stack_val!(self.pop(), Int) }
    pub fn pop_cell(&mut self) -> Result<TonCellRef, TonlibError> { extract_stack_val!(self.pop(), Cell) }
    pub fn pop_cell_slice(&mut self) -> Result<TonCellRef, TonlibError> { extract_stack_val!(self.pop(), CellSlice) }
}

impl TLBType for VMStack {
    fn read_definition(parser: &mut CellParser) -> Result<Self, TonlibError> {
        let depth: u32 = parser.read_num(24)?;
        if depth == 0 {
            return Ok(Self::default());
        }

        let mut vm_stack = Self::default();
        let mut rest = parser.read_next_ref()?.clone();
        vm_stack.push(TLBType::read(parser)?);
        for _ in 1..depth {
            let mut rest_parser = CellParser::new(&rest);
            let new_rest = rest_parser.read_next_ref()?.clone(); // read "rest" first
            vm_stack.push(TLBType::read(&mut rest_parser)?); // then read "item" itself
            rest = new_rest;
        }
        vm_stack.reverse();
        Ok(vm_stack)
    }

    fn write_definition(&self, builder: &mut CellBuilder) -> Result<(), TonlibError> {
        builder.write_num(&self.0.len(), 24)?;
        if self.0.is_empty() {
            return Ok(());
        }
        let mut cur_rest = TonCell::EMPTY;
        // we fill cell chain from the end
        for item in self.0.iter() {
            let mut rest_builder = CellBuilder::new();
            rest_builder.write_ref(cur_rest.into_ref())?; // write "rest" first
            item.write(&mut rest_builder)?; // then write "item" itself
            cur_rest = rest_builder.build()?
        }
        builder.write_cell(&cur_rest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::ton_cell::TonCell;
    use crate::types::tlb::tlb_type::TLBType;
    use crate::types::ton_address::TonAddress;

    use tokio_test::assert_ok;

    #[test]
    fn test_vm_stack_empty() -> anyhow::Result<()> {
        let stack = VMStack::default();
        let cell = stack.to_cell()?;
        let mut parser = CellParser::new(&cell);
        let depth: u32 = parser.read_num(24)?;
        assert_eq!(depth, 0);
        assert_ok!(parser.ensure_empty());

        let stack_parsed = VMStack::from_cell(&cell)?;
        assert!(stack_parsed.is_empty());
        Ok(())
    }

    #[test]
    fn test_vm_stack_tiny_int() -> anyhow::Result<()> {
        let mut stack = VMStack::new(vec![]);
        stack.push_tiny_int(1);
        let stack_cell = stack.to_cell()?;

        let mut parser = CellParser::new(&stack_cell);
        let depth: u32 = parser.read_num(24)?;
        assert_eq!(depth, 1);
        assert_eq!(parser.read_next_ref()?.deref(), &TonCell::EMPTY);

        match VMStackValue::read(&mut parser)? {
            VMStackValue::TinyInt(val) => assert_eq!(val.value, 1),
            _ => panic!("Expected TinyInt"),
        }

        let mut stack_parsed = VMStack::from_cell(&stack_cell)?;
        assert_eq!(stack_parsed.len(), 1);
        assert_eq!(stack_parsed.pop_tiny_int()?, 1);
        Ok(())
    }

    #[test]
    fn test_vm_stack_cell_slice() -> anyhow::Result<()> {
        let mut stack = VMStack::new(vec![]);
        stack.push_cell_slice(TonAddress::ZERO.to_cell_ref()?);
        let stack_cell = stack.to_cell()?;

        let mut parser = CellParser::new(&stack_cell);
        let depth: u32 = parser.read_num(24)?;
        assert_eq!(depth, 1);
        assert_eq!(parser.read_next_ref()?.deref(), &TonCell::EMPTY);

        match VMStackValue::read(&mut parser)? {
            VMStackValue::CellSlice(val) => assert_eq!(val.value.deref(), &TonAddress::ZERO.to_cell()?),
            _ => panic!("Expected CellSlice"),
        }

        let mut stack_parsed = VMStack::from_cell(&stack_cell)?;
        assert_eq!(stack_parsed.len(), 1);
        assert_eq!(stack_parsed.pop_cell_slice()?.deref(), &TonAddress::ZERO.to_cell()?);
        Ok(())
    }

    #[test]
    fn test_vm_stack_deep_3() -> anyhow::Result<()> {
        // TODO update test to cover all VMValue types instead
        let mut stack = VMStack::new(vec![]);
        stack.push_tiny_int(1);
        stack.push_int(2.into());
        stack.push_cell_slice(TonAddress::ZERO.to_cell_ref()?);
        let stack_cell = stack.to_cell()?;

        let mut deep1_parser = CellParser::new(&stack_cell);
        let depth: u32 = deep1_parser.read_num(24)?;
        assert_eq!(depth, 3);

        let rest1 = deep1_parser.read_next_ref()?.clone();
        match VMStackValue::read(&mut deep1_parser)? {
            VMStackValue::CellSlice(val) => assert_eq!(val.value.deref(), &TonAddress::ZERO.to_cell()?),
            _ => panic!("Expected CellSlice"),
        }

        let mut deep2_parser = CellParser::new(&rest1);
        let rest2 = deep2_parser.read_next_ref()?.clone();
        match VMStackValue::read(&mut deep2_parser)? {
            VMStackValue::Int(val) => assert_eq!(val.value, 2.into()),
            _ => panic!("Expected Int"),
        }

        let mut deep3_parser = CellParser::new(&rest2);
        let rest3 = deep3_parser.read_next_ref()?.clone();
        match VMStackValue::read(&mut deep3_parser)? {
            VMStackValue::TinyInt(val) => assert_eq!(val.value, 1),
            _ => panic!("Expected TinyInt"),
        }

        assert_eq!(rest3.deref(), &TonCell::EMPTY);

        let mut stack_parsed = VMStack::from_cell(&stack_cell)?;
        assert_eq!(stack_parsed.len(), 3);
        assert_eq!(stack_parsed.pop_cell_slice()?.deref(), &TonAddress::ZERO.to_cell()?);
        assert_eq!(stack_parsed.pop_int()?, 2.into());
        assert_eq!(stack_parsed.pop_tiny_int()?, 1);
        Ok(())
    }
}
