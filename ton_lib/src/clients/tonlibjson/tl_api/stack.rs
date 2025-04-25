use crate::cell::boc::BOC;
use crate::clients::tonlibjson::tl_api::Base64Standard;
use crate::errors::TonlibError;
use crate::types::tlb::tlb_type::TLBType;
use crate::types::ton_address::TonAddress;
use num_bigint::{BigInt, BigUint};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Deref;

// tonlib_api.tl_api, line 166
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct TLTvmSlice {
    #[serde(with = "Base64Standard")]
    pub bytes: Vec<u8>,
}

// tonlib_api.tl_api, line 167
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct TLTvmCell {
    #[serde(with = "Base64Standard")]
    pub bytes: Vec<u8>,
}

// tonlib_api.tl_api, line 168
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TLTvmNumber {
    pub number: String,
}

// tonlib_api.tl_api, line 169
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TLTvmTuple {
    pub elements: Vec<TLTvmStackEntry>,
}

// tonlib_api.tl_api, line 170
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TvmList {
    pub elements: Vec<TLTvmStackEntry>,
}

// tonlib_api.tl_api, line 172
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(tag = "@type")]
pub enum TLTvmStackEntry {
    // tonlib_api.tl_api, line 172
    #[serde(rename = "tvm.stackEntrySlice")]
    // tonlib_api.tl_api, line 173
    TLSlice { slice: TLTvmSlice },
    #[serde(rename = "tvm.stackEntryCell")]
    TLCell { cell: TLTvmCell },
    // tonlib_api.tl_api, line 174
    #[serde(rename = "tvm.stackEntryNumber")]
    TLNumber { number: TLTvmNumber },
    // tonlib_api.tl_api, line 175
    #[serde(rename = "tvm.stackEntryTuple")]
    TLTuple { tuple: TLTvmTuple },
    // tonlib_api.tl_api, line 176
    #[serde(rename = "tvm.stackEntryList")]
    TLList { list: TvmList },
    // tonlib_api.tl_api, line 177
    #[serde(rename = "tvm.stackEntryUnsupported")]
    TLUnsupported {},
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TLTvmStack {
    pub elements: Vec<TLTvmStackEntry>,
}

impl<'de> Deserialize<'de> for TLTvmStack {
    fn deserialize<D>(deserializer: D) -> Result<TLTvmStack, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|e| TLTvmStack { elements: e })
    }
}

impl Serialize for TLTvmStack {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.elements.serialize(serializer)
    }
}

impl TLTvmStack {
    pub fn new() -> TLTvmStack { TLTvmStack { elements: Vec::new() } }

    pub fn from(elements: &[TLTvmStackEntry]) -> TLTvmStack {
        TLTvmStack {
            elements: elements.to_vec(),
        }
    }

    pub fn get_string(&self, index: usize) -> Result<String, TonlibError> {
        self.get(index, TLTvmStack::extract_string)
    }

    pub fn get_i32(&self, index: usize) -> Result<i32, TonlibError> { self.get(index, TLTvmStack::extract_i32) }

    pub fn get_i64(&self, index: usize) -> Result<i64, TonlibError> { self.get(index, TLTvmStack::extract_i64) }

    pub fn get_biguint(&self, index: usize) -> Result<BigUint, TonlibError> {
        self.get(index, TLTvmStack::extract_biguint)
    }

    pub fn get_bigint(&self, index: usize) -> Result<BigInt, TonlibError> {
        self.get(index, TLTvmStack::extract_bigint)
    }

    pub fn get_boc(&self, index: usize) -> Result<BOC, TonlibError> { self.get(index, TLTvmStack::extract_boc) }

    pub fn get_address(&self, index: usize) -> Result<TonAddress, TonlibError> {
        TonAddress::from_cell(self.get_boc(index)?.single_root()?.deref())
    }

    // pub fn get_dict<K, V>(
    //     &self,
    //     index: usize,
    //     key_size: usize,
    //     key_reader: KeyReader<K>,
    //     val_reader: ValReader<V>,
    // ) -> Result<HashMap<K, V>, TonLibError>
    // where
    //     K: Hash + Eq + Clone,
    // {
    //     todo!();
    //     // let boc = self.get_boc(index)?;
    //     // let cell = boc.single_root()?;
    //     // let mut parser = cell.parser();
    //     // parser.load_dict(key_size, key_reader, val_reader)
    // }

