#[cfg(test)]
mod test_block_data;

pub mod block_extra;
pub mod block_id_ext;
pub mod block_info;
pub mod block_prev_info;
pub mod mc_block_extra;
pub mod shard_ident;

use crate::cell::ton_cell::TonCellRef;
use crate::types::tlb::adapters::TLBRef;
use crate::types::tlb::block_tlb::block::block_info::BlockInfo;
use ton_lib_macros::TLBDerive;

// TODO doesn't work properly yet.
// https://github.com/ton-blockchain/ton/blob/6f745c04daf8861bb1791cffce6edb1beec62204/crypto/block/block.tlb#L462
#[derive(Debug, Clone, PartialEq, TLBDerive)]
#[tlb_derive(prefix = 0x11ef55aa, bits_len = 32)]
pub struct Block {
    pub global_id: i32,
    #[tlb_derive(adapter = "TLBRef")]
    pub info: BlockInfo,
    pub value_flow: TonCellRef,   // TODO
    pub state_update: TonCellRef, // TODO
    // #[tlb_derive(adapter = "TLBRef")]
    pub extra: TonCellRef,
}

#[cfg(test)]
mod tests {
    // use crate::cell::ton_cell::{TonCell, TonCellRef};
    // use crate::cell::ton_hash::TonHash;
    // use crate::types::tlb::block_tlb::block::test_block_data::MASTER_BLOCK_BOC_HEX;
    // use crate::types::tlb::block_tlb::block::Block;
    // use crate::types::tlb::tlb_type::TLBType;
    // use std::collections::VecDeque;
    // use std::ops::Deref;
    // use std::str::FromStr;

    // #[test]
    // fn test_block_tbl_block_serde_hashes() -> anyhow::Result<()> {
    //     let parsed = Block::from_boc_hex(MASTER_BLOCK_BOC_HEX)?;
    //     let extra = parsed.extra.clone();
    //     println!("{}", extra);
    //     let extra_parsed_back = TonCellRef::from_boc(&extra.to_boc(false)?)?;
    //     println!("{}", extra_parsed_back);
    //     // assert_eq!(extra, extra_parsed_back);
    //     // assert_eq!(parsed.cell_hash()?, TonHash::from_str("0AEF3393C6CDECA446C71E3C67BDC20C6C9F8DC14FC0D52C39675B2C066F3BBA")?); // TODO find the way to calc actual hash
    //     // let parsed_back = Block::from_boc(&parsed.to_boc(false)?)?;
    //     // assert_eq!(parsed_back.global_id, parsed.global_id);
    //     // assert_eq!(parsed.info, parsed_back.info);
    //     // assert_eq!(parsed.value_flow, parsed_back.value_flow);
    //     // assert_eq!(parsed.state_update, parsed_back.state_update);
    //     // println!("extra: {}", parsed.extra);
    //     // assert_eq!(parsed.extra, parsed_back.extra);
    //     assert!(false);
    //     // assert_eq!(parsed, parsed_back);
    //     // assert_eq!(parsed.cell_hash()?, parsed_back.cell_hash()?);
    //     Ok(())
    // }
}
