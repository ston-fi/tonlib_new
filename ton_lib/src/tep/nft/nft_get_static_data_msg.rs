use ton_lib_core::TLBDerive;

/// ```raw
/// get_static_data#2fcb26a2
///   query_id:uint64
/// = InternalMsgBody;
/// ```
#[derive(Clone, Debug, PartialEq, TLBDerive)]
#[tlb_derive(prefix = 0x2fcb26a2, bits_len = 32, ensure_empty = true)]
pub struct NFTGetStaticDataMsg {
    pub query_id: u64,
}

impl Default for NFTGetStaticDataMsg {
    fn default() -> Self { Self::new(0) }
}

impl NFTGetStaticDataMsg {
    pub fn new(query_id: u64) -> Self { NFTGetStaticDataMsg { query_id } }
}
