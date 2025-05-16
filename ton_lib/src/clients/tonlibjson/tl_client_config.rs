use crate::clients::tonlibjson::tl_api::tl_types::{TLConfig, TLKeyStoreType, TLOptions};
use crate::clients::tonlibjson::tl_callback::TLCallbacksStore;
use crate::net_config::{TON_NET_CONF_MAINNET, TON_NET_CONF_TESTNET};

#[derive(Debug, PartialEq)]
pub enum LiteNodeFilter {
    Health,  // connected only to healthy node
    Archive, // connected only to archive node
}

pub struct TLClientConfig {
    pub init_opts: TLOptions,
    pub connection_check: LiteNodeFilter,
    pub connections_count: usize,
    pub max_parallel_requests: usize, // => (max_parallel_requests / connections_count) parallel requests per connection
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
            max_parallel_requests: 1000,
            update_init_block: true,
            update_init_block_timeout_sec: 10,
            tonlib_verbosity_level: 1,
            callbacks: TLCallbacksStore::default(),
        }
    }
    pub fn new_mainnet(archive_only: bool) -> TLClientConfig {
        TLClientConfig::new(TON_NET_CONF_MAINNET.to_string(), archive_only)
    }
    pub fn new_testnet(archive_only: bool) -> TLClientConfig {
        TLClientConfig::new(TON_NET_CONF_TESTNET.to_string(), archive_only)
    }
}
