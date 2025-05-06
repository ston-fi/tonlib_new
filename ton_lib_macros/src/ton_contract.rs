use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

pub(crate) fn ton_contract_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;
    let vis = &input.vis;
    let attrs = &input.attrs;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut new_fields = input.fields.clone();
    if let syn::Fields::Named(fields) = &mut new_fields {
        fields.named.push(syn::parse_quote! {
            contract_ctx: ContractCtx
        });
    } else {
        return syn::Error::new_spanned(&input.ident, "ton_contract works only for structs named fields")
            .to_compile_error()
            .into();
    }

    let output = quote! {
        #(#attrs)*
        #vis struct #struct_name #generics #new_fields

        impl #impl_generics TonContractTrait for #struct_name #ty_generics #where_clause {
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
