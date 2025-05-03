use crate::cell::ton_cell::TonCellRef;
use crate::cell::ton_cell_utils::TonCellUtils;
use crate::clients::tonlib::tl_api::tl_types::{TLRawFullAccountState, TLTxId};
use crate::clients::tonlib::TLClient;
use crate::emulators::tvm::{TVMEmulator, TVMEmulatorC7, TVMMethodId, TVMRunMethodSuccess};
use crate::errors::TonlibError;
use crate::types::tlb::block_tlb::coins::Coins;
use crate::types::tlb::block_tlb::tvm::VMStack;
use crate::types::tlb::tlb_type::TLBType;
use crate::types::ton_address::TonAddress;
use std::ops::Deref;
use ton_lib_macros::ton_contract;

pub struct ContractCtx {
    pub address: TonAddress,
    pub tl_client: TLClient,
    pub code_cell: TonCellRef,
    pub data_cell: TonCellRef,
    pub state_raw: TLRawFullAccountState,
}

#[ton_contract]
pub struct TonContract {}

#[async_trait::async_trait]
pub trait TonContractTrait: Send + Sync + Sized {
    fn ctx(&self) -> &ContractCtx;
    fn ctx_mut(&mut self) -> &mut ContractCtx;
    fn from_ctx(ctx: ContractCtx) -> Self;

    async fn new(address: TonAddress, tl_client: TLClient, tx_id: Option<TLTxId>) -> Result<Self, TonlibError> {
        let state_raw = match tx_id {
            Some(tx_id) => tl_client.get_account_state_raw_by_tx(&address, tx_id).await?,
            None => tl_client.get_account_state_raw(&address).await?,
        };
        let code_cell = TonCellRef::from_boc(&state_raw.code)?;
        let data_cell = TonCellRef::from_boc(&state_raw.data)?;
        Ok(Self::from_ctx(ContractCtx {
            address,
            tl_client,
            code_cell,
            data_cell,
            state_raw,
        }))
    }

    async fn update(&mut self, tx_id: Option<TLTxId>) -> Result<(), TonlibError> {
        let ctx = self.ctx_mut();
        let state_raw = match tx_id {
            Some(tx_id) => {
                if tx_id == ctx.state_raw.last_tx_id {
                    return Ok(());
                }
                ctx.tl_client.get_account_state_raw_by_tx(&ctx.address, tx_id).await?
            }
            None => ctx.tl_client.get_account_state_raw(&ctx.address).await?,
        };
        ctx.state_raw = state_raw;
        ctx.code_cell = TonCellRef::from_boc(&ctx.state_raw.code)?;
        ctx.data_cell = TonCellRef::from_boc(&ctx.state_raw.data)?;
        Ok(())
    }

    async fn run_method<M>(
        &self,
        method: M,
        stack: &VMStack,
        c7: Option<&TVMEmulatorC7>,
    ) -> Result<TVMRunMethodSuccess, TonlibError>
    where
        M: Into<TVMMethodId> + Send,
    {
        self.make_emulator(c7).await?.run_method(method, &stack.to_boc(false)?)
    }

    fn send_int_msg(&self, _msg_boc: &[u8], _amount: Coins) -> Result<(), TonlibError> { todo!() }

    fn send_ext_msg(&self, _msg_boc: &[u8]) -> Result<(), TonlibError> { todo!() }

    fn parse_data<D: TLBType>(&self) -> Result<D, TonlibError> { D::from_cell(&self.ctx().data_cell) }

    async fn make_emulator(&self, c7: Option<&TVMEmulatorC7>) -> Result<TVMEmulator, TonlibError> {
        let ctx = self.ctx();

        let mut emulator = match c7 {
            Some(c7) => TVMEmulator::new(&ctx.state_raw.code, &ctx.state_raw.data, c7)?,
            None => {
                let bc_config = ctx.tl_client.get_config_boc_all(0).await?;
                let c7 = TVMEmulatorC7::new(ctx.address.clone(), bc_config)?;
                TVMEmulator::new(&ctx.state_raw.code, &ctx.state_raw.data, &c7)?
            }
        };

        let lib_ids = TonCellUtils::extract_lib_ids([ctx.code_cell.deref(), ctx.code_cell.deref()])?;
        if !lib_ids.is_empty() {
            if let Some(libs_dict) = ctx.tl_client.get_libs(lib_ids).await? {
                emulator.set_libs(&libs_dict.to_boc(false)?)?;
            }
        }
        Ok(emulator)
    }
}
