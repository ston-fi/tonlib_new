use crate::clients::tonlib::clients_impl::TLConnection;
use crate::clients::tonlib::tl_client::TLClient;
use crate::clients::tonlib::tl_client_config::TLClientConfig;
use crate::clients::tonlib::utils::prepare_client_env;
use crate::errors::TonlibError;
use async_trait::async_trait;
use rand::prelude::{IndexedRandom, StdRng};
use rand::SeedableRng;
use std::ops::DerefMut;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};

/// Simple client with many connectionse
#[derive(Clone)]
pub struct TLClientDefault(Arc<Inner>);

impl TLClientDefault {
    pub async fn new(mut config: TLClientConfig) -> Result<impl TLClient, TonlibError> {
        prepare_client_env(&mut config).await?;

        let semaphore = Arc::new(Semaphore::new(config.max_parallel_requests));
        let mut connections = Vec::with_capacity(config.connections_count);
        for _ in 0..config.connections_count {
            let connection = TLConnection::new(&config, semaphore.clone()).await?;
            connections.push(connection);
        }
        let inner = Inner {
            rnd: Mutex::new(StdRng::from_rng(&mut rand::rng())),
            connections,
        };
        Ok(TLClientDefault(Arc::new(inner)))
    }
}

#[async_trait]
impl TLClient for TLClientDefault {
    async fn get_connection(&self) -> Result<&TLConnection, TonlibError> {
        let mut rng_lock = self.0.rnd.lock().await;
        let conn = self.0.connections.choose(&mut rng_lock.deref_mut()).unwrap();
        Ok(conn)
    }
}

struct Inner {
    rnd: Mutex<StdRng>,
    connections: Vec<TLConnection>,
}
