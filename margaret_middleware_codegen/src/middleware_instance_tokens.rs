use proc_macro2::TokenStream;
use quote::quote;

use margaret_container::container_bindings::ContainerBindings;

use crate::layer_application::LayerApplication;

pub(crate) fn middleware_instance_tokens(
    application: &LayerApplication,
    module_path: &TokenStream,
    bindings: &ContainerBindings,
) -> TokenStream {
    let field = &application.field;
    let wrapper = &application.wrapper;
    let threaded = bindings.console_threads(bindings.console_arguments(&application.concrete));
    let routes_init = application
        .injects_routes
        .then(|| quote! { , routes: routes.clone() });
    let views_init = application
        .injects_views
        .then(|| quote! { , views: views.clone() });

    quote! {
        std::sync::Arc::new(#module_path::#wrapper { inner: container.#field(#(#threaded),*).await #routes_init #views_init })
    }
}
