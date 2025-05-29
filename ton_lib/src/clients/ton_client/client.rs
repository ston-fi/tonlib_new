use crate::clients::ton_client::connection::TonConnection;
use crate::clients::ton_client::utils::prepare_client_env;
use crate::clients::ton_client::TonlibjsonClientRetryStrategy;
use crate::clients::ton_client::{config::TonClientConfig, tonlibjson::interface::TonlibjsonInterface};
use crate::errors::TonlibError;
use async_trait::async_trait;
use rand::prelude::{IndexedRandom, StdRng};
use rand::SeedableRng;
use std::ops::DerefMut;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};

// /// Simple client with many connections
#[derive(Clone)]
pub struct TonClient {
    inner: Arc<Inner>,
}

#[async_trait]
impl TonlibjsonInterface for TonClient {
    async fn get_connection(&self) -> &TonConnection {
        let mut rng_lock = self.inner.rnd.lock().await;
        self.inner.connections.choose(&mut rng_lock.deref_mut()).unwrap()
    }

    fn get_retry_strategy(&self) -> &TonlibjsonClientRetryStrategy { &self.inner.config.retry_strategy }
}

impl TonClient {
    pub async fn new(mut config: TonClientConfig) -> Result<TonClient, TonlibError> {
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
        Ok(TonClient { inner: Arc::new(inner) })
    }
}

struct Inner {
    rnd: Mutex<StdRng>,
    connections: Vec<TonConnection>,
    config: TonClientConfig,
}
