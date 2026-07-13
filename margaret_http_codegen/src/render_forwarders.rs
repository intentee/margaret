use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::http_route::HttpRoute;
use crate::http_server::HttpServer;
use crate::path_parameter_names::path_parameter_names;

fn route_name(route: &HttpRoute) -> &str {
    route
        .name
        .as_deref()
        .expect("only named routes reach the forwarders module")
}

fn forwardable_routes_for<'route>(
    routes: &'route [HttpRoute],
    server: &HttpServer,
) -> Vec<&'route HttpRoute> {
    let mut forwardable: Vec<&HttpRoute> = routes
        .iter()
        .filter(|route| {
            route.server == server.name() && route.name.is_some() && route.method == "GET"
        })
        .collect();

    forwardable.sort_by(|first, second| route_name(first).cmp(route_name(second)));

    forwardable
}

fn forward_method(route: &HttpRoute) -> TokenStream {
    let method = format_ident!("{}", route_name(route));
    let name = route_name(route);
    let placeholders =
        path_parameter_names(&route.path).expect("the route path was validated during http codegen");
    let parameters = placeholders.iter().map(|placeholder| {
        let parameter = format_ident!("{}", placeholder);

        quote! { #parameter: ::std::string::String }
    });
    let path_params = if placeholders.is_empty() {
        quote! { ::std::collections::HashMap::new() }
    } else {
        let entries = placeholders.iter().map(|placeholder| {
            let parameter = format_ident!("{}", placeholder);

            quote! { (#placeholder.to_string(), #parameter) }
        });

        quote! { ::std::collections::HashMap::from([#(#entries),*]) }
    };

    quote! {
        pub fn #method(&self, #(#parameters),*) -> margaret_http::forward::Forward {
            margaret_http::forward::Forward::new(#name, #path_params)
        }
    }
}

fn server_forwarder(routes: &[HttpRoute], server: &HttpServer) -> TokenStream {
    let methods = forwardable_routes_for(routes, server)
        .into_iter()
        .map(forward_method);

    quote! {
        pub struct Forwarder;

        impl Forwarder {
            #(#methods)*
        }
    }
}

pub(crate) fn render_forwarders(
    routes: &[HttpRoute],
    servers: &[HttpServer],
) -> Vec<GeneratedModuleTokens> {
    let server_declarations = servers.iter().map(|server| {
        let field = format_ident!("{}", server.name());

        quote! {
            #[rustfmt::skip]
            pub mod #field;
        }
    });

    let mut modules = vec![GeneratedModuleTokens::new(
        "forwarders",
        quote! { #(#server_declarations)* },
    )];

    for server in servers {
        modules.push(GeneratedModuleTokens::new(
            format!("forwarders/{}", server.name()),
            server_forwarder(routes, server),
        ));
    }

    modules
}
