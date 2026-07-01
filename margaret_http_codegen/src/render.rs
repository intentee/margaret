use std::collections::BTreeSet;

use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::path_tokens::path_tokens;

use crate::http_route::HttpRoute;
use crate::http_server::HttpServer;
use crate::responder_output::ResponderOutput;
use crate::route_parameter::RouteParameter;
use crate::route_parameter_binding::RouteParameterBinding;

fn access(call: TokenStream) -> TokenStream {
    quote! { #call.await }
}

fn onion(route: &HttpRoute) -> TokenStream {
    let responder = &route.responder_field;
    let responder_type = path_tokens(&route.responder_path);
    let responder_access = access(quote! { container.#responder() });
    let request_binding =
        if route.route_parameters.is_empty() && route.site_action_guards.is_empty() {
            format_ident!("_request")
        } else {
            format_ident!("request")
        };

    let user_resolution = if route.needs_user() {
        quote! {
            let authenticated_actor = gatekeeper.authenticate(&request).await;
        }
    } else {
        quote! {}
    };
    let site_guards = route.site_action_guards.iter().map(|action| {
        quote! {
            if !gatekeeper.can_site_action(authenticated_actor.as_ref(), #action).await {
                return margaret_http::response::Response::forbidden().into();
            }
        }
    });
    let general_bindings = route
        .route_parameters
        .iter()
        .filter(|route_parameter| !route_parameter.binding.is_actor())
        .map(|route_parameter| parameter_binding(route_parameter, &request_binding));
    let actor_bindings = route
        .route_parameters
        .iter()
        .filter(|route_parameter| route_parameter.binding.is_actor())
        .map(|route_parameter| parameter_binding(route_parameter, &request_binding));
    let arguments = route
        .route_parameters
        .iter()
        .map(|route_parameter| &route_parameter.holder);
    let respond_call = quote! { responder.respond(#(#arguments),*).await };
    let body_tail = match &route.responder_output {
        ResponderOutput::Plain => quote! {
            margaret_http::responded::Responded::from(#respond_call)
        },
        ResponderOutput::Intercepted { interceptor } => {
            let interceptor_field = format_ident!("{}", interceptor.field_name());

            quote! {
                margaret_http::responded::Responded::Intercept(Box::new(
                    margaret_http::interception::Interception::new(#interceptor_field, #respond_call),
                ))
            }
        }
    };
    let body = quote! {
        #user_resolution
        #(#site_guards)*
        #(#general_bindings)*
        #(#actor_bindings)*
        #body_tail
    };

    let captures = capture_fields(route);

    let mut handler = if captures.is_empty() {
        quote! {
            margaret_http::responder_handler::responder_handler(
                #responder_access,
                |responder: std::sync::Arc<#responder_type>,
                 #request_binding: margaret_http::request::Request| async move {
                    #body
                },
            )
        }
    } else {
        let capture_bindings = captures.iter().map(|field| {
            let field_access = access(quote! { container.#field() });

            quote! { let #field = #field_access; }
        });
        let capture_clones = captures
            .iter()
            .map(|field| quote! { let #field = #field.clone(); });

        quote! {
            {
                #(#capture_bindings)*
                margaret_http::responder_handler::responder_handler(
                    #responder_access,
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
        let middleware_access = access(quote! { container.#middleware() });

        handler = quote! {
            margaret_http::layer::layer(#middleware_access, #marker, #handler)
        };
    }

    handler
}

fn parameter_binding(route_parameter: &RouteParameter, request: &Ident) -> TokenStream {
    let holder = &route_parameter.holder;

    match &route_parameter.binding {
        RouteParameterBinding::Raw { path_key } => {
            let message = format!("the route guarantees the '{path_key}' path parameter");

            quote! {
                let #holder = #request.path_param(#path_key).expect(#message).to_string();
            }
        }
        RouteParameterBinding::CurrentRequest => quote! {
            let #holder = &#request;
        },
        RouteParameterBinding::RequiredActor => quote! {
            let #holder = match authenticated_actor {
                Some(authenticated_actor) => authenticated_actor,
                None => {
                    return margaret_http::response::Response::forbidden().into();
                }
            };
        },
        RouteParameterBinding::OptionalActor => quote! {
            let #holder = authenticated_actor;
        },
        RouteParameterBinding::Bound {
            binder,
            intent,
            path_key,
        } => {
            let binder_field = format_ident!("{}", binder.field_name());
            let message = format!("the route guarantees the '{path_key}' path parameter");
            let load = quote! {
                let #holder = match #binder_field
                    .bind(#request.path_param(#path_key).expect(#message).to_string())
                    .await
                {
                    Some(value) => value,
                    None => return margaret_http::response::Response::not_found().into(),
                };
            };

            match intent {
                Some(intent) => quote! {
                    #load
                    if !gatekeeper.can_crud(authenticated_actor.as_ref(), &#holder, #intent).await {
                        return margaret_http::response::Response::forbidden().into();
                    }
                },
                None => load,
            }
        }
    }
}

fn capture_fields(route: &HttpRoute) -> Vec<Ident> {
    let mut fields: BTreeSet<String> = BTreeSet::new();

    for route_parameter in &route.route_parameters {
        if let RouteParameterBinding::Bound { binder, .. } = &route_parameter.binding {
            fields.insert(binder.field_name());
        }
    }

    if route.needs_user() {
        fields.insert("gatekeeper".to_string());
    }

    if let ResponderOutput::Intercepted { interceptor } = &route.responder_output {
        fields.insert(interceptor.field_name());
    }

    fields
        .iter()
        .map(|field| format_ident!("{}", field))
        .collect()
}

fn route_call(route: &HttpRoute) -> TokenStream {
    let method = &route.method;
    let path = &route.path;
    let handler = onion(route);

    match &route.name {
        Some(name) => quote! {
            .route_with_name(
                margaret_http::method::Method::#method,
                #path,
                #name,
                #handler,
            )
        },
        None => quote! {
            .route(
                margaret_http::method::Method::#method,
                #path,
                #handler,
            )
        },
    }
}

pub(crate) fn render(routes: &[HttpRoute], servers: &[HttpServer]) -> String {
    let binder_import = if routes.iter().any(HttpRoute::is_bound) {
        quote! { use margaret_http::http_route_parameter_binder::HttpRouteParameterBinder; }
    } else {
        quote! {}
    };
    let crud_import = if routes.iter().any(HttpRoute::is_authorized) {
        quote! { use margaret_security::crud_action::CrudAction; }
    } else {
        quote! {}
    };

    let server_functions = servers.iter().map(|server| {
        let function_name = server.function_name();
        let route_calls = routes
            .iter()
            .filter(|route| route.server == server.name())
            .map(route_call);

        quote! {
            pub async fn #function_name(container: &Container) -> margaret_http::server::Server {
                let router = margaret_http::router::Router::empty()
                    #(#route_calls)*;

                margaret_http::server::Server::new(router)
            }
        }
    });

    let tokens = quote! {
        use super::container::Container;
        #binder_import
        #crud_import

        #(#server_functions)*
    };

    let file =
        syn::parse2::<syn::File>(tokens).expect("the generated tokens form a valid Rust file");

    prettyplease::unparse(&file)
}
