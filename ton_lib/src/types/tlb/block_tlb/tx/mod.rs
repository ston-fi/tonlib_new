pub mod compute_skip_reason;
pub mod tr_phase;
pub mod tx_descr;

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
    use crate::types::tlb::block_tlb::account::AccountStatusActive;
    use crate::types::tlb::block_tlb::coins::Grams;
    use crate::types::tlb::block_tlb::tx::tr_phase::{
        AccStatusChange, AccStatusChangeUnchanged, ComputePhaseVMInfo, StorageUsedShort, TrActionPhase, TrComputePhase,
        TrComputePhaseVM, TrCreditPhase, TrStoragePhase,
    };
    use crate::types::tlb::block_tlb::tx::tx_descr::{TxDescrOrd, TxDescrTickTock};
    use crate::types::tlb::block_tlb::var_len::VarLen;
    use crate::types::tlb::tlb_type::TLBType;
    use std::str::FromStr;

    #[test]
    fn test_block_tlb_tx_tick_tock() -> anyhow::Result<()> {
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
        let serial = tx.to_boc_hex()?;
        let parsed_back = Tx::from_boc_hex(&serial)?;
        assert_eq!(tx, parsed_back);
        Ok(())
    }

    #[test]
    fn test_block_tlb_tx_ordinal() -> anyhow::Result<()> {
        // https://tonviewer.com/transaction/cd4c4f0f3e7962b90c92f5f0c27967fd4468acfa15d4df50faf8d2704a489e0b
        let tx = Tx::from_boc_hex("b5ee9c720102220100070a0003b57949a19cfd6eb82bb5ff6573b11208c71abb9398411b3b4672f78a7e34ea706d9000030a49dab0285b78a4a3e91ae0ddf8c49983a554e010cc4764ccc990500728e8202f958c7fc40000030a3c2065f45679cb7df00054693b1668050401021904825a890327c89418686858110302006fc989d2d84c1a32240000000000040000000000041d33f5c45a08b114815d645d1af4523cb3f221d59424fe5228ab5c828568aeaa41104eac009e45618c204fb40000000000000000d900000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008272edc0c091d2c05021d1493b2a4c266f6ac6f6faf6d88a47b05bc7d70b3121d085ad2e937c5b6dab2c4c8053b8c697409e310fc5f9c455346f9dd9df1a355b2e1b0201e00c060201dd09070101200800c748012934339fadd70576bfecae76224118e357727308236768ce5ef14fc69d4e0db30024bae441b2880a544cb3985c3b388ce2e840d084d28f283679267c8726d225f90ffe24340608235a000061493b56050ecf396fbe6a993b6d8000000000000000400101200a01b148012934339fadd70576bfecae76224118e357727308236768ce5ef14fc69d4e0db3002526867315d8639b77efa65b2ef84f52dab9a47871fa97a8f7c033575f7366ad5029b9270006120ef4000061493b56050ccf396fbec00b01667362d09c00000000000000005012a05f20080125d7220d944052a2659cc2e1d9c4671742068426947941b3c933e43936912fc90e02b1680125d7220ebaa477a4c50ab937088b600f1d397c4c3cdfbc350becd4e25ff43e610025268673f5bae0aed7fd95cec448231c6aee4e61046ced19cbde29f8d3a9c1b650327c8940065dc45a000061493b560508cf396fbfe00f0d01b1178d451900000000000000005012a05f20080125d7220d944052a2659cc2e1d9c4671742068426947941b3c933e43936912fc90024bae441b2880a544cb3985c3b388ce2e840d084d28f283679267c8726d225f91029b927030e0099259385618012934339d11465553b2f3e428ae79b0b1e2fd250b80784d4996dd44741736528ca0259f3a0f90024bae441b2880a544cb3985c3b388ce2e840d084d28f283679267c8726d225f9100203f9a011100187080129343398aec31cdbbf7d32d977c27a96d5cd23c38fd4bd47be019abafb9b356b001ece9afb55cc82c82739247aa35879be66afeb1502a81a72f2a982ec7625b5fb20110114ff00f4a413f4bcf2c80b120201621413001ba0f605da89a1f401f481f481a8610202cc1f15020120191602014818170083200835c87b51343e803e903e90350c0134c7e08405e3514654882ea0841ef765f784ee84ac7cb8b174cfcc7e800c04e81408f214013e809633c58073c5b3327b552000db3b51343e803e903e90350c01f4cffe803e900c145468549271c17cb8b049f0bffcb8b0a0823938702a8005a805af3cb8b0e0841ef765f7b232c7c572cfd400fe8088b3c58073c5b25c60063232c14933c59c3e80b2dab33260103ec01004f214013e809633c58073c5b3327b55200201581d1a01f53b51343e803e903e90350c0234cffe80145468017e903e9002fe911d3232c084b281f2fff27414d431c1551cdb48965c150804d50500f214013e809633c58073c5b33248a0079c7232c032c132c004bd003d0032c032407e910c6af8407e40006ab84061386c2c5c1d3232c0b281f2fff2741631c16c7cb8b0c2a01b01fefa0051a8a18208989680820898968012b608a18208e4e1c0a018a1278e385279a018a182107362d09cc8cb1f5230cb3f58fa025007cf165007cf16c9718010c8cb0524cf165006fa0215cb6a14ccc971fb00102410239710491038375f04e225d70b01c30023c200b093356c21e30d03c85004fa0258cf1601cf16ccc9ed541c00428210d53276db708010c8cb055008cf165004fa0216cb6a12cb1f12cb3fc972fb0001f300f4cffe803e90087c007b51343e803e903e90350c144da8548ab1c17cb8b04a30bffcb8b0951d009c150804d50500f214013e809633c58073c5b33248a0079c7232c032c132c004bd003d0032c0325481be910c6af8407e40006ab84061386c2c5c1d3232c0b281f2fff274013e903d010c7e800835d27080201e00d8f2e2c4778018c8cb055008cf1670fa0217cb6b17cc8210178d4519c8cb1f19cb3f5007fa0222cf165006cf1624fa025003cf16c95005cc2291729171e25008a812a08208e4e1c0aa008208989680a0a014bcf2e2c504c98040fb004130c85004fa0258cf1601cf16ccc9ed540201d4212000113e910c1c2ebcb8536000c30831c02497c138007434c0c05c6c2544d7c0fc03783e903e900c7e800c5c75c87e800c7e800c1cea6d0000b4c7e08403e29fa954882ea54c4d167c02b8208405e3514654882ea58c511100fc02f80d60841657c1ef2ea4d67c033817c12103fcbc20")?;
        let expected = Tx {
            account_addr: TonHash::from_str("949a19cfd6eb82bb5ff6573b11208c71abb9398411b3b4672f78a7e34ea706d9")?,
            lt: 53483578000005,
            prev_tx_hash: TonHash::from_str("b78a4a3e91ae0ddf8c49983a554e010cc4764ccc990500728e8202f958c7fc40")?,
            prev_tx_lt: 53479893000005,
            now: 1738323935,
            out_msgs_cnt: 2,
            orig_status: AccountStatus::Active(AccountStatusActive {}),
            end_status: AccountStatus::Active(AccountStatusActive {}),
            msgs: TxMsgs {
                in_msg: Some(Message::from_boc_hex("b5ee9c72010216010004c10002b1680125d7220ebaa477a4c50ab937088b600f1d397c4c3cdfbc350becd4e25ff43e610025268673f5bae0aed7fd95cec448231c6aee4e61046ced19cbde29f8d3a9c1b650327c8940065dc45a000061493b560508cf396fbfe0030101b1178d451900000000000000005012a05f20080125d7220d944052a2659cc2e1d9c4671742068426947941b3c933e43936912fc90024bae441b2880a544cb3985c3b388ce2e840d084d28f283679267c8726d225f91029b92703020099259385618012934339d11465553b2f3e428ae79b0b1e2fd250b80784d4996dd44741736528ca0259f3a0f90024bae441b2880a544cb3985c3b388ce2e840d084d28f283679267c8726d225f9100203f9a005040187080129343398aec31cdbbf7d32d977c27a96d5cd23c38fd4bd47be019abafb9b356b001ece9afb55cc82c82739247aa35879be66afeb1502a81a72f2a982ec7625b5fb20050114ff00f4a413f4bcf2c80b060201620807001ba0f605da89a1f401f481f481a8610202cc13090201200d0a0201480c0b0083200835c87b51343e803e903e90350c0134c7e08405e3514654882ea0841ef765f784ee84ac7cb8b174cfcc7e800c04e81408f214013e809633c58073c5b3327b552000db3b51343e803e903e90350c01f4cffe803e900c145468549271c17cb8b049f0bffcb8b0a0823938702a8005a805af3cb8b0e0841ef765f7b232c7c572cfd400fe8088b3c58073c5b25c60063232c14933c59c3e80b2dab33260103ec01004f214013e809633c58073c5b3327b5520020158110e01f53b51343e803e903e90350c0234cffe80145468017e903e9002fe911d3232c084b281f2fff27414d431c1551cdb48965c150804d50500f214013e809633c58073c5b33248a0079c7232c032c132c004bd003d0032c032407e910c6af8407e40006ab84061386c2c5c1d3232c0b281f2fff2741631c16c7cb8b0c2a00f01fefa0051a8a18208989680820898968012b608a18208e4e1c0a018a1278e385279a018a182107362d09cc8cb1f5230cb3f58fa025007cf165007cf16c9718010c8cb0524cf165006fa0215cb6a14ccc971fb00102410239710491038375f04e225d70b01c30023c200b093356c21e30d03c85004fa0258cf1601cf16ccc9ed541000428210d53276db708010c8cb055008cf165004fa0216cb6a12cb1f12cb3fc972fb0001f300f4cffe803e90087c007b51343e803e903e90350c144da8548ab1c17cb8b04a30bffcb8b0951d009c150804d50500f214013e809633c58073c5b33248a0079c7232c032c132c004bd003d0032c0325481be910c6af8407e40006ab84061386c2c5c1d3232c0b281f2fff274013e903d010c7e800835d27080201200d8f2e2c4778018c8cb055008cf1670fa0217cb6b17cc8210178d4519c8cb1f19cb3f5007fa0222cf165006cf1624fa025003cf16c95005cc2291729171e25008a812a08208e4e1c0aa008208989680a0a014bcf2e2c504c98040fb004130c85004fa0258cf1601cf16ccc9ed540201d4151400113e910c1c2ebcb8536000c30831c02497c138007434c0c05c6c2544d7c0fc03783e903e900c7e800c5c75c87e800c7e800c1cea6d0000b4c7e08403e29fa954882ea54c4d167c02b8208405e3514654882ea58c511100fc02f80d60841657c1ef2ea4d67c033817c12103fcbc20")?), // TODO
                out_msgs: HashMap::from([
                    (0, Message::from_boc_hex("b5ee9c720101030100e10001b148012934339fadd70576bfecae76224118e357727308236768ce5ef14fc69d4e0db3002526867315d8639b77efa65b2ef84f52dab9a47871fa97a8f7c033575f7366ad5029b9270006120ef4000061493b56050ccf396fbec00101667362d09c00000000000000005012a05f20080125d7220d944052a2659cc2e1d9c4671742068426947941b3c933e43936912fc9020099259385618012934339d11465553b2f3e428ae79b0b1e2fd250b80784d4996dd44741736528ca0259f3a0f90024bae441b2880a544cb3985c3b388ce2e840d084d28f283679267c8726d225f910")?),
                    (1, Message::from_boc_hex("b5ee9c720101010100660000c748012934339fadd70576bfecae76224118e357727308236768ce5ef14fc69d4e0db30024bae441b2880a544cb3985c3b388ce2e840d084d28f283679267c8726d225f90ffe24340608235a000061493b56050ecf396fbe6a993b6d800000000000000040")?),
                ]),
            },
            total_fees: CurrencyCollection::new(4839603u32),
            state_update: HashUpdate {
                old: TonHash::from_str("edc0c091d2c05021d1493b2a4c266f6ac6f6faf6d88a47b05bc7d70b3121d085")?,
                new: TonHash::from_str("ad2e937c5b6dab2c4c8053b8c697409e310fc5f9c455346f9dd9df1a355b2e1b")?,
            },
            descr: TxDescr::Ord(TxDescrOrd {
                credit_first: false,
                storage_phase: Some(TrStoragePhase {
                    storage_fees_collected: Grams::new(2410u32),
                    storage_fees_due: None,
                    status_change: AccStatusChange::Unchanged(AccStatusChangeUnchanged{}),
                }),
                credit_phase: Some(TrCreditPhase {
                    due_fees_collected: None,
                    credit: CurrencyCollection::new(211755600u32),
                }),
                compute_phase: TrComputePhase::VM(Box::new(TrComputePhaseVM {
                    success: true,
                    msg_state_used: false,
                    account_activated: false,
                    gas_fees: Grams::new(4408000u32),
                    compute_phase_vm_info: ComputePhaseVMInfo {
                        gas_used: VarLen::new(11020u32, 16),
                        gas_limit: VarLen::new(529389u32, 24),
                        gas_credit: None,
                        mode: 0,
                        exit_code: 0,
                        exit_arg: None,
                        vm_steps: 217,
                        vm_init_state_hash: TonHash::from_str("0000000000000000000000000000000000000000000000000000000000000000")?,
                        vm_final_state_hash: TonHash::from_str("0000000000000000000000000000000000000000000000000000000000000000")?,
                    },
                })),
                action: Some(TrActionPhase {
                    success: true,
                    valid: true,
                    no_funds: false,
                    status_change: AccStatusChange::Unchanged(AccStatusChangeUnchanged{}),
                    total_fwd_fees: Some(Grams::new(1287600u32)),
                    total_action_fees: Some(Grams::new(429193u32)),
                    result_code: 0,
                    result_arg: None,
                    tot_actions: 2,
                    spec_actions: 0,
                    skipped_actions: 0,
                    msgs_created: 2,
                    action_list_hash: TonHash::from_str("0e99fae22d04588a40aeb22e8d7a291e59f910eaca127f291455ae4142b45755")?,
                    tot_msg_size: StorageUsedShort {
                        cells: VarLen::new(4u32, 8),
                        bits: VarLen::new(2517u32, 16),
                    },
                }),
                aborted: false,
                bounce: None,
                destroyed: false,
            }),
        };
        assert_eq!(tx, expected);
        assert_eq!(
            tx.cell_hash()?,
            TonHash::from_str("CD4C4F0F3E7962B90C92F5F0C27967FD4468ACFA15D4DF50FAF8D2704A489E0B")?
        );
        let serial = tx.to_boc_hex()?;
        let parsed_back = Tx::from_boc_hex(&serial)?;
        assert_eq!(tx, parsed_back);
        Ok(())
    }

    #[test]
    fn test_tx_9b7f973ac5e6a89c5ae3f6e986af5e6723f0298da219407afb39c9ef7c9352ef() -> anyhow::Result<()> {
        let tx = Tx::from_boc_hex("b5ee9c7201021a0100044a0003b572857bdf1ed7104d7d1c322e08f3b730dca9fe4f8abc7ff5a73d23f3c2a6c7ec1000014c012b30b01db6e6bef8edbc032cc47fff0839cbaa80f4761fc4407804b0d54732bb1c0a518000014c012577d83618856180002477c79028050401020f0c40461ab442c4400302006fc987a1204c145840000000000002000000000002c02f43991d8b2b6fcd5f70a7e14fd5d9da33b774def7741d91db16b9a4b781064050174c009d42c4e3138800000000000000001d0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000020008272b8cb33e034a8bfc1ecaf17f3a28131eaceba9348892bf85538e283a87550d402bfa047253b23d66835809070746a8fb3adcda563fb05bc0d4e9a1a6082eb14920201e008060101df0700bb480050af7be3dae209afa38645c11e76e61b953fc9f1578ffeb4e7a47e7854d8fd83000a15ef7c7b5c4135f470c8b823cedcc372a7f93e2af1ffd69cf48fcf0a9b1fb054094d0ded3c061458600000298025661604c310ac300507f454c002df880050af7be3dae209afa38645c11e76e61b953fc9f1578ffeb4e7a47e7854d8fd82119b8d0af3551672b1251ad29c2ae353bbf6187b6caae6babf52299f5f22cd0599c1951230e83128abf29ecac664daf80235fcb6622832862866ffac96f21f5260f0000017cfc8225ac68b55f3f80b090101c00a0043d01d1c2f610107a1a0abcc4e5fd9c1f15149fd34ab30482fd6d9d05ba16066d57560142402671115889b65652707f0ab10d8838340c7a888d4a6d6f8abb1320836c1634300068aed5320e30320c0ffe30220c0fee302f20b170d0c19028aed44d0d749c301f86621db3cd300019f810200d71820f90158f842f910f2a8ded33f01f84321b9f2b420f8238103e8a882081b7740a0b9f2b4f863d31f01db3cf8476ef27c100e0262ed44d0d749c301f86622d0d70b03a93800dc21c700209f3021d70d1ff2bc21c00020926c21dedfe30201db3cf8476ef27c140e02282082100a0fe8a9bae30220821068b55f3fbae302120f027830f8426ee300f846f273d1f800f82870c8cf8580ca0073cf40ce8d0480000000000000000000000000000507f454c0cf16c9810080fb00db3c7ff86710150216ed44d0d749c2018a8e80e21611012e70ed44d0f40588f86a8040f40ef2bdd70bfff86270f86319032630f846f2e04cf8426ee300d1db3cdb3c7ff8671613150072f84a20c8cc2101cc2101ccccc9f86af82870c8cf8580ca0073cf40ce8d0480000000000000000000000000000507f454c0cf16c9810080fb00027af846f2e04cf8426ee3008d0890801c52cadbcc81c8e6906459edafbdc9b71081994e92d387e139e6acb69cb20003d0c8ce806fcf40c98100a0fb00db3c16150022f84af843f842c8cbffcb3fcf83ccc9ed540024ed44d0d3ffd33fd30031d4d1f86af863f862020af4a420f4a119180014736f6c20302e34392e300000")?;
        assert_eq!(
            tx.account_addr,
            TonHash::from_str("2857bdf1ed7104d7d1c322e08f3b730dca9fe4f8abc7ff5a73d23f3c2a6c7ec1")?
        );
        assert_eq!(
            tx.cell_hash()?,
            TonHash::from_str("9b7f973ac5e6a89c5ae3f6e986af5e6723f0298da219407afb39c9ef7c9352ef")?
        );
        Ok(())
    }
}
