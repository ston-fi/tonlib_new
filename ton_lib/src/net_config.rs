use crate::errors::TonLibError;
use serde::Deserialize;

pub const TON_NET_CONF_MAINNET: &str = include_str!("../resources/net_config/mainnet.json");
pub const TON_NET_CONF_TESTNET: &str = include_str!("../resources/net_config/testnet.json");

// can't use ConfigLiteServer directly because it doesn't implement Clone
#[derive(Debug, Clone, Deserialize)]
pub struct LiteID {
    pub key: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LiteEndpoint {
    pub ip: i32,
    pub port: u16,
    pub id: LiteID,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TonNetConf {
    #[serde(rename = "liteservers")]
    pub lite_endpoints: Vec<LiteEndpoint>,
}

impl TonNetConf {
    pub fn new(json: &str) -> Result<Self, TonLibError> { Ok(serde_json::from_str(json)?) }
}
