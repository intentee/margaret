use proc_macro2::TokenStream;
use quote::quote;

use crate::layer_application::LayerApplication;
use crate::middleware_instance_tokens::middleware_instance_tokens;

#[must_use]
pub fn fold_layers(
    layers: &[LayerApplication],
    base: TokenStream,
    module_path: &TokenStream,
) -> TokenStream {
    let mut handler = base;

    for application in layers.iter().rev() {
        let instance = middleware_instance_tokens(application, module_path);

        handler = quote! {
            margaret_http::layer::layer(#instance, #handler)
        };
    }

    handler
}
