use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_codegen_tokens::too_many_arguments_expect::too_many_arguments_expect;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::accessor_failure::AccessorFailure;
use margaret_container::construction_error_path::construction_error_path;
use margaret_container::container_bindings::ContainerBindings;

use crate::view::View;

pub(crate) fn render_build(
    views: &[View],
    console_arguments: &[ConsoleArgument],
    bindings: &ContainerBindings,
) -> TokenStream {
    let container = format_ident!("container");
    let parameters = bindings.console_parameters(console_arguments);
    let initializers = views.iter().map(|view| {
        let name = format_ident!("{}", view.name);
        let woven = bindings.console_weaves(bindings.console_arguments(&view.concrete_path));
        let access = bindings.accessor_invocation(
            &container,
            &view.accessor,
            &woven,
            &AccessorFailure::Propagate,
        );

        quote! { #name: #access, }
    });

    let parameter_count = 1 + parameters.len();
    let too_many_arguments = too_many_arguments_expect(parameter_count);
    let views_value = quote! {
        super::Views {
            #(#initializers)*
        }
    };

    let (return_type, body) = if bindings.has_fallible_accessors() {
        let error = construction_error_path();

        (
            quote! { Result<super::Views, #error> },
            quote! { Ok(#views_value) },
        )
    } else {
        (quote! { super::Views }, views_value)
    };

    quote! {
        #too_many_arguments
        pub async fn build(
            container: &super::super::container::Container,
            #(#parameters)*
        ) -> #return_type {
            #body
        }
    }
}
