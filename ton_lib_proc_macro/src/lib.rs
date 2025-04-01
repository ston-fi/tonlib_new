use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, Fields};

#[derive(deluxe::ExtractAttributes, Debug)]
#[deluxe(attributes(tlb_derive))] // Match only `tlb_prefix` attributes
struct TLBDeriveAttrs {
    prefix: Option<u128>,
    bits_len: Option<u32>,
}

/// Implements `TLBType` for the type:
// #[derive(ton_lib_proc_macro::TLBDerive)]
// #[tlb_derive(prefix="0x12345678", bits_len=32)]
// struct MyStruct {}
#[proc_macro_derive(TLBDerive, attributes(tlb_derive))]
pub fn tlb_derive(input: TokenStream) -> TokenStream {
    let mut input = syn::parse::<syn::DeriveInput>(input).unwrap();
    // Extract a description, modifying `input.attrs` to remove the matched attributes.
    let args: TLBDeriveAttrs = match deluxe::extract_attributes(&mut input) {
        Ok(desc) => desc,
        Err(e) => return e.into_compile_error().into(),
    };

    let value = args.prefix.unwrap_or(0);
    let bits_len = args.bits_len.unwrap_or(0);

    let ident = &input.ident;
    // let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    // Extract fields if it's a struct
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,     // For struct { field1: T, field2: T }
            Fields::Unnamed(fields) => &fields.unnamed, // For tuple struct (T, T)
            Fields::Unit => panic!("MyDerive only supports structs"),
        },
        _ => panic!("MyDerive only supports structs"),
    };

    let read_def_str = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            quote!(#ident: TLBType::read(parser)?,)
        })
        .collect::<Vec<_>>();

    let write_def_str = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            quote!(self.#ident.write(dst)?;)
        })
        .collect::<Vec<_>>();

    let tokens = quote::quote! {
        impl TLBType for #ident {
            const PREFIX: TLBPrefix = TLBPrefix::new(#value, #bits_len);

            fn read_def(parser: &mut CellParser) -> Result<Self, TonLibError> {
                Ok(Self {
                    #(#read_def_str)*
                })
            }

            fn write_def(&self, dst: &mut CellBuilder) -> Result<(), TonLibError> {
                #(#write_def_str)*
                Ok(())
            }
        }
    };
    tokens.into()
}
