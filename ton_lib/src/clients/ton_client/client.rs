use crate::clients::ton_client::connection::TonConnection;
use crate::clients::ton_client::env::prepare_client_env;
use crate::clients::ton_client::RetryStrategy;
use crate::clients::ton_client::{config::TLClientConfig, tl::client::TLClientTrait};
use crate::errors::TonlibError;
use async_trait::async_trait;
use rand::prelude::{IndexedRandom, StdRng};
use rand::SeedableRng;
use std::ops::DerefMut;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};

// /// Simple client with many connections
#[derive(Clone)]
pub struct TLClient {
    inner: Arc<Inner>,
}

struct Inner {
    rnd: Mutex<StdRng>,
    connections: Vec<TonConnection>,
    config: TLClientConfig,
}

#[async_trait]
impl TLClientTrait for TLClient {
    async fn get_connection(&self) -> &TonConnection {
        let mut rng_lock = self.inner.rnd.lock().await;
        self.inner.connections.choose(&mut rng_lock.deref_mut()).unwrap()
    }

    fn get_retry_strategy(&self) -> &RetryStrategy { &self.inner.config.retry_strategy }
}

impl TLClient {
    pub async fn new(mut config: TLClientConfig) -> Result<TLClient, TonlibError> {
        prepare_client_env(&mut config).await?;

        let semaphore = Arc::new(Semaphore::new(config.max_parallel_requests));
        let mut connections = Vec::with_capacity(config.connections_count);
        for _ in 0..config.connections_count {
            let connection = TonConnection::new(&config, semaphore.clone()).await?;
            connections.push(connection);
        }
        let inner = Inner {
            rnd: Mutex::new(StdRng::from_rng(&mut rand::rng())),
            connections,
            config,
        };
        Ok(TLClient { inner: Arc::new(inner) })
    }
}
