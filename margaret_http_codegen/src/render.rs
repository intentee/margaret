use std::collections::BTreeSet;

use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::authorization::Authorization;
use crate::http_route::HttpRoute;
use crate::route_parameter::RouteParameter;
use crate::route_parameter_binding::RouteParameterBinding;

pub(crate) fn render(routes: &[HttpRoute]) -> String {
    let route_calls = routes.iter().map(|route| {
        let method = &route.method;
        let path = &route.path;

        let handler = onion(route);

        quote! {
            .route(margaret_http::method::Method::#method, #path, #handler)
        }
    });

    let binder_import = if routes.iter().any(route_is_bound) {
        quote! { use margaret_http::route_parameter_binder::RouteParameterBinder; }
    } else {
        quote! {}
    };
    let gate_import = if routes.iter().any(route_is_authorized) {
        quote! {
            use margaret_http::crud_action::CrudAction;
            use margaret_http::crud_action_gate::CrudActionGate;
        }
    } else {
        quote! {}
    };

    let tokens = quote! {
        use super::container::Container;
        #binder_import
        #gate_import

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
    let bindings = route
        .route_parameters
        .iter()
        .map(|route_parameter| parameter_binding(route_parameter, &request_binding));
    let arguments = route
        .route_parameters
        .iter()
        .map(|route_parameter| format_ident!("{}", route_parameter.name));
    let body = quote! {
        #(#bindings)*
        responder.respond(#(#arguments),*).await
    };

    let captures = capture_fields(route);

    let mut handler = if captures.is_empty() {
        quote! {
            margaret_http::responder_handler::responder_handler(
                container.#responder(),
                |responder: std::sync::Arc<#responder_type>,
                 #request_binding: margaret_http::request::Request| async move {
                    #body
                },
            )
        }
    } else {
        let capture_bindings = captures
            .iter()
            .map(|field| quote! { let #field = container.#field(); });
        let capture_clones = captures
            .iter()
            .map(|field| quote! { let #field = #field.clone(); });

        quote! {
            {
                #(#capture_bindings)*
                margaret_http::responder_handler::responder_handler(
                    container.#responder(),
                    move |responder: std::sync::Arc<#responder_type>,
                          #request_binding: margaret_http::request::Request| {
                        #(#capture_clones)*
                        async move {
                            #body
                        }
                    },
                )
            }
        }
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

fn parameter_binding(route_parameter: &RouteParameter, request: &Ident) -> TokenStream {
    let name = format_ident!("{}", route_parameter.name);
    let key = &route_parameter.name;
    let message = format!("the route guarantees the '{key}' path parameter");

    match &route_parameter.binding {
        RouteParameterBinding::Raw => quote! {
            let #name = #request.path_param(#key).expect(#message).to_string();
        },
        RouteParameterBinding::Bound {
            binder,
            authorization,
        } => {
            let binder_field = format_ident!("{}", binder.field_name());
            let load = quote! {
                let #name = match #binder_field
                    .bind(#request.path_param(#key).expect(#message).to_string())
                    .await
                {
                    Some(value) => value,
                    None => return margaret_http::response::Response::not_found(),
                };
            };

            match authorization {
                Some(Authorization { gate, intent }) => {
                    let gate_field = format_ident!("{}", gate.field_name());

                    quote! {
                        #load
                        if !#gate_field.can(&#request, &#name, #intent).await {
                            return margaret_http::response::Response::forbidden();
                        }
                    }
                }
                None => load,
            }
        }
    }
}

fn capture_fields(route: &HttpRoute) -> Vec<Ident> {
    let mut fields: BTreeSet<String> = BTreeSet::new();

    for route_parameter in &route.route_parameters {
        if let RouteParameterBinding::Bound {
            binder,
            authorization,
        } = &route_parameter.binding
        {
            fields.insert(binder.field_name());

            if let Some(Authorization { gate, .. }) = authorization {
                fields.insert(gate.field_name());
            }
        }
    }

    fields
        .iter()
        .map(|field| format_ident!("{}", field))
        .collect()
}

fn route_is_bound(route: &HttpRoute) -> bool {
    route.route_parameters.iter().any(|route_parameter| {
        matches!(
            &route_parameter.binding,
            RouteParameterBinding::Bound { .. }
        )
    })
}

fn route_is_authorized(route: &HttpRoute) -> bool {
    route.route_parameters.iter().any(|route_parameter| {
        matches!(
            &route_parameter.binding,
            RouteParameterBinding::Bound {
                authorization: Some(_),
                ..
            }
        )
    })
}

fn crate_path_tokens(path: &CanonicalPath) -> TokenStream {
    let mut segments = path.segments().iter();
    segments
        .next()
        .expect("a canonical path has at least one segment");
    let rest = segments.map(|segment| format_ident!("{}", segment));

    quote! { crate #(:: #rest)* }
}
