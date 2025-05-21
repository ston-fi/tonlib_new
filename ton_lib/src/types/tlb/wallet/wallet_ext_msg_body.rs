use crate::cell::build_parse::builder::CellBuilder;
use crate::cell::build_parse::parser::CellParser;
use crate::cell::ton_cell::TonCellRef;
use crate::errors::TonlibError;
use crate::types::tlb::wallet::wallet_ext_msg_utils::{
    build_inner_request, parse_inner_request, read_up_to_4_msgs, write_up_to_4_msgs, InnerRequest,
};
use crate::types::tlb::{TLBPrefix, TLB};

// Important!!!
// TLBType implementation assumes there is no signature in cell
// To read signed cell, use read_signed method

/// https://docs.ton.org/participate/wallets/contracts#wallet-v2
#[derive(Debug, PartialEq, Clone)]
pub struct WalletV2ExtMsgBody {
    pub msg_seqno: u32,
    pub valid_until: u32,
    pub msgs_modes: Vec<u8>,
    pub msgs: Vec<TonCellRef>,
}

/// https://docs.ton.org/participate/wallets/contracts#wallet-v3
/// signature is not considered as part of msg body
/// documentation is not correct about body layout
#[derive(Debug, PartialEq, Clone)]
pub struct WalletV3ExtMsgBody {
    pub subwallet_id: i32,
    pub valid_until: u32,
    pub msg_seqno: u32,
    pub msgs_modes: Vec<u8>,
    pub msgs: Vec<TonCellRef>,
}

/// https://docs.ton.org/participate/wallets/contracts#wallet-v4
/// signature is not considered as part of msg body
#[derive(Debug, PartialEq, Clone)]
pub struct WalletV4ExtMsgBody {
    pub subwallet_id: i32,
    pub valid_until: u32,
    pub msg_seqno: u32,
    pub opcode: u8,
    pub msgs_modes: Vec<u8>,
    pub msgs: Vec<TonCellRef>,
}

/// https://docs.ton.org/participate/wallets/contracts#wallet-v5
/// signature is not considered as part of msg body
/// https://github.com/ton-blockchain/wallet-contract-v5/blob/main/types.tlb
/// This implementation support only jetton transfer messages
#[derive(Debug, PartialEq, Clone)]
pub struct WalletV5ExtMsgBody {
    pub wallet_id: i32,
    pub valid_until: u32,
    pub msg_seqno: u32,
    pub msgs_modes: Vec<u8>,
    pub msgs: Vec<TonCellRef>,
}

impl TLB for WalletV2ExtMsgBody {
    fn read_definition(parser: &mut CellParser) -> Result<Self, TonlibError> {
        let msg_seqno = TLB::read(parser)?;
        let valid_until = TLB::read(parser)?;
        let (msgs_modes, msgs) = read_up_to_4_msgs(parser)?;
        Ok(Self {
            msg_seqno,
            valid_until,
            msgs_modes,
            msgs,
        })
    }

    fn write_definition(&self, dst: &mut CellBuilder) -> Result<(), TonlibError> {
        self.msg_seqno.write(dst)?;
        self.valid_until.write(dst)?;
        write_up_to_4_msgs(dst, &self.msgs, &self.msgs_modes)?;
        Ok(())
    }
}

impl WalletV2ExtMsgBody {
    pub fn read_signed(parser: &mut CellParser) -> Result<(Self, Vec<u8>), TonlibError> {
        let signature = parser.read_bits(512)?;
        Ok((Self::read(parser)?, signature))
    }
}

impl TLB for WalletV3ExtMsgBody {
    fn read_definition(parser: &mut CellParser) -> Result<Self, TonlibError> {
        let subwallet_id = TLB::read(parser)?;
        let valid_until = TLB::read(parser)?;
        let msg_seqno = TLB::read(parser)?;
        let (msgs_modes, msgs) = read_up_to_4_msgs(parser)?;
        Ok(Self {
            subwallet_id,
            msg_seqno,
            valid_until,
            msgs_modes,
            msgs,
        })
    }

