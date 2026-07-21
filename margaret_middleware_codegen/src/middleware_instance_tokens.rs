use proc_macro2::TokenStream;
use quote::quote;

use crate::layer_application::LayerApplication;

pub(crate) fn middleware_instance_tokens(
    application: &LayerApplication,
    module_path: &TokenStream,
) -> TokenStream {
    let field = &application.field;
    let wrapper = &application.wrapper;
    let routes_init = application
        .injects_routes
        .then(|| quote! { , routes: routes.clone() });
    let views_init = application
        .injects_views
        .then(|| quote! { , views: views.clone() });

    quote! {
        std::sync::Arc::new(#module_path::#wrapper { inner: container.#field().await #routes_init #views_init })
    }
}
