mod compute_skip_reason;
mod tr_phase;
mod tx_descr;

use crate::cell::ton_hash::TonHash;
use crate::types::tlb::adapters::dict_key_adapters::DictKeyAdapterInto;
use crate::types::tlb::adapters::dict_val_adapters::DictValAdapterTLBRef;
use crate::types::tlb::adapters::ConstLen;
use crate::types::tlb::adapters::DictRef;
use crate::types::tlb::adapters::TLBOptRef;
use crate::types::tlb::adapters::TLBRef;
use crate::types::tlb::block_tlb::account::AccountStatus;
use crate::types::tlb::block_tlb::coins::CurrencyCollection;
use crate::types::tlb::block_tlb::hash_update::HashUpdate;
use crate::types::tlb::block_tlb::msg::Message;
use crate::types::tlb::block_tlb::tx::tx_descr::TxDescr;
use std::collections::HashMap;
use ton_lib_macros::TLBDerive;

// https://github.com/ton-blockchain/ton/blob/ed4682066978f69ffa38dd98912ca77d4f660f66/crypto/block/block.tlb#L291
#[derive(Clone, Debug, PartialEq, TLBDerive)]
#[tlb_derive(prefix = 0b0111, bits_len = 4)]
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
    pub msgs: TxMsgs,
    pub total_fees: CurrencyCollection,
    #[tlb_derive(adapter = "TLBRef")]
    pub state_update: HashUpdate,
    #[tlb_derive(adapter = "TLBRef")]
    pub descr: TxDescr,
}

#[derive(Debug, Clone, PartialEq, TLBDerive)]
pub struct TxMsgs {
    #[tlb_derive(adapter = "TLBOptRef")]
    pub in_msg: Option<Message>,
    #[tlb_derive(adapter = "DictRef::<DictKeyAdapterInto, DictValAdapterTLBRef, _, _>::new(15)")]
    pub out_msgs: HashMap<u32, Message>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::ton_cell::TonCell;
    use crate::types::tlb::block_tlb::account::AccountStatusActive;
    use crate::types::tlb::block_tlb::coins::Grams;
    use crate::types::tlb::block_tlb::tx::tr_phase::{
        AccStatusChange, AccStatusChangeUnchanged, ComputePhaseVMInfo, StorageUsedShort, TrActionPhase, TrComputePhase,
        TrComputePhaseVM, TrStoragePhase,
    };
    use crate::types::tlb::block_tlb::tx::tx_descr::TxDescrTickTock;
    use crate::types::tlb::block_tlb::var_len::VarLen;
    use crate::types::tlb::tlb_type::TLBType;
    use std::str::FromStr;

