use super::*;
use ton_lib_core::TLBDerive;

#[derive(Debug, Clone, PartialEq, TLBDerive)]
pub enum JettonWalletMsgBody {
    Burn(JettonBurnMsg),
    BurnNotification(JettonBurnNotification),
    InternalTransfer(JettonInternalTransferMsg),
    Transfer(JettonTransferMsg),
    TransferNotification(JettonTransferNotificationMsg),
}