    fn get<T>(
        &self,
        index: usize,
        _extract: fn(&TLTvmStackEntry, usize) -> Result<T, TonlibError>,
    ) -> Result<T, TonlibError> {
        let maybe_elem = self.elements.get(index);
        todo!();
        // match maybe_elem {
        //     None => Err(TvmStackError::InvalidTvmStackIndex {
        //         index,
        //         len: self.elements.len(),
        //     }),
        //     Some(e) => extract(e, index),
        // }
    }

    fn extract_string(entry: &TLTvmStackEntry, index: usize) -> Result<String, TonlibError> {
        if let TLTvmStackEntry::TLNumber { number } = entry {
            Ok(number.number.to_string())
        } else {
            Err(TonlibError::TvmStackError(format!(
                "Unsupported conversion to BigInt (TvmStackEntry: {entry:?}, index: {index})"
            )))
        }
    }

    fn extract_i32(entry: &TLTvmStackEntry, index: usize) -> Result<i32, TonlibError> {
        if let TLTvmStackEntry::TLNumber { number } = entry {
            Ok(number.number.parse()?)
        } else {
            Err(TonlibError::TvmStackError(format!(
                "Unsupported conversion to BigInt (TvmStackEntry: {entry:?}, index: {index})"
            )))
        }
    }

    fn extract_i64(entry: &TLTvmStackEntry, index: usize) -> Result<i64, TonlibError> {
        if let TLTvmStackEntry::TLNumber { number } = entry {
            Ok(number.number.parse()?)
        } else {
            Err(TonlibError::TvmStackError(format!(
                "Unsupported conversion to i64 (TvmStackEntry: {entry:?}, index: {index})"
            )))
        }
    }

    fn extract_biguint(entry: &TLTvmStackEntry, index: usize) -> Result<BigUint, TonlibError> {
        if let TLTvmStackEntry::TLNumber { number } = entry {
            Ok(number.number.parse()?)
        } else {
            Err(TonlibError::TvmStackError(format!(
                "Unsupported conversion to BigUInt (TvmStackEntry: {entry:?}, index: {index})"
            )))
        }
    }

    fn extract_bigint(entry: &TLTvmStackEntry, index: usize) -> Result<BigInt, TonlibError> {
        if let TLTvmStackEntry::TLNumber { number } = entry {
            Ok(number.number.parse()?)
        } else {
            Err(TonlibError::TvmStackError(format!(
                "Unsupported conversion to BigInt (TvmStackEntry: {entry:?}, index: {index})"
            )))
        }
    }

    fn extract_boc(entry: &TLTvmStackEntry, index: usize) -> Result<BOC, TonlibError> {
        match entry {
            TLTvmStackEntry::TLSlice { slice } => BOC::from_bytes(&slice.bytes),
            TLTvmStackEntry::TLCell { cell } => BOC::from_bytes(&cell.bytes),
            _ => Err(TonlibError::TvmStackError(format!("Unsupported BOC type: {entry:?}, index: {index}"))),
        }
    }
}

impl Default for TLTvmStack {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use crate::clients::tonlibjson::tl_api::stack::{TLTvmNumber, TLTvmStack, TLTvmStackEntry};

    const SERIAL: &str = r#"[{"@type":"tvm.stackEntryNumber","number":{"number":"100500"}}]"#;

    #[test]
    fn serialize_works() {
        let mut stack = TLTvmStack::default();
        stack.elements.push(TLTvmStackEntry::TLNumber {
            number: TLTvmNumber {
                number: String::from("100500"),
            },
        });
        let serial = serde_json::to_string(&stack).unwrap();
        println!("{}", serial);
        assert_eq!(serial.as_str(), SERIAL)
    }

    #[test]
    fn deserialize_works() {
        let stack: TLTvmStack = serde_json::from_str(SERIAL).unwrap();
        assert_eq!(stack.elements.len(), 1);
        assert_eq!(100500, stack.get_i32(0).unwrap());
    }
}
