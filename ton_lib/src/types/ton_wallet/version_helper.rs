use crate::cell::ton_cell::TonCellRef;
use crate::cell::ton_hash::TonHash;
use crate::errors::TonlibError;
use crate::types::tlb::tlb_type::TLBType;
use crate::types::tlb::wallet::wallet_data::{
    WalletHLV2R2Data, WalletV1V2Data, WalletV3Data, WalletV4Data, WalletV5Data,
};
use crate::types::ton_wallet::mnemonic::KeyPair;
use crate::types::ton_wallet::versions::UserWalletVersion;
use crate::types::ton_wallet::versions::UserWalletVersion::*;
use crate::types::ton_wallet::wallet_code::TON_WALLET_CODE_BY_VERSION;

pub struct VersionHelper;

impl VersionHelper {
    pub fn get_data(version: UserWalletVersion, key_pair: &KeyPair, wallet_id: i32) -> Result<TonCellRef, TonlibError> {
        let public_key = TonHash::from_bytes(&key_pair.public_key)?;
        match version {
            V1R1 | V1R2 | V1R3 | V2R1 | V2R2 => WalletV1V2Data::new(public_key).to_cell_ref(),
            V3R1 | V3R2 => WalletV3Data::new(wallet_id, public_key).to_cell_ref(),
            V4R1 | V4R2 => WalletV4Data::new(wallet_id, public_key).to_cell_ref(),
            V5R1 => WalletV5Data::new(wallet_id, public_key).to_cell_ref(),
            HLV2R2 => WalletHLV2R2Data::new(wallet_id, public_key).to_cell_ref(),
            HLV1R1 | HLV1R2 | HLV2 | HLV2R1 => {
                Err(TonlibError::CustomError(format!("initial_data for {version:?} is unsupported")))
            }
        }
    }

    pub fn get_code(version: UserWalletVersion) -> Result<&'static TonCellRef, TonlibError> {
        TON_WALLET_CODE_BY_VERSION
            .get(&version)
            .ok_or_else(|| TonlibError::CustomError(format!("No code found for {version:?}")))
    }

    // pub fn build_ext_msg<T: AsRef<[ArcCell]>>(
    //     version: WalletVersion,
    //     valid_until: u32,
    //     msg_seqno: u32,
    //     wallet_id: i32,
    //     msgs_refs: T,
    // ) -> Result<Cell, TonCellError> {
    //     let msgs: Vec<ArcCell> = msgs_refs.as_ref().to_vec();
    //
    //     match version {
    //         WalletVersion::V2R1 | WalletVersion::V2R2 => WalletExtMsgBodyV2 {
    //             msg_seqno,
    //             valid_until,
    //             msgs_modes: vec![3u8; msgs.len()],
    //             msgs,
    //         }
    //             .to_cell(),
    //         WalletVersion::V3R1 | WalletVersion::V3R2 => WalletExtMsgBodyV3 {
    //             subwallet_id: wallet_id,
    //             msg_seqno,
    //             valid_until,
    //             msgs_modes: vec![3u8; msgs.len()],
    //             msgs,
    //         }
    //             .to_cell(),
    //         WalletVersion::V4R1 | WalletVersion::V4R2 => WalletExtMsgBodyV4 {
    //             subwallet_id: wallet_id,
    //             valid_until,
    //             msg_seqno,
    //             opcode: 0,
    //             msgs_modes: vec![3u8; msgs.len()],
    //             msgs,
    //         }
    //             .to_cell(),
    //         WalletVersion::V5R1 => WalletExtMsgBodyV5 {
    //             wallet_id,
    //             valid_until,
    //             msg_seqno,
    //             msgs_modes: vec![3u8; msgs.len()],
    //             msgs,
    //         }
    //             .to_cell(),
    //         _ => {
    //             let err_str = format!("build_ext_msg for {version:?} is unsupported");
    //             Err(TonCellError::InternalError(err_str))
    //         }
    //     }
    // }
    //
    // pub fn sign_msg(
    //     version: WalletVersion,
    //     msg_cell: &Cell,
    //     sign: &[u8],
    // ) -> Result<Cell, TonCellError> {
    //     let signed_cell = match version {
    //         // different order
    //         WalletVersion::V5R1 => {
    //             let mut builder = CellBuilder::new();
    //             builder.store_cell(msg_cell)?;
    //             builder.store_slice(sign)?;
    //             builder.build()?
    //         }
    //         _ => {
    //             let mut builder = CellBuilder::new();
    //             builder.store_slice(sign)?;
    //             builder.store_cell(msg_cell)?;
    //             builder.build()?
    //         }
    //     };
    //     Ok(signed_cell)
    // }
}
