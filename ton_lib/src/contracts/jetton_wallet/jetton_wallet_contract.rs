// use crate::clients::tonlib::tl_api::tl_types::{TLRawFullAccountState, TLTxId};
// use crate::clients::tonlib::tl_client::TLClient;
// use crate::contracts::jetton_wallet::jetton_wallet_data::JettonWalletData;
// use crate::contracts::ton_contract::TonContract;
// use crate::contracts::contract_ctx::TonContractState;
// use crate::errors::TonlibError;
// use crate::types::ton_address::TonAddress;
//
// pub struct JettonWalletContract<T: TLClient> {
//     state: TonContractState<T>
// }
//
// impl<T: TLClient> JettonWalletContract<T> {
//     pub async fn new(address: TonAddress, client: T, tx_id: Option<TLTxId>) -> Result<Self, TonlibError> {
//         let state = TonContractState::new(address, client, tx_id).await?;
//         Ok(Self { state })
//     }
//
//     pub fn get_wallet_data(&self) -> Result<JettonWalletData, TonlibError> {
//         let state = self.state.get_state();
//         let jetton_data = state.data.clone();
//         let jetton_data = JettonWalletData::construct_from_cell(jetton_data)?;
//         Ok(jetton_data)
//     }
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test() {
//         let vec = Vec::from("123");
//         let hex = "ab16d92db235c410bb794753493b578d3e456e31bad97a8dc46bf6422c36e96d";
//         let bytes = hex::decode(hex).unwrap();
//         let b64 = base64::encode(&bytes);
//         println!("{}", b64);
//         assert!(false)
//     }
// }
