use crate::cell::ton_cell::TonCellRef;
use crate::types::tlb::tlb_type::TLBType;
use crate::types::tlb::wallet::versions::WalletVersion;
use crate::types::tlb::wallet::versions::WalletVersion::*;
use lazy_static::lazy_static;
use std::collections::HashMap;

macro_rules! load_code {
    ($path:expr) => {
        TonCellRef::from_boc_b64(include_str!($path)).unwrap()
    };
}

lazy_static! {
    #[allow(clippy::all)]
    pub static ref TON_WALLET_CODE_BY_VERSION: HashMap<WalletVersion, TonCellRef> =
        HashMap::from([
            (V1R1, load_code!("../../../resources/user_wallet_code/wallet_v1r1.code")),
            (V1R2, load_code!("../../../resources/user_wallet_code/wallet_v1r2.code")),
            (V1R3, load_code!("../../../resources/user_wallet_code/wallet_v1r3.code")),
            (V2R1, load_code!("../../../resources/user_wallet_code/wallet_v2r1.code")),
            (V2R2, load_code!("../../../resources/user_wallet_code/wallet_v2r2.code")),
            (V3R1, load_code!("../../../resources/user_wallet_code/wallet_v3r1.code")),
            (V3R2, load_code!("../../../resources/user_wallet_code/wallet_v3r2.code")),
            (V4R1, load_code!("../../../resources/user_wallet_code/wallet_v4r1.code")),
            (V4R2, load_code!("../../../resources/user_wallet_code/wallet_v4r2.code")),
            (V5R1, load_code!("../../../resources/user_wallet_code/wallet_v5.code")),
            (HLV1R1, load_code!("../../../resources/user_wallet_code/highload_v1r1.code")),
            (HLV1R2, load_code!("../../../resources/user_wallet_code/highload_v1r2.code")),
            (HLV2, load_code!("../../../resources/user_wallet_code/highload_v2.code")),
            (HLV2R1, load_code!("../../../resources/user_wallet_code/highload_v2r1.code")),
            (HLV2R2, load_code!("../../../resources/user_wallet_code/highload_v2r2.code")),
        ]);
}
