use proc_macro2::TokenStream;
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

        pub fn server() -> margaret_http::server::Server {
            let container = Container::default();
            let router = margaret_http::router::Router::default()
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
    let mut handler = quote! {
        margaret_http::responder_handler::responder_handler(container.#responder.clone())
    };

    for application in &route.layers {
        let middleware = &application.middleware_field;
        let marker = &application.marker_value;

        handler = quote! {
            margaret_http::layer::layer(container.#middleware.clone(), #marker, #handler)
        };
    }

    handler
}
