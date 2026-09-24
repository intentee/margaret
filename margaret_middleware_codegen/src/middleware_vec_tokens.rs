use proc_macro2::TokenStream;
use quote::quote;

use margaret_container::container_bindings::ContainerBindings;

use crate::layer_application::LayerApplication;
use crate::middleware_instance_tokens::middleware_instance_tokens;

#[must_use]
pub fn middleware_vec_tokens(
    layers: &[LayerApplication],
    module_path: &TokenStream,
    bindings: &ContainerBindings,
) -> TokenStream {
    let elements = layers
        .iter()
        .map(|application| middleware_instance_tokens(application, module_path, bindings));

    quote! {
        {
            let middleware: [
                ::std::sync::Arc<dyn margaret::framework::http::http_middleware::HttpMiddleware>;
                _
            ] = [#(#elements),*];

            ::std::vec::Vec::from(middleware)
        }
    }
}
