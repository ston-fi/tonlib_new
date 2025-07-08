use ton_lib_core::{
    cell::{TonCellRef, TonHash},
    traits::tlb::TLB,
    TLBDerive,
};

use crate::tep::SnakeFormat;

/// Metadata content representation
/// https://github.com/ton-blockchain/TEPs/blob/master/text/0064-token-data-standard.md

#[derive(Debug, Clone, PartialEq, TLBDerive)]
pub enum JettonMetaData {
    /// Off-chain content layout
    /// The first byte is 0x01 and the rest is the URI pointing to the JSON document containing the token metadata.
    /// The URI is encoded as ASCII.
    /// If the URI does not fit into one cell, then it uses the "Snake format"
    /// described in the "Data serialization" paragraph, the snake-format-prefix 0x00 is dropped.
    ///
    /// snake#00 {n:#} data:(SnakeData ~n) = ContentData;
    OffChain(OffChainMetaData),
    // On-chain content layout The first byte is 0x00 and the rest is key/value dictionary.
    // Key is sha256 hash of string.
    // Value is data encoded as described in "Data serialization" paragraph.
    //#[tlb_derive(adapter = "TLBHashMapE::<DictKeyAdapterTonHash, DictValAdapterTLB, _, _>::new(256)")]

    //OnChainOrSemiChain(HashMap<TonHash, TonCellRef>),
    // Semi-chain content layout Data encoded as described in "2. On-chain content layout".
    // The dictionary must have uri key with a value containing the URI pointing to the JSON document with token metadata.
    // Clients in this case should merge the keys of the on-chain dictionary and off-chain JSON doc.
    // In case of collisions (the field exists in both off-chain data and on-chain data), on-chain values are used.
    //SemiChain
}

#[derive(Debug, Clone, PartialEq, TLBDerive)]
#[tlb_derive(prefix = 0x00, bits_len = 8)]

pub struct OffChainMetaData {
    snake_data: i32,
}
