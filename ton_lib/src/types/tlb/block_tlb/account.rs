use crate::cell::ton_hash::TonHash;
use crate::types::tlb::adapters::TLBRef;
use crate::types::tlb::block_tlb::coins::{CurrencyCollection, Grams};
use crate::types::tlb::block_tlb::msg_address::MsgAddressInt;
use crate::types::tlb::block_tlb::state_init::StateInit;
use crate::types::tlb::block_tlb::var_len::VarLenBytes;
use num_bigint::BigUint;
use ton_lib_macros::TLBDerive;

#[derive(Debug, Clone, TLBDerive)]
pub struct ShardAccount {
    #[tlb_derive(adapter = "TLBRef")]
    pub account: MaybeAccount,
    pub last_tx_hash: TonHash,
    pub last_tx_lt: u64,
}

// https://github.com/ton-blockchain/ton/blob/59a8cf0ae5c3062d14ec4c89a04fee80b5fd05c1/crypto/block/block.tlb#L259
#[derive(Debug, Clone, TLBDerive)]
pub enum MaybeAccount {
    None(AccountNone),
    #[rustfmt::skip]
    Some(Box::<Account>),
}

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0b0, bits_len = 1)]
pub struct AccountNone {}

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0b1, bits_len = 1)]
pub struct Account {
    pub addr: MsgAddressInt,
    pub storage_stat: StorageInfo,
    pub storage: AccountStorage,
}

#[derive(Debug, Clone, TLBDerive)]
pub struct StorageUsed {
    pub cells: VarLenBytes<BigUint, 3>,
    pub bits: VarLenBytes<BigUint, 3>,
    pub public_cells: VarLenBytes<BigUint, 3>,
}

#[derive(Debug, Clone, TLBDerive)]
pub struct StorageInfo {
    pub used: StorageUsed,
    pub last_paid: u32,
    pub due_payment: Option<Grams>,
}

#[derive(Debug, Clone, TLBDerive)]
pub struct AccountStorage {
    pub last_trans_lt: u64,
    pub balance: CurrencyCollection,
    pub state: AccountState,
}

#[derive(Debug, Clone, TLBDerive)]
pub enum AccountState {
    Uninit(AccountStateUninit),
    Frozen(AccountStateFrozen),
    Active(AccountStateActive),
}

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0b00, bits_len = 2)]
pub struct AccountStateUninit {}

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0b01, bits_len = 2)]
pub struct AccountStateFrozen {
    pub state_hash: TonHash,
}

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0b1, bits_len = 1)]
pub struct AccountStateActive {
    pub state_init: StateInit,
}

// Almost like AccountState, but quite different ¯\_(ツ)_/¯
// https://github.com/ton-blockchain/ton/blob/ed4682066978f69ffa38dd98912ca77d4f660f66/crypto/block/block.tlb#L271
#[derive(Debug, Clone, TLBDerive)]
pub enum AccountStatus {
    Uninit(AccountStatusUninit),
    Frozen(AccountStatusFrozen),
    Active(AccountStatusActive),
    NonExist(AccountStatusNotExist),
}

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0b00, bits_len = 2)]
pub struct AccountStatusUninit {}

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0b01, bits_len = 2)]
pub struct AccountStatusFrozen {
    pub state_hash: TonHash,
}

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0b10, bits_len = 2)]
pub struct AccountStatusActive {
    pub state_init: StateInit,
}

#[derive(Debug, Clone, TLBDerive)]
#[tlb_derive(prefix = 0b11, bits_len = 2)]
pub struct AccountStatusNotExist {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::ton_cell::TonCell;
    use crate::types::tlb::block_tlb::msg_address::MsgAddressIntStd;
    use crate::types::tlb::tlb_type::TLBType;
    use std::str::FromStr;

