use crate::tep::nft::*;
use ton_lib_core::TLBDerive;

#[derive(Clone, Debug, PartialEq, TLBDerive)]
pub enum NFTMsgBody {
    Excesses(NFTExcessesMsg),
    GetStaticData(NFTGetStaticDataMsg),
    OwnershipAssigned(NFTOwnershipAssignedMsg),
    ReportStaticData(NFTReportStaticDataMsg),
    Transfer(NFTTransferMsg),
}
