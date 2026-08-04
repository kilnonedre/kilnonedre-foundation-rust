use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(AuditMeta)]
pub fn derive_audit_meta(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    let has_meta = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .any(|field| field.ident.as_ref().is_some_and(|ident| ident == "meta")),
            _ => false,
        },
        _ => false,
    };

    if !has_meta {
        return syn::Error::new_spanned(ident, "AuditMeta derive requires a named field `meta`")
            .to_compile_error()
            .into();
    }

    quote! {
        impl kilnonedre_common_type::HasAuditMeta for #ident {
            #[inline]
            fn meta(&self) -> &kilnonedre_common_type::CommonAuditResp {
                &self.meta
            }
        }
    }
    .into()
}