    fn write_definition(&self, dst: &mut CellBuilder) -> Result<(), TonlibError> {
        self.subwallet_id.write(dst)?;
        self.valid_until.write(dst)?;
        self.msg_seqno.write(dst)?;
        write_up_to_4_msgs(dst, &self.msgs, &self.msgs_modes)?;
        Ok(())
    }
}

impl WalletV3ExtMsgBody {
    pub fn read_signed(parser: &mut CellParser) -> Result<(Self, Vec<u8>), TonlibError> {
        let signature = parser.read_bits(512)?;
        Ok((Self::read(parser)?, signature))
    }
}

impl TLB for WalletV4ExtMsgBody {
    fn read_definition(parser: &mut CellParser) -> Result<Self, TonlibError> {
        let subwallet_id = TLB::read(parser)?;
        let valid_until = TLB::read(parser)?;
        let msg_seqno = TLB::read(parser)?;
        let opcode = TLB::read(parser)?;
        if opcode != 0 {
            return Err(TonlibError::CustomError(format!("Unsupported opcode: {opcode}")));
        }
        let (msgs_modes, msgs) = read_up_to_4_msgs(parser)?;
        Ok(Self {
            subwallet_id,
            valid_until,
            msg_seqno,
            opcode,
            msgs_modes,
            msgs,
        })
    }

    fn write_definition(&self, dst: &mut CellBuilder) -> Result<(), TonlibError> {
        if self.opcode != 0 {
            return Err(TonlibError::CustomError(format!("Unsupported opcode: {}", self.opcode)));
        }
        self.subwallet_id.write(dst)?;
        self.valid_until.write(dst)?;
        self.msg_seqno.write(dst)?;
        self.opcode.write(dst)?;
        write_up_to_4_msgs(dst, &self.msgs, &self.msgs_modes)?;
        Ok(())
    }
}

impl WalletV4ExtMsgBody {
    pub fn read_signed(parser: &mut CellParser) -> Result<(Self, Vec<u8>), TonlibError> {
        let signature = parser.read_bits(512)?;
        Ok((Self::read(parser)?, signature))
    }
}

impl TLB for WalletV5ExtMsgBody {
    const PREFIX: TLBPrefix = TLBPrefix::new(0x7369676e, 32);
    fn read_definition(parser: &mut CellParser) -> Result<Self, TonlibError> {
        let wallet_id = TLB::read(parser)?;
        let valid_until = TLB::read(parser)?;
        let msg_seqno = TLB::read(parser)?;
        let inner_request = InnerRequest::read(parser)?;
        let (msgs, msgs_modes) = parse_inner_request(inner_request)?;
        Ok(Self {
            wallet_id,
            valid_until,
            msg_seqno,
            msgs_modes,
            msgs,
        })
    }

    fn write_definition(&self, dst: &mut CellBuilder) -> Result<(), TonlibError> {
        self.wallet_id.write(dst)?;
        self.valid_until.write(dst)?;
        self.msg_seqno.write(dst)?;
        let inner_req = build_inner_request(&self.msgs, &self.msgs_modes)?;
        inner_req.write(dst)?;
        Ok(())
    }
}

