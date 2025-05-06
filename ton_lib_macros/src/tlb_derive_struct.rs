use crate::tlb_derive::TLBFieldAttrs;
use crate::TLBHeaderAttrs;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::process::exit;
use syn::{DataStruct, Fields, Index};

struct FieldInfo {
    ident: Option<syn::Ident>,
    position: usize,
    attrs: TLBFieldAttrs,
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
            let mut field_attrs: TLBFieldAttrs = match deluxe::extract_attributes(&mut field.attrs) {
                Ok(desc) => desc,
                Err(_err) => exit(777),
            };
            // bits_len=XXX is alias for ConstLen adapter
            if field_attrs.bits_len.is_some() {
                let adapter_str =
                    format!("ConstLen::<{}>::new({})", field.ty.to_token_stream(), field_attrs.bits_len.unwrap());
                field_attrs.adapter = Some(adapter_str);
            }
            if field_attrs.adapter.is_some() && field_attrs.adapter.as_ref().unwrap().starts_with("TLBRef") {
                field_attrs.adapter = Some(format!("TLBRef::<{}>::new()", field.ty.to_token_stream()));
            }

            FieldInfo {
                ident: ident.clone(),
                position,
                attrs: field_attrs,
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
        if let Some(adapter) = &field.attrs.adapter {
            let adapter_ident: TokenStream = syn::parse_str(adapter).unwrap();
            read_tokens.push(quote!(let #ident = #adapter_ident.read(parser)?.into();));
            init_tokens.push(quote!(#ident,));
            write_tokens.push(quote!(#adapter_ident.write(builder, &self.#ident)?;));
            continue;
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
        if let Some(adapter) = &field.attrs.adapter {
            let adapter_ident: TokenStream = syn::parse_str(adapter).unwrap();
            read_tokens.push(quote!(let #read_ident = #adapter_ident.read(parser)?;));
            init_tokens.push(quote!(#read_ident,));
            write_tokens.push(quote!(#adapter_ident.write(builder, &self.#position)?;));
            continue;
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
