use crate::errors::TonLibError;
use std::fmt::{Display, UpperHex};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Hash, Eq, Ord, PartialOrd)]
pub struct TonHash(TonHashData);

#[derive(Debug, PartialOrd, Ord, Clone)]
enum TonHashData {
    Slice([u8; 32]),
    Vec(Vec<u8>),
}

impl TonHash {
    pub const BYTES_LEN: usize = 32;
    pub const BITS_LEN: usize = 256;
    pub const ZERO: TonHash = TonHash(TonHashData::Slice([0u8; 32]));
    pub const EMPTY_CELL_HASH: TonHash = TonHash(TonHashData::Slice([
        150, 162, 150, 210, 36, 242, 133, 198, 123, 238, 147, 195, 15, 138, 48, 145, 87, 240, 218, 163, 93, 197, 184,
        126, 65, 11, 120, 99, 10, 9, 207, 199,
    ]));

    pub fn from_slice<T: AsRef<[u8]>>(data: T) -> Result<Self, TonLibError> {
        let bytes = data.as_ref();
        check_bytes_len(bytes)?;
        Ok(Self(TonHashData::Slice(bytes[..32].try_into().unwrap())))
    }

    pub fn from_vec(data: Vec<u8>) -> Result<Self, TonLibError> {
        check_bytes_len(&data)?;
        Ok(Self(TonHashData::Vec(data)))
    }

    pub fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, TonLibError> {
        let bytes = hex::decode(hex)?;
        check_bytes_len(&bytes)?;
        Ok(Self(TonHashData::Vec(bytes)))
    }

    pub fn as_slice(&self) -> &[u8] { self.0.as_slice() }

    pub fn as_slice_sized(&self) -> &[u8; 32] {
        match &self.0 {
            TonHashData::Slice(data) => data,
            TonHashData::Vec(data) => data.as_slice().try_into().unwrap(),
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        match &mut self.0 {
            TonHashData::Slice(data) => data,
            TonHashData::Vec(data) => data.as_mut_slice(),
        }
    }

    pub fn as_hex(&self) -> String { hex::encode(self.as_slice()) }

    pub fn into_vec(self) -> Vec<u8> {
        match self.0 {
            TonHashData::Slice(data) => data.to_vec(),
            TonHashData::Vec(data) => data,
        }
    }
}

impl TonHashData {
    fn as_slice(&self) -> &[u8] {
        match self {
            TonHashData::Slice(data) => data.as_slice(),
            TonHashData::Vec(data) => data.as_slice(),
        }
    }
}

fn check_bytes_len(bytes: &[u8]) -> Result<(), TonLibError> {
    if bytes.len() != TonHash::BYTES_LEN {
        return Err(TonLibError::TonHashWrongLen {
            exp: TonHash::BYTES_LEN,
            given: bytes.len(),
        });
    }
    Ok(())
}

// Must implement it manually, because we don't distinguish between Vec and Slice
impl PartialEq for TonHashData {
    fn eq(&self, other: &Self) -> bool { self.as_slice() == other.as_slice() }
}

impl Eq for TonHashData {}

impl Hash for TonHashData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { state.write(self.as_slice()); }
}

impl From<[u8; 32]> for TonHash {
    fn from(data: [u8; 32]) -> Self { Self(TonHashData::Slice(data)) }
}

impl AsRef<[u8]> for TonHash {
    fn as_ref(&self) -> &[u8] { self.as_slice() }
}

impl UpperHex for TonHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.as_hex().to_uppercase()) }
}

impl Display for TonHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{self:X}") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ton_hash_display() -> anyhow::Result<()> {
        let data = [255u8; 32];
        let hash = TonHash::from(data);
        assert_eq!(format!("{}", hash), "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
        assert_eq!(format!("{:X}", hash), "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
        Ok(())
    }

    #[test]
    fn test_ton_hash_from_slice() -> anyhow::Result<()> {
        let data = [1u8; 32];
        let hash = TonHash::from(data);
        assert_eq!(hash.as_slice(), &data);
        Ok(())
    }

    #[test]
    fn test_ton_hash_from_bytes() -> anyhow::Result<()> {
        let data = [1u8; 32];
        let hash = TonHash::from_slice(data)?;
        assert_eq!(hash.as_slice(), &data);
        Ok(())
    }

    #[test]
    fn test_ton_hash_from_vec() -> anyhow::Result<()> {
        let data = [1u8; 32];
        let hash = TonHash::from_slice(data)?;
        assert_eq!(hash.as_slice(), &data);
        Ok(())
    }

    #[test]
    fn test_ton_hash_from_hex() -> anyhow::Result<()> {
        let data = [255u8; 32];
        let hash = TonHash::from_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF")?;
        assert_eq!(hash.as_slice(), &data);
        Ok(())
    }
}
