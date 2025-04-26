use crate::errors::TonlibError;
use crate::types::tlb::block_tlb::block::BlockIdExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const TON_NET_CONF_MAINNET: &str = include_str!("../resources/net_config/mainnet.json");
pub const TON_NET_CONF_TESTNET: &str = include_str!("../resources/net_config/testnet.json");

// can't use ConfigLiteServer directly because it doesn't implement Clone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiteID {
    #[serde(rename = "@type")]
    pub config_type: Value,
    pub key: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiteEndpoint {
    pub ip: i32,
    pub port: u16,
    pub id: LiteID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validator {
    #[serde(rename = "@type")]
    pub config_type: Value,
    pub zero_state: Value,
    pub init_block: Value,
    pub hardforks: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TonNetConfig {
    #[serde(rename = "@type")]
    pub conf_type: Value,
    pub dht: Value,
    #[serde(rename = "liteservers")]
    pub lite_endpoints: Vec<LiteEndpoint>,
    pub validator: Validator,
}

impl TonNetConfig {
    pub fn new(json: &str) -> Result<Self, TonlibError> { Ok(serde_json::from_str(json)?) }

    pub fn to_json(&self) -> Result<String, serde_json::Error> { serde_json::to_string(self) }

    pub fn get_init_block_seqno(&self) -> u64 { self.validator.init_block["seqno"].as_u64().unwrap_or(0) }

    pub fn set_init_block(&mut self, block_id: &BlockIdExt) {
        self.validator.init_block["workchain"] = serde_json::json!(block_id.shard_id.workchain);
        self.validator.init_block["shard"] = serde_json::json!(block_id.shard_id.shard as i64);
        self.validator.init_block["seqno"] = serde_json::json!(block_id.seqno);
        self.validator.init_block["root_hash"] = serde_json::json!(block_id.root_hash.to_b64());
        self.validator.init_block["file_hash"] = serde_json::json!(block_id.file_hash.to_b64());
    }
}
