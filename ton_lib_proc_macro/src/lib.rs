use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, Fields};

#[derive(deluxe::ExtractAttributes, Debug)]
#[deluxe(attributes(tlb_derive))] // Match only `tlb_prefix` attributes
struct TLBDeriveAttrs {
    prefix: Option<u128>,
    bits_len: Option<u32>,
}

/// Automatic `TLBType` implementation
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
    let tokens = match &input.data {
        Data::Struct(data) => {
            let fields = match &data.fields {
                Fields::Named(fields) => &fields.named,     // For struct { field1: T, field2: T }
                Fields::Unnamed(fields) => &fields.unnamed, // For tuple struct (T, T)
                Fields::Unit => panic!("MyDerive only supports structs"),
            };
            let read_def_str = fields
                .iter()
                .map(|f| {
                    let ident = &f.ident;
                    quote!(let #ident = TLBType::read(parser)?;)
                })
                .collect::<Vec<_>>();
            let init_obj_str = fields
                .iter()
                .map(|f| {
                    let ident = &f.ident;
                    quote!(#ident,)
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
                        #(#read_def_str)*
                        Ok(Self {
                            #(#init_obj_str)*
                        })
                    }

                    fn write_def(&self, dst: &mut CellBuilder) -> Result<(), TonLibError> {
                        #(#write_def_str)*
                        Ok(())
                    }
                }
            };
            return tokens.into();
        }
        Data::Enum(data) => {
            let variant_readers = data.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                // Expect single unnamed field (like `Std(...)`)
                if let Fields::Unnamed(fields) = &variant.fields {
                    if fields.unnamed.len() != 1 {
                        panic!("Each enum variant must have exactly one unnamed field");
                    }
                    let field_type = &fields.unnamed.first().unwrap().ty;
                    quote! {
                            match #field_type::read(parser) {
                                Ok(res) => return Ok(#ident::#variant_name(res)),
                                Err(TonLibError::TLBWrongPrefix { .. }) => {},
                                Err(err) => return Err(err),
                            };
                    }
                } else {
                    panic!("TryFromParser only supports tuple-like enums");
                }
            });

            let variant_writers = data.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                // Expect single unnamed field (like `Std(...)`)
                if let Fields::Unnamed(fields) = &variant.fields {
                    if fields.unnamed.len() != 1 {
                        panic!("Each enum variant must have exactly one unnamed field");
                    }
                    quote! {
                        Self::#variant_name(ref value) => value.write(dst)?,
                    }
                } else {
                    panic!("TryFromParser only supports tuple-like enums");
                }
            });

            quote! {
                impl TLBType for #ident {
                    const PREFIX: TLBPrefix = TLBPrefix::new(#value, #bits_len);

                    fn read_def(parser: &mut CellParser) -> Result<Self, TonLibError> {
                        #(#variant_readers)*
                        Err(TonLibError::TLBEnumOutOfOptions)
                    }

                    fn write_def(&self, dst: &mut CellBuilder) -> Result<(), TonLibError> {
                        match self {
                            #(#variant_writers)*
                        }
                        Ok(())
                    }
                }
            }
        }
        _ => panic!("MyDerive only supports structs"),
    };
    tokens.into()
}
