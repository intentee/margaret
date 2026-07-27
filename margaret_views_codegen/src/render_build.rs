use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_container::container_bindings::ContainerBindings;

use crate::view::View;

pub(crate) fn render_build(views: &[View], bindings: &ContainerBindings) -> TokenStream {
    let container = format_ident!("container");
    let initializers = views.iter().map(|view| {
        let name = format_ident!("{}", view.name);
        let access = bindings.accessor_invocation(&container, &view.accessor);

        quote! { #name: #access, }
    });

    quote! {
        pub async fn build(
            container: &super::super::container::Container,
        ) -> super::Views {
            super::Views {
                #(#initializers)*
            }
        }
    }
}
