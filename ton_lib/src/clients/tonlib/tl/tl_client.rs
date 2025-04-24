use crate::clients::tonlib::tl::serial::{
    deserialize_result, deserialize_result_extra, serialize_function, serialize_function_extra,
};
use crate::clients::tonlib::tl::tl_request::TLRequest;
use crate::clients::tonlib::tl::tl_response::TLResponse;
use crate::errors::TonLibError;
use std::ffi::{c_char, CStr};
use tonlib_sys::{
    tonlib_client_json_create, tonlib_client_json_destroy, tonlib_client_json_execute, tonlib_client_json_receive,
    tonlib_client_json_send,
};
// Wrapper around ton client with support for TL data types

pub struct TLClientUnsafe {
    ptr: *mut ::std::os::raw::c_void,
    tag: String,
}

impl TLClientUnsafe {
    pub fn new(tag: String) -> TLClientUnsafe {
        unsafe {
            let ptr = tonlib_client_json_create();
            TLClientUnsafe { ptr, tag }
        }
    }

    pub fn get_tag(&self) -> &str { self.tag.as_str() }

    pub fn execute(&self, req: &TLRequest) -> Result<TLResponse, TonLibError> {
        let f_str = serialize_function(req)?;
        let tag = self.get_tag();
        log::trace!("[{tag}] execute: {f_str:?}");
        unsafe {
            let c_str = tonlib_client_json_execute(self.ptr, f_str.as_ptr());
            let res = deserialize_result(c_str)?;
            log::trace!("[{tag}] result: {res:?}");
            deserialize_result(c_str)
        }
    }

    pub fn send(&self, req: &TLRequest, extra: &str) -> Result<(), TonLibError> {
        let f_str = serialize_function_extra(req, extra)?;
        log::trace!("[{}] send: {f_str:?}", self.tag);
        unsafe { tonlib_client_json_send(self.ptr, f_str.as_ptr()) };
        Ok(())
    }

    pub fn receive(&self, timeout: f64) -> Result<Option<(TLResponse, Option<String>)>, TonLibError> {
        let c_str = unsafe { tonlib_client_json_receive(self.ptr, timeout) };
        if c_str.is_null() {
            Ok(None)
        } else {
            let c_str_slice = unsafe { CStr::from_ptr(c_str) };
            if let Ok(c_str_str) = c_str_slice.to_str() {
                log::trace!("[{}] receive: {}", self.tag, c_str_str);
            } else {
                log::trace!("[{}] receive: <Error decoding string as UTF-8>", self.tag);
            }
            let c_str_bytes = c_str_slice.to_bytes();
            let (result, extra) = unsafe { deserialize_result_extra(c_str_bytes.as_ptr() as *const c_char)? };
            Ok(Some((result, extra)))
        }
    }

    pub fn set_log_verbosity_level(verbosity_level: u32) {
        unsafe { tonlib_sys::tonlib_client_set_verbosity_level(verbosity_level) }
    }
}

impl Drop for TLClientUnsafe {
    fn drop(&mut self) { unsafe { tonlib_client_json_destroy(self.ptr) } }
}

unsafe impl Send for TLClientUnsafe {}
unsafe impl Sync for TLClientUnsafe {}

#[cfg(test)]
mod tests {
    use crate::clients::tonlib::tl::tl_client::TLClientUnsafe;
    use crate::clients::tonlib::tl::tl_request::TLRequest;

    #[test]
    fn set_log_verbosity_level_works() -> anyhow::Result<()> {
        let level = 1;
        TLClientUnsafe::set_log_verbosity_level(level);
        Ok(())
    }

    #[test]
    fn it_executes_functions() -> anyhow::Result<()> {
        let client = TLClientUnsafe::new("test".to_string());
        let _ = client.execute(&TLRequest::GetLogVerbosityLevel {})?;
        Ok(())
    }
}
