use proc_macro2::TokenStream;
use quote::quote;

use crate::container_plan::ContainerPlan;
use crate::field_ident::field_ident;
use crate::ordered_providers::ordered_providers;
use crate::provider::Provider;

fn field_initializer(provider: &Provider) -> TokenStream {
    let name = field_ident(provider);

    quote! { #name: tokio::sync::OnceCell::new() }
}

pub(crate) fn render_build(plan: &ContainerPlan) -> TokenStream {
    let ordered = ordered_providers(plan);
    let initializers = ordered.iter().copied().map(field_initializer);

    quote! {
        pub fn build() -> super::Container {
            super::Container {
                #(#initializers,)*
            }
        }
    }
}
