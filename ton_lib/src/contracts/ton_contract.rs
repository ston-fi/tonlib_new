use crate::cell::ton_cell::TonCellRef;
use crate::clients::tonlib::tl_api::tl_types::{TLRawFullAccountState, TLTxId};
use crate::clients::tonlib::TLClient;
use crate::errors::TonlibError;
use crate::types::tlb::block_tlb::account::{Account, AccountState, AccountStateActive, MaybeAccount};
use crate::types::tlb::block_tlb::coins::Coins;
use crate::types::tlb::block_tlb::state_init::StateInit;
use crate::types::ton_address::TonAddress;

#[async_trait::async_trait]
pub trait TonContract<T: TLClient>: Sized {
    // init contract with
    async fn new(address: TonAddress, client: T, tx_id: Option<TLTxId>) -> Result<Self, TonlibError>;
    fn get_client(&self) -> &T;
    fn set_state(&mut self, state: TLRawFullAccountState);
    fn get_address(&self) -> &TonAddress;
    fn get_state(&self) -> &TLRawFullAccountState;

    async fn get_code_boc(&self) -> &[u8] { &self.get_state().code }
    async fn get_data_boc(&self) -> &[u8] { &self.get_state().data }
    async fn get_balance(&self) -> Coins { Coins::new(self.get_state().balance as u128) }

    async fn update_state(&mut self, tx_id: Option<TLTxId>) -> Result<(), TonlibError> {
        let address = self.get_address().clone();
        let raw_account = match tx_id {
            Some(tx_id) => self.get_client().get_account_state_raw_by_tx(address, tx_id).await?,
            None => self.get_client().get_account_state_raw(address).await?,
        };
        self.set_state(raw_account);
        Ok(())
    }
}
