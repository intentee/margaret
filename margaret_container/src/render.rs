use std::collections::BTreeMap;
use std::collections::BTreeSet;

use proc_macro2::TokenStream;
use quote::quote;

use crate::container_field_ident::container_field_ident;
use crate::field_ident::field_ident;
use crate::field_type::field_type;
use crate::planned_provider::PlannedProvider;
use crate::provider::Provider;

fn field_declaration(position: usize, provider: &Provider, accessible: bool) -> TokenStream {
    let name = container_field_ident(position, accessible);
    let field_type = field_type(provider);

    quote! { #name: #field_type }
}

fn accessor(position: usize, provider: &Provider) -> TokenStream {
    let name = field_ident(provider);
    let field = container_field_ident(position, true);
    let field_type = field_type(provider);

    quote! {
        pub(crate) fn #name(&self) -> #field_type {
            ::std::sync::Arc::clone(&self.#field)
        }
    }
}

pub(crate) fn render(
    construction_roots: &[&PlannedProvider],
    accessible_roots: &[&PlannedProvider],
) -> TokenStream {
    let accessible: BTreeSet<_> = accessible_roots.iter().map(|entry| &entry.key).collect();
    let positions: BTreeMap<_, _> = construction_roots
        .iter()
        .enumerate()
        .map(|(position, entry)| (&entry.key, position))
        .collect();
    let fields = construction_roots
        .iter()
        .enumerate()
        .map(|(position, entry)| {
            field_declaration(position, &entry.provider, accessible.contains(&entry.key))
        });
    let accessors: BTreeMap<String, TokenStream> = accessible_roots
        .iter()
        .filter_map(|entry| {
            positions.get(&entry.key).map(|position| {
                (
                    field_ident(&entry.provider).to_string(),
                    accessor(*position, &entry.provider),
                )
            })
        })
        .collect();
    let accessors = accessors.values();
    let accessor_impl = (!accessible_roots.is_empty()).then(|| {
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
