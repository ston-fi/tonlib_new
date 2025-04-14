use crate::cell::ton_cell::TonCellRef;
use crate::cell::ton_hash::TonHash;
use crate::errors::TonLibError;
use crate::errors::TonLibError::TonAddressParseError;
use crate::tlb::block_tlb::state_init::StateInit;
use crate::tlb::tlb_type::TLBType;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::Engine;
use crc::Crc;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TonAddress {
    pub wc: i32,
    pub hash: TonHash,
}

impl TonAddress {
    pub fn new(wc: i32, hash: TonHash) -> Self { Self { wc, hash } }

    pub fn derive(wc: i32, code: TonCellRef, data: TonCellRef) -> Result<TonAddress, TonLibError> {
        let state_init = StateInit::new(code, data);
        Ok(TonAddress::new(wc, state_init.cell_hash()?))
    }

    pub fn from_b64<T: AsRef<str>>(addr: T) -> Result<TonAddress, TonLibError> {
        let addr_str = addr.as_ref();
        if addr_str.chars().any(|c| c == '-' || c == '_') {
            TonAddress::from_bytes(&URL_SAFE_NO_PAD.decode(addr_str)?, addr_str)
        } else {
            TonAddress::from_bytes(&STANDARD.decode(addr_str)?, addr_str)
        }
    }

    pub fn from_hex<T: AsRef<str>>(addr: T) -> Result<TonAddress, TonLibError> {
        let addr_str = addr.as_ref();
        let parts: Vec<&str> = addr_str.split(':').collect();

        if parts.len() != 2 {
            raise_address_error(addr_str, "expecting 2 parts divided by ':'")?;
        }

        let wc = parts[0].parse::<i32>()?;

        let hash = TonHash::from_vec(hex::decode(parts[1])?)?;
        Ok(TonAddress::new(wc, hash))
    }

    pub fn from_bytes(bytes: &[u8], addr_str: &str) -> Result<Self, TonLibError> {
        const CRC_16_XMODEM: Crc<u16> = Crc::<u16>::new(&crc::CRC_16_XMODEM);

        if bytes.len() != 36 {
            raise_address_error(addr_str, format!("expecting 36 bytes, got {}", bytes.len()))?;
        }

        let addr_crc = ((bytes[34] as u16) << 8) | bytes[35] as u16;
        if addr_crc != CRC_16_XMODEM.checksum(&bytes[0..34]) {
            raise_address_error(addr_str, "crc32 mismatch")?;
        }

        let address = TonAddress {
            wc: bytes[1] as i8 as i32,
            hash: TonHash::from_bytes(&bytes[2..34])?,
        };
        Ok(address)
    }
}

impl FromStr for TonAddress {
    type Err = TonLibError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 48 {
            return TonAddress::from_b64(s);
        }
        TonAddress::from_hex(s)
    }
}

fn raise_address_error<T: AsRef<str>>(address: &str, msg: T) -> Result<(), TonLibError> {
    Err(TonAddressParseError(address.to_string(), msg.as_ref().to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boc::boc::BOC;

    #[test]
    fn test_ton_address_derive_stonfi_pool() -> anyhow::Result<()> {
        let code_cell = BOC::from_hex(
            "b5ee9c7201010101002300084202a9338ecd624ca15d37e4a8d9bf677ddc9b84f0e98f05f2fb84c7afe332a281b4",
        )?
        .single_root()?;
        let data_cell = BOC::from_hex("b5ee9c720101040100b900010d000000000000050102c9801459f7c0a12bb4ac4b78a788c425ee4d52f8b6041dda17b77b09fc5a03e894d6900287cd9fbe2ea663415da0aa6bbdf0cb136abe9c4f45214dd259354b80da8c265a006aebb27f5d0f1daf43e200f52408f3eb9ff5610f5b43284224644e7c6a590d14400203084202c00836440d084e44fb94316132ac5a21417ef4f429ee09b5560b5678b334c3e8084202c95a2ed22ab516f77f9d4898dc4578e72f18a2448e8f6832334b0b4bf501bc79")?.single_root()?;
        let address = TonAddress::derive(0, code_cell, data_cell)?;
        let exp_addr = TonAddress::from_str("EQAdltEfzXG_xteLFaKFGd-HPVKrEJqv_FdC7z2roOddRNdM")?;
        assert_eq!(address, exp_addr);
        Ok(())
    }
}
