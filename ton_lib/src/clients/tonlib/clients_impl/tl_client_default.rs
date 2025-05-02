use crate::clients::tonlib::clients_impl::TLConnDefault;
use crate::clients::tonlib::tl_client::TLClientTrait;
use crate::clients::tonlib::tl_client_config::TLClientConfig;
use crate::clients::tonlib::utils::prepare_client_env;
use crate::clients::tonlib::{TLClient, TLConnection};
use crate::errors::TonlibError;
use async_trait::async_trait;
use rand::prelude::{IndexedRandom, StdRng};
use rand::SeedableRng;
use std::ops::DerefMut;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};

// /// Simple client with many connections
pub struct TLClientDefault {
    rnd: Mutex<StdRng>,
    connections: Vec<TLConnDefault>,
}

#[async_trait]
impl TLClientTrait for TLClientDefault {
    async fn get_connection(&self) -> Result<&dyn TLConnection, TonlibError> {
        let mut rng_lock = self.rnd.lock().await;
        let conn = self.connections.choose(&mut rng_lock.deref_mut()).unwrap();
        Ok(conn)
    }
}

impl TLClientDefault {
    #[allow(clippy::new_ret_no_self)]
    pub async fn new(mut config: TLClientConfig) -> Result<TLClient, TonlibError> {
        prepare_client_env(&mut config).await?;

        let semaphore = Arc::new(Semaphore::new(config.max_parallel_requests));
        let mut connections = Vec::with_capacity(config.connections_count);
        for _ in 0..config.connections_count {
            let connection = TLConnDefault::new(&config, semaphore.clone()).await?;
            connections.push(connection);
        }
        let client = TLClientDefault {
            rnd: Mutex::new(StdRng::from_rng(&mut rand::rng())),
            connections,
        };
        Ok(TLClient::new(Arc::new(client)))
    }
}
