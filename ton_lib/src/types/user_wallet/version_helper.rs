use crate::cell::ton_cell::TonCellRef;
use crate::cell::ton_hash::TonHash;
use crate::errors::TonlibError;
use crate::types::tlb::tlb_type::TLBType;
use crate::types::tlb::user_wallet::{
    UserWalletDataHLV2R2, UserWalletDataV1V2, UserWalletDataV3, UserWalletDataV4, UserWalletDataV5,
};
use crate::types::user_wallet::mnemonic::KeyPair;
use crate::types::user_wallet::user_wallet_code::USER_WALLET_CODE_BY_VERSION;
use crate::types::user_wallet::versions::UserWalletVersion;
use crate::types::user_wallet::versions::UserWalletVersion::*;

pub struct VersionHelper;

impl VersionHelper {
    pub fn get_data(version: UserWalletVersion, key_pair: &KeyPair, wallet_id: i32) -> Result<TonCellRef, TonlibError> {
        let public_key = TonHash::from_bytes(&key_pair.public_key)?;
        match version {
            V1R1 | V1R2 | V1R3 | V2R1 | V2R2 => UserWalletDataV1V2::new(public_key).to_cell_ref(),
            V3R1 | V3R2 => UserWalletDataV3::new(wallet_id, public_key).to_cell_ref(),
            V4R1 | V4R2 => UserWalletDataV4::new(wallet_id, public_key).to_cell_ref(),
            V5R1 => UserWalletDataV5::new(wallet_id, public_key).to_cell_ref(),
            HighloadV2R2 => UserWalletDataHLV2R2::new(wallet_id, public_key).to_cell_ref(),
            HighloadV1R1 | HighloadV1R2 | HighloadV2 | HighloadV2R1 => {
                Err(TonlibError::CustomError(format!("initial_data for {version:?} is unsupported")))
            }
        }
    }

    pub fn get_code(version: UserWalletVersion) -> Result<&'static TonCellRef, TonlibError> {
        USER_WALLET_CODE_BY_VERSION
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
