use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use crate::field_ident::field_ident;
use crate::field_type::field_type;
use crate::planned_provider::PlannedProvider;
use crate::provider::Provider;

fn field_declaration(provider: &Provider, accessible: bool) -> TokenStream {
    let field = field_ident(provider);
    let name = if accessible {
        field
    } else {
        format_ident!("_{field}")
    };
    let field_type = field_type(provider);

    quote! { #name: #field_type }
}

fn accessor(provider: &Provider) -> TokenStream {
    let name = field_ident(provider);
    let field_type = field_type(provider);

    quote! {
        pub(crate) fn #name(&self) -> #field_type {
            ::std::sync::Arc::clone(&self.#name)
        }
    }
}

pub(crate) fn render(
    construction_roots: &[&PlannedProvider],
    accessible_roots: &[&PlannedProvider],
) -> TokenStream {
    let accessible: std::collections::BTreeSet<_> =
        accessible_roots.iter().map(|entry| &entry.key).collect();
    let entries = construction_roots.iter().map(|entry| &entry.provider);
    let fields = entries
        .map(|provider| field_declaration(provider, accessible.contains(&provider.provided.key())));
    let accessors = accessible_roots
        .iter()
        .map(|entry| accessor(&entry.provider));
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
