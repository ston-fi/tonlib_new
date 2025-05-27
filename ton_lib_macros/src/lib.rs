mod tlb_derive;
mod tlb_derive_enum;
mod tlb_derive_struct;
mod ton_contract;

use crate::tlb_derive::{tlb_derive_impl, TLBHeaderAttrs};
use crate::ton_contract::ton_contract_impl;
use proc_macro::TokenStream;

/// Automatic `TLBType` implementation
/// ```rust
/// use ton_lib::types::tlb::adapters::ConstLen;
/// #[derive(ton_lib_macros::TLBDerive)]
/// #[tlb_derive(prefix=0x12345678, bits_len=32, ensure_empty=true)]
/// struct MyStruct {
///    #[tlb_derive(bits_len=24)]
///    pub field1: u32,
/// }
///```
#[proc_macro_derive(TLBDerive, attributes(tlb_derive))]
pub fn tlb_derive(input: TokenStream) -> TokenStream { tlb_derive_impl(input).into() }

/// Automatic `TonContract` implementation
#[proc_macro_attribute]
pub fn ton_contract(_attr: TokenStream, item: TokenStream) -> TokenStream { ton_contract_impl(_attr, item) }
