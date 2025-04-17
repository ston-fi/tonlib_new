use std::time::Duration;

use crate::errors::TonLibError;
use crate::net_config::TonNetConf;

#[derive(Debug, Clone)]
pub struct LiteClientConfig {
    pub net_conf: TonNetConf,
    pub connections_per_node: u32,
    pub conn_timeout: Duration,
    pub retry_count: u32,
    pub retry_waiting: Duration,
    pub query_timeout: Duration,
    pub last_seqno_polling_period: Duration,
    pub metrics_enabled: bool,
}

impl LiteClientConfig {
    pub fn new(json: &str) -> Result<Self, TonLibError> {
        Ok(Self {
            net_conf: TonNetConf::new(json)?,
            connections_per_node: 1,
            conn_timeout: Duration::from_millis(500),
            retry_count: 10,
            retry_waiting: Duration::from_millis(100),
            query_timeout: Duration::from_millis(5000),
            last_seqno_polling_period: Duration::from_millis(5000),
            metrics_enabled: true,
        })
    }
}
