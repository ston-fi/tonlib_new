use crate::cell::ton_cell::{TonCell, TonCellRef};
use crate::tlb::tlb_type::TLBType;

pub enum TLBObject<T: TLBType> {
    Cell(Box<TonCell>),
    CellRef(TonCellRef),
    Plain(T),
}
