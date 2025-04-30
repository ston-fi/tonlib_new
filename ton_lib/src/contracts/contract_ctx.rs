use crate::clients::tonlib::tl_api::tl_types::{TLRawFullAccountState, TLTxId};
use crate::clients::tonlib::TLClient;
use crate::contracts::ton_contract::TonContract;
use crate::errors::TonlibError;
use crate::types::ton_address::TonAddress;

pub struct TonContractState<T: TLClient> {
    address: TonAddress,
    state: Option<TLRawFullAccountState>,
    client: T,
}

#[async_trait::async_trait]
impl<T: TLClient> TonContract<T> for TonContractState<T> {
    async fn new(address: TonAddress, client: T, tx_id: Option<TLTxId>) -> Result<Self, TonlibError> {
        let mut contract = Self {
            address,
            state: None,
            client,
        };
        contract.update_state(tx_id).await?;
        Ok(contract)
    }

    fn get_client(&self) -> &T { &self.client }

    fn set_state(&mut self, state: TLRawFullAccountState) { self.state = Some(state); }

    fn get_address(&self) -> &TonAddress { &self.address }

    fn get_state(&self) -> &TLRawFullAccountState {
        self.state.as_ref().unwrap() // invariant guaranteed state always present
    }
}
