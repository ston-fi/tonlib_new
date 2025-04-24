use crate::clients::tonlib::tl::tl_response::TLResponse;
use crate::clients::tonlib::tl::types::UpdateSyncState;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TonNotification {
    // tonlib_api.tl, line 194
    UpdateSyncState(UpdateSyncState),
}

impl TonNotification {
    pub fn from_result(r: &TLResponse) -> Option<TonNotification> {
        match r {
            TLResponse::UpdateSyncState(sync_state) => Some(TonNotification::UpdateSyncState(sync_state.clone())),
            _ => None,
        }
    }
}
