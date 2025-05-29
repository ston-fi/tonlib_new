use base64_serde::base64_serde_type;
base64_serde_type!(pub Base64Standard, base64::engine::general_purpose::STANDARD);

pub mod client;
pub mod interface;
pub mod request;
pub mod response;
pub(super) mod ser_de;
pub mod types;
