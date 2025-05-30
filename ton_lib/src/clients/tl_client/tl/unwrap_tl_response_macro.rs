#[macro_export]
macro_rules! unwrap_tl_response {
    ($result:expr, $variant:ident) => {
        match $result {
            TLResponse::$variant(inner) => Ok(inner),
            TLResponse::Error { code, message } => Err(TonlibError::TLClientResponseError { code, message }),
            _ => Err(TonlibError::TLClientWrongResponse(stringify!($variant).to_string(), format!("{:?}", $result))),
        }
    };
}
