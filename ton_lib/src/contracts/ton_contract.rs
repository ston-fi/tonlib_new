use crate::block_tlb::TVMStack;
use crate::contracts::client::contract_client::ContractClient;
use crate::emulators::tvm::tvm_method_id::TVMGetMethodID;
use crate::error::TLError;
use std::sync::Arc;
use ton_lib_core::traits::contract_provider::ContractState;
use ton_lib_core::traits::tlb::TLB;
use ton_lib_core::types::{TonAddress, TxIdLTHash};

pub struct ContractCtx {
    pub client: ContractClient,
    pub address: TonAddress,
    pub state: Arc<ContractState>,
}

#[async_trait::async_trait]
pub trait TonContract: Send + Sync + Sized {
    fn ctx(&self) -> &ContractCtx;
    fn from_ctx(ctx: ContractCtx) -> Self;

    async fn new(client: &ContractClient, address: TonAddress, tx_id: Option<TxIdLTHash>) -> Result<Self, TLError> {
        let state = client.get_contract(&address, tx_id.as_ref()).await?;
        Self::from_state(client.clone(), address, state)
    }

    fn from_state(client: ContractClient, address: TonAddress, state: Arc<ContractState>) -> Result<Self, TLError> {
        Ok(Self::from_ctx(ContractCtx { client, address, state }))
    }

    async fn get_state(&self) -> Result<&Arc<ContractState>, TLError> { Ok(&self.ctx().state) }

    async fn emulate_get_method<M>(&self, method: M, stack: Option<&TVMStack>) -> Result<TVMStack, TLError>
    where
        M: Into<TVMGetMethodID> + Send,
    {
        let ctx = self.ctx();
        let method_id = method.into().to_id();
        let stack_raw = stack.map(|s| s.to_boc()).transpose()?;
        let stack_slice = stack_raw.as_deref();
        let response = ctx.client.emulate_get_method(&ctx.state, method_id, stack_slice).await?;
        response.stack_parsed()
    }

    async fn get_parsed_data<D: TLB>(&self) -> Result<D, TLError> {
        let state = self.get_state().await?;
        match &state.data_boc {
            Some(data_boc) => Ok(D::from_boc(data_boc)?),
            None => Err(TLError::TonContractNoData {
                address: state.address.clone(),
                tx_id: Some(state.last_tx_id.clone()),
            }),
        }
    }
}
