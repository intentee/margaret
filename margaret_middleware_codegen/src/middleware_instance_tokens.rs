use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_container::accessor_failure::AccessorFailure;
use margaret_container::container_bindings::ContainerBindings;

use crate::layer_application::LayerApplication;

pub(crate) fn middleware_instance_tokens(
    application: &LayerApplication,
    module_path: &TokenStream,
    bindings: &ContainerBindings,
) -> TokenStream {
    let container = format_ident!("container");
    let wrapper = &application.wrapper;
    let woven = bindings.console_weaves(bindings.console_arguments(&application.concrete));
    let inner = bindings.accessor_invocation(
        &container,
        &application.field.to_string(),
        &woven,
        &AccessorFailure::Propagate,
    );
    let routes_init = application
        .injects_routes
        .then(|| quote! { , routes: routes.clone() });
    let views_init = application
        .injects_views
        .then(|| quote! { , views: views.clone() });

    quote! {
        std::sync::Arc::new(#module_path::#wrapper { inner: #inner #routes_init #views_init })
    }
}
