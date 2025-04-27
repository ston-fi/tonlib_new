use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::cell::ton_cell::TonCellRef;
use crate::cell::ton_hash::TonHash;
use crate::errors::TonlibError;
use crate::types::tlb::adapters::dict_key_adapters::DictKeyAdapterTonHash;
use crate::types::tlb::adapters::dict_val_adapters::DictValAdapterTLB;
use crate::types::tlb::adapters::Dict;
use crate::types::tlb::tlb_type::TLBType;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct LibsDict(HashMap<TonHash, TonCellRef>);

impl Deref for LibsDict {
    type Target = HashMap<TonHash, TonCellRef>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for LibsDict {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl TLBType for LibsDict {
    fn read_definition(parser: &mut CellParser) -> Result<Self, TonlibError> {
        let data = Dict::<DictKeyAdapterTonHash, DictValAdapterTLB, _, _>::new(256).read(parser)?;
        Ok(LibsDict(data))
    }

    fn write_definition(&self, builder: &mut CellBuilder) -> Result<(), TonlibError> {
        Dict::<DictKeyAdapterTonHash, DictValAdapterTLB, _, _>::new(256).write(builder, &self.0)?;
        Ok(())
    }
}
