use crate::cell::ton_cell::{TonCell, TonCellRef};
use crate::tlb::tlb_type::TLBType;

pub enum TLBObject<T: TLBType, C: TonCell> {
    Cell(C),
    CellRef(TonCellRef),
    Plain(T),
}
