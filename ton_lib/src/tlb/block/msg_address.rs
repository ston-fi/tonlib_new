use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::errors::TonLibError;
use crate::tlb::primitives::dyn_len::{ConstLen, VarLen};
use crate::tlb::tlb_type::TLBPrefix;
use crate::tlb::tlb_type::TLBType;
use ton_lib_proc_macro::TLBDerive;

// https://github.com/ton-blockchain/ton/blob/59a8cf0ae5c3062d14ec4c89a04fee80b5fd05c1/crypto/block/block.tlb#L100
#[derive(Debug, Clone, PartialEq, TLBDerive)]
pub enum MsgAddress {
    Int(MsgAddressInt),
    Ext(MsgAddressExt),
}

// Ext
#[derive(Debug, Clone, PartialEq, TLBDerive)]
pub enum MsgAddressExt {
    None(MsgAddressNone),
    Extern(MsgAddressExtern),
}

#[derive(Debug, Clone, Copy, PartialEq, TLBDerive)]
#[tlb_derive(prefix = 0b00, bits_len = 2)]
pub struct MsgAddressNone {}

#[derive(Debug, Clone, PartialEq, TLBDerive)]
#[tlb_derive(prefix = 0b01, bits_len = 2)]
pub struct MsgAddressExtern {
    pub address: VarLen<Vec<u8>, 9>,
}

// Int
#[derive(Debug, Clone, PartialEq, TLBDerive)]
pub enum MsgAddressInt {
    Std(MsgAddrIntStd),
    Var(MsgAddrIntVar),
}

#[derive(Debug, Clone, PartialEq, TLBDerive)]
#[tlb_derive(prefix = 0b10, bits_len = 2)]
pub struct MsgAddrIntStd {
    pub anycast: Option<Anycast>,
    pub workchain: i8,
    pub address: ConstLen<Vec<u8>, 256>,
}

// peculiar object - addr_bits_len is separated from addr value
#[derive(Debug, Clone, PartialEq)]
pub struct MsgAddrIntVar {
    pub anycast: Option<Anycast>,
    pub addr_bits_len: ConstLen<u16, 9>,
    pub workchain: i32,
    pub address: Vec<u8>,
}

impl TLBType for MsgAddrIntVar {
    #[rustfmt::skip]
    const PREFIX: TLBPrefix = TLBPrefix { value: 0b11, bits_len: 2};

    fn read_def(parser: &mut CellParser) -> Result<Self, TonLibError> {
        let anycast = TLBType::read(parser)?;
        let addr_bits_len: ConstLen<u16, 9> = TLBType::read(parser)?;
        let workchain = TLBType::read(parser)?;
        let address = parser.read_bits(*addr_bits_len as u32)?;
        Ok(Self {
            anycast,
            addr_bits_len,
            workchain,
            address,
        })
    }

    fn write_def(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> {
        self.anycast.write(builder)?;
        self.addr_bits_len.write(builder)?;
        self.workchain.write(builder)?;
        builder.write_bits(&self.address, *self.addr_bits_len as u32)?;
        Ok(())
    }
}

#[rustfmt::skip]
mod from_impl {
    use crate::tlb::block::msg_address::*;
    impl From<MsgAddressNone> for MsgAddressExt { fn from(value: MsgAddressNone) -> Self { Self::None(value) } }
    impl From<MsgAddressExtern> for MsgAddressExt { fn from(value: MsgAddressExtern) -> Self { Self::Extern(value) } }
    impl From<MsgAddrIntStd> for MsgAddressInt { fn from(value: MsgAddrIntStd) -> Self { Self::Std(value) } }
    impl From<MsgAddrIntVar> for MsgAddressInt { fn from(value: MsgAddrIntVar) -> Self { Self::Var(value) } }
    impl From<MsgAddressInt> for MsgAddress { fn from(value: MsgAddressInt) -> Self { Self::Int(value) } }
    impl From<MsgAddressExt> for MsgAddress { fn from(value: MsgAddressExt) -> Self { Self::Ext(value) } }
    impl From<MsgAddressNone> for MsgAddress { fn from(value: MsgAddressNone) -> Self { Self::Ext(value.into()) } }
    impl From<MsgAddressExtern> for MsgAddress { fn from(value: MsgAddressExtern) -> Self { Self::Ext(value.into()) } }
    impl From<MsgAddrIntStd> for MsgAddress { fn from(value: MsgAddrIntStd) -> Self { Self::Int(value.into()) } }
    impl From<MsgAddrIntVar> for MsgAddress { fn from(value: MsgAddrIntVar) -> Self { Self::Int(value.into()) } }
}

#[derive(Debug, Clone, PartialEq, TLBDerive)]
pub struct Anycast {
    pub rewrite_pfx: VarLen<Vec<u8>, 5>,
}

impl Anycast {
    pub fn new(depth: u32, rewrite_pfx: Vec<u8>) -> Self {
        Self {
            rewrite_pfx: VarLen::new(depth, rewrite_pfx),
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio_test::assert_ok;

    use super::*;
    use crate::tlb::tlb_type::TLBType;

    #[test]
    fn test_read_write_msg_address() -> anyhow::Result<()> {
        // Anyhow read/write is covered under the hood
        let boc =
            "b5ee9c7201010101002800004bbe031053100134ea6c68e2f2cee9619bdd2732493f3a1361eccd7c5267a9eb3c5dcebc533bb6";
        let parsed = MsgAddress::from_boc_hex(boc)?;
        let expected = MsgAddrIntStd {
            anycast: Some(Anycast::new(30, vec![3, 16, 83, 16])),
            workchain: 0,
            address: vec![
                77, 58, 155, 26, 56, 188, 179, 186, 88, 102, 247, 73, 204, 146, 79, 206, 132, 216, 123, 51, 95, 20,
                153, 234, 122, 207, 23, 115, 175, 20, 206, 237,
            ]
            .into(),
        };
        assert_eq!(parsed, expected.into());

        let serial_cell = parsed.to_cell()?;
        let parsed_back = assert_ok!(MsgAddress::from_cell(&serial_cell));
        assert_eq!(parsed_back, parsed);
        Ok(())
    }

    #[test]
    fn test_read_msg_address_int_i8_workchain() -> anyhow::Result<()> {
        let boc = "b5ee9c720101010100240000439fe00000000000000000000000000000000000000000000000000000000000000010";
        let parsed = assert_ok!(MsgAddress::from_boc_hex(boc));

        let expected = MsgAddrIntStd {
            anycast: None,
            workchain: -1,
            address: vec![0; 32].into(),
        };
        assert_eq!(parsed, expected.into());

        // don't support same layout, so check deserialized data again
        let serial_cell = parsed.to_cell()?;
        let parsed_back = assert_ok!(MsgAddress::from_cell(&serial_cell));
        assert_eq!(parsed, parsed_back);
        Ok(())
    }

    #[test]
    fn test_read_msg_address_int() -> anyhow::Result<()> {
        let boc = "b5ee9c720101010100240000439fe00000000000000000000000000000000000000000000000000000000000000010";
        let parsed = assert_ok!(MsgAddressInt::from_boc_hex(boc));

        let expected = MsgAddrIntStd {
            anycast: None,
            workchain: -1,
            address: vec![0; 32].into(),
        };
        assert_eq!(parsed, expected.into());

        // don't support same layout, so check deserialized data again
        let serial_cell = parsed.to_cell()?;
        let parsed_back = assert_ok!(MsgAddressInt::from_cell(&serial_cell));
        assert_eq!(parsed, parsed_back);
        Ok(())
    }
}
