use std::collections::BTreeSet;

use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::name_allocator::NameAllocator;
use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_codegen_tokens::vec_literal_tokens::vec_literal_tokens;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::form_request_extraction::FormRequestExtraction;
use crate::http_route::HttpRoute;
use crate::http_route_table::HttpRouteTable;
use crate::http_server::HttpServer;
use crate::middleware_argument::MiddlewareArgument;
use crate::middleware_plan::MiddlewarePlan;
use crate::responder_argument::ResponderArgument;
use crate::responder_argument_binding::ResponderArgumentBinding;
use crate::route_group::RouteGroup;

fn access(call: TokenStream) -> TokenStream {
    quote! { #call.await }
}

fn argument_needs_request(argument: &ResponderArgument) -> bool {
    !matches!(
        argument.binding,
        ResponderArgumentBinding::CookieJar
            | ResponderArgumentBinding::Routes
            | ResponderArgumentBinding::Forwarder
    )
}

fn holder_shadows_injected_binding(binding: &ResponderArgumentBinding) -> bool {
    matches!(
        binding,
        ResponderArgumentBinding::Raw { .. }
            | ResponderArgumentBinding::Bound { .. }
            | ResponderArgumentBinding::CookieJar
            | ResponderArgumentBinding::FormRequest { .. }
            | ResponderArgumentBinding::PeerSpiffeId
    )
}

fn responder_injects_routes(route: &HttpRoute) -> bool {
    route
        .arguments
        .iter()
        .any(|argument| matches!(argument.binding, ResponderArgumentBinding::Routes))
}

fn route_references_routes(route: &HttpRoute) -> bool {
    responder_injects_routes(route) || route.layers.iter().any(|layer| layer.injects_routes)
}

fn handler_binding(route: &HttpRoute) -> Ident {
    format_ident!("{}_handler", route.responder_field)
}

fn onion(route: &HttpRoute, index: &AttributeIndex) -> TokenStream {
    let responder = &route.responder_field;
    let responder_type = path_tokens(&route.responder_path);
    let responder_access = access(quote! { container.#responder() });
    let captures = capture_fields(route, index);

    let mut allocator = NameAllocator::new();

    for argument in &route.arguments {
        if holder_shadows_injected_binding(&argument.binding) {
            allocator.reserve(&argument.holder.to_string());
        }
    }

    for field in &captures {
        allocator.reserve(&field.to_string());
    }

    let request_binding = if route.arguments.iter().any(argument_needs_request) {
        format_ident!("{}", allocator.allocate("request").field())
    } else {
        format_ident!("_request")
    };
    for argument in &route.arguments {
        if matches!(argument.binding, ResponderArgumentBinding::CurrentRequest) {
            allocator.reserve(&argument.holder.to_string());
        }
    }

    let responder_binding = format_ident!("{}", allocator.allocate("responder").field());
    let routes_local = format_ident!("{}", allocator.allocate("routes").field());

    let bindings = route
        .arguments
        .iter()
        .map(|argument| argument_binding(argument, &request_binding, index));
    let server = format_ident!("{}", route.server);
    let argument_values = route
        .arguments
        .iter()
        .map(|argument| argument_value(argument, &routes_local, &server));
    let respond_call = quote! { #responder_binding.respond(#(#argument_values),*).await };
    let body = quote! {
        #(#bindings)*
        margaret_http::response_continuation::ResponseContinuation::from(#respond_call)
    };

    let captures_routes = responder_injects_routes(route);
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

    let mut handler = if captures.is_empty() && !captures_routes {
        quote! {
            margaret_http::responder_handler::responder_handler(
                #responder_access,
                |#responder_binding: std::sync::Arc<#responder_type>,
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
        let routes_setup = captures_routes.then(|| quote! { let #routes_local = routes.clone(); });
        let routes_reclone =
            captures_routes.then(|| quote! { let #routes_local = #routes_local.clone(); });

        quote! {
            {
                #(#capture_bindings)*
                #routes_setup
                margaret_http::responder_handler::responder_handler(
                    #responder_access,
                    move |#responder_binding: std::sync::Arc<#responder_type>,
                          #request_binding: &margaret_http::request::Request|
                          -> #outcome_future {
                        #(#capture_clones)*
                        #routes_reclone
                        std::boxed::Box::pin(async move {
                            #body
                        })
                    },
                )
            }
        }
    };

    for application in &route.layers {
        let wrapper = &application.wrapper;
        let field = &application.field;
        let middleware_access = access(quote! { container.#field() });
        let middleware_expr = if application.injects_routes {
            quote! { std::sync::Arc::new(super::#wrapper { inner: #middleware_access, routes: routes.clone() }) }
        } else {
            quote! { std::sync::Arc::new(super::#wrapper { inner: #middleware_access }) }
        };

        handler = quote! {
            margaret_http::layer::layer(#middleware_expr, #handler)
        };
    }

    handler
}

fn argument_value(
    argument: &ResponderArgument,
    routes_binding: &Ident,
    server: &Ident,
) -> TokenStream {
    match &argument.binding {
        ResponderArgumentBinding::Routes => quote! { #routes_binding.as_ref() },
        ResponderArgumentBinding::Forwarder => {
            quote! { super::super::forwarders::#server::Forwarder }
        }
        _ => {
            let holder = &argument.holder;

            quote! { #holder }
        }
    }
}

fn argument_binding(
    argument: &ResponderArgument,
    request: &Ident,
    index: &AttributeIndex,
) -> TokenStream {
    let holder = &argument.holder;

    match &argument.binding {
        ResponderArgumentBinding::CookieJar => {
            quote! {
                let #holder = match margaret_http::require_cookie_jar::require_cookie_jar(
                    #request,
                ) {
                    Ok(value) => value,
                    Err(response) => return response.into(),
                };
            }
        }
        ResponderArgumentBinding::Raw { path_key } => {
            quote! {
                let #holder = match margaret_http::require_route_parameter::require_route_parameter(
                    #request,
                    #path_key,
                ) {
                    Ok(value) => value,
                    Err(response) => return response.into(),
                };
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
        ResponderArgumentBinding::Routes | ResponderArgumentBinding::Forwarder => {
            TokenStream::new()
        }
        ResponderArgumentBinding::PeerSpiffeId => {
            quote! {
                let #holder = match margaret_http::require_peer_spiffe_id::require_peer_spiffe_id(
                    #request,
                ) {
                    Ok(value) => value,
                    Err(response) => return response.into(),
                };
            }
        }
        ResponderArgumentBinding::Bound { binder, path_key } => {
            let binder_field = format_ident!("{}", index.field_name(binder));

            quote! {
                let #holder = match margaret_http::require_bound_route_parameter::require_bound_route_parameter(
                    #request,
                    #path_key,
                    #binder_field.as_ref(),
                ).await {
                    Ok(value) => value,
                    Err(response) => return response.into(),
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

fn capture_fields(route: &HttpRoute, index: &AttributeIndex) -> Vec<Ident> {
    let mut fields: BTreeSet<String> = BTreeSet::new();

    for argument in &route.arguments {
        if let ResponderArgumentBinding::Bound { binder, .. } = &argument.binding {
            fields.insert(index.field_name(binder).to_string());
        }
    }

    fields
        .iter()
        .map(|field| format_ident!("{}", field))
        .collect()
}

fn middleware_wrapper(plan: &MiddlewarePlan) -> TokenStream {
    let MiddlewarePlan {
        arguments,
        concrete,
        injects_routes,
        wrapper,
        ..
    } = plan;
    let concrete = path_tokens(concrete);
    let routes_field =
        injects_routes.then(|| quote! { routes: std::sync::Arc<super::routes::Routes>, });
    let injects_cookie_jar = arguments
        .iter()
        .any(|argument| matches!(argument, MiddlewareArgument::CookieJar));
    let request_binding = if injects_cookie_jar
        || arguments
            .iter()
            .any(|argument| matches!(argument, MiddlewareArgument::CurrentRequest))
    {
        format_ident!("request")
    } else {
        format_ident!("_request")
    };
    let cookie_jar_binding = injects_cookie_jar.then(|| {
        quote! {
            let cookie_jar = match margaret_http::require_cookie_jar::require_cookie_jar(request) {
                Ok(value) => value,
                Err(response) => return response.into(),
            };
        }
    });
    let next_binding = if arguments
        .iter()
        .any(|argument| matches!(argument, MiddlewareArgument::Next))
    {
        format_ident!("next")
    } else {
        format_ident!("_next")
    };
    let call_arguments = arguments.iter().map(|argument| match argument {
        MiddlewareArgument::CookieJar => quote! { cookie_jar },
        MiddlewareArgument::CurrentRequest => quote! { request },
        MiddlewareArgument::Next => quote! { next },
        MiddlewareArgument::Routes => quote! { &self.routes },
    });

    quote! {
        struct #wrapper {
            inner: std::sync::Arc<#concrete>,
            #routes_field
        }

        #[async_trait::async_trait]
        impl margaret_http::http_middleware::HttpMiddleware for #wrapper {
            async fn process(
                &self,
                #request_binding: &margaret_http::request::Request,
                #next_binding: margaret_http::next::Next,
            ) -> margaret_http::response_continuation::ResponseContinuation {
                #cookie_jar_binding
                self.inner.process(#(#call_arguments),*).await
            }
        }
    }
}

fn server_module(
    table: &HttpRouteTable,
    server: &HttpServer,
    index: &AttributeIndex,
) -> TokenStream {
    let function_name = server.function_name();
    let server_routes: Vec<&HttpRoute> = table
        .route_groups(server.name())
        .flat_map(RouteGroup::method_routes)
        .collect();
    let routes_param = if server_routes.iter().copied().any(route_references_routes) {
        format_ident!("routes")
    } else {
        format_ident!("_routes")
    };

    let handler_bindings = server_routes
        .iter()
        .filter(|route| route.name.is_some())
        .map(|route| {
            let binding = handler_binding(route);
            let handler = onion(route, index);

            quote! { let #binding = #handler; }
        });

    let route_entries = table.route_groups(server.name()).map(|group| {
        let path = group.path().pattern();
        let method_handlers = group.method_routes().map(|route| {
            let method = &route.method;
            let handler = if route.name.is_some() {
                let binding = handler_binding(route);

                quote! { #binding.clone() }
            } else {
                onion(route, index)
            };

            quote! {
                margaret_http::method_handler::MethodHandler::new(#method, #handler)
            }
        });
        let method_handlers = vec_literal_tokens(method_handlers);

        quote! {
            margaret_http::route_entry::RouteEntry::new(#path, #method_handlers)
        }
    });
    let route_entries = vec_literal_tokens(route_entries);

    let named_handlers = server_routes.iter().filter_map(|route| {
        route.name.as_ref().map(|name| {
            let binding = handler_binding(route);

            quote! { margaret_http::named_handler::NamedHandler::new(#name, #binding) }
        })
    });
    let named_handlers = vec_literal_tokens(named_handlers);

    quote! {
        pub async fn #function_name(
            container: &super::super::container::Container,
            #routes_param: &::std::sync::Arc<super::super::routes::Routes>,
        ) -> ::std::result::Result<
            margaret_http::server_routes::ServerRoutes,
            margaret_http::matchit::InsertError,
        > {
            #(#handler_bindings)*

            margaret_http::router::Router::build(#route_entries).map(|router| {
                margaret_http::server_routes::ServerRoutes::new(router, #named_handlers)
            })
        }
    }
}

pub(crate) fn render(
    table: &HttpRouteTable,
    servers: &[HttpServer],
    middleware_plans: &[MiddlewarePlan],
    index: &AttributeIndex,
) -> Vec<GeneratedModuleTokens> {
    let middleware_wrappers = middleware_plans.iter().map(middleware_wrapper);
    let server_declarations = servers.iter().map(|server| {
        let function_name = server.function_name();

        quote! {
            #[rustfmt::skip]
            pub mod #function_name;
        }
    });

    let http_tokens = quote! {
        #(#server_declarations)*
        #(#middleware_wrappers)*
    };

    let mut modules = vec![GeneratedModuleTokens::new("http", http_tokens)];

    for server in servers {
        modules.push(GeneratedModuleTokens::new(
            format!("http/{}", server.function_name()),
            server_module(table, server, index),
        ));
    }

    modules
}
