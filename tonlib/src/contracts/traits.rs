use crate::block_tlb::TVMStack;
use crate::contracts::contract_client::ContractClient;
use crate::emulators::tvm::tvm_method_id::TVMGetMethodID;
use crate::error::TLError;
use std::sync::Arc;
use ton_lib_core::traits::contract_provider::{ContractMethodArgs, ContractMethodState, ContractState};
use ton_lib_core::traits::tlb::TLB;
use ton_lib_core::types::{TonAddress, TxIdLTHash};

pub struct ContractCtx {
    pub client: ContractClient,
    pub address: TonAddress,
    pub state: ContractMethodState,
}

#[async_trait::async_trait]
pub trait ContractTrait: Send + Sync + Sized {
    fn ctx(&self) -> &ContractCtx;
    fn from_ctx(ctx: ContractCtx) -> Self;

    fn new(client: &ContractClient, address: TonAddress, tx_id: Option<TxIdLTHash>) -> Result<Self, TLError> {
        match tx_id {
            Some(tx_id) => Self::from_state(client.clone(), address, ContractMethodState::TxId(tx_id)),
            None => Self::from_state(client.clone(), address, ContractMethodState::Latest),
        }
    }

    fn from_state(client: ContractClient, address: TonAddress, state: ContractMethodState) -> Result<Self, TLError> {
        Ok(Self::from_ctx(ContractCtx { client, address, state }))
    }

    async fn get_state(&self) -> Result<Arc<ContractState>, TLError> {
        let ctx = self.ctx();
        let tx_id = match &ctx.state {
            ContractMethodState::Latest => None,
            ContractMethodState::TxId(tx_id) => Some(tx_id),
            ContractMethodState::Custom(state) => return Ok(state.clone()),
        };
        Ok(ctx.client.get_state(&ctx.address, tx_id).await?)
    }

    async fn run_get_method<M>(&self, method: M, stack: Option<&TVMStack>) -> Result<TVMStack, TLError>
    where
        M: Into<TVMGetMethodID> + Send,
    {
        let ctx = self.ctx();
        let args = ContractMethodArgs {
            address: ctx.address.clone(),
            method_state: ctx.state.clone(),
            method_id: method.into().to_id(),
            stack_boc: stack.map(|x| x.to_boc()).transpose()?,
        };
        let response = ctx.client.run_get_method(args).await?;
        Ok(TVMStack::from_boc(&response.stack_boc)?)
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
