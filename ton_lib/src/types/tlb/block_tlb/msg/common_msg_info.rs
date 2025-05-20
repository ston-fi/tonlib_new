use crate::types::tlb::block_tlb::coins::{Coins, CurrencyCollection};
use crate::types::tlb::block_tlb::msg_address::{MsgAddress, MsgAddressExt, MsgAddressInt};
use ton_lib_macros::TLBDerive;

// https://github.com/ton-blockchain/ton/blob/050a984163a53df16fb03f66cc445c34bfed48ed/crypto/block/block.tlb#L155
#[derive(Debug, Clone, PartialEq, TLBDerive)]
pub enum CommonMsgInfo {
    Int(CommonMsgInfoInt),
    ExtIn(CommonMsgInfoExtIn),
    ExtOut(CommonMsgInfoExtOut), // is not tested
}

#[derive(Clone, Debug, PartialEq, TLBDerive)]
#[tlb_derive(prefix = 0b10, bits_len = 2)]
pub struct CommonMsgInfoExtIn {
    pub src: MsgAddressExt,
    pub dest: MsgAddressInt,
    pub import_fee: Coins,
}

#[derive(Clone, Debug, PartialEq, TLBDerive)]
#[tlb_derive(prefix = 0b0, bits_len = 1)]
pub struct CommonMsgInfoInt {
    pub ihr_disabled: bool,
    pub bounce: bool,
    pub bounced: bool,
    pub src: MsgAddress,  // it's MsgAddressInt in tlb, but in fact it can be at least MsgAddressNone
    pub dest: MsgAddress, // the same
    pub value: CurrencyCollection,
    pub ihr_fee: Coins,
    pub fwd_fee: Coins,
    pub created_lt: u64,
    pub created_at: u32,
}

#[derive(Clone, Debug, PartialEq, TLBDerive)]
#[tlb_derive(prefix = 0b11, bits_len = 2)]
pub struct CommonMsgInfoExtOut {
    pub src: MsgAddressInt,
    pub dest: MsgAddressExt,
    pub created_lt: u64,
    pub created_at: u32,
}
