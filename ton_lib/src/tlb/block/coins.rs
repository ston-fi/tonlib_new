use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::cell::ton_cell::TonCellRef;
use crate::errors::TonLibError;
use crate::tlb::primitives::dyn_len::VarLen;
use crate::tlb::tlb_type::TLBPrefix;
use crate::tlb::tlb_type::TLBType;
#[cfg(feature = "num-bigint")]
use num_bigint::BigUint;
use ton_lib_proc_macro::TLBDerive;

/// https://github.com/ton-blockchain/ton/blob/050a984163a53df16fb03f66cc445c34bfed48ed/crypto/block/block.tlb#L116
#[derive(Clone, Debug, PartialEq, TLBDerive)]
pub struct Grams {
    pub amount: VarLen<BigUint, 4, true>,
}

/// https://github.com/ton-blockchain/ton/blob/050a984163a53df16fb03f66cc445c34bfed48ed/crypto/block/block.tlb#L124
#[derive(Clone, Debug, PartialEq, TLBDerive)]
pub struct CurrencyCollection {
    pub grams: Grams,
    pub other: Option<TonCellRef>, // dict, but it's equal to Option<TonCellRef> in tlb format
}

impl Grams {
    pub fn new<T: Into<BigUint>>(amount: T) -> Self {
        Self {
            amount: (4, amount.into()).into(),
        }
    }
}

impl<T: Into<BigUint>> From<T> for Grams {
    fn from(amount: T) -> Self { Self::new(amount) }
}

impl CurrencyCollection {
    pub fn new<T: Into<BigUint>>(grams: T) -> Self {
        Self {
            grams: Grams::new(grams),
            other: None,
        }
    }
}

impl<T: Into<BigUint>> From<T> for CurrencyCollection {
    fn from(amount: T) -> Self { Self::new(amount) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tlb::block::coins::CurrencyCollection;

    #[test]
    fn test_currency_collection() -> anyhow::Result<()> {
        let parsed = CurrencyCollection::from_boc_hex("b5ee9c720101010100070000094c143b1d14")?;
        assert_eq!(*parsed.grams.amount, 3242439121u32.into());

        let cell_serial = parsed.to_cell()?;
        let parsed_back = CurrencyCollection::from_cell(&cell_serial)?;
        assert_eq!(parsed, parsed_back);
        Ok(())
    }

    #[test]
    fn test_currency_collection_zero_grams() -> anyhow::Result<()> {
        let currency = CurrencyCollection::new(0u32);
        let cell = currency.to_cell()?;
        let parsed = CurrencyCollection::from_cell(&cell)?;
        assert_eq!(*parsed.grams.amount, 0u32.into());

        let cell_serial = parsed.to_cell()?;
        assert_eq!(cell_serial, cell);
        Ok(())
    }
}
