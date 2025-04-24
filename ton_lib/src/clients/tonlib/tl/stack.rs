use crate::cell::boc::BOC;
use crate::clients::tonlib::tl::Base64Standard;
use crate::errors::TonLibError;
use crate::tlb::tlb_type::TLBType;
use crate::types::ton_address::TonAddress;
use num_bigint::{BigInt, BigUint};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::ops::Deref;

// tonlib_api.tl, line 166
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct TvmSlice {
    #[serde(with = "Base64Standard")]
    pub bytes: Vec<u8>,
}

impl Debug for TvmSlice {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TvmSlice{{ bytes: [{}]}}",
            self.bytes.iter().map(|&byte| format!("{:02X}", byte)).collect::<Vec<_>>().join(""),
        )?;
        Ok(())
    }
}

// tonlib_api.tl, line 167
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct TvmCell {
    #[serde(with = "Base64Standard")]
    pub bytes: Vec<u8>,
}

impl Debug for TvmCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Print bytes as a hexadecimal string
        write!(f, "TvmCell {{ bytes: 0x")?;

        for byte in &self.bytes {
            write!(f, "{:02x}", byte)?;
        }

        write!(f, " }}")
    }
}

// tonlib_api.tl, line 168
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TvmNumber {
    pub number: String,
}

// tonlib_api.tl, line 169
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TvmTuple {
    pub elements: Vec<TvmStackEntry>,
}

// tonlib_api.tl, line 170
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TvmList {
    pub elements: Vec<TvmStackEntry>,
}

// tonlib_api.tl, line 172
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(tag = "@type")]
pub enum TvmStackEntry {
    // tonlib_api.tl, line 172
    #[serde(rename = "tvm.stackEntrySlice")]
    // tonlib_api.tl, line 173
    Slice { slice: TvmSlice },
    #[serde(rename = "tvm.stackEntryCell")]
    Cell { cell: TvmCell },
    // tonlib_api.tl, line 174
    #[serde(rename = "tvm.stackEntryNumber")]
    Number { number: TvmNumber },
    // tonlib_api.tl, line 175
    #[serde(rename = "tvm.stackEntryTuple")]
    Tuple { tuple: TvmTuple },
    // tonlib_api.tl, line 176
    #[serde(rename = "tvm.stackEntryList")]
    List { list: TvmList },
    // tonlib_api.tl, line 177
    #[serde(rename = "tvm.stackEntryUnsupported")]
    Unsupported {},
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TvmStack {
    pub elements: Vec<TvmStackEntry>,
}

impl<'de> Deserialize<'de> for TvmStack {
    fn deserialize<D>(deserializer: D) -> Result<TvmStack, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|e| TvmStack { elements: e })
    }
}

impl Serialize for TvmStack {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.elements.serialize(serializer)
    }
}

impl TvmStack {
    pub fn new() -> TvmStack { TvmStack { elements: Vec::new() } }

    pub fn from(elements: &[TvmStackEntry]) -> TvmStack {
        TvmStack {
            elements: elements.to_vec(),
        }
    }

    pub fn get_string(&self, index: usize) -> Result<String, TonLibError> { self.get(index, TvmStack::extract_string) }

    pub fn get_i32(&self, index: usize) -> Result<i32, TonLibError> { self.get(index, TvmStack::extract_i32) }

    pub fn get_i64(&self, index: usize) -> Result<i64, TonLibError> { self.get(index, TvmStack::extract_i64) }

    pub fn get_biguint(&self, index: usize) -> Result<BigUint, TonLibError> {
        self.get(index, TvmStack::extract_biguint)
    }

    pub fn get_bigint(&self, index: usize) -> Result<BigInt, TonLibError> { self.get(index, TvmStack::extract_bigint) }

    pub fn get_boc(&self, index: usize) -> Result<BOC, TonLibError> { self.get(index, TvmStack::extract_boc) }

    pub fn get_address(&self, index: usize) -> Result<TonAddress, TonLibError> {
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
        _extract: fn(&TvmStackEntry, usize) -> Result<T, TonLibError>,
    ) -> Result<T, TonLibError> {
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

    fn extract_string(entry: &TvmStackEntry, index: usize) -> Result<String, TonLibError> {
        if let TvmStackEntry::Number { number } = entry {
            Ok(number.number.to_string())
        } else {
            Err(TonLibError::TvmStackError(format!(
                "Unsupported conversion to BigInt (TvmStackEntry: {entry:?}, index: {index})"
            )))
        }
    }

    fn extract_i32(entry: &TvmStackEntry, index: usize) -> Result<i32, TonLibError> {
        if let TvmStackEntry::Number { number } = entry {
            Ok(number.number.parse()?)
        } else {
            Err(TonLibError::TvmStackError(format!(
                "Unsupported conversion to BigInt (TvmStackEntry: {entry:?}, index: {index})"
            )))
        }
    }

    fn extract_i64(entry: &TvmStackEntry, index: usize) -> Result<i64, TonLibError> {
        if let TvmStackEntry::Number { number } = entry {
            Ok(number.number.parse()?)
        } else {
            Err(TonLibError::TvmStackError(format!(
                "Unsupported conversion to i64 (TvmStackEntry: {entry:?}, index: {index})"
            )))
        }
    }

    fn extract_biguint(entry: &TvmStackEntry, index: usize) -> Result<BigUint, TonLibError> {
        if let TvmStackEntry::Number { number } = entry {
            Ok(number.number.parse()?)
        } else {
            Err(TonLibError::TvmStackError(format!(
                "Unsupported conversion to BigUInt (TvmStackEntry: {entry:?}, index: {index})"
            )))
        }
    }

    fn extract_bigint(entry: &TvmStackEntry, index: usize) -> Result<BigInt, TonLibError> {
        if let TvmStackEntry::Number { number } = entry {
            Ok(number.number.parse()?)
        } else {
            Err(TonLibError::TvmStackError(format!(
                "Unsupported conversion to BigInt (TvmStackEntry: {entry:?}, index: {index})"
            )))
        }
    }

    fn extract_boc(entry: &TvmStackEntry, index: usize) -> Result<BOC, TonLibError> {
        match entry {
            TvmStackEntry::Slice { slice } => BOC::from_bytes(&slice.bytes),
            TvmStackEntry::Cell { cell } => BOC::from_bytes(&cell.bytes),
            _ => Err(TonLibError::TvmStackError(format!("Unsupported BOC type: {entry:?}, index: {index}"))),
        }
    }
}

impl Default for TvmStack {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use crate::clients::tonlib::tl::stack::{TvmNumber, TvmStack, TvmStackEntry};

    const SERIAL: &str = r#"[{"@type":"tvm.stackEntryNumber","number":{"number":"100500"}}]"#;

    #[test]
    fn serialize_works() {
        let mut stack = TvmStack::default();
        stack.elements.push(TvmStackEntry::Number {
            number: TvmNumber {
                number: String::from("100500"),
            },
        });
        let serial = serde_json::to_string(&stack).unwrap();
        println!("{}", serial);
        assert_eq!(serial.as_str(), SERIAL)
    }

    #[test]
    fn deserialize_works() {
        let stack: TvmStack = serde_json::from_str(SERIAL).unwrap();
        assert_eq!(stack.elements.len(), 1);
        assert_eq!(100500, stack.get_i32(0).unwrap());
    }
}
