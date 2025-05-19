use crate::cell::ton_hash::TonHash;
use crate::types::tlb::block_tlb::tx::tr_phase::{
    TrActionPhase, TrBouncePhase, TrComputePhase, TrCreditPhase, TrStoragePhase,
};
use crate::types::tlb::block_tlb::tx::ConstLen;
use crate::types::tlb::block_tlb::tx::TLBOptRef;
use crate::types::tlb::block_tlb::tx::TLBRef;
use crate::types::tlb::block_tlb::tx::Tx;
use ton_lib_macros::TLBDerive;

// https://github.com/ton-blockchain/ton/blob/ed4682066978f69ffa38dd98912ca77d4f660f66/crypto/block/block.tlb#L353
#[derive(Debug, Clone, PartialEq, TLBDerive)]
pub enum TxDescr {
    Ord(TxDescrOrd),
    Storage(TxDescrStorage),
    TickTock(TxDescrTickTock),
    SplitPrepare(TxDescrSplitPrepare),
    SplitInstall(TxDescrSplitInstall),
    MergePrepare(TxDescrMergePrepare),
    MergeInstall(TxDescrMergeInstall),
}

#[derive(Debug, Clone, PartialEq, TLBDerive)]
#[tlb_derive(prefix = 0b0000, bits_len = 4)]
pub struct TxDescrOrd {
    pub credit_first: bool,
    pub storage_phase: Option<TrStoragePhase>,
    pub credit_phase: Option<TrCreditPhase>,
    pub compute_phase: TrComputePhase,
    #[tlb_derive(adapter = "TLBOptRef")]
    pub action: Option<TrActionPhase>,
    pub aborted: bool,
    pub bounce: Option<TrBouncePhase>,
    pub destroyed: bool,
}

#[derive(Debug, Clone, PartialEq, TLBDerive)]
#[tlb_derive(prefix = 0b0001, bits_len = 4)]
pub struct TxDescrStorage {
    pub storage_phase: TrStoragePhase,
}

#[derive(Debug, Clone, PartialEq, TLBDerive)]
#[tlb_derive(prefix = 0b001, bits_len = 3)]
pub struct TxDescrTickTock {
    pub is_tock: bool,
    pub storage_phase: TrStoragePhase,
    pub compute_phase: TrComputePhase,
    #[tlb_derive(adapter = "TLBOptRef")]
    pub action: Option<TrActionPhase>,
    pub aborted: bool,
    pub destroyed: bool,
}

#[derive(Debug, Clone, PartialEq, TLBDerive)]
#[tlb_derive(prefix = 0b0110, bits_len = 4)]
pub struct TxDescrSplitPrepare {
    pub split_info: SplitMergeInfo,
    pub storage_phase: Option<TrStoragePhase>,
    pub compute_phase: TrComputePhase,
    #[tlb_derive(adapter = "TLBOptRef")]
    pub action: Option<TrActionPhase>,
    pub aborted: bool,
    pub destroyed: bool,
}

#[derive(Debug, Clone, PartialEq, TLBDerive)]
#[tlb_derive(prefix = 0b0101, bits_len = 4)]
pub struct TxDescrSplitInstall {
    pub split_info: SplitMergeInfo,
    #[tlb_derive(adapter = "TLBRef")]
    pub prepare_tx: Box<Tx>,
    pub installed: bool,
}

#[derive(Debug, Clone, PartialEq, TLBDerive)]
#[tlb_derive(prefix = 0b0110, bits_len = 4)]
pub struct TxDescrMergePrepare {
    pub split_info: SplitMergeInfo,
    pub storage_phase: TrStoragePhase,
    pub aborted: bool,
}

#[derive(Debug, Clone, PartialEq, TLBDerive)]
#[tlb_derive(prefix = 0b0111, bits_len = 4)]
pub struct TxDescrMergeInstall {
    pub split_info: SplitMergeInfo,
    #[tlb_derive(adapter = "TLBRef")]
    pub prepare_tx: Box<Tx>,
    #[tlb_derive(adapter = "TLBOptRef")]
    pub storage_phase: Option<TrStoragePhase>,
    #[tlb_derive(adapter = "TLBOptRef")]
    pub credit_phase: Option<TrCreditPhase>,
    pub compute_phase: TrComputePhase,
    #[tlb_derive(adapter = "TLBOptRef")]
    pub action: Option<TrActionPhase>,
    pub aborted: bool,
    pub destroyed: bool,
}

#[derive(Debug, Clone, PartialEq, TLBDerive)]
pub struct SplitMergeInfo {
    #[tlb_derive(bits_len = 6)]
    pub cur_shard_pfx_len: u8,
    #[tlb_derive(bits_len = 6)]
    pub acc_split_depth: u8,
    pub this_addr: TonHash,
    pub sibling_addr: TonHash,
}
