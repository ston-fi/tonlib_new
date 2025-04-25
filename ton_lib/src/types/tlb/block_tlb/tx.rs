use crate::cell::ton_cell::TonCell;
use crate::cell::ton_hash::TonHash;
use crate::types::tlb::adapters::ConstLen;
use crate::types::tlb::adapters::TLBRef;
use crate::types::tlb::block_tlb::account::AccountStatus;
use crate::types::tlb::block_tlb::coins::CurrencyCollection;
use crate::types::tlb::block_tlb::hash_update::HashUpdate;
use crate::types::tlb::block_tlb::msg::Message;
use ton_lib_proc_macro::TLBDerive;

// https://github.com/ton-blockchain/ton/blob/ed4682066978f69ffa38dd98912ca77d4f660f66/crypto/block/block.tlb#L291
#[derive(Clone, Debug, TLBDerive)]
pub struct Tx {
    pub account_addr: TonHash,
    pub lt: u64,
    pub prev_tx_hash: TonHash,
    pub prev_tx_lt: u64,
    pub now: u32,
    #[tlb_derive(bits_len = 15)]
    pub out_msgs_cnt: u16,
    pub orig_status: AccountStatus,
    pub end_status: AccountStatus,
    #[tlb_derive(adapter = "TLBRef")]
    pub in_msg: Option<Message>,
    pub out_msgs: i32, // TODO
    pub total_fees: CurrencyCollection,
    #[tlb_derive(adapter = "TLBRef")]
    pub state_update: HashUpdate,
    #[tlb_derive(adapter = "TLBRef")]
    pub descr: TxDescr,
}

#[derive(Debug, Clone, TLBDerive)]
pub struct TxDescr {
    pub tmp: TonCell, // TODO
}
