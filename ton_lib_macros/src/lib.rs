mod tlb_derive_enum;
mod tlb_derive_struct;

use crate::tlb_derive_enum::tlb_derive_enum;
use crate::tlb_derive_struct::tlb_derive_struct;
use proc_macro::TokenStream;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, ItemStruct};

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(tlb_derive))]
struct TLBHeaderAttrs {
    prefix: Option<u128>,       // use 0 as default
    bits_len: Option<usize>,    // use 0 as default
    ensure_empty: Option<bool>, // use false as default
}

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(tlb_derive))]
struct TLBFieldAttrs {
    bits_len: Option<u32>,
    adapter: Option<String>,
}

/// Automatic `TLBType` implementation
// #[derive(ton_lib_macros::TLBDerive)]
// #[tlb_derive(prefix="0x12345678", bits_len=32, ensure_empty=true)]
// struct MyStruct {}
#[proc_macro_derive(TLBDerive, attributes(tlb_derive))]
pub fn tlb_derive(input: TokenStream) -> TokenStream {
    let mut input = syn::parse::<syn::DeriveInput>(input).unwrap();
    // Extract a description, modifying `input.attrs` to remove the matched attributes.
    let header_attrs: TLBHeaderAttrs = match deluxe::extract_attributes(&mut input) {
        Ok(desc) => desc,
        Err(e) => return e.into_compile_error().into(),
    };

    let found_crate = crate_name("ton_lib").expect("ton_lib crate not found");

    let crate_path = match found_crate {
        FoundCrate::Itself => quote::quote! { crate },
        FoundCrate::Name(name) => {
            let ident = format_ident!("{}", name);
            quote! { #ident }
        }
    };

    let ident = &input.ident;

    let (read_def_tokens, write_def_tokens) = match &mut input.data {
        Data::Struct(data) => tlb_derive_struct(&header_attrs, data),
        Data::Enum(data) => tlb_derive_enum(&crate_path, ident, data),
        _ => panic!("TLBDerive only supports structs and enums"),
    };

    let prefix_val = header_attrs.prefix.unwrap_or(0);
    let prefix_bits_len = header_attrs.bits_len.unwrap_or(0);

    quote::quote! {
        impl #crate_path::types::tlb::tlb_type::TLBType for #ident {
            const PREFIX: #crate_path::types::tlb::tlb_type::TLBPrefix = #crate_path::types::tlb::tlb_type::TLBPrefix::new(#prefix_val, #prefix_bits_len);

            fn read_definition(parser: &mut #crate_path::cell::build_parse::parser::CellParser) -> Result<Self, #crate_path::errors::TonlibError> {
                use #crate_path::types::tlb::tlb_type::TLBType;

                #read_def_tokens
            }

            fn write_definition(&self, builder: &mut #crate_path::cell::build_parse::builder::CellBuilder) -> Result<(), #crate_path::errors::TonlibError> {
                #write_def_tokens
            }
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn ton_contract(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;
    let vis = &input.vis;
    let attrs = &input.attrs;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut new_fields = input.fields.clone();
    if let syn::Fields::Named(fields) = &mut new_fields {
        fields.named.push(syn::parse_quote! {
            pub contract_ctx: ContractCtx
        });
    } else {
        return syn::Error::new_spanned(&input.ident, "ton_contract works only for structs named fields")
            .to_compile_error()
            .into();
    }

    let output = quote! {
        #(#attrs)*
        #vis struct #struct_name #generics #new_fields

        impl #impl_generics TonContract for #struct_name #ty_generics #where_clause {
            fn ctx(&self) -> &ContractCtx {
                &self.contract_ctx
            }

            fn ctx_mut(&mut self) -> &mut ContractCtx {
                &mut self.contract_ctx
            }

            fn from_ctx(ctx: ContractCtx) -> Self {
                Self {
                    contract_ctx: ctx,
                }
            }
        }
    };
    output.into()
}
