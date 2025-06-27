use crate::tep::nft::*;
use ton_lib_core::TLBDerive;

#[derive(Clone, Debug, PartialEq, TLBDerive)]
pub enum NftMsgBody {
    Excesses(NftExcessesMsg),
    GetStaticData(NftGetStaticDataMsg),
    OwnershipAssigned(NftOwnershipAssignedMsg),
    ReportStaticData(NftReportStaticDataMsg),
    Transfer(NftTransferMsg),
}
