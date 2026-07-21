use proc_macro2::TokenStream;
use quote::quote;

use crate::layer_application::LayerApplication;
use crate::middleware_instance_tokens::middleware_instance_tokens;

#[must_use]
pub fn middleware_vec_tokens(
    layers: &[LayerApplication],
    module_path: &TokenStream,
) -> TokenStream {
    let pushes = layers.iter().map(|application| {
        let element = middleware_instance_tokens(application, module_path);

        quote! { middleware.push(#element); }
    });

    quote! {
        {
            let mut middleware: ::std::vec::Vec<
                ::std::sync::Arc<dyn margaret_http::http_middleware::HttpMiddleware>,
            > = ::std::vec::Vec::new();

            #(#pushes)*

            middleware
        }
    }
}
