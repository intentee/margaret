use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::console_argument_ident::console_argument_ident;
use margaret_console_argument_codegen::argument_value::argument_value;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_console_argument_codegen::owned_weave::owned_weave;
use margaret_container::console_argument_binding::ConsoleArgumentBinding;
use margaret_container::container_bindings::ContainerBindings;

use crate::service_codegen_error::ServiceCodegenError;

pub(crate) struct ServeInputs {
    pub(crate) construction_arguments: Vec<ConsoleArgumentBinding>,
    pub(crate) prelude: TokenStream,
}

impl ServeInputs {
    pub(crate) fn resolve(
        serve_arguments: &[ConsoleArgument],
        bindings: &ContainerBindings,
    ) -> Result<Self, ServiceCodegenError> {
        let mut resolutions = Vec::with_capacity(serve_arguments.len());
        let mut construction_arguments = Vec::with_capacity(serve_arguments.len());

        for argument in serve_arguments {
            let slot = bindings.console_slot(&argument.slot_key())?;
            let ident = console_argument_ident(slot);
            let value = argument_value(argument);

            resolutions.push(quote! { let #ident = #value; });
            construction_arguments.push(ConsoleArgumentBinding {
                slot,
                value: owned_weave(argument, &quote! { #ident }, true),
            });
        }

        Ok(Self {
            construction_arguments,
            prelude: quote! { #(#resolutions)* },
        })
    }
}
