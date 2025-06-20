use crate::block_tlb::TVMStack;
use crate::emulators::tvm::tvm_method_id::TVMGetMethodID;
use crate::error::TLError;
use std::sync::Arc;
use ton_lib_core::traits::contract_provider::{ContractProvider, ContractState, GetMethodArgs, GetMethodState};
use ton_lib_core::traits::tlb::TLB;
use ton_lib_core::types::{TonAddress, TxId, TxIdLTAddress};

pub struct ContractCtx {
    pub client: Arc<dyn ContractProvider>,
    pub address: TonAddress,
    pub state: GetMethodState,
}

#[async_trait::async_trait]
pub trait TonContract: Send + Sync + Sized {
    fn ctx(&self) -> &ContractCtx;
    fn from_ctx(ctx: ContractCtx) -> Self;

    fn new(client: Arc<dyn ContractProvider>, address: TonAddress, tx_id: Option<TxId>) -> Result<Self, TLError> {
        match tx_id {
            Some(tx_id) => Self::from_state(client, address, GetMethodState::TxId(tx_id)),
            None => Self::from_state(client, address, GetMethodState::Latest),
        }
    }

    fn from_state(
        client: Arc<dyn ContractProvider>,
        address: TonAddress,
        state: GetMethodState,
    ) -> Result<Self, TLError> {
        Ok(Self::from_ctx(ContractCtx { client, address, state }))
    }

    async fn get_state(&self) -> Result<Arc<ContractState>, TLError> {
        let ctx = self.ctx();
        let tx_id = match &ctx.state {
            GetMethodState::Latest => None,
            GetMethodState::TxId(tx_id) => Some(tx_id),
            GetMethodState::Custom {
                code_boc,
                data_boc,
                balance,
            } => {
                return Ok(Arc::new(ContractState {
                    address: ctx.address.clone(),
                    mc_seqno: 0,
                    last_tx_id: TxId::LTAddress(TxIdLTAddress {
                        lt: 0,
                        address: ctx.address.clone(),
                    }),
                    code_boc: Some(code_boc.clone()),
                    data_boc: data_boc.clone(),
                    frozen_hash: None,
                    balance: *balance,
                }))
            }
        };
        Ok(ctx.client.get_state(&ctx.address, tx_id).await?)
    }

    async fn run_get_method<M>(&self, method: M, stack: Option<&TVMStack>) -> Result<TVMStack, TLError>
    where
        M: Into<TVMGetMethodID> + Send,
    {
        let ctx = self.ctx();
        let args = GetMethodArgs {
            address: ctx.address.clone(),
            state: ctx.state.clone(),
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
