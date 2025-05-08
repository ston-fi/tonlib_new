use std::sync::Arc;
use crate::clients::contract_client::data_provider::DataProvider;

mod data_provider;

pub struct ContractClient {
    inner: Arc<Inner>
}

struct Inner {
    data_provider: Arc<dyn DataProvider>,
    cache: 
}