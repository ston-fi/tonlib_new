use crate::errors::TonlibError;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use std::ffi::CString;

pub(super) fn convert_emulator_response(c_str: *const std::os::raw::c_char) -> Result<String, TonlibError> {
    if c_str.is_null() {
        return Err(TonlibError::EmulatorNullResponse);
    }
    let json_str = unsafe {
        let json_str = std::ffi::CStr::from_ptr(c_str).to_str()?.to_string();
        libc::free(c_str as *mut std::ffi::c_void); // emulator doesn't free the string
        json_str
    };
    Ok(json_str)
}

pub(super) fn require_field<T>(val: Option<T>, field: &'static str, raw_response: &str) -> Result<T, TonlibError> {
    val.ok_or(TonlibError::EmulatorParseResponseError {
        field,
        raw_response: raw_response.to_string(),
    })
}

pub(super) fn set_param_failed(param: &'static str) -> Result<(), TonlibError> {
    Err(TonlibError::EmulatorSetParamFailed(param))
}

pub(super) fn make_b64_c_str(data: &[u8]) -> Result<CString, TonlibError> {
    Ok(CString::new(BASE64_STANDARD.encode(data))?)
}
