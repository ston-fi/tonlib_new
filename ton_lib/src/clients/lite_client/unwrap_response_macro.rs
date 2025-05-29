#[macro_export]
macro_rules! unwrap_lite_response {
    ($result:expr, $variant:ident) => {
        match $result {
            Response::$variant(inner) => Ok(inner),
            Response::Error(err) => Err(TonlibError::LiteClientErrorResponse(err)),
            _ => Err(TonlibError::LiteClientWrongResponse(stringify!($variant).to_string(), format!("{:?}", $result))),
        }
    };
}
