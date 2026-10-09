use proc_macro2::TokenStream;
use quote::quote;

use margaret_container::container_bindings::ContainerBindings;
use margaret_container::serve_input_binding::ServeInputBinding;
use margaret_container::slotted_serve_input::SlottedServeInput;
use margaret_input_weaving::owned_weave::owned_weave;
use margaret_serve_input_codegen::serve_input_read::serve_input_read;

pub(crate) struct ServeInputs {
    pub(crate) construction_arguments: Vec<ServeInputBinding>,
    pub(crate) prelude: TokenStream,
}

impl ServeInputs {
    pub(crate) fn resolve(
        serve_inputs: &[SlottedServeInput],
        bindings: &ContainerBindings,
    ) -> Self {
        let mut resolutions = Vec::with_capacity(serve_inputs.len());
        let mut construction_arguments = Vec::with_capacity(serve_inputs.len());
        let naming = bindings.serve_input_naming();

        for SlottedServeInput { input, slot } in serve_inputs {
            let ident = naming.ident(*slot);
            let value = serve_input_read(input);

            resolutions.push(quote! { let #ident = #value; });
            construction_arguments.push(ServeInputBinding {
                slot: *slot,
                value: owned_weave(&input.weaving(), &quote! { #ident }, true),
            });
        }

        Self {
            construction_arguments,
            prelude: quote! { #(#resolutions)* },
        }
    }
}
