mod var_len_num;
mod var_len_vec;

use std::ops::{Deref, DerefMut};

pub type VarLenBits<T, const BITS_LEN: u32> = VarLen<T, BITS_LEN, false>;
pub type VarLenBytes<T, const BITS_LEN: u32> = VarLen<T, BITS_LEN, true>;

/// VarLen: store data len, and then data itself
///
/// BITS_LEN_LEN - number of bits used to store length
///
/// LEN_IN_BYTES - if true, data len is specified in bytes. Otherwise - in bits
#[derive(Debug, Clone, PartialEq)]
pub struct VarLen<T, const BITS_LEN: u32, const LEN_IN_BYTES: bool = false> {
    pub data: T,
    pub len: u32,
}

impl<T, const L: u32, const LEN_IN_BYTES: bool> VarLen<T, L, LEN_IN_BYTES> {
    pub fn new<D: Into<T>>(data: D, bits_len: u32) -> Self {
        Self {
            data: data.into(),
            len: if LEN_IN_BYTES { bits_len.div_ceil(8) } else { bits_len },
        }
    }
}

impl<T, const L: u32, const BL: bool> Deref for VarLen<T, L, BL> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.data }
}

impl<T, const L: u32, const BL: bool> DerefMut for VarLen<T, L, BL> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.data }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::tlb::tlb_type::TLBType;

    #[test]
    fn test_var_len() -> anyhow::Result<()> {
        // len in bits
        let obj = VarLen::<u32, 8>::new(1u8, 4);
        let cell = obj.to_cell()?;
        // 8 bits of length (value = 4) + 4 bits of data (value = 1)
        assert_eq!(&cell.data, &[0b00000100, 0b00010000]);
        let parsed = VarLen::<u32, 8>::from_cell(&cell)?;
        assert_eq!(obj, parsed);

        // len in bytes
        let obj = VarLen::<u32, 16, true>::new(1u8, 16);
        let cell = obj.to_cell()?;
        // 16 bits of length (value = 2), and then 16 (value * 8) bits of data (value = 1)
        assert_eq!(&cell.data, &[0b00000000, 0b00000010, 0b00000000, 0b00000001]);
        let parsed = VarLen::<u32, 16, true>::from_cell(&cell)?;
        assert_eq!(obj, parsed);

        Ok(())
    }
}
