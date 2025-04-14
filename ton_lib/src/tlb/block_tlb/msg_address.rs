use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::errors::TonLibError;
use crate::tlb::adapters::const_len::ConstLen;
use crate::tlb::adapters::const_len::ConstLenRef;
use crate::tlb::block_tlb::var_len::var_len::VarLenBits;
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
    pub address: VarLenBits<Vec<u8>, 9>,
}

// Int
#[derive(Debug, Clone, PartialEq, TLBDerive)]
pub enum MsgAddressInt {
    Std(MsgAddressIntStd),
    Var(MsgAddressIntVar),
}

#[derive(Debug, Clone, PartialEq, TLBDerive)]
#[tlb_derive(prefix = 0b10, bits_len = 2)]
pub struct MsgAddressIntStd {
    pub anycast: Option<Anycast>,
    pub workchain: i8,
    #[tlb_derive(bits_len = 256)]
    pub address: Vec<u8>,
}

// peculiar object - addr_bits_len is separated from addr value,
// so TLBType must be specified manually
#[derive(Debug, Clone, PartialEq)]
pub struct MsgAddressIntVar {
    pub anycast: Option<Anycast>,
    pub addr_bits_len: u32, // 9 bit
    pub workchain: i32,
    pub address: Vec<u8>,
}

impl TLBType for MsgAddressIntVar {
    #[rustfmt::skip]
    const PREFIX: TLBPrefix = TLBPrefix { value: 0b11, bits_len: 2};

    fn read_definition(parser: &mut CellParser) -> Result<Self, TonLibError> {
        let anycast = TLBType::read(parser)?;
        let addr_bits_len = ConstLen::<u32, 9>::read(parser)?.0;
        let workchain = TLBType::read(parser)?;
        let address = parser.read_bits(addr_bits_len)?;
        Ok(Self {
            anycast,
            addr_bits_len,
            workchain,
            address,
        })
    }

    fn write_definition(&self, builder: &mut CellBuilder) -> Result<(), TonLibError> {
        self.anycast.write(builder)?;
        ConstLen::<_, 9>(self.addr_bits_len).write(builder)?;
        self.workchain.write(builder)?;
        builder.write_bits(&self.address, self.addr_bits_len)?;
        Ok(())
    }
}

/// Allows easily convert enum variants to parent type
#[rustfmt::skip]
mod from_impl {
    use crate::tlb::block_tlb::msg_address::*;
    impl From<MsgAddressNone> for MsgAddressExt { fn from(value: MsgAddressNone) -> Self { Self::None(value) } }
    impl From<MsgAddressExtern> for MsgAddressExt { fn from(value: MsgAddressExtern) -> Self { Self::Extern(value) } }
    impl From<MsgAddressIntStd> for MsgAddressInt { fn from(value: MsgAddressIntStd) -> Self { Self::Std(value) } }
    impl From<MsgAddressIntVar> for MsgAddressInt { fn from(value: MsgAddressIntVar) -> Self { Self::Var(value) } }
    impl From<MsgAddressInt> for MsgAddress { fn from(value: MsgAddressInt) -> Self { Self::Int(value) } }
    impl From<MsgAddressExt> for MsgAddress { fn from(value: MsgAddressExt) -> Self { Self::Ext(value) } }
    impl From<MsgAddressNone> for MsgAddress { fn from(value: MsgAddressNone) -> Self { Self::Ext(value.into()) } }
    impl From<MsgAddressExtern> for MsgAddress { fn from(value: MsgAddressExtern) -> Self { Self::Ext(value.into()) } }
    impl From<MsgAddressIntStd> for MsgAddress { fn from(value: MsgAddressIntStd) -> Self { Self::Int(value.into()) } }
    impl From<MsgAddressIntVar> for MsgAddress { fn from(value: MsgAddressIntVar) -> Self { Self::Int(value.into()) } }
}

#[derive(Debug, Clone, PartialEq, TLBDerive)]
pub struct Anycast {
    pub rewrite_pfx: VarLenBits<Vec<u8>, 5>,
}

impl Anycast {
    pub fn new(depth: u32, rewrite_pfx: Vec<u8>) -> Self {
        Self {
            rewrite_pfx: VarLenBits::new(rewrite_pfx, depth),
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
        let expected = MsgAddressIntStd {
            anycast: Some(Anycast::new(30, vec![3, 16, 83, 16])),
            workchain: 0,
            address: vec![
                77, 58, 155, 26, 56, 188, 179, 186, 88, 102, 247, 73, 204, 146, 79, 206, 132, 216, 123, 51, 95, 20,
                153, 234, 122, 207, 23, 115, 175, 20, 206, 237,
            ],
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

        let expected = MsgAddressIntStd {
            anycast: None,
            workchain: -1,
            address: vec![0; 32],
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

        let expected = MsgAddressIntStd {
            anycast: None,
            workchain: -1,
            address: vec![0; 32],
        };
        assert_eq!(parsed, expected.into());

        // don't support same layout, so check deserialized data again
        let serial_cell = parsed.to_cell()?;
        let parsed_back = assert_ok!(MsgAddressInt::from_cell(&serial_cell));
        assert_eq!(parsed, parsed_back);
        Ok(())
    }
}
