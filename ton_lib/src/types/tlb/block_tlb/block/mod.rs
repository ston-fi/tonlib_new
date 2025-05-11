#[cfg(test)]
mod test_block_data;

mod block_extra;
mod block_id_ext;
mod block_info;
mod block_prev_info;
mod shard_ident;
mod mc_block_extra;

use crate::types::tlb::adapters::TLBOptRef;
pub use block_extra::*;
pub use block_id_ext::*;
pub use block_info::*;
pub use block_prev_info::*;
pub use shard_ident::*;

use crate::cell::ton_cell::{TonCell, TonCellRef};
use crate::cell::ton_hash::TonHash;
use crate::types::tlb::adapters::TLBRef;
use ton_lib_macros::TLBDerive;

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0x11ef55aa, bits_len = 32)]
pub struct Block {
    pub global_id: i32,
    #[tlb_derive(adapter = "TLBRef")]
    pub info: BlockInfo,
    pub value_flow: TonCellRef,   // TODO
    pub state_update: TonCellRef, // TODO
    #[tlb_derive(adapter = "TLBRef")]
    pub extra: BlockExtra,
}