    #[test]
    fn test_block_tlb_tx_tick_tock() -> anyhow::Result<()> {
        let cell = TonCell::from_boc_hex("b5ee9c72010206010001320003af734517c7bdf5187c55af4f8b61fdc321588c7ab768dee24b006df29106458d7cf000016e2cc89c18399602ce40fd84286bddb06f8bcc9fceb7e3027f9826c8985017f16cba12363cc000016e2cc89c18161fa4c700001408050401020530303403020069600000009600000004000600000000000519ae84f17b8f8b22026a975ff55f1ab19fde4a768744d2178dfa63bb533e107a409026bc009e42664e625a00000000000000000030000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000082721fb68f3dbf22da4d92562a5895d490994d960e83e2e82a05e9ff86f7e1cafb2812bfed72f3e7140856bfe23e76bd419a6de0a046c29fa08833ecc7dff85e1ffd000120")?;
        println!("cell: {}", cell);
        let tx = Tx::from_boc_hex("b5ee9c72010206010001320003af734517c7bdf5187c55af4f8b61fdc321588c7ab768dee24b006df29106458d7cf000016e2cc89c18399602ce40fd84286bddb06f8bcc9fceb7e3027f9826c8985017f16cba12363cc000016e2cc89c18161fa4c700001408050401020530303403020069600000009600000004000600000000000519ae84f17b8f8b22026a975ff55f1ab19fde4a768744d2178dfa63bb533e107a409026bc009e42664e625a00000000000000000030000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000082721fb68f3dbf22da4d92562a5895d490994d960e83e2e82a05e9ff86f7e1cafb2812bfed72f3e7140856bfe23e76bd419a6de0a046c29fa08833ecc7dff85e1ffd000120")?;
        let expected = Tx {
            account_addr: TonHash::from_str("34517c7bdf5187c55af4f8b61fdc321588c7ab768dee24b006df29106458d7cf")?,
            lt: 25163350000003,
            prev_tx_hash: TonHash::from_str("99602ce40fd84286bddb06f8bcc9fceb7e3027f9826c8985017f16cba12363cc")?,
            prev_tx_lt: 25163350000001,
            now: 1643793520,
            out_msgs_cnt: 0,
            orig_status: AccountStatus::Active(AccountStatusActive {}),
            end_status: AccountStatus::Active(AccountStatusActive {}),
            msgs: TxMsgs {
                in_msg: None,
                out_msgs: Default::default(),
            },
            total_fees: CurrencyCollection::new(0u32),
            state_update: HashUpdate {
                old: TonHash::from_str("1fb68f3dbf22da4d92562a5895d490994d960e83e2e82a05e9ff86f7e1cafb28")?,
                new: TonHash::from_str("12bfed72f3e7140856bfe23e76bd419a6de0a046c29fa08833ecc7dff85e1ffd")?,
            },
            descr: TxDescr::TickTock(TxDescrTickTock {
                is_tock: true,
                storage_phase: TrStoragePhase {
                    storage_fees_collected: Grams::zero(),
                    storage_fees_due: None,
                    status_change: AccStatusChange::Unchanged(AccStatusChangeUnchanged {}),
                },
                compute_phase: TrComputePhase::VM(Box::new(TrComputePhaseVM {
                    success: true,
                    msg_state_used: false,
                    account_activated: false,
                    gas_fees: Grams::zero(),
                    compute_phase_vm_info: ComputePhaseVMInfo {
                        gas_used: VarLen::new(4914u32, 16),
                        gas_limit: VarLen::new(10000000u32, 24),
                        gas_credit: None,
                        mode: 0,
                        exit_code: 0,
                        exit_arg: None,
                        vm_steps: 48,
                        vm_init_state_hash: TonHash::from_str(
                            "0000000000000000000000000000000000000000000000000000000000000000",
                        )?,
                        vm_final_state_hash: TonHash::from_str(
                            "0000000000000000000000000000000000000000000000000000000000000000",
                        )?,
                    },
                })),
                action: Some(TrActionPhase {
                    success: false,
                    valid: true,
                    no_funds: true,
                    status_change: AccStatusChange::Unchanged(AccStatusChangeUnchanged {}),
                    total_fwd_fees: None,
                    total_action_fees: None,
                    result_code: 37,
                    result_arg: Some(2),
                    tot_actions: 3,
                    spec_actions: 0,
                    skipped_actions: 0,
                    msgs_created: 2,
                    action_list_hash: TonHash::from_str(
                        "8cd74278bdc7c59101354baffaaf8d58cfef253b43a2690bc6fd31dda99f083d",
                    )?,
                    tot_msg_size: StorageUsedShort {
                        cells: VarLen::new(2u32, 8),
                        bits: VarLen::new(1239u32, 16),
                    },
                }),
                aborted: true,
                destroyed: false,
            }),
        };
        assert_eq!(tx, expected);
        assert_eq!(
            tx.cell_hash()?,
            TonHash::from_str("0735F1ED2915B7D5E0D7861129CBDFF339E9038C45F61799ED9AA12E3106D342")?
        );
        let serial = tx.to_boc_hex(false)?;
        let parsed_back = Tx::from_boc_hex(&serial)?;
        assert_eq!(tx, parsed_back);
        Ok(())
    }
}
