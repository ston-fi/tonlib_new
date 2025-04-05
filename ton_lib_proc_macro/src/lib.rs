use proc_macro::TokenStream;
use quote::quote;
use std::process::exit;
use syn::{Data, Fields};

#[derive(deluxe::ExtractAttributes, Debug)]
#[deluxe(attributes(tlb_derive))] // Match only `tlb_prefix` attributes
struct TLBPrefixAttrs {
    prefix: Option<u128>,
    bits_len: Option<u32>,
}

#[derive(deluxe::ExtractAttributes, Debug)]
#[deluxe(attributes(tlb_derive))] // Match only `tlb_prefix` attributes
struct TLBFieldAttrs {
    bits_len: Option<u32>,
}

struct FieldInfo {
    ident: syn::Ident,
    attrs: TLBFieldAttrs,
}

/// Automatic `TLBType` implementation
// #[derive(ton_lib_proc_macro::TLBDerive)]
// #[tlb_derive(prefix="0x12345678", bits_len=32)]
// struct MyStruct {}
#[proc_macro_derive(TLBDerive, attributes(tlb_derive))]
pub fn tlb_derive(input: TokenStream) -> TokenStream {
    let mut input = syn::parse::<syn::DeriveInput>(input).unwrap();
    // Extract a description, modifying `input.attrs` to remove the matched attributes.
    let prefix_attrs: TLBPrefixAttrs = match deluxe::extract_attributes(&mut input) {
        Ok(desc) => desc,
        Err(e) => return e.into_compile_error().into(),
    };

    let ident = &input.ident;
    // let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let prefix_val = prefix_attrs.prefix.unwrap_or(0);
    let prefix_bits_len = prefix_attrs.bits_len.unwrap_or(0);
    // Extract fields if it's a struct
    let tokens = match &mut input.data {
        Data::Struct(data) => {
            let fields = match &mut data.fields {
                Fields::Named(fields) => &mut fields.named, // For struct { field1: T, field2: T }
                Fields::Unnamed(fields) => &mut fields.unnamed, // For tuple struct (T, T)
                Fields::Unit => panic!("MyDerive only supports structs"),
            };

            let fields_info = fields
                .iter_mut()
                .map(|f| {
                    let ident = &f.ident;

                    let field_attrs: TLBFieldAttrs = match deluxe::extract_attributes(&mut f.attrs) {
                        Ok(desc) => desc,
                        Err(_err) => exit(777),
                    };
                    FieldInfo {
                        ident: ident.clone().unwrap(),
                        attrs: field_attrs,
                    }
                })
                .collect::<Vec<_>>();

            let read_def_str = fields_info
                .iter()
                .map(|f| {
                    let ident = &f.ident;
                    if let Some(bits_len) = f.attrs.bits_len {
                        quote!(
                            let ident_tmp: ConstLen<_, #bits_len> = TLBType::read(parser)?;
                            let #ident = ident_tmp.0;
                        )
                    } else {
                        quote!(let #ident = TLBType::read(parser)?;)
                    }
                })
                .collect::<Vec<_>>();

            let init_obj_str = fields_info
                .iter()
                .map(|f| {
                    let ident = &f.ident;
                    quote!(#ident,)
                })
                .collect::<Vec<_>>();

            let write_def_str = fields_info
                .iter()
                .map(|f| {
                    let ident = &f.ident;
                    if let Some(bits_len) = f.attrs.bits_len {
                        quote!(
                            let tmp_ident = ConstLen::<_, #bits_len>::from(&self.#ident);
                            tmp_ident.write(dst)?;
                        )
                    } else {
                        quote!(self.#ident.write(dst)?;)
                    }
                })
                .collect::<Vec<_>>();

            let tokens = quote::quote! {
                impl TLBType for #ident {
                    const PREFIX: TLBPrefix = TLBPrefix::new(#prefix_val, #prefix_bits_len);

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
                    const PREFIX: TLBPrefix = TLBPrefix::new(#prefix_val, #prefix_bits_len);

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
