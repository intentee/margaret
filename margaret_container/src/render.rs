use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::quote;

use crate::container_field_ident::container_field_ident;
use crate::field_ident::field_ident;
use crate::field_type::field_type;
use crate::planned_provider::PlannedProvider;
use crate::provider::Provider;

fn field_declaration(position: usize, provider: &Provider) -> TokenStream {
    let name = container_field_ident(position);
    let field_type = field_type(provider);

    quote! { #name: #field_type }
}

fn accessor(position: usize, provider: &Provider) -> TokenStream {
    let name = field_ident(provider);
    let field = container_field_ident(position);
    let field_type = field_type(provider);

    quote! {
        pub(crate) fn #name(&self) -> #field_type {
            ::std::sync::Arc::clone(&self.#field)
        }
    }
}

pub(crate) fn render(served_roots: &[&PlannedProvider]) -> TokenStream {
    let fields = served_roots
        .iter()
        .enumerate()
        .map(|(position, entry)| field_declaration(position, &entry.provider));
    let accessors: BTreeMap<String, TokenStream> = served_roots
        .iter()
        .enumerate()
        .map(|(position, entry)| {
            (
                field_ident(&entry.provider).to_string(),
                accessor(position, &entry.provider),
            )
        })
        .collect();
    let accessors = accessors.values();
    let accessor_impl = (!served_roots.is_empty()).then(|| {
        quote! {
            impl Container {
                #(#accessors)*
            }
        }
    });

    quote! {
        #[rustfmt::skip]
        pub mod build;

        pub struct Container {
            #(#fields,)*
        }

        #accessor_impl
    }
}
