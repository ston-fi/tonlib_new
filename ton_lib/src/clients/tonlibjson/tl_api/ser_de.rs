pub(super) mod serde_ton_address_hex {
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

pub(super) mod serde_ton_address_b64 {
    use crate::types::ton_address::TonAddress;
    use serde::{de::Error, Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S: Serializer>(hash: &TonAddress, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(hash.to_string().as_str())
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<TonAddress, D::Error> {
        TonAddress::from_str(&String::deserialize(deserializer)?).map_err(Error::custom)
    }
}

pub(super) mod serde_ton_hash_b64 {
    use crate::cell::ton_hash::TonHash;
    use serde::{de::Error, Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S: Serializer>(hash: &TonHash, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(hash.to_b64().as_str())
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<TonHash, D::Error> {
        TonHash::from_str(&String::deserialize(deserializer)?).map_err(Error::custom)
    }
}

pub(super) mod serde_ton_hash_vec_b64 {
    use crate::cell::ton_hash::TonHash;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::str::FromStr;

    pub fn serialize<S: Serializer>(data: &[TonHash], serializer: S) -> Result<S::Ok, S::Error> {
        let b64_strings: Vec<String> = data.iter().map(|h| h.to_b64()).collect();
        b64_strings.serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<TonHash>, D::Error> {
        let b64_vec: Vec<String> = Vec::deserialize(deserializer)?;
        b64_vec.into_iter().map(|s| TonHash::from_str(&s).map_err(serde::de::Error::custom)).collect()
    }
}

pub(super) mod serde_block_id_ext {
    use crate::cell::ton_hash::TonHash;
    use crate::clients::tonlibjson::tl_api::Base64Standard;
    use crate::types::tlb::block_tlb::block::block_id_ext::BlockIdExt;
    use crate::types::tlb::block_tlb::block::shard_ident::ShardIdent;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use serde_aux::prelude::deserialize_number_from_string;

    // tonlib_api.tl_api, line 51
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
    struct TLBlockIdExt {
        pub workchain: i32,
        #[serde(deserialize_with = "deserialize_number_from_string")]
        pub shard: i64,
        pub seqno: i32,
        #[serde(with = "Base64Standard")]
        pub root_hash: Vec<u8>,
        #[serde(with = "Base64Standard")]
        pub file_hash: Vec<u8>,
    }

    pub fn serialize<S: Serializer>(data: &BlockIdExt, serializer: S) -> Result<S::Ok, S::Error> {
        TLBlockIdExt {
            workchain: data.shard_id.workchain,
            shard: data.shard_id.shard as i64,
            seqno: data.seqno as i32,
            root_hash: data.root_hash.as_slice().to_vec(),
            file_hash: data.file_hash.as_slice().to_vec(),
        }
        .serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<BlockIdExt, D::Error> {
        let tl_block_id_ext = TLBlockIdExt::deserialize(deserializer)?;
        Ok(BlockIdExt {
            shard_id: ShardIdent {
                workchain: tl_block_id_ext.workchain,
                shard: tl_block_id_ext.shard as u64,
            },
            seqno: tl_block_id_ext.seqno as u32,
            root_hash: TonHash::from_vec(tl_block_id_ext.root_hash).map_err(serde::de::Error::custom)?,
            file_hash: TonHash::from_vec(tl_block_id_ext.file_hash).map_err(serde::de::Error::custom)?,
        })
    }
}

pub(super) mod serde_block_id_ext_vec {
    use crate::clients::tonlibjson::tl_api::ser_de::serde_block_id_ext;
    use crate::types::tlb::block_tlb::block::block_id_ext::BlockIdExt;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(data: &[BlockIdExt], serializer: S) -> Result<S::Ok, S::Error> {
        let tl_wrapped: Vec<_> = data
            .iter()
            .map(|b| serde_block_id_ext::serialize(b, serde_json::value::Serializer))
            .collect::<Result<_, _>>()
            .map_err(serde::ser::Error::custom)?;
        tl_wrapped.serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<BlockIdExt>, D::Error> {
        let values: Vec<serde_json::Value> = Vec::deserialize(deserializer)?;
        values
            .into_iter()
            .map(serde_block_id_ext::deserialize)
            .collect::<Result<_, _>>()
            .map_err(serde::de::Error::custom)
    }
}

pub(super) mod serde_block_id_ext_vec_opt {
    use crate::clients::tonlibjson::tl_api::ser_de::serde_block_id_ext_vec;
    use crate::types::tlb::block_tlb::block::block_id_ext::BlockIdExt;
    use serde::de::IntoDeserializer;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(data: &Option<Vec<BlockIdExt>>, serializer: S) -> Result<S::Ok, S::Error> {
        match data {
            Some(vec) => serde_block_id_ext_vec::serialize(vec, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Vec<BlockIdExt>>, D::Error> {
        let opt = Option::<Vec<serde_json::Value>>::deserialize(deserializer)?;
        match opt {
            Some(v) => {
                let vec =
                    serde_block_id_ext_vec::deserialize(v.into_deserializer()).map_err(serde::de::Error::custom)?;
                Ok(Some(vec))
            }
            None => Ok(None),
        }
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
            hash: TonHash::from_bytes(&[1u8; 32])?,
            hash_vec: vec![TonHash::from_bytes(&[2u8; 32])?, TonHash::from_bytes(&[3u8; 32])?],
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
