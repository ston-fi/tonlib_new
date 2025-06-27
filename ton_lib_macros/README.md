# ton_lib_macros

Automatically derive TLB and TonContract traits for your types

## TLB Derive

```rust
use ton_lib_macros::TLBDerive;

#[derive(Debug, Clone, PartialEq, TLBDerive)]
#[tlb_derive(prefix = 0xc4, bits_len = 8)]
pub struct GlobalVersion {
    pub version: u32,
    pub capabilities: u64,
}

// specify custom adapter (ser/de functions for TLB)
#[derive(Debug, Clone, PartialEq, TLBDerive)]
pub struct StateInit {
    #[tlb_derive(bits_len = 5)]
    pub split_depth: Option<u8>,
    pub tick_tock: Option<TickTock>,
    pub code: Option<TonCellRef>,
    pub data: Option<TonCellRef>,
    #[tlb_derive(adapter = "TLBHashMapE::<DictKeyAdapterTonHash, DictValAdapterTLB, _, _>::new(256)")]
    pub library: HashMap<TonHash, SimpleLib>,
}
```

## TonContract

```rust
#[ton_contract]
pub struct JettonMaster;
impl GetJettonData for JettonMaster {}
impl GetWalletAddress for JettonMaster {}
```