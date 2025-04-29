use crate::cell::ton_cell::{TonCell, TonCellRef};
use crate::errors::TonlibError;
use crate::types::tlb::block_tlb::tvm::VMStack;
use crate::types::tlb::tlb_type::TLBType;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct TVMSendMsgSuccess {
    pub new_code: TonCellRef,
    pub new_data: TonCellRef,
    pub accepted: bool,
    pub vm_exit_code: i32,
    pub vm_log: String,
    pub missing_library: Option<String>,
    pub gas_used: i32,
    pub actions: Option<TonCellRef>,
}

impl TVMSendMsgSuccess {
    pub fn exit_success(&self) -> bool { self.vm_exit_code == 0 || self.vm_exit_code == 1 }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TVMSendMsgResponse {
    pub success: bool,
    pub new_code: Option<String>,
    pub new_data: Option<String>,
    pub accepted: Option<bool>,
    pub vm_exit_code: Option<i32>,
    pub vm_log: Option<String>,
    pub missing_library: Option<String>,
    pub gas_used: Option<String>,
    pub actions: Option<String>,
    pub error: Option<String>,
}

impl TVMSendMsgResponse {
    pub fn into_result(self) -> Result<TVMSendMsgSuccess, TonlibError> {
        if !self.success {
            let error = unwrap_opt(self.error, "error is None")?;
            return Err(TonlibError::TVMEmulatorError(error));
        }

        let new_code = TonCellRef::from_boc_b64(&unwrap_opt(self.new_code, "new_code")?)?;
        let new_data = TonCellRef::from_boc_b64(&unwrap_opt(self.new_data, "new_data")?)?;
        let accepted = unwrap_opt(self.accepted, "accepted")?;
        let vm_log = unwrap_opt(self.vm_log, "vm_log")?;
        let vm_exit_code = unwrap_opt(self.vm_exit_code, "vm_exit_code")?;
        let missing_library = self.missing_library;
        let gas_used = unwrap_opt(self.gas_used, "gas_used")?.parse::<i32>()?;
        let actions = self.actions.map(|x| TonCellRef::from_boc_b64(&x)).transpose()?;
        Ok(TVMSendMsgSuccess {
            new_code,
            new_data,
            accepted,
            vm_exit_code,
            vm_log,
            missing_library,
            gas_used,
            actions,
        })
    }
}

#[derive(Debug)]
pub struct TVMRunGetMethodSuccess {
    pub vm_log: Option<String>,
    pub vm_exit_code: i32,
    pub stack: VMStack,
    pub missing_library: Option<String>,
    pub gas_used: i32,
}

impl TVMRunGetMethodSuccess {
    pub fn exit_success(&self) -> bool { self.vm_exit_code == 0 || self.vm_exit_code == 1 }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TVMRunMethodResponse {
    pub success: bool,
    pub vm_log: Option<String>,
    pub vm_exit_code: Option<i32>,
    pub stack: Option<String>,
    pub missing_library: Option<String>,
    pub gas_used: Option<String>,
    pub error: Option<String>,
}

impl TVMRunMethodResponse {
    pub fn into_result(self) -> Result<TVMRunGetMethodSuccess, TonlibError> {
        if !self.success {
            let error = unwrap_opt(self.error, "error is None")?;
            return Err(TonlibError::TVMEmulatorError(error));
        }

        let vm_log = self.vm_log;
        let vm_exit_code = unwrap_opt(self.vm_exit_code, "vm_exit_code")?;
        let stack = unwrap_opt(self.stack, "stack")?;
        let missing_library = self.missing_library;
        let gas_used = unwrap_opt(self.gas_used, "gas_used")?.parse::<i32>()?;

        Ok(TVMRunGetMethodSuccess {
            vm_log,
            vm_exit_code,
            stack: TLBType::from_boc_b64(&stack)?,
            missing_library,
            gas_used,
        })
    }
}

fn unwrap_opt<T>(val: Option<T>, field: &'static str) -> Result<T, TonlibError> {
    val.ok_or(TonlibError::TVMResponseParseError(format!("{field} is None")))
}
