use crate::cell::ton_cell::TonCell;
use crate::cell::ton_cell_utils::TonCellUtils;
use crate::clients::tonlib::tl_api::tl_types::{TLRawFullAccountState, TLTxId};
use crate::clients::tonlib::TLClient;
use crate::emulators::tvm::{TVMEmulator, TVMEmulatorC7, TVMMethodId, TVMRunMethodSuccess};
use crate::errors::TonlibError;
use crate::types::tlb::block_tlb::coins::Coins;
use crate::types::tlb::block_tlb::tvm::VMStack;
use crate::types::tlb::tlb_type::TLBType;
use crate::types::ton_address::TonAddress;

pub struct ContractCtx {
    pub address: TonAddress,
    pub state_raw: TLRawFullAccountState,
    pub tl_client: TLClient,
}

#[async_trait::async_trait]
pub trait TonContract: Send + Sync + Sized {
    fn ctx(&self) -> &ContractCtx;
    fn ctx_mut(&mut self) -> &mut ContractCtx;
    fn from_ctx(ctx: ContractCtx) -> Self;

    async fn new(address: TonAddress, tl_client: TLClient, tx_id: Option<TLTxId>) -> Result<Self, TonlibError> {
        let state = match tx_id {
            Some(tx_id) => tl_client.get_account_state_raw_by_tx(&address, tx_id).await?,
            None => tl_client.get_account_state_raw(&address).await?,
        };
        Ok(Self::from_ctx(ContractCtx {
            address,
            state_raw: state,
            tl_client,
        }))
    }

    async fn update(&mut self, tx_id: Option<TLTxId>) -> Result<(), TonlibError> {
        let ctx = self.ctx_mut();
        let state = match tx_id {
            Some(tx_id) => ctx.tl_client.get_account_state_raw_by_tx(&ctx.address, tx_id).await?,
            None => ctx.tl_client.get_account_state_raw(&ctx.address).await?,
        };
        ctx.state_raw = state;
        Ok(())
    }

    fn parse_data<D: TLBType>(&self) -> Result<D, TonlibError> { D::from_boc(&self.ctx().state_raw.data) }

    async fn run_method<M>(&self, method: M, stack: &VMStack) -> Result<TVMRunMethodSuccess, TonlibError>
    where
        M: Into<TVMMethodId> + Send,
    {
        let ctx = self.ctx();

        let config = ctx.tl_client.get_config_boc_all(0).await?;
        let c7 = TVMEmulatorC7::new(ctx.address.clone(), config)?;

        let (code_cell, data_cell) = (TonCell::from_boc(&ctx.state_raw.code)?, TonCell::from_boc(&ctx.state_raw.data)?);
        let lib_ids = TonCellUtils::extract_lib_ids([&code_cell, &data_cell])?;
        let libs_boc = ctx.tl_client.get_libs(lib_ids).await?;

        let mut emulator = TVMEmulator::new_with_c7(&ctx.state_raw.code, &ctx.state_raw.data, &c7)?;
        if !libs_boc.is_empty() {
            emulator.set_libs(&libs_boc.to_boc(false)?)?;
        }
        emulator.run_method(method, &stack.to_boc(false)?)
    }

    fn send_int_msg(&self, _msg_boc: &[u8], _amount: Coins) -> Result<(), TonlibError> { todo!() }

    fn send_ext_msg(&self, _msg_boc: &[u8]) -> Result<(), TonlibError> { todo!() }
}
