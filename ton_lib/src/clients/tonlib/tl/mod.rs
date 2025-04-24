use base64::engine::general_purpose::STANDARD;
use base64_serde::base64_serde_type;

base64_serde_type!(Base64Standard, STANDARD);

pub mod error;
pub mod notification;
pub mod serial;
pub mod stack;
#[cfg(feature = "tonlib-sys")]
pub mod tl_client;
pub mod tl_request;
pub mod tl_response;
pub mod types;
