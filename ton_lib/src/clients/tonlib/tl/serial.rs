use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::clients::tonlib::tl::tl_request::TLRequest;
use crate::clients::tonlib::tl::tl_response::TLResponse;
use crate::errors::TonLibError;
use serde_json::Value;

pub(crate) fn serialize_function(function: &TLRequest) -> Result<CString, TonLibError> {
    // TODO: Optimize to avoid copying
    let str = serde_json::to_string(function)?;
    let cstr = CString::new(str)?;
    Ok(cstr)
}

pub(crate) fn serialize_function_extra(function: &TLRequest, extra: &str) -> Result<CString, TonLibError> {
    let mut value = serde_json::to_value(function)?;
    let obj = value.as_object_mut().unwrap();
    obj.insert(String::from("@extra"), serde_json::Value::from(extra));
    // TODO: Optimize to avoid copying
    let str = serde_json::to_string(&value)?;
    let cstr = CString::new(str)?;
    Ok(cstr)
}

pub(crate) unsafe fn deserialize_result(c_str: *const c_char) -> Result<TLResponse, TonLibError> {
    let cstr = CStr::from_ptr(c_str);
    // TODO: Optimize to avoid copying
    let str = cstr.to_str()?;
    let r = serde_json::from_str(str)?;
    Ok(r)
}

pub(crate) unsafe fn deserialize_result_extra(
    c_str: *const c_char,
) -> Result<(TLResponse, Option<String>), TonLibError> {
    let cstr = CStr::from_ptr(c_str);
    // TODO: Optimize to avoid copying
    let value: Value = serde_json::from_str(cstr.to_str()?)?;
    let extra: Option<String> =
        value.as_object().and_then(|m| m.get("@extra")).and_then(|v| v.as_str()).map(|s| s.to_string());
    let response = serde_json::from_value(value)?;
    Ok((response, extra))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clients::tonlib::tl::tl_request::TLRequest;
    use std::ffi::CString;

    #[test]
    fn it_serializes_function_extra() {
        let func = TLRequest::SetLogVerbosityLevel {
            new_verbosity_level: 100500,
        };
        let cstr: CString = serialize_function_extra(&func, "some_extra").unwrap();
        assert_eq!(
            "{\"@extra\":\"some_extra\",\"@type\":\"setLogVerbosityLevel\",\"new_verbosity_level\":100500}",
            cstr.to_str().unwrap()
        )
    }

    #[test]
    fn it_deserializes_result_extra() -> anyhow::Result<()> {
        let cstr =
            CString::new("{\"@extra\":\"some_extra\",\"@type\":\"logVerbosityLevel\",\"verbosity_level\":100500}")?;
        let (result, extra) = unsafe { deserialize_result_extra(cstr.as_ptr()) }?;
        let expected = Some(String::from("some_extra"));
        assert_eq!(extra, expected);
        match result {
            TLResponse::LogVerbosityLevel(verbosity_level) => assert_eq!(verbosity_level.verbosity_level, 100500),
            _ => panic!("Unexpected result"),
        }
        Ok(())
    }

    #[test]
    fn it_deserializes_options_info() -> anyhow::Result<()> {
        let cstr = CString::new(
            r#"{"@type":"options.info","config_info":
        {"@type":"options.configInfo","default_wallet_id":"698983191",
        "default_rwallet_init_public_key":"Puasxr0QfFZZnYISRphVse7XHKfW7pZU5SJarVHXvQ+rpzkD"},"@extra":"0"}"#,
        )?;
        let (_, extra) = unsafe { deserialize_result_extra(cstr.as_ptr()) }?;
        assert_eq!(extra, Some(String::from("0")));
        Ok(())
    }
}
