use proc_macro2::TokenStream;
use quote::quote;

use crate::construction_slot_path::construction_slot_path;
use crate::container_plan::ContainerPlan;
use crate::field_ident::field_ident;
use crate::ordered_providers::ordered_providers;
use crate::provider::Provider;

fn field_initializer(provider: &Provider) -> TokenStream {
    let name = field_ident(provider);
    let slot = construction_slot_path();

    quote! { #name: #slot::default() }
}

pub(crate) fn render_build(plan: &ContainerPlan) -> TokenStream {
    let ordered = ordered_providers(plan);
    let initializers = ordered
        .iter()
        .map(|(_, provider)| field_initializer(provider));

    quote! {
        #[must_use]
        pub fn build() -> super::Container {
            super::Container {
                #(#initializers,)*
            }
        }
    }
}
