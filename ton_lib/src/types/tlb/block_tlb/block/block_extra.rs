use crate::cell::ton_cell::{TonCell, TonCellRef};
use crate::cell::ton_hash::TonHash;
use crate::types::tlb::block_tlb::block::TLBOptRef;
use ton_lib_macros::TLBDerive;

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0x4a33f6fd, bits_len = 32)]
pub struct BlockExtra {
    pub in_msg_descr: TonCellRef, // TODO
    pub out_msg_descr: TonCellRef, // TODO
    pub account_blocks: TonCellRef, // TODO
    pub rand_seed: TonHash,
    pub created_by: TonHash,
    #[tlb_derive(adapter = "TLBOptRef")]
    pub mc_block_extra: Option<TonCell>, // TODO
}

