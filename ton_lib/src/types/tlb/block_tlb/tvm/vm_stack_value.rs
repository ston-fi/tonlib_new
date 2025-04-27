use crate::cell::ton_cell::TonCellRef;
use crate::types::tlb::adapters::dict_key_adapters::DictKeyAdapterInto;
use crate::types::tlb::adapters::dict_val_adapters::DictValAdapterTLB;
use crate::types::tlb::adapters::ConstLen;
use crate::types::tlb::adapters::Dict;
use crate::types::tlb::adapters::TLBRef;
use crate::types::tlb::block_tlb::tvm::{VMCellSlice, VMStack};
use num_bigint::BigInt;
use std::collections::HashMap;
use std::sync::Arc;
use ton_lib_macros::TLBDerive;

#[derive(Clone, Debug, TLBDerive)]
pub enum VMStackValue {
    Null(VMStackNull),
    TinyInt(VMTinyInt),
    Int(VMInt),
    Nan(VMNan),
    Cell(VMCell),
    CellSlice(VMCellSlice),
    Builder(VMBuilder), // TODO is not tested
    Cont(VMCont),       // TODO is not tested
    Tuple(VMTuple),     // TODO is not tested
}

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0x00, bits_len = 8)]
pub struct VMStackNull {}

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0x01, bits_len = 8)]
pub struct VMTinyInt {
    pub value: i64,
}

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0x0201, bits_len = 16)]
pub struct VMInt {
    #[tlb_derive(bits_len = 257)]
    pub value: BigInt,
}

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0x02ff, bits_len = 16)]
pub struct VMNan {}

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0x03, bits_len = 8)]
pub struct VMCell {
    pub value: TonCellRef,
}

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0x05, bits_len = 8)]
pub struct VMBuilder {
    pub cell: TonCellRef,
}

#[derive(Debug, Clone, TLBDerive)]
pub enum VMCont {
    Std(VMContStd),
    Envelope(TVMContEnvelope),
    Quit(VMContQuit),
    QuitExc(TVMContQuitExc),
    Repeat(VMContRepeat),
    Until(VMContUntil),
    Again(VMContAgain),
    WhileCond(VMContWhileCond),
    WhileBody(VMContWhileBody),
    PushInt(VMContPushInt),
}

#[derive(Debug, Clone, TLBDerive)]
pub struct VMControlData {
    #[tlb_derive(bits_len = 13)]
    pub nargs: Option<u16>,
    pub stack: Option<Arc<VMStack>>,
    pub save: VMSaveList,
    pub cp: Option<i16>,
}

#[derive(Debug, Clone, TLBDerive)]
pub struct VMSaveList {
    #[tlb_derive(adapter = "Dict::<DictKeyAdapterInto, DictValAdapterTLB, _, _>::new(4)")]
    pub cregs: HashMap<u8, VMStackValue>,
}

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0x00, bits_len = 8)]
pub struct VMContStd {
    pub data: Arc<VMControlData>,
    pub code: Arc<VMCellSlice>,
}

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0x01, bits_len = 8)]
pub struct TVMContEnvelope {
    pub data: VMControlData,
    #[tlb_derive(adapter = "TLBRef")]
    pub next: Arc<VMCont>,
}

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0x1000, bits_len = 16)]
pub struct VMContQuit {
    pub exit_code: i32,
}

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0x1001, bits_len = 16)]
pub struct TVMContQuitExc {}

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0x10100, bits_len = 20)]
pub struct VMContRepeat {
    #[tlb_derive(bits_len = 63)]
    pub count: u64,
    #[tlb_derive(adapter = "TLBRef")]
    pub body: Arc<VMCont>,
    #[tlb_derive(adapter = "TLBRef")]
    pub after: Arc<VMCont>,
}

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0x110000, bits_len = 24)]
pub struct VMContUntil {
    #[tlb_derive(adapter = "TLBRef")]
    pub body: Arc<VMCont>,
    #[tlb_derive(adapter = "TLBRef")]
    pub after: Arc<VMCont>,
}

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0x110001, bits_len = 24)]
pub struct VMContAgain {
    #[tlb_derive(adapter = "TLBRef")]
    pub body: Arc<VMCont>,
}

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0x110010, bits_len = 24)]
pub struct VMContWhileCond {
    #[tlb_derive(adapter = "TLBRef")]
    pub cond: Arc<VMCont>,
    #[tlb_derive(adapter = "TLBRef")]
    pub body: Arc<VMCont>,
    #[tlb_derive(adapter = "TLBRef")]
    pub after: Arc<VMCont>,
}

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0x110011, bits_len = 24)]
pub struct VMContWhileBody {
    #[tlb_derive(adapter = "TLBRef")]
    pub cond: Arc<VMCont>,
    #[tlb_derive(adapter = "TLBRef")]
    pub body: Arc<VMCont>,
    #[tlb_derive(adapter = "TLBRef")]
    pub after: Arc<VMCont>,
}

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0x1111, bits_len = 16)]
pub struct VMContPushInt {
    pub value: i32,
    #[tlb_derive(adapter = "TLBRef")]
    pub next: Arc<VMCont>,
}

#[derive(Debug, Clone, TLBDerive)]
pub enum VMTuple {
    Cons(VMTupleCons),
    Nil(VMTupleNil),
}

#[derive(Debug, Clone, TLBDerive)]
pub struct VMTupleNil {}

#[derive(Debug, Clone, TLBDerive)]
pub struct VMTupleCons {
    #[tlb_derive(adapter = "TLBRef")]
    pub head: Arc<VMTuple>,
    #[tlb_derive(adapter = "TLBRef")]
    pub tail: Arc<VMStackValue>,
}

#[derive(Debug, Clone, TLBDerive)]
pub enum VMTupleRef {
    Nil(VMTupleRefNil),
    Single(VMTupleRefSingle),
    Any(VMTupleRefAny),
}

#[derive(Debug, Clone, TLBDerive)]
pub struct VMTupleRefNil {}

#[derive(Debug, Clone, TLBDerive)]
pub struct VMTupleRefSingle {
    #[tlb_derive(adapter = "TLBRef")]
    pub entry: Arc<VMStackValue>,
}

#[derive(Debug, Clone, TLBDerive)]
pub struct VMTupleRefAny {
    #[tlb_derive(adapter = "TLBRef")]
    pub tuple_ref: Arc<VMTuple>,
}
