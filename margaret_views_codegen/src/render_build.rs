use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use crate::view::View;

pub(crate) fn render_build(views: &[View]) -> TokenStream {
    let initializers = views.iter().map(|view| {
        let name = format_ident!("{}", view.name);
        let accessor = format_ident!("{}", view.accessor);

        quote! { #name: container.#accessor().await, }
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
