use crate::clients::tonlibjson::clients_impl::TLJConnection;
use crate::clients::tonlibjson::tlj_client::TLJClient;
use crate::clients::tonlibjson::tlj_config::TLJClientConfig;
use crate::clients::tonlibjson::tlj_utils::prepare_client_env;
use crate::errors::TonlibError;
use async_trait::async_trait;
use rand::prelude::{IndexedRandom, StdRng};
use rand::SeedableRng;
use std::ops::DerefMut;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};

/// Simple client with many connections
pub struct TLJClientDefault(Arc<Inner>);

impl TLJClientDefault {
    pub async fn new(mut config: TLJClientConfig) -> Result<Self, TonlibError> {
        prepare_client_env(&mut config).await?;

        let semaphore = Arc::new(Semaphore::new(config.max_parallel_requests));
        let mut connections = Vec::with_capacity(config.connections_count);
        for _ in 0..config.connections_count {
            let connection = TLJConnection::new(&config, semaphore.clone()).await?;
            connections.push(connection);
        }
        let inner = Inner {
            rnd: Mutex::new(StdRng::from_rng(&mut rand::rng())),
            connections,
        };
        Ok(TLJClientDefault(Arc::new(inner)))
    }
}

#[async_trait]
impl TLJClient for TLJClientDefault {
    async fn get_connection(&self) -> Result<&TLJConnection, TonlibError> {
        let mut rng_lock = self.0.rnd.lock().await;
        let conn = self.0.connections.choose(&mut rng_lock.deref_mut()).unwrap();
        Ok(conn)
    }
}

struct Inner {
    rnd: Mutex<StdRng>,
    connections: Vec<TLJConnection>,
}
