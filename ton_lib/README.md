# TonLib

This crate is heavily based on the [tonlib-rs](https://github.com/ston-fi/tonlib-rs) repository and also uses [tonlib-sys](https://github.com/ston-fi/tonlib-sys) underneath for the [tonlibjson_client](src/clients/tonlibjson) implementation.

## Features

### [cell](src/cell) module
Build and read custom cells using [TonCell](src/cell/ton_cell.rs), and serialize them to bytes using [BOC](src/cell/boc/mod.rs):

```rust
fn main() -> anyhow::Result<()> {
    let mut builder = ton_lib::cell::build_parse::builder::CellBuilder::new();
    builder.write_bit(true)?;
    builder.write_bits([1, 2, 3], 24)?;
    builder.write_num(&42, 32)?;
    let cell = builder.build()?;
    let boc = BOC::new(cell);
    let boc_b64 = boc.to_b64()?;
}
```

---

### [types](src/types) module
Contains 3 different layers:

1. [tlb](src/types/tlb):  
   The `TLBType` trait allows you to implement `serde` for your objects automatically.  
   It also includes a collection of predefined TLB types.  
   (Apologies for the `Dict` implementation — it's still in progress.)

2. [client_types](src/types/client_types):  
   Additional types for clients. (try don't use them!)

3. The rest:  
   High-level TON types such as `TonAddress` or `Wallet`.

```rust
#[derive(Debug, Clone, PartialEq, TLBDerive)]
#[tlb_derive(ensure_empty = true)]
pub struct StateInit {
    #[tlb_derive(bits_len = 5)]
    pub split_depth: Option<u8>,
    pub tick_tock: Option<TickTock>,
    pub code: Option<TonCellRef>,
    pub data: Option<TonCellRef>,
    #[tlb_derive(adapter = "Dict::<DictKeyAdapterTonHash, DictValAdapterTLB, _, _>::new(256)")]
    pub library: HashMap<TonHash, TonCellRef>,
}
```

---

### [clients](src/clients) module
- [LiteClient](src/clients/lite):  
  A "native" lite-node client that uses ADNL.  
  More straightforward to use, but less flexible.

- [TLJClient](src/clients/tonlibjson):  
  A client based on the `tonlibjson` library from the TON monorepo (requires `tonlib-sys`).  
  A bit tricky to use at times, but offers more features.\
  **Does not support `smc` methods - use `MethodEmulator` instead.** 

```rust
async fn main() -> anyhow::Result<()> {
    // LiteClient example
    let config = LiteClientConfig::new(&TON_NET_CONF_MAINNET)?;
    let lite_client = LiteClient::new(config)?;
    let mc_info = lite_client.get_mc_info().await?;
    let block_id = lite_client.lookup_mc_block(mc_info.last.seqno).await?;
    
    // TLJClient example
    let config = TLJClientConfig::new(TON_NET_CONF_MAINNET, archive_only);
    let tlj_client = TLJClient::new(config).await?;
    let mc_info = tlj_client.get_mc_info().await?;
    let block = tlj_client.lookup_mc_block(mc_info.last.seqno - 100).await?;
}
```

---

### [emulators](src/emulators) module
> Work in progress (WIP).
