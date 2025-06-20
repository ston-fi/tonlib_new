use std::time::Duration;

pub struct ContractClientConfig {
    pub loop_error_sleep_duration: Duration,
    pub tx_ids: CacheUnit,
    pub state_by_tx: CacheUnit,
    pub state_by_address: CacheUnit,
}

pub struct CacheUnit {
    pub capacity: u64,
    pub ttl: Duration,
}
