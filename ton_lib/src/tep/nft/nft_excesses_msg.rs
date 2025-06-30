use ton_lib_core::TLBDerive;

#[derive(Clone, Debug, PartialEq, TLBDerive)]
#[tlb_derive(prefix = 0xd53276db, bits_len = 32, ensure_empty = true)]
pub struct NftExcessesMsg {
    pub query_id: u64,
}
