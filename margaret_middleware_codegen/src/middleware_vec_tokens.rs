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
    let pushes = layers.iter().map(|application| {
        let element = middleware_instance_tokens(application, module_path, bindings);

        quote! { middleware.push(#element); }
    });

    quote! {
        {
            let mut middleware: ::std::vec::Vec<
                ::std::sync::Arc<dyn margaret::framework::http::http_middleware::HttpMiddleware>,
            > = ::std::vec::Vec::new();

            #(#pushes)*

            middleware
        }
    }
}
