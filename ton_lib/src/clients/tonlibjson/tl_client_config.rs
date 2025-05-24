use crate::clients::net_config::TonNetConfig;
use crate::clients::tonlibjson::tl_api::tl_types::{TLConfig, TLKeyStoreType, TLOptions};
use crate::clients::tonlibjson::tl_callback::TLCallbacksStore;
use std::time::Duration;

#[derive(Debug, PartialEq)]
pub enum LiteNodeFilter {
    Health,  // connected only to healthy node
    Archive, // connected only to archive node
}

pub struct TLClientRetryStrategy {
    pub retry_count: usize,
    pub retry_waiting: Duration,
}

pub struct TLClientConfig {
    pub init_opts: TLOptions,
    pub connection_check: LiteNodeFilter,
    pub connections_count: usize,
    pub max_parallel_requests: usize, // => (max_parallel_requests / connections_count) parallel requests per connection
    pub retry_strategy: TLClientRetryStrategy,
    pub update_init_block: bool,
    pub update_init_block_timeout_sec: u64,
    pub tonlib_verbosity_level: u32,
    pub callbacks: TLCallbacksStore,
}

impl TLClientConfig {
    pub fn new(net_config: String, archive_only: bool) -> TLClientConfig {
        let connection_check = match archive_only {
            true => LiteNodeFilter::Archive,
            false => LiteNodeFilter::Health,
        };
        TLClientConfig {
            init_opts: TLOptions {
                config: TLConfig {
                    net_config,
                    blockchain_name: None,
                    use_callbacks_for_network: false,
                    ignore_cache: false,
                },
                keystore_type: TLKeyStoreType::Directory {
                    directory: "/tmp/tonlibjson_keystore".to_string(),
                },
            },
            connection_check,
            connections_count: 10,
            max_parallel_requests: 200,
            retry_strategy: TLClientRetryStrategy {
                retry_count: 10,
                retry_waiting: Duration::from_millis(10),
            },
            update_init_block: true,
            update_init_block_timeout_sec: 10,
            tonlib_verbosity_level: 1,
            callbacks: TLCallbacksStore::default(),
        }
    }
    pub fn new_mainnet(archive_only: bool) -> TLClientConfig {
        TLClientConfig::new(TonNetConfig::get_json(true), archive_only)
    }
    pub fn new_testnet(archive_only: bool) -> TLClientConfig {
        TLClientConfig::new(TonNetConfig::get_json(false), archive_only)
    }
}
