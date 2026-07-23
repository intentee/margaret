use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::container_bindings::ContainerBindings;

use crate::view::View;

pub(crate) fn render_build(
    views: &[View],
    console_arguments: &[ConsoleArgument],
    bindings: &ContainerBindings,
) -> TokenStream {
    let parameters = bindings.console_parameters(console_arguments);
    let initializers = views.iter().map(|view| {
        let name = format_ident!("{}", view.name);
        let accessor = format_ident!("{}", view.accessor);
        let threaded = bindings.console_threads(bindings.console_arguments(&view.concrete_path));

        quote! { #name: container.#accessor(#(#threaded),*).await, }
    });

    quote! {
        pub async fn build(
            container: &super::super::container::Container,
            #(#parameters)*
        ) -> super::Views {
            super::Views {
                #(#initializers)*
            }
        }
    }
}
