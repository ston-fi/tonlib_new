use crate::clients::tonlibjson::tl_api::tl_types::{TLConfig, TLKeyStoreType, TLOptions};
use crate::net_config::{TON_NET_CONF_MAINNET, TON_NET_CONF_TESTNET};

#[derive(Debug, PartialEq)]
pub enum LiteNodeFilter {
    Health,  // connected only to healthy node
    Archive, // connected only to archive node
}

pub struct TLJClientConfig {
    pub init_opts: TLOptions,
    pub connection_check: LiteNodeFilter,
    pub connections_count: usize,
    pub max_parallel_requests: usize, // => (max_parallel_requests / connections_count) parallel requests per connection
    pub update_init_block: bool,
    pub update_init_block_timeout_sec: u64,
    pub sys_verbosity_level: u32,
    pub callbacks: Option<()>, // TODO
}

impl TLJClientConfig {
    pub fn new(net_config: String, archive_only: bool) -> TLJClientConfig {
        let connection_check = match archive_only {
            true => LiteNodeFilter::Archive,
            false => LiteNodeFilter::Health,
        };
        TLJClientConfig {
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
            sys_verbosity_level: 1,
            callbacks: None,
        }
    }
    pub fn new_mainnet(archive_only: bool) -> TLJClientConfig {
        TLJClientConfig::new(TON_NET_CONF_MAINNET.to_string(), archive_only)
    }
    pub fn new_testnet(archive_only: bool) -> TLJClientConfig {
        TLJClientConfig::new(TON_NET_CONF_TESTNET.to_string(), archive_only)
    }
}
