use base64_serde::base64_serde_type;
base64_serde_type!(pub Base64Standard, base64::engine::general_purpose::STANDARD);

pub mod client;
pub mod request;
pub mod request_context;
pub mod response;
pub(super) mod ser_de;
pub mod types;
