use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::quote;

use crate::container_field_ident::container_field_ident;
use crate::field_ident::field_ident;
use crate::field_type::field_type;
use crate::planned_provider::PlannedProvider;
use crate::provider::Provider;
use crate::served_container::ServedContainer;

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

fn container_declaration(served_roots: &[&PlannedProvider]) -> TokenStream {
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

    quote! {
        pub struct Container {
            #(#fields,)*
        }

        impl Container {
            #(#accessors)*
        }
    }
}

pub(crate) fn render(served: &ServedContainer) -> TokenStream {
    let container = match served {
        ServedContainer::Served { roots } => container_declaration(roots),
        ServedContainer::Unserved => TokenStream::new(),
    };

    quote! {
        #[rustfmt::skip]
        pub mod build;

        #container
    }
}
