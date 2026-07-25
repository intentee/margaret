use proc_macro2::TokenStream;
use quote::quote;

use margaret_container::container_bindings::ContainerBindings;

use crate::layer_application::LayerApplication;
use crate::middleware_instance_tokens::middleware_instance_tokens;

#[must_use]
pub fn fold_layers(
    layers: &[LayerApplication],
    base: TokenStream,
    module_path: &TokenStream,
    bindings: &ContainerBindings,
) -> TokenStream {
    let mut handler = base;

    for application in layers.iter().rev() {
        let instance = middleware_instance_tokens(application, module_path, bindings);

        handler = quote! {
            margaret::framework::http::layer::layer(#instance, #handler)
        };
    }

    handler
}
