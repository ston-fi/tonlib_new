use crate::cell::ton_hash::TonHash;
use crate::errors::TonLibError;
use num_bigint::BigUint;

pub trait DictKeyAdapter<K> {
    fn make_key(src_key: &K) -> Result<BigUint, TonLibError>;
    fn extract_key(dict_key: &BigUint) -> Result<K, TonLibError>;
}

pub struct DictKeyAdapterTonHash; // TODO is not covered by tests
pub struct DictKeyAdapterInto;
pub struct DictKeyAdapterString; // TODO is not covered by tests

impl DictKeyAdapter<TonHash> for DictKeyAdapterTonHash {
    fn make_key(src_key: &TonHash) -> Result<BigUint, TonLibError> { Ok(BigUint::from_bytes_le(src_key.as_slice())) }

    fn extract_key(dict_key: &BigUint) -> Result<TonHash, TonLibError> {
        let mut hash_bytes = vec![0; TonHash::BYTES_LEN];
        let key_bytes = dict_key.to_bytes_le();
        if key_bytes.len() > TonHash::BYTES_LEN {
            return Err(TonLibError::TLBDictWrongKeyLen {
                exp: TonHash::BYTES_LEN,
                got: key_bytes.len(),
                key: dict_key.clone(),
            });
        }
        let offset = TonHash::BYTES_LEN - key_bytes.len();
        hash_bytes.as_mut_slice()[offset..].copy_from_slice(key_bytes.as_slice());
        TonHash::from_bytes(hash_bytes)
    }
}

impl<T: Clone + Into<BigUint> + TryFrom<BigUint>> DictKeyAdapter<T> for DictKeyAdapterInto {
    fn make_key(src_key: &T) -> Result<BigUint, TonLibError> { Ok(src_key.clone().into()) }

    fn extract_key(dict_key: &BigUint) -> Result<T, TonLibError> {
        match T::try_from(dict_key.clone()) {
            Ok(key) => Ok(key),
            Err(_) => {
                // TODO cleanup error handling
                Err(TonLibError::CustomError("fail to extract dict key".to_string()))
            }
        }
    }
}

impl DictKeyAdapter<String> for DictKeyAdapterString {
    fn make_key(src_key: &String) -> Result<BigUint, TonLibError> {
        let bytes = src_key.as_bytes();
        Ok(BigUint::from_bytes_le(bytes))
    }

    fn extract_key(dict_key: &BigUint) -> Result<String, TonLibError> {
        let bytes = dict_key.to_bytes_le();
        Ok(String::from_utf8(bytes)?)
    }
}
