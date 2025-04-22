use super::msg_address::{MsgAddress, MsgAddressExt, MsgAddressInt};
use crate::cell::ton_cell::TonCell;
use crate::tlb::block_tlb::coins::{Coins, CurrencyCollection};
use crate::tlb::block_tlb::state_init::StateInit;
use crate::tlb::primitives::EitherRef;
use ton_lib_proc_macro::TLBDerive;

// https://github.com/ton-blockchain/ton/blob/050a984163a53df16fb03f66cc445c34bfed48ed/crypto/block/block.tlb#L157
#[derive(Debug, Clone, PartialEq, TLBDerive)]
pub struct Message {
    pub info: CommonMsgInfo,
    pub init: Option<EitherRef<StateInit>>,
    pub body: EitherRef<TonCell>,
}

// https://github.com/ton-blockchain/ton/blob/050a984163a53df16fb03f66cc445c34bfed48ed/crypto/block/block.tlb#L155
#[derive(Debug, Clone, PartialEq, TLBDerive)]
pub enum CommonMsgInfo {
    Int(IntMsgInfo),
    ExtIn(ExtInMsgInfo),
    ExtOut(ExtOutMsgInfo), // is not tested
}

#[derive(Clone, Debug, PartialEq, TLBDerive)]
#[tlb_derive(prefix = 0b10, bits_len = 2)]
pub struct ExtInMsgInfo {
    pub src: MsgAddressExt,
    pub dest: MsgAddressInt,
    pub import_fee: Coins,
}

#[derive(Clone, Debug, PartialEq, TLBDerive)]
#[tlb_derive(prefix = 0b0, bits_len = 1)]
pub struct IntMsgInfo {
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
pub struct ExtOutMsgInfo {
    pub src: MsgAddressInt,
    pub dest: MsgAddressExt,
    pub created_lt: u64,
    pub created_at: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::ton_cell::TonCell;
    use crate::tlb::block_tlb::coins::{Coins, CurrencyCollection};
    use crate::tlb::tlb_type::TLBType;
    use crate::types::ton_address::TonAddress;
    use std::str::FromStr;
    use tokio_test::assert_ok;

    #[test]
    fn test_common_msg_info_int() -> anyhow::Result<()> {
        let msg_cell = TonCell::from_boc_hex("b5ee9c720101010100580000ab69fe00000000000000000000000000000000000000000000000000000000000000013fccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccd3050ec744000000617bc90dda80cf41ab8e40")?;
        let parsed_msg = Message::from_cell(&msg_cell)?;
        assert!(parsed_msg.init.is_none());
        assert_eq!(parsed_msg.body.value.data_bits_len, 0); // quite useless assert, but let it be here

        let info = match parsed_msg.info.clone() {
            CommonMsgInfo::Int(info) => info,
            _ => panic!("Expected CommonMsgInfo::Int"),
        };
        assert!(info.ihr_disabled);
        assert!(info.bounce);
        assert!(!info.bounced);

        let expected_src = TonAddress::from_str("Ef8AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADAU")?;
        let expected_dest = TonAddress::from_str("Ef8zMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzM0vF")?;
        assert_eq!(TonAddress::from_msg_address(info.src)?, expected_src);
        assert_eq!(TonAddress::from_msg_address(info.dest)?, expected_dest);
        assert_eq!(info.value, CurrencyCollection::new(3242439121u32));
        assert_eq!(info.ihr_fee, Coins::new(0u32));
        assert_eq!(info.fwd_fee, Coins::new(0u32));
        assert_eq!(info.created_lt, 53592141000000);
        assert_eq!(info.created_at, 1738593735u32);

        let serialized = parsed_msg.to_cell()?;
        let parsed_back = assert_ok!(Message::from_cell(&serialized));
        assert_eq!(parsed_back, parsed_msg);
        Ok(())
    }

    #[test]
    fn test_ext_in_msg_info() -> anyhow::Result<()> {
        let ext_in_msg_info = ExtInMsgInfo::from_boc_hex(
            "b5ee9c7201010101002500004588010319f77e4d761f956e78f9c9fd45f1e914b7ffab9b5c1ea514858979c1560dee10",
        )?;
        let expected_dst = TonAddress::from_str("EQCBjPu_JrsPyrc8fOT-ovj0ilv_1c2uD1KKQsS84KsG90PM")?;
        let dst = TonAddress::from_msg_address(ext_in_msg_info.dest.clone())?;
        assert_eq!(dst, expected_dst);
        assert_eq!(ext_in_msg_info.import_fee, Coins::new(0u32));

        let cell = ext_in_msg_info.to_cell()?;
        let parsed = ExtInMsgInfo::from_cell(&cell)?;
        assert_eq!(parsed, ext_in_msg_info);
        Ok(())
    }
}
