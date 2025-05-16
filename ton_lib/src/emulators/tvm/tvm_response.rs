use crate::errors::TonlibError;
use crate::types::tlb::block_tlb::tvm::TVMStack;
use crate::types::tlb::tlb_type::TLBType;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct TVMRunMethodSuccess {
    pub vm_exit_code: i32,
    pub vm_log: Option<String>,
    pub stack_boc_b64: String, // b64(String) <--> Vec<u8> <--> TVMStack
    pub gas_used: i32,
    pub raw_response: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TVMRunMethodResponse {
    pub success: bool,
    pub vm_exit_code: Option<i32>,
    pub vm_log: Option<String>,
    pub stack: Option<String>,
    pub gas_used: Option<String>,
    pub missing_library: Option<String>,
    pub error: Option<String>,
    #[serde(skip)]
    pub raw_response: String,
}

impl TVMRunMethodResponse {
    pub fn from_json(json: String) -> Result<Self, TonlibError> {
        let mut value: Self = serde_json::from_str(&json)?;
        value.raw_response = json;
        Ok(value)
    }

    pub fn into_success(self) -> Result<TVMRunMethodSuccess, TonlibError> {
        if !self.success {
            return Err(TonlibError::TVMRunMethodError {
                vm_exit_code: self.vm_exit_code,
                response_raw: self.raw_response,
            });
        }
        let vm_exit_code = unwrap_opt(self.vm_exit_code, "vm_exit_code")?;
        if vm_exit_code != 0 && vm_exit_code != 1 {
            return Err(TonlibError::TVMRunMethodError {
                vm_exit_code: self.vm_exit_code,
                response_raw: self.raw_response,
            });
        }

        let vm_log = self.vm_log;
        let stack_boc_b64 = unwrap_opt(self.stack, "stack")?;
        let gas_used = unwrap_opt(self.gas_used, "gas_used")?.parse::<i32>()?;

        Ok(TVMRunMethodSuccess {
            vm_log,
            vm_exit_code,
            stack_boc_b64,
            gas_used,
            raw_response: self.raw_response,
        })
    }
}

impl TVMRunMethodSuccess {
    pub fn stack_parsed(&self) -> Result<TVMStack, TonlibError> { TVMStack::from_boc_b64(&self.stack_boc_b64) }

    pub fn stack_boc(&self) -> Result<Vec<u8>, TonlibError> {
        Ok(BASE64_STANDARD.decode(self.stack_boc_b64.as_bytes())?)
    }

    pub fn exit_success(&self) -> bool { self.vm_exit_code == 0 || self.vm_exit_code == 1 }
}

#[derive(Debug)]
pub struct TVMSendMsgSuccess {
    pub new_code_boc_b64: Option<String>,
    pub new_data_boc_b64: Option<String>,
    pub accepted: bool,
    pub vm_exit_code: i32,
    pub vm_log: String,
    pub missing_library: Option<String>,
    pub gas_used: i32,
    pub actions_boc_b64: Option<String>,
    pub raw_response: String,
}

impl TVMSendMsgSuccess {
    pub fn exit_success(&self) -> bool { self.vm_exit_code == 0 || self.vm_exit_code == 1 }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TVMSendMsgResponse {
    pub success: bool,
    pub new_code_boc_b64: Option<String>,
    pub new_data_boc_b64: Option<String>,
    pub accepted: Option<bool>,
    pub vm_exit_code: Option<i32>,
    pub vm_log: Option<String>,
    pub missing_library: Option<String>,
    pub gas_used: Option<String>,
    pub actions: Option<String>,
    pub error: Option<String>,
    #[serde(skip)]
    pub raw_response: String,
}

impl TVMSendMsgResponse {
    pub fn from_json(json: String) -> Result<Self, TonlibError> {
        let mut value: Self = serde_json::from_str(&json)?;
        value.raw_response = json;
        Ok(value)
    }

    pub fn into_success(self) -> Result<TVMSendMsgSuccess, TonlibError> {
        if !self.success {
            let error = unwrap_opt(self.error, "error is None")?;
            return Err(TonlibError::TVMEmulatorError(error));
        }

        let accepted = unwrap_opt(self.accepted, "accepted")?;
        let vm_log = unwrap_opt(self.vm_log, "vm_log")?;
        let vm_exit_code = unwrap_opt(self.vm_exit_code, "vm_exit_code")?;
        let missing_library = self.missing_library;
        let gas_used = unwrap_opt(self.gas_used, "gas_used")?.parse::<i32>()?;
        let actions_boc_b64 = self.actions;
        Ok(TVMSendMsgSuccess {
            new_code_boc_b64: self.new_code_boc_b64,
            new_data_boc_b64: self.new_data_boc_b64,
            accepted,
            vm_exit_code,
            vm_log,
            missing_library,
            gas_used,
            actions_boc_b64,
            raw_response: self.raw_response,
        })
    }
}

fn unwrap_opt<T>(val: Option<T>, field: &'static str) -> Result<T, TonlibError> {
    val.ok_or(TonlibError::TVMEmulatorResponseParseError(format!("{field} is None")))
}
