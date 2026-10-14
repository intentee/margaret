use std::collections::BTreeMap;

use proc_macro2::Ident;
use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::container_bindings::ContainerBindings;
use margaret_middleware_codegen::middleware_vec_tokens::middleware_vec_tokens;

use crate::session_plan::SessionPlan;

pub(crate) fn render_server_routes(
    server: &str,
    sessions: &[&SessionPlan],
    bindings: &ContainerBindings,
) -> TokenStream {
    let function = format_ident!("{server}_routes");
    let routes_name = if sessions
        .iter()
        .any(|session_plan| session_plan.session.references_routes())
    {
        format_ident!("routes")
    } else {
        format_ident!("_routes")
    };
    let middleware_module = quote! { super::super::super::middleware };
    let target = quote! { TTarget };
    let layered_wrappers: BTreeMap<&CanonicalPath, &Ident> = sessions
        .iter()
        .flat_map(|session_plan| &session_plan.session.layers)
        .map(|application| (&application.concrete, &application.wrapper))
        .collect();
    let wrapper_bounds = layered_wrappers.values().map(|wrapper| {
        quote! {
            #middleware_module::#wrapper: margaret::framework::http::http_middleware::HttpMiddleware<#target>,
        }
    });
    let entries = sessions.iter().map(|session_plan| {
        let module = format_ident!("{}", session_plan.session.module_name);
        let routes_argument = session_plan
            .session
            .injects_routes()
            .then(|| quote! { routes, });
        let middleware = if session_plan.session.layers.is_empty() {
            quote! { ::std::vec::Vec::new() }
        } else {
            middleware_vec_tokens(
                &session_plan.session.layers,
                &middleware_module,
                &target,
                bindings,
            )
        };
        let upgrade_call = quote! {
            super::super::sessions::#module::upgrade_entry(container, #routes_argument)
        };
        quote! {
            margaret::framework::http::route_entry::RouteEntry::web_socket(
                #upgrade_call,
                #middleware,
            ),
        }
    });
    let session_count = Literal::usize_unsuffixed(sessions.len());

    quote! {
        #[must_use]
        pub fn #function<#target: margaret::framework::http::forward_scope::ForwardScope>(
            container: &super::super::super::container::Container,
            #routes_name: &::std::sync::Arc<super::super::super::routes::Routes>,
        ) -> [margaret::framework::http::route_entry::RouteEntry<#target>; #session_count]
        where
            #(#wrapper_bounds)*
        {
            [#(#entries)*]
        }
    }
}
