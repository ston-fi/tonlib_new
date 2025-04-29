use base64_serde::base64_serde_type;
base64_serde_type!(pub Base64Standard, base64::engine::general_purpose::STANDARD);

pub mod serde_ton_hash_b64 {
    use crate::cell::ton_hash::TonHash;
    use serde::{de::Error, Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(hash: &TonHash, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(hash.to_b64().as_str())
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<TonHash, D::Error> {
        TonHash::from_b64(String::deserialize(deserializer)?).map_err(Error::custom)
    }
}

pub mod serde_ton_hash_vec_b64 {
    use crate::cell::ton_hash::TonHash;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(data: &[TonHash], serializer: S) -> Result<S::Ok, S::Error> {
        let b64_strings: Vec<String> = data.iter().map(|h| h.to_b64()).collect();
        b64_strings.serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<TonHash>, D::Error> {
        let b64_vec: Vec<String> = Vec::deserialize(deserializer)?;
        b64_vec.into_iter().map(|s| TonHash::from_b64(&s).map_err(serde::de::Error::custom)).collect()
    }
}

pub mod serde_ton_address_hex {
    use crate::types::ton_address::TonAddress;
    use serde::{de::Error, Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S: Serializer>(hash: &TonAddress, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(hash.to_hex().as_str())
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<TonAddress, D::Error> {
        TonAddress::from_str(&String::deserialize(deserializer)?).map_err(Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::ton_hash::TonHash;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[test]
    fn test_ton_hash_serde() -> anyhow::Result<()> {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct TestStruct {
            #[serde(with = "serde_ton_hash_b64")]
            hash: TonHash,
            #[serde(with = "serde_ton_hash_vec_b64")]
            hash_vec: Vec<TonHash>,
        }

        let val = TestStruct {
            hash: TonHash::from_slice([1u8; 32])?,
            hash_vec: vec![TonHash::from_slice([2u8; 32])?, TonHash::from_slice([3u8; 32])?],
        };
        let val_json = serde_json::to_string(&val)?;
        let expected = json!({
            "hash": "AQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQE=",
            "hash_vec": [
                "AgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgI=",
                "AwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwM="
            ]
        })
        .to_string();
        assert_eq!(val_json, expected);
        Ok(())
    }
}
