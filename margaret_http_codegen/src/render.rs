use margaret_attributes::canonical_path::CanonicalPath;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use crate::http_route::HttpRoute;

pub(crate) fn render(routes: &[HttpRoute]) -> String {
    let route_calls = routes.iter().map(|route| {
        let method = &route.method;
        let path = &route.path;

        let handler = onion(route);

        quote! {
            .route(margaret_http::method::Method::#method, #path, #handler)
        }
    });

    let tokens = quote! {
        use super::container::Container;

        pub fn server(container: &Container) -> margaret_http::server::Server {
            let router = margaret_http::router::Router::empty()
                #(#route_calls)*;

            margaret_http::server::Server::new(router)
        }
    };

    let file =
        syn::parse2::<syn::File>(tokens).expect("the generated tokens form a valid Rust file");

    prettyplease::unparse(&file)
}

fn onion(route: &HttpRoute) -> TokenStream {
    let responder = &route.responder_field;
    let responder_type = crate_path_tokens(&route.responder_path);
    let request_binding = if route.route_parameters.is_empty() {
        format_ident!("_request")
    } else {
        format_ident!("request")
    };
    let arguments = route.route_parameters.iter().map(|route_parameter| {
        let name = &route_parameter.name;
        let message = format!("the route guarantees the '{name}' path parameter");

        quote! {
            #request_binding.path_param(#name).expect(#message).to_string()
        }
    });

    let mut handler = quote! {
        margaret_http::responder_handler::responder_handler(
            container.#responder(),
            |responder: std::sync::Arc<#responder_type>,
             #request_binding: margaret_http::request::Request| async move {
                responder.respond(#(#arguments),*).await
            },
        )
    };

    for application in &route.layers {
        let middleware = &application.middleware_field;
        let marker = &application.marker_value;

        handler = quote! {
            margaret_http::layer::layer(container.#middleware(), #marker, #handler)
        };
    }

    handler
}

fn crate_path_tokens(path: &CanonicalPath) -> TokenStream {
    let mut segments = path.segments().iter();
    segments
        .next()
        .expect("a canonical path has at least one segment");
    let rest = segments.map(|segment| format_ident!("{}", segment));

    quote! { crate #(:: #rest)* }
}
