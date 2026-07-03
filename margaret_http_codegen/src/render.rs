use std::collections::BTreeSet;

use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::path_tokens::path_tokens;

use crate::form_request_extraction::FormRequestExtraction;
use crate::http_route::HttpRoute;
use crate::http_server::HttpServer;
use crate::responder_argument::ResponderArgument;
use crate::responder_argument_binding::ResponderArgumentBinding;
use crate::responder_output::ResponderOutput;

fn access(call: TokenStream) -> TokenStream {
    quote! { #call.await }
}

fn onion(route: &HttpRoute) -> TokenStream {
    let responder = &route.responder_field;
    let responder_type = path_tokens(&route.responder_path);
    let responder_access = access(quote! { container.#responder() });
    let request_binding = if route.arguments.is_empty() {
        format_ident!("_request")
    } else {
        format_ident!("request")
    };

    let bindings = route
        .arguments
        .iter()
        .map(|argument| argument_binding(argument, &request_binding));
    let arguments = route.arguments.iter().map(|argument| &argument.holder);
    let respond_call = quote! { responder.respond(#(#arguments),*).await };
    let body_tail = match &route.responder_output {
        ResponderOutput::Plain => quote! {
            margaret_http::response_continuation::ResponseContinuation::from(#respond_call)
        },
        ResponderOutput::Intercepted { interceptor } => {
            let interceptor_field = format_ident!("{}", interceptor.field_name());

            quote! {
                margaret_http::response_continuation::ResponseContinuation::Intercept(Box::new(
                    margaret_http::interception::Interception::new(#interceptor_field, #respond_call),
                ))
            }
        }
    };
    let body = quote! {
        #(#bindings)*
        #body_tail
    };

    let captures = capture_fields(route);
    let outcome_future = quote! {
        std::pin::Pin<
            std::boxed::Box<
                dyn std::future::Future<
                    Output = margaret_http::response_continuation::ResponseContinuation,
                > + Send
                + '_,
            >,
        >
    };

    let mut handler = if captures.is_empty() {
        quote! {
            margaret_http::responder_handler::responder_handler(
                #responder_access,
                |responder: std::sync::Arc<#responder_type>,
                 #request_binding: &margaret_http::request::Request|
                 -> #outcome_future {
                    std::boxed::Box::pin(async move {
                        #body
                    })
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
                          #request_binding: &margaret_http::request::Request|
                          -> #outcome_future {
                        #(#capture_clones)*
                        std::boxed::Box::pin(async move {
                            #body
                        })
                    },
                )
            }
        }
    };

    for application in &route.layers {
        let middleware = &application.middleware_field;
        let middleware_access = access(quote! { container.#middleware() });

        handler = quote! {
            margaret_http::layer::layer(#middleware_access, #handler)
        };
    }

    handler
}

fn argument_binding(argument: &ResponderArgument, request: &Ident) -> TokenStream {
    let holder = &argument.holder;

    match &argument.binding {
        ResponderArgumentBinding::Raw { path_key } => {
            let message = format!("the route guarantees the '{path_key}' path parameter");

            quote! {
                let #holder = #request.path_param(#path_key).expect(#message).to_string();
            }
        }
        ResponderArgumentBinding::CurrentRequest => {
            if holder == request {
                TokenStream::new()
            } else {
                quote! {
                    let #holder = #request;
                }
            }
        }
        ResponderArgumentBinding::Bound { binder, path_key } => {
            let binder_field = format_ident!("{}", binder.field_name());
            let message = format!("the route guarantees the '{path_key}' path parameter");

            quote! {
                let #holder = match #binder_field
                    .bind(#request.path_param(#path_key).expect(#message).to_string())
                    .await
                {
                    Some(value) => value,
                    None => return margaret_http::response::Response::not_found().into(),
                };
            }
        }
        ResponderArgumentBinding::FormRequest { source, extraction } => {
            let variant = source.variant();

            match extraction {
                FormRequestExtraction::Result => quote! {
                    let #holder = margaret_http_validation::validate_input::validate_input(
                        #request,
                        margaret_http_validation::request_input::RequestInput::#variant,
                    );
                },
                FormRequestExtraction::Model => quote! {
                    let #holder = match margaret_http_validation::require_input::require_input(
                        #request,
                        margaret_http_validation::request_input::RequestInput::#variant,
                    ) {
                        Ok(model) => model,
                        Err(response) => return response.into(),
                    };
                },
            }
        }
    }
}

fn capture_fields(route: &HttpRoute) -> Vec<Ident> {
    let mut fields: BTreeSet<String> = BTreeSet::new();

    for argument in &route.arguments {
        if let ResponderArgumentBinding::Bound { binder, .. } = &argument.binding {
            fields.insert(binder.field_name());
        }
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

        #(#server_functions)*
    };

    let file =
        syn::parse2::<syn::File>(tokens).expect("the generated tokens form a valid Rust file");

    prettyplease::unparse(&file)
}
