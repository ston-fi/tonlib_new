#[cfg(test)]
mod test_tvm_emulator;

use crate::emulators::tvm_emulator::c7_register::TVMEmulatorC7;
use crate::emulators::tvm_emulator::method_id::TVMMethodId;
use crate::emulators::tvm_emulator::tvm_response::{TVMRunMethodResponse, TVMSendIntMsgResponse};
use crate::errors::TonlibError;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::ffi::CString;
use tonlib_sys::{
    tvm_emulator_create, tvm_emulator_run_get_method, tvm_emulator_send_external_message,
    tvm_emulator_send_internal_message, tvm_emulator_set_c7, tvm_emulator_set_debug_enabled,
    tvm_emulator_set_gas_limit, tvm_emulator_set_libraries,
};

pub mod c7_register;
pub mod method_id;
pub mod tvm_response;
pub mod utils;

#[derive(Debug)]
pub struct TVMEmulator {
    ptr: *mut std::os::raw::c_void,
}

const DEFAULT_TVM_LOG_VERBOSITY: u32 = 1;

impl TVMEmulator {
    pub fn new<T: AsRef<[u8]>>(code_boc: T, data_boc: T) -> Result<Self, TonlibError> {
        let code = CString::new(STANDARD.encode(code_boc.as_ref()))?;
        let data = CString::new(STANDARD.encode(data_boc.as_ref()))?;
        let ptr = unsafe { tvm_emulator_create(code.as_ptr(), data.as_ptr(), DEFAULT_TVM_LOG_VERBOSITY) };
        Ok(TVMEmulator { ptr })
    }

    pub fn set_c7(&mut self, c7: &TVMEmulatorC7) -> Result<(), TonlibError> {
        let success = unsafe {
            tvm_emulator_set_c7(
                self.ptr,
                CString::new(c7.address.to_hex().as_bytes())?.into_raw(),
                c7.unix_time,
                c7.balance,
                CString::new(c7.rand_seed.to_hex().as_bytes())?.into_raw(),
                CString::new(STANDARD.encode(&c7.config))?.into_raw(),
            )
        };
        match success {
            true => Ok(()),
            false => Err(TonlibError::TVMEmulatorSetFailed("C7")),
        }
    }

    pub fn set_debug_enabled(&mut self, enabled: bool) -> Result<(), TonlibError> {
        let success = unsafe { tvm_emulator_set_debug_enabled(self.ptr, enabled as i32) };
        match success {
            true => Ok(()),
            false => Err(TonlibError::TVMEmulatorSetFailed("debug_enabled")),
        }
    }

    pub fn set_gas_limit(&mut self, limit: u64) -> Result<(), TonlibError> {
        let success = unsafe { tvm_emulator_set_gas_limit(self.ptr, limit) };
        match success {
            true => Ok(()),
            false => Err(TonlibError::TVMEmulatorSetFailed("gas_limit")),
        }
    }

    pub fn set_libs(&mut self, libs_boc: &[u8]) -> Result<(), TonlibError> {
        let libs = CString::new(STANDARD.encode(libs_boc))?;
        let success = unsafe { tvm_emulator_set_libraries(self.ptr, libs.into_raw()) };
        match success {
            true => Ok(()),
            false => Err(TonlibError::TVMEmulatorSetFailed("libraries")),
        }
    }

    pub fn send_int_msg<T: AsRef<[u8]>>(
        &mut self,
        msg_boc: T,
        amount: u64,
    ) -> Result<TVMSendIntMsgResponse, TonlibError> {
        let msg_boc_ref = msg_boc.as_ref();
        log::trace!("[TVMEmulator][send_int_msg]: msg_boc: {msg_boc_ref:?}, amount: {amount}");
        let msg = CString::new(STANDARD.encode(msg_boc_ref))?;
        let json_str = unsafe {
            let c_str = tvm_emulator_send_internal_message(self.ptr, msg.into_raw(), amount);
            convert_emulator_response(c_str)?
        };
        log::trace!("[TVMEmulator][send_int_msg]: msg_boc: {msg_boc_ref:?}, amount: {amount}, rsp: {json_str}");
        Ok(serde_json::from_str(&json_str)?)
    }

    pub fn send_ext_msg<T: AsRef<[u8]>>(&mut self, msg_boc: T) -> Result<TVMSendIntMsgResponse, TonlibError> {
        let msg_boc_ref = msg_boc.as_ref();
        log::trace!("[TVMEmulator][send_ext_msg]: msg_boc: {msg_boc_ref:?}");
        let msg = CString::new(STANDARD.encode(msg_boc_ref))?;
        let json_str = unsafe {
            let c_str = tvm_emulator_send_external_message(self.ptr, msg.into_raw());
            convert_emulator_response(c_str)?
        };
        log::trace!("[TVMEmulator][send_ext_msg]: msg_boc: {msg_boc_ref:?}, rsp: {json_str}");
        Ok(serde_json::from_str(&json_str)?)
    }

    pub fn run_method<M: Into<TVMMethodId>, T: AsRef<[u8]>>(&mut self, method: M, stack_boc: T) -> Result<TVMRunMethodResponse, TonlibError> {
        let tvm_method = method.into();
        let stack_boc_ref = stack_boc.as_ref();
        log::trace!("[TVMEmulator][run_get_method]: method: {tvm_method}, stack: {stack_boc_ref:?}");
        let stack = CString::new(STANDARD.encode(stack_boc_ref))?;
        let json_str = unsafe {
            let c_str = tvm_emulator_run_get_method(self.ptr, tvm_method.to_id(), stack.into_raw());
            convert_emulator_response(c_str)?
        };
        log::trace!("[TVMEmulator][run_get_method]: method: {tvm_method}, stack_boc: {stack_boc_ref:?}, rsp: {json_str}");
        Ok(serde_json::from_str(&json_str)?)
    }
}

unsafe fn convert_emulator_response(c_str: *const std::os::raw::c_char) -> Result<String, TonlibError> {
    let json_str = std::ffi::CStr::from_ptr(c_str).to_str()?.to_string();
    libc::free(c_str as *mut std::ffi::c_void); // emulator doesn't free the string
    Ok(json_str)
}
