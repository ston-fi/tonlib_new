use crate::clients::tonlibjson::tl_api::tl_types::TLTxId;
use crate::types::ton_address::TonAddress;

pub trait DataProvider {
    fn get_account_state(&self, address: TonAddress, Option<TLTxId>) -> String;
}