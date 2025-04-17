use crate::{TLBFieldAttrs, TLBHeaderAttrs};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::process::exit;
use syn::{DataStruct, Fields, Index};

struct FieldInfo {
    ident: Option<syn::Ident>,
    position: usize,
    attrs: TLBFieldAttrs,
    ty: syn::Type,
}

pub(crate) fn tlb_derive_struct(header_attrs: &TLBHeaderAttrs, data: &mut DataStruct) -> (TokenStream, TokenStream) {
    let fields = match &mut data.fields {
        Fields::Named(fields) => &mut fields.named, // For struct { field1: T, field2: T }
        Fields::Unnamed(fields) => &mut fields.unnamed, // For tuple struct (T, T)
        Fields::Unit => panic!("MyDerive only supports structs"),
    };

    let fields_info = fields
        .iter_mut()
        .enumerate()
        .map(|(position, field)| {
            let ident = &field.ident;
            let field_attrs: TLBFieldAttrs = match deluxe::extract_attributes(&mut field.attrs) {
                Ok(desc) => desc,
                Err(_err) => exit(777),
            };
            FieldInfo {
                ident: ident.clone(),
                position,
                attrs: field_attrs,
                ty: field.ty.clone(),
            }
        })
        .collect::<Vec<_>>();

    if fields_info.is_empty() || fields[0].ident.is_some() {
        derive_named_struct(header_attrs, &fields_info)
    } else {
        derive_unnamed_struct(header_attrs, &fields_info)
    }
}

fn derive_named_struct(header_attrs: &TLBHeaderAttrs, fields: &[FieldInfo]) -> (TokenStream, TokenStream) {
    let mut read_tokens = Vec::with_capacity(fields.len());
    let mut init_tokens = Vec::with_capacity(fields.len());
    let mut write_tokens = Vec::with_capacity(fields.len());
    for field in fields {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        if let Some(adapter) = &field.attrs.adapter {
            if adapter.starts_with("Dict") {
                let key_bits_len = match field.attrs.key_bits_len {
                    Some(key_bits_len) => key_bits_len,
                    None => panic!("for dict, bits_len attr is required"),
                };
                let dict_ident: TokenStream = syn::parse_str(adapter).unwrap();
                read_tokens.push(quote!(let #ident = #dict_ident::read(parser, #key_bits_len)?;));
                init_tokens.push(quote!(#ident,));
                write_tokens.push(quote!(#dict_ident::write(builder, &self.#ident, #key_bits_len)?;));
                continue;
            }
            if adapter.starts_with("ConstLen") {
                let bits_len = match field.attrs.bits_len {
                    Some(bits_len) => bits_len,
                    None => panic!("for const len, bits_len attr is required"),
                };
                read_tokens.push(quote!(let #ident = ConstLen::<#ty>::read(parser, #bits_len)?;));
                init_tokens.push(quote!(#ident,));
                write_tokens.push(quote!(ConstLen::<#ty>::write(builder, &self.#ident, #bits_len)?;));
                continue;
            }
            panic!("Unsupported adapter: {}", adapter);
        } else {
            read_tokens.push(quote!(let #ident = TLBType::read(parser)?;));
            init_tokens.push(quote!(#ident,));
            write_tokens.push(quote!(self.#ident.write(builder)?;));
        }
    }

    if header_attrs.ensure_empty.unwrap_or(false) {
        read_tokens.push(quote!(parser.ensure_empty()?;));
    }

    let read_impl_token = quote::quote! {
        #(#read_tokens)*
        Ok(Self {
            #(#init_tokens)*
        })
    };

    let write_impl_token = quote::quote! {
        #(#write_tokens)*
        Ok(())
    };
    (read_impl_token, write_impl_token)
}

fn derive_unnamed_struct(header_attrs: &TLBHeaderAttrs, fields: &[FieldInfo]) -> (TokenStream, TokenStream) {
    let mut read_tokens = Vec::with_capacity(fields.len());
    let mut init_tokens = Vec::with_capacity(fields.len());
    let mut write_tokens = Vec::with_capacity(fields.len());
    for field in fields {
        let position = Index::from(field.position);
        let read_ident = format_ident!("field_{}", field.position);
        let ty = &field.ty;
        if let Some(adapter) = &field.attrs.adapter {
            if adapter.starts_with("Dict") {
                let key_bits_len = match field.attrs.key_bits_len {
                    Some(key_bits_len) => key_bits_len,
                    None => panic!("for dict, key_bits_len attr is required"),
                };
                let dict_ident: TokenStream = syn::parse_str(adapter).unwrap();
                read_tokens.push(quote!(let #read_ident = #dict_ident::read(parser, #key_bits_len)?;));
                init_tokens.push(quote!(#read_ident,));
                write_tokens.push(quote!(#dict_ident::write(builder, &self.#position, #key_bits_len)?;));
                continue;
            }
            if adapter.starts_with("ConstLen") {
                let bits_len = match field.attrs.bits_len {
                    Some(key_bits_len) => key_bits_len,
                    None => panic!("for dict, bits_len attr is required"),
                };
                read_tokens.push(quote!(let #read_ident = ConstLen<#ty>::read(parser, #bits_len)?;));
                init_tokens.push(quote!(#read_ident,));
                write_tokens.push(quote!(ConstLen<#ty>::write(builder, &self.#position, #bits_len)?;));
                continue;
            }
            panic!("Unsupported adapter: {}", adapter);
        } else {
            read_tokens.push(quote!(let #read_ident = TLBType::read(parser)?;));
            init_tokens.push(quote!(#read_ident,));
            write_tokens.push(quote!(self.#position.write(builder)?;));
        }
    }

    if header_attrs.ensure_empty.unwrap_or(false) {
        read_tokens.push(quote!(parser.ensure_empty()?;));
    }

    let read_impl_token = quote::quote! {
        #(#read_tokens)*
        Ok(Self(
            #(#init_tokens)*
        ))
    };

    let write_impl_token = quote::quote! {
        #(#write_tokens)*
        Ok(())
    };
    (read_impl_token, write_impl_token)
}