impl WalletV5ExtMsgBody {
    pub fn read_signed(parser: &mut CellParser) -> Result<(Self, Vec<u8>), TonlibError> {
        let body = Self::read(parser)?;
        let signature = parser.read_bits(512)?;
        Ok((body, signature))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cell::ton_cell::TonCell;
    use crate::types::tlb::wallet::constants::{WALLET_DEFAULT_ID, WALLET_V5R1_DEFAULT_ID};

    #[test]
    fn test_wallet_ext_msg_body_v3() -> anyhow::Result<()> {
        // https://tonviewer.com/transaction/b4bd316c74b4c99586e07c167979ce4a6e18db31704abd7e85b1cacb065ce66c
        let body_signed_cell = TonCell::from_boc_hex("b5ee9c7201010201008500019a86be376ea96e2f1252377976716a3d252906151feabc8e4b51506405035e45a7b4ff81f783cfe3f86483c822bcbb4f9481804990868bac69caf7af56e30fe70b29a9a317ffffffff000000000301006642007847b4630eb08d9f486fe846d5496878556dfd5a084f82a9a3fb01224e67c84c187a120000000000000000000000000000")?;
        let mut parser = body_signed_cell.parser();
        parser.read_bits(512)?;
        let body_no_sign = parser.read_cell()?;

        let body = WalletV3ExtMsgBody::read_signed(&mut body_signed_cell.parser())?.0;
        assert_eq!(body.subwallet_id, WALLET_DEFAULT_ID);
        assert_eq!(body.msg_seqno, 0);
        assert_eq!(body.valid_until, 4294967295);
        assert_eq!(body.msgs_modes, vec![3]);
        assert_eq!(body.msgs.len(), 1);

        let serial_cell = body.to_cell()?;
        assert_eq!(body_no_sign, serial_cell);
        Ok(())
    }

    #[test]
    fn test_wallet_ext_msg_body_v4() -> anyhow::Result<()> {
        // https://tonviewer.com/transaction/891dbceffb986251768d4c33bb8dcf11d522408ff78b8e683d135304ca377b8b
        let body_signed_cell = TonCell::from_boc_hex("b5ee9c7201010201008700019c9dcd3a68926ad6fb9d094c5b72901bfc359ada50f22b648c6c2223c767135d397c7489c121071e45a5316a94a533d80c41450049ebeed406c419fea99117f40629a9a31767ad328900000013000301006842007847b4630eb08d9f486fe846d5496878556dfd5a084f82a9a3fb01224e67c84c200989680000000000000000000000000000")?;
        let mut parser = body_signed_cell.parser();
        parser.read_bits(512)?;
        let body_no_sign = parser.read_cell()?;

        let body = WalletV4ExtMsgBody::read_signed(&mut body_signed_cell.parser())?.0;
        assert_eq!(body.subwallet_id, WALLET_DEFAULT_ID);
        assert_eq!(body.valid_until, 1739403913);
        assert_eq!(body.msg_seqno, 19);
        assert_eq!(body.opcode, 0);
        assert_eq!(body.msgs_modes, vec![3]);
        assert_eq!(body.msgs.len(), 1);

        let serial_cell = body.to_cell()?;
        assert_eq!(body_no_sign, serial_cell);
        Ok(())
    }

    #[test]
    fn test_wallet_ext_msg_body_v5() -> anyhow::Result<()> {
        // https://tonviewer.com/transaction/b4c5eddc52d0e23dafb2da6d022a5b6ae7eba52876fa75d32b2a95fa30c7e2f0
        let body_signed_cell = TonCell::from_boc_hex("b5ee9c720101040100940001a17369676e7fffff11ffffffff00000000bc04889cb28b36a3a00810e363a413763ec34860bf0fce552c5d36e37289fafd442f1983d740f92378919d969dd530aec92d258a0779fb371d4659f10ca1b3826001020a0ec3c86d030302006642007847b4630eb08d9f486fe846d5496878556dfd5a084f82a9a3fb01224e67c84c187a1200000000000000000000000000000000")?;
        let mut parser = body_signed_cell.parser();
        parser.read_bits(body_signed_cell.data_bits_len - 512)?;
        let sign = parser.read_bits(512)?;

        let body = WalletV5ExtMsgBody::read_signed(&mut body_signed_cell.parser())?.0;

        assert_eq!(body.wallet_id, WALLET_V5R1_DEFAULT_ID);
        assert_eq!(body.valid_until, 4294967295);
        assert_eq!(body.msg_seqno, 0);
        assert_eq!(body.msgs_modes, vec![3]);
        assert_eq!(body.msgs.len(), 1);

        let mut signed_builder = TonCell::builder();
        signed_builder.write_cell(&body.to_cell()?)?;
        signed_builder.write_bits(&sign, 512)?;
        let signed_serial = signed_builder.build()?;
        assert_eq!(body_signed_cell, signed_serial);
        let parsed_back = WalletV5ExtMsgBody::from_cell(&signed_serial)?;
        assert_eq!(body, parsed_back);
        Ok(())
    }
}
