use proc_macro2::TokenStream;
use quote::quote;

use crate::layer_application::LayerApplication;

pub(crate) fn middleware_instance_tokens(
    application: &LayerApplication,
    module_path: &TokenStream,
) -> TokenStream {
    let LayerApplication {
        field,
        injects_routes,
        wrapper,
    } = application;

    if *injects_routes {
        quote! {
            std::sync::Arc::new(#module_path::#wrapper { inner: container.#field().await, routes: routes.clone() })
        }
    } else {
        quote! {
            std::sync::Arc::new(#module_path::#wrapper { inner: container.#field().await })
        }
    }
}
