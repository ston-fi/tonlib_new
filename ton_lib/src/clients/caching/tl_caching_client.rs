use crate::cell::ton_cell::TonCellRef;
use crate::cell::ton_hash::TonHash;
use crate::clients::tonlib::{TLClient, TLClientTrait, TLConnection};
use crate::errors::TonlibError;
use std::sync::Arc;

#[derive(Clone)]
pub struct TLClientCaching {
    inner: Arc<Inner>,
}

impl TLClientCaching {
    pub fn new(tl_client: TLClient) -> Self {
        let inner = Arc::new(Inner::new(tl_client));
        Self { inner }
    }
}

#[async_trait::async_trait]
impl TLClientTrait for TLClientCaching {
    async fn get_connection(&self) -> Result<&dyn TLConnection, TonlibError> {
        self.inner.tl_client.get_connection().await
    }
}

struct Inner {
    tl_client: TLClient,
    libs_cache: moka::future::Cache<TonHash, TonCellRef>,
}

impl Inner {
    pub fn new(tl_client: TLClient) -> Self {
        let libs_cache =
            moka::future::Cache::builder().max_capacity(100).time_to_live(std::time::Duration::from_secs(60)).build();
        Self { tl_client, libs_cache }
    }
}
