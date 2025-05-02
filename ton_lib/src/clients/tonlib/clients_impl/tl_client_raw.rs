use crate::clients::tonlib::tl_api::tl_request::TLRequest;
use crate::clients::tonlib::tl_api::tl_response::TLResponse;
use crate::errors::TonlibError;
use tonlib_sys::{
    tonlib_client_json_create, tonlib_client_json_destroy, tonlib_client_json_receive, tonlib_client_json_send,
};
// Wrapper around ton client with support for TL data types

pub struct TLClientRaw {
    ptr: *mut std::os::raw::c_void,
    tag: String,
}

impl TLClientRaw {
    pub fn new(tag: String) -> Result<TLClientRaw, TonlibError> {
        let client_ptr = unsafe { tonlib_client_json_create() };
        if client_ptr.is_null() {
            return Err(TonlibError::TLClientCreationFailed);
        }
        Ok(TLClientRaw { ptr: client_ptr, tag })
    }

    pub fn tag(&self) -> &str { self.tag.as_str() }

    pub fn send(&self, req: &TLRequest, extra: &str) -> Result<(), TonlibError> {
        let c_str = req.to_c_str_json(extra)?;
        log::trace!("[{}] send: {c_str:?}", self.tag);
        unsafe { tonlib_client_json_send(self.ptr, c_str.as_ptr()) };
        Ok(())
    }

    pub fn receive(&self, timeout: f64) -> Option<Result<(TLResponse, Option<String>), TonlibError>> {
        let c_str = unsafe { tonlib_client_json_receive(self.ptr, timeout) };
        if c_str.is_null() {
            return None;
        }
        unsafe { Some(TLResponse::from_c_str_json(c_str)) }
    }
}

impl Drop for TLClientRaw {
    fn drop(&mut self) { unsafe { tonlib_client_json_destroy(self.ptr) } }
}

unsafe impl Send for TLClientRaw {}
unsafe impl Sync for TLClientRaw {}

#[cfg(test)]
mod tests {
    use crate::clients::tonlib::clients_impl::tl_client_raw::TLClientRaw;
    use crate::clients::tonlib::tl_api::tl_request::TLRequest;
    use crate::sys_utils::sys_tonlib_set_verbosity_level;

    #[test]
    fn it_executes_functions() -> anyhow::Result<()> {
        sys_tonlib_set_verbosity_level(1);
        let client = TLClientRaw::new("test".to_string())?;
        client.send(&TLRequest::GetLogVerbosityLevel {}, "test2")?;
        Ok(())
    }
}