    #[test]
    fn test_account_some() -> anyhow::Result<()> {
        let account_boc = "b5ee9c7201021d0100066d000271c00b113a994b5024a16719f69139328eb759596c38a25f59028b146fecdc3621dfe23a8bce83401229200000cc73d58b950d75499e8106934001020114ff00f4a413f4bcf2c80b030253705148e3baabcb0800c881fc78d28207072c728a2e7896228f37e17369ae121cb0eef7b4b0385f3330401a1b02016204050202cb0607020120161702f3d0cb434c0c05c6c238ecc200835c874c7c0608405e351466ea44c38601035c87e800c3b51343e803e903e90353534541168504d3214017e809400f3c58073c5b333327b55383e903e900c7e800c7d007e800c7e80004c5c3e0e80b4c7c04074cfc044bb51343e803e903e9035353449a084190adf41eeb8c089a0809001da23864658380e78b64814183fa0bc0019635355161c705f2e04904fa4021fa4430c000f2e14dfa00d4d120d0d31f018210178d4519baf2e0488040d721fa00fa4031fa4031fa0020d70b009ad74bc00101c001b0f2b19130e254431b0a03fa82107bdd97deba8ee7363805fa00fa40f82854120a70546004131503c8cb0358fa0201cf1601cf16c921c8cb0113f40012f400cb00c9f9007074c8cb02ca07cbffc9d05008c705f2e04a12a14414506603c85005fa025003cf1601cf16ccccc9ed54fa40d120d70b01c000b3915be30de02682102c76b973bae30235250c0d0e018e2191729171e2f839206e938124279120e2216e94318128739101e25023a813a0738103a370f83ca00270f83612a00170f836a07381040982100966018070f837a0bcf2b025597f0b00ec82103b9aca0070fb02f828450470546004131503c8cb0358fa0201cf1601cf16c921c8cb0113f40012f400cb00c920f9007074c8cb02ca07cbffc9d0c8801801cb0501cf1658fa02029858775003cb6bcccc9730017158cb6acce2c98011fb005005a04314c85005fa025003cf1601cf16ccccc9ed540044c8801001cb0501cf1670fa027001cb6a8210d53276db01cb1f0101cb3fc98042fb0001fc145f04323401fa40d2000101d195c821cf16c9916de2c8801001cb055004cf1670fa027001cb6a8210d173540001cb1f500401cb3f23fa4430c0008e35f828440470546004131503c8cb0358fa0201cf1601cf16c921c8cb0113f40012f400cb00c9f9007074c8cb02ca07cbffc9d012cf1697316c127001cb01e2f400c90f04f882106501f354ba8e223134365145c705f2e04902fa40d1103402c85005fa025003cf1601cf16ccccc9ed54e0258210fb88e119ba8e2132343603d15131c705f2e0498b025512c85005fa025003cf1601cf16ccccc9ed54e034248210235caf52bae30237238210cb862902bae302365b2082102508d66abae3026c311011121300088050fb0002ec3031325033c705f2e049fa40fa00d4d120d0d31f01018040d7212182100f8a7ea5ba8e4d36208210595f07bcba8e2c3004fa0031fa4031f401d120f839206e943081169fde718102f270f8380170f836a0811a7770f836a0bcf2b08e138210eed236d3ba9504d30331d19434f2c048e2e2e30d50037014150044335142c705f2e049c85003cf16c9134440c85005fa025003cf1601cf16ccccc9ed54001e3002c705f2e049d4d4d101ed54fb0400188210d372158cbadc840ff2f000ce31fa0031fa4031fa4031f401fa0020d70b009ad74bc00101c001b0f2b19130e25442162191729171e2f839206e938124279120e2216e94318128739101e25023a813a0738103a370f83ca00270f83612a00170f836a07381040982100966018070f837a0bcf2b000c082103b9aca0070fb02f828450470546004131503c8cb0358fa0201cf1601cf16c921c8cb0113f40012f400cb00c920f9007074c8cb02ca07cbffc9d0c8801801cb0501cf1658fa02029858775003cb6bcccc9730017158cb6acce2c98011fb000025bd9adf6a2687d007d207d206a6a6888122f82402027118190085adbcf6a2687d007d207d206a6a688a2f827c1400b82a3002098a81e46581ac7d0100e78b00e78b6490e4658089fa00097a00658064fc80383a6465816503e5ffe4e84000cfaf16f6a2687d007d207d206a6a68bf99e836c1783872ebdb514d9c97c283b7f0ae5179029e2b6119c39462719e4f46ed8f7413e62c780a417877407e978f01a40711411b1acb773a96bdd93fa83bb5ca8435013c8c4b3ac91f4589b4780a38646583fa0064a180400842028f452d7a4dfd74066b682365177259ed05734435be76b5fd4bd5d8af2b7c3d6801001c003e68747470733a2f2f7465746865722e746f2f757364742d746f6e2e6a736f6e";
        let cell = TonCell::from_boc_hex(account_boc)?;
        let account = MaybeAccount::from_cell(&cell)?;
        if let MaybeAccount::Some(account) = &account {
            assert_eq!(
                account.addr,
                MsgAddressIntStd {
                    anycast: None,
                    workchain: 0,
                    address: TonHash::from_str("B113A994B5024A16719F69139328EB759596C38A25F59028B146FECDC3621DFE")?,
                }
                .into()
            );
            assert_eq!(account.storage_stat.used.cells, VarLenBytes::new(29u32, 8));
            assert_eq!(account.storage_stat.used.bits, VarLenBytes::new(12090u32, 16));
            assert_eq!(account.storage_stat.used.public_cells, VarLenBytes::new(0u32, 0));

            assert_eq!(account.storage.last_trans_lt, 56199469000003u64);
            assert_eq!(account.storage.balance, CurrencyCollection::new(915473564698u64));
            if let AccountState::Active(state) = &account.storage.state {
                let code = state.state_init.code.as_ref().unwrap();
                println!("{}", code.hash());
                assert_eq!(
                    code.hash(),
                    &TonHash::from_str("18d5b6e780ff0bb451254c2c760d09d6e485638cd1407abb97078752c3c1c9ee")?
                );
            }
        } else {
            panic!("Expected Some account");
        }
        let serialized_back = account.to_cell()?;
        assert_eq!(serialized_back, cell);
        Ok(())
    }
}
