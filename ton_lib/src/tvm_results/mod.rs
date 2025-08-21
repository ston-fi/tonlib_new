// It would be better to put it somewhere else (in contracts folder, for example),
// But then we stuck with feature-flag to make it available
mod get_jetton_data_result;
mod get_wallet_address_result;
mod get_wallet_data_result;

pub use get_jetton_data_result::*;
pub use get_wallet_address_result::*;
pub use get_wallet_data_result::*;
