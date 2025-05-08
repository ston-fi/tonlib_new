use crate::clients::tonlibjson::tl_client_trait::TLClientTrait;
use crate::clients::tonlibjson::tl_client_config::TLClientConfig;
use crate::clients::tonlibjson::utils::prepare_client_env;
use crate::errors::TonlibError;
use async_trait::async_trait;
use rand::prelude::{IndexedRandom, StdRng};
use rand::SeedableRng;
use std::ops::DerefMut;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use crate::clients::tonlibjson::tl_connection::TLConnection;

// /// Simple client with many connections
pub struct TLClient {
    rnd: Mutex<StdRng>,
    connections: Vec<TLConnection>,
}

#[async_trait]
impl TLClientTrait for TLClient {
    async fn get_connection(&self) -> Result<&TLConnection, TonlibError> {
        let mut rng_lock = self.rnd.lock().await;
        let conn = self.connections.choose(&mut rng_lock.deref_mut()).unwrap();
        Ok(conn)
    }
}

impl TLClient {
    #[allow(clippy::new_ret_no_self)]
    pub async fn new(mut config: TLClientConfig) -> Result<TLClient, TonlibError> {
        prepare_client_env(&mut config).await?;

        let semaphore = Arc::new(Semaphore::new(config.max_parallel_requests));
        let mut connections = Vec::with_capacity(config.connections_count);
        for _ in 0..config.connections_count {
            let connection = TLConnection::new(&config, semaphore.clone()).await?;
            connections.push(connection);
        }
        Ok(TLClient {
            rnd: Mutex::new(StdRng::from_rng(&mut rand::rng())),
            connections,
        })
    }
}
