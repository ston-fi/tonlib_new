// https://github.com/ton-blockchain/TEPs/blob/master/text/0074-jettons-standard.md#1-transfer

use crate::cell::ton_cell::{TonCell, TonCellRef};
use crate::types::tlb::block_tlb::coins::Coins;
use crate::types::tlb::block_tlb::msg_address::{MsgAddress, MsgAddressInt, MsgAddressNone};
use crate::types::tlb::primitives::EitherRef;
use num_bigint::BigUint;
use ton_lib_macros::TLBDerive;

#[derive(Clone, Debug, TLBDerive)]
#[tlb_derive(prefix = 0x0f8a7ea5, bits_len = 32)]
pub struct JettonTransferMsg {
    pub query_id: u64, // arbitrary number to identify the transfer
    pub amount: Coins, // amount of transferred jettons in elementary units
    pub dst: MsgAddress,
    pub response_dst: MsgAddress, // address where to send a response with confirmation of a successful transfer and the rest of the incoming message Toncoins.
    pub custom_payload: Option<TonCellRef>, // optional custom data (which is used by either sender or receiver jetton wallet for inner logic).
    pub forward_ton_amount: Coins,          // the amount of nano-tons to be sent to the destination address.
    pub forward_payload: EitherRef<TonCell>, // optional custom data that should be sent to the destination address.
}

impl JettonTransferMsg {
    pub fn new<A: Into<BigUint>>(dst: MsgAddressInt, amount: A) -> Self {
        JettonTransferMsg {
            query_id: 0,
            amount: Coins::new(amount),
            dst: dst.into(),
            response_dst: MsgAddressNone {}.into(),
            custom_payload: None,
            forward_ton_amount: Coins::zero(),
            forward_payload: EitherRef::new(TonCell::EMPTY),
        }
    }
}
