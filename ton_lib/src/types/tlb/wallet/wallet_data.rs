use crate::cell::ton_cell::TonCellRef;
use crate::cell::ton_hash::TonHash;
use ton_lib_macros::TLBDerive;

/// Is not covered by tests and it generally means unsupported
/// WalletVersion::V1R1 | WalletVersion::V1R2 | WalletVersion::V1R3 | WalletVersion::V2R1 | WalletVersion::V2R2
#[derive(Debug, PartialEq, Clone, TLBDerive)]
pub struct WalletV1V2Data {
    pub seqno: u32,
    pub public_key: TonHash,
}

impl WalletV1V2Data {
    pub fn new(public_key: TonHash) -> Self { Self { seqno: 0, public_key } }
}

/// WalletVersion::V3R1 | WalletVersion::V3R2
#[derive(Debug, PartialEq, Clone, TLBDerive)]
pub struct WalletV3Data {
    pub seqno: u32,
    pub wallet_id: i32,
    pub public_key: TonHash,
}

impl WalletV3Data {
    pub fn new(wallet_id: i32, public_key: TonHash) -> Self {
        Self {
            seqno: 0,
            wallet_id,
            public_key,
        }
    }
}

#[derive(Debug, PartialEq, Clone, TLBDerive)]
pub struct WalletV4Data {
    pub seqno: u32,
    pub wallet_id: i32,
    pub public_key: TonHash,
    pub plugins: Option<TonCellRef>,
}

impl WalletV4Data {
    pub fn new(wallet_id: i32, public_key: TonHash) -> Self {
        Self {
            seqno: 0,
            wallet_id,
            public_key,
            plugins: None,
        }
    }
}

/// WalletVersion::V5R1
/// https://github.com/ton-blockchain/wallet-contract-v5/blob/main/types.tlb#L29
#[derive(Debug, PartialEq, Clone, TLBDerive)]
pub struct WalletV5Data {
    pub sign_allowed: bool,
    pub seqno: u32,
    pub wallet_id: i32,
    pub public_key: TonHash,
    pub extensions: Option<TonCellRef>,
}

impl WalletV5Data {
    pub fn new(wallet_id: i32, public_key: TonHash) -> Self {
        Self {
            sign_allowed: true,
            seqno: 0,
            wallet_id,
            public_key,
            extensions: None,
        }
    }
}

/// WalletVersion::HighloadV2R2, not tested
#[derive(Clone, Debug, TLBDerive)]
pub struct WalletHLV2R2Data {
    pub wallet_id: i32,
    pub last_cleaned_time: u64,
    pub public_key: TonHash,
    pub queries: Option<TonCellRef>,
}

impl WalletHLV2R2Data {
    pub fn new(wallet_id: i32, public_key: TonHash) -> Self {
        Self {
            wallet_id,
            last_cleaned_time: 0,
            public_key,
            queries: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::tlb::tlb_type::TLBType;
    use crate::types::tlb::wallet::constants::WALLET_DEFAULT_ID;
    use crate::types::tlb::wallet::constants::{WALLET_V5R1_DEFAULT_ID, WALLET_V5R1_DEFAULT_ID_TESTNET};
    use core::str::FromStr;

    #[test]
    fn test_wallet_data_v3() -> anyhow::Result<()> {
        // https://tonviewer.com/UQAMY2B4xfQO6m3YpmzfX5Za-Ning4kWKFjPdubbPPV3Ffel
        let src_boc_hex = "b5ee9c7241010101002a0000500000000129a9a317cbf377c9b73604c70bf73488ddceba14f763baef2ac70f68d1d6032a120149f4b6de3f10";
        let wallet_data = WalletV3Data::from_boc_hex(src_boc_hex)?;
        assert_eq!(wallet_data.seqno, 1);
        assert_eq!(wallet_data.wallet_id, WALLET_DEFAULT_ID);
        assert_eq!(
            wallet_data.public_key,
            TonHash::from_str("cbf377c9b73604c70bf73488ddceba14f763baef2ac70f68d1d6032a120149f4")?
        );
        let serial_boc_hex = wallet_data.to_boc_hex(false)?;
        let restored = WalletV3Data::from_boc_hex(&serial_boc_hex)?;
        assert_eq!(wallet_data, restored);
        Ok(())
    }

    #[test]
    fn test_wallet_data_v4() -> anyhow::Result<()> {
        // https://tonviewer.com/UQCS65EGyiApUTLOYXDs4jOLoQNCE0o8oNnkmfIcm0iX5FRT
        let src_boc_hex = "b5ee9c7241010101002b0000510000001429a9a317cbf377c9b73604c70bf73488ddceba14f763baef2ac70f68d1d6032a120149f440a6c9f37d";
        let wallet_data = WalletV4Data::from_boc_hex(src_boc_hex)?;
        assert_eq!(wallet_data.seqno, 20);
        assert_eq!(wallet_data.wallet_id, WALLET_DEFAULT_ID);
        assert_eq!(
            wallet_data.public_key,
            TonHash::from_str("cbf377c9b73604c70bf73488ddceba14f763baef2ac70f68d1d6032a120149f4")?
        );
        assert_eq!(wallet_data.plugins, None);

        let serial_boc_hex = wallet_data.to_boc_hex(false)?;
        let restored = WalletV4Data::from_boc_hex(&serial_boc_hex)?;
        assert_eq!(wallet_data, restored);
        Ok(())
    }

    #[test]
    fn test_wallet_data_v5() -> anyhow::Result<()> {
        // https://tonviewer.com/UQDwj2jGHWEbPpDf0I2qktDwqtv6tBCfBVNH9gJEnM-QmHDa
        let src_boc_hex = "b5ee9c7241010101002b00005180000000bfffff88e5f9bbe4db9b026385fb9a446ee75d0a7bb1dd77956387b468eb01950900a4fa20cbe13a2a";
        let wallet_data = WalletV5Data::from_boc_hex(src_boc_hex)?;
        assert_eq!(wallet_data.seqno, 1);
        assert_eq!(wallet_data.wallet_id, WALLET_V5R1_DEFAULT_ID);
        assert_eq!(
            wallet_data.public_key,
            TonHash::from_str("cbf377c9b73604c70bf73488ddceba14f763baef2ac70f68d1d6032a120149f4")?
        );
        assert_eq!(wallet_data.extensions, None);

        let serial_boc_hex = wallet_data.to_boc_hex(true)?;
        assert_eq!(src_boc_hex, serial_boc_hex);
        let restored = WalletV5Data::from_boc_hex(&serial_boc_hex)?;
        assert_eq!(wallet_data, restored);
        Ok(())
    }

    #[test]
    fn test_wallet_data_v5_testnet() -> anyhow::Result<()> {
        let src_boc_hex = "b5ee9c7201010101002b000051800000013ffffffed2b31b23dbe5144a626b9d5d1d4208e36d97e4adb472d42c073bfff85b3107e4a0";
        let wallet_data = WalletV5Data::from_boc_hex(src_boc_hex)?;
        assert_eq!(wallet_data.seqno, 2);
        assert_eq!(wallet_data.wallet_id, WALLET_V5R1_DEFAULT_ID_TESTNET);
        Ok(())
    }
}
