use std::collections::BTreeSet;

use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::name_allocator::NameAllocator;
use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_codegen_tokens::too_many_lines_allow::too_many_lines_allow;
use margaret_codegen_tokens::vec_literal_tokens::vec_literal_tokens;
use margaret_container::container_bindings::ContainerBindings;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_middleware_codegen::fold_layers::fold_layers;
use margaret_request_binding_codegen::authenticated_user_application::AuthenticatedUserApplication;
use margaret_request_binding_codegen::binding_reads_request::binding_reads_request;
use margaret_request_binding_codegen::binding_shadows_request::binding_shadows_request;
use margaret_request_binding_codegen::bound_parameter::BoundParameter;
use margaret_request_binding_codegen::captured_provider::CapturedProvider;
use margaret_request_binding_codegen::captured_provider_kind::CapturedProviderKind;
use margaret_request_binding_codegen::captured_providers::CapturedProviders;
use margaret_request_binding_codegen::extraction_context::ExtractionContext;
use margaret_request_binding_codegen::injects_routes::injects_routes;
use margaret_request_binding_codegen::injects_views::injects_views;
use margaret_request_binding_codegen::render_bound_request_extractions::render_bound_request_extractions;
use margaret_request_binding_codegen::render_request_extraction::render_request_extraction;
use margaret_request_binding_codegen::request_binding::RequestBinding;

use crate::http_route::HttpRoute;
use crate::http_route_table::HttpRouteTable;
use crate::http_server::HttpServer;
use crate::route_group::RouteGroup;

fn responder_injects_routes(route: &HttpRoute) -> bool {
    injects_routes(&route.arguments)
}

fn responder_injects_views(route: &HttpRoute) -> bool {
    injects_views(&route.arguments)
}

fn route_applications(route: &HttpRoute) -> impl Iterator<Item = &AuthenticatedUserApplication> {
    route.arguments.iter().filter_map(|argument| {
        let RequestBinding::AuthenticatedUser { application, .. } = &argument.binding else {
            return None;
        };

        Some(application)
    })
}

fn providers_inject_routes(route: &HttpRoute) -> bool {
    route_applications(route).any(|application| application.injects_routes)
}

fn providers_inject_views(route: &HttpRoute) -> bool {
    route_applications(route).any(|application| application.injects_views)
}

fn route_references_routes(route: &HttpRoute) -> bool {
    responder_injects_routes(route)
        || providers_inject_routes(route)
        || route.layers.iter().any(|layer| layer.injects_routes)
}

fn route_references_views(route: &HttpRoute) -> bool {
    responder_injects_views(route)
        || providers_inject_views(route)
        || route.layers.iter().any(|layer| layer.injects_views)
}

struct HandlerNames {
    captured: CapturedProviders,
    request_binding: Ident,
    responder_binding: Ident,
    routes_local: Ident,
    views_local: Ident,
}

fn allocate_handler_names(route: &HttpRoute) -> HandlerNames {
    let mut allocator = NameAllocator::new();

    for argument in &route.arguments {
        if binding_shadows_request(&argument.binding) {
            allocator.reserve(&argument.holder.to_string());
        }
    }

    let request_binding = if route
        .arguments
        .iter()
        .any(|argument| binding_reads_request(&argument.binding))
    {
        format_ident!("{}", allocator.allocate("request").field())
    } else {
        format_ident!("_request")
    };

    for argument in &route.arguments {
        if matches!(argument.binding, RequestBinding::CurrentRequest) {
            allocator.reserve(&argument.holder.to_string());
        }
    }

    let responder_binding = format_ident!("{}", allocator.allocate("responder").field());
    let routes_local = format_ident!("{}", allocator.allocate("routes").field());
    let views_local = format_ident!("{}", allocator.allocate("views").field());
    let captured = CapturedProviders::capture(&route.arguments, &mut allocator);

    HandlerNames {
        captured,
        request_binding,
        responder_binding,
        routes_local,
        views_local,
    }
}

fn responder_body(
    route: &HttpRoute,
    HandlerNames {
        captured,
        request_binding,
        responder_binding,
        routes_local,
        views_local,
    }: &HandlerNames,
) -> TokenStream {
    let bound_bindings = render_bound_request_extractions(
        &route.arguments,
        captured,
        &TokenStream::new(),
        request_binding,
        &quote! { return ::std::result::Result::Err(error.into()) },
        &quote! {
            return ::std::result::Result::Ok(
                margaret::framework::http::response_continuation::ResponseContinuation::from(
                    margaret::framework::http::response::Response::not_found(),
                ),
            )
        },
    );
    let bindings_tokens = route
        .arguments
        .iter()
        .map(|argument| argument_binding(argument, request_binding, captured));
    let server = format_ident!("{}", route.server);
    let argument_values = route
        .arguments
        .iter()
        .map(|argument| argument_value(argument, routes_local, views_local, &server));
    let method_name = &route.method_name;
    let respond_call = if route.is_async {
        quote! { #responder_binding.#method_name(#(#argument_values),*).await }
    } else {
        quote! { #responder_binding.#method_name(#(#argument_values),*) }
    };
    let body = quote! {
        #bound_bindings
        #(#bindings_tokens)*
        #respond_call
            .map(margaret::framework::http::response_continuation::ResponseContinuation::from)
            .map_err(margaret::framework::http::handler_error::HandlerError::consumer)
    };

    body
}

fn onion(route: &HttpRoute, bindings: &ContainerBindings) -> TokenStream {
    let responder_type = path_tokens(&route.responder_path);
    let responder_access = bindings.accessor_invocation(
        &format_ident!("container"),
        &route.responder_field.to_string(),
    );
    let names = allocate_handler_names(route);
    let HandlerNames {
        captured,
        request_binding,
        responder_binding,
        routes_local,
        views_local,
    } = &names;

    let body = responder_body(route, &names);

    let captures_routes = responder_injects_routes(route);
    let captures_views = responder_injects_views(route);
    let outcome_future = quote! {
        margaret::framework::http::handler_future::HandlerFuture<'_>
    };

    let handler = if captured.is_empty() && !captures_routes && !captures_views {
        quote! {
            margaret::framework::http::responder_handler::responder_handler(
                #responder_access,
                |#responder_binding: std::sync::Arc<#responder_type>,
                 #request_binding: &margaret::framework::http::request::Request|
                 -> #outcome_future {
                    std::boxed::Box::pin(async move {
                        #body
                    })
                },
            )
        }
    } else {
        let capture_bindings = captured
            .entries()
            .map(|capture| capture_binding(capture, bindings));
        let capture_clones = captured.entries().map(|CapturedProvider { local, .. }| {
            quote! { let #local = #local.clone(); }
        });
        let routes_setup = captures_routes.then(|| quote! { let #routes_local = routes.clone(); });
        let routes_reclone =
            captures_routes.then(|| quote! { let #routes_local = #routes_local.clone(); });
        let views_setup = captures_views.then(|| quote! { let #views_local = views.clone(); });
        let views_reclone =
            captures_views.then(|| quote! { let #views_local = #views_local.clone(); });

        quote! {
            {
                #(#capture_bindings)*
                #routes_setup
                #views_setup
                margaret::framework::http::responder_handler::responder_handler(
                    #responder_access,
                    move |#responder_binding: std::sync::Arc<#responder_type>,
                          #request_binding: &margaret::framework::http::request::Request|
                          -> #outcome_future {
                        #(#capture_clones)*
                        #routes_reclone
                        #views_reclone
                        std::boxed::Box::pin(async move {
                            #body
                        })
                    },
                )
            }
        }
    };

    fold_layers(
        &route.layers,
        handler,
        &quote! { super::super::middleware },
        bindings,
    )
}

fn argument_value(
    argument: &BoundParameter,
    routes_binding: &Ident,
    views_binding: &Ident,
    server: &Ident,
) -> TokenStream {
    match &argument.binding {
        RequestBinding::Routes => quote! { #routes_binding.as_ref() },
        RequestBinding::Views => quote! { #views_binding.as_ref() },
        RequestBinding::Forwarder => {
            quote! { super::super::forwarders::#server::Forwarder }
        }
        _ => {
            let holder = &argument.holder;

            quote! { #holder }
        }
    }
}

fn capture_binding(
    CapturedProvider { kind, local }: &CapturedProvider,
    bindings: &ContainerBindings,
) -> TokenStream {
    let container = format_ident!("container");

    match kind {
        CapturedProviderKind::AuthenticatedUser { application } => {
            let wrapper = &application.wrapper;
            let inner_access = bindings.accessor_invocation(&container, kind.accessor());
            let routes_init = application
                .injects_routes
                .then(|| quote! { routes: routes.clone(), });
            let views_init = application
                .injects_views
                .then(|| quote! { views: views.clone(), });

            quote! {
                let #local = std::sync::Arc::new(
                    super::super::authenticated_users::#wrapper {
                        inner: #inner_access,
                        #routes_init
                        #views_init
                    },
                );
            }
        }
        CapturedProviderKind::Binder { .. } => {
            let binder_access = bindings.accessor_invocation(&container, kind.accessor());

            quote! { let #local = #binder_access; }
        }
    }
}

fn argument_binding(
    argument: &BoundParameter,
    request: &Ident,
    captured: &CapturedProviders,
) -> TokenStream {
    let provider_access = captured.access(&argument.binding, &TokenStream::new());

    render_request_extraction(
        &argument.binding,
        &argument.holder,
        &ExtractionContext {
            continuation_return: &quote! {
                return ::std::result::Result::Ok(response)
            },
            error_return: &quote! {
                return ::std::result::Result::Err(
                    margaret::framework::http::handler_error::HandlerError::consumer(error),
                )
            },
            provider_access: &provider_access,
            request_local: request,
            response_return: &quote! {
                return ::std::result::Result::Ok(response.into())
            },
        },
    )
}

struct RenderedHandler {
    function: Ident,
    tokens: TokenStream,
}

fn render_handler(
    route: &HttpRoute,
    position: usize,
    has_views: bool,
    bindings: &ContainerBindings,
) -> RenderedHandler {
    let function = format_ident!("route_handler_{position}");
    let routes = if route_references_routes(route) {
        format_ident!("routes")
    } else {
        format_ident!("_routes")
    };
    let views = if route_references_views(route) {
        format_ident!("views")
    } else {
        format_ident!("_views")
    };
    let views_parameter = has_views.then(|| {
        quote! {
            #views: &::std::sync::Arc<super::super::views::Views>,
        }
    });
    let handler = onion(route, bindings);
    let too_many_lines = too_many_lines_allow();
    let tokens = quote! {
        #too_many_lines
        fn #function(
            container: &super::super::container::Container,
            #routes: &::std::sync::Arc<super::super::routes::Routes>,
            #views_parameter
        ) -> ::std::sync::Arc<dyn margaret::framework::http::handler::Handler> {
            #handler
        }
    };

    RenderedHandler { function, tokens }
}

fn server_router(
    has_websocket_routes: bool,
    server: &HttpServer,
    routes_param: &Ident,
    route_entries: &TokenStream,
) -> TokenStream {
    if has_websocket_routes {
        let websocket_routes = format_ident!("{}_routes", server.name());
        let websocket_call = quote! {
            super::super::websocket::#websocket_routes(container, #routes_param)
        };
        quote! {
            {
                let mut route_entries = #route_entries;

                route_entries.extend(#websocket_call);

                margaret::framework::http::server_routes::ServerRoutes::build(route_entries)
            }
        }
    } else {
        quote! { margaret::framework::http::server_routes::ServerRoutes::build(#route_entries) }
    }
}

fn route_entries<'handler>(
    table: &HttpRouteTable,
    server: &HttpServer,
    routes_param: &Ident,
    views_argument: Option<&TokenStream>,
    rendered_handler_entries: &mut impl Iterator<Item = &'handler RenderedHandler>,
) -> Vec<TokenStream> {
    table
        .route_groups(server.name())
        .map(|group| {
            let path = group.path().pattern();
            let method_handlers = group
                .method_routes()
                .zip(rendered_handler_entries.by_ref())
                .map(|(route, rendered_handler)| {
                    let function = &rendered_handler.function;
                    let method = &route.method;
                    let handler_call = quote! {
                        #function(container, #routes_param, #views_argument)
                    };

                    if let Some(name) = &route.name {
                        quote! {
                            margaret::framework::http::method_handler::MethodHandler::named(
                                #method,
                                #name,
                                #handler_call,
                            )
                        }
                    } else {
                        quote! {
                            margaret::framework::http::method_handler::MethodHandler::anonymous(
                                #method,
                                #handler_call,
                            )
                        }
                    }
                })
                .collect::<Vec<TokenStream>>();
            let method_handlers = vec_literal_tokens(method_handlers);

            quote! {
                margaret::framework::http::route_entry::RouteEntry::new(#path, #method_handlers)
            }
        })
        .collect()
}

fn server_module(
    table: &HttpRouteTable,
    server: &HttpServer,
    has_views: bool,
    has_websocket_routes: bool,
    bindings: &ContainerBindings,
) -> TokenStream {
    let function_name = server.function_name();
    let server_routes: Vec<&HttpRoute> = table
        .route_groups(server.name())
        .flat_map(RouteGroup::method_routes)
        .collect();
    let routes_param = format_ident!("routes");
    let views_param = if has_views && !server_routes.is_empty() {
        format_ident!("views")
    } else {
        format_ident!("_views")
    };
    let views_parameter = has_views.then(|| {
        quote! {
            #views_param: &::std::sync::Arc<super::super::views::Views>,
        }
    });
    let rendered_handlers = server_routes
        .iter()
        .enumerate()
        .map(|(position, route)| render_handler(route, position, has_views, bindings))
        .collect::<Vec<_>>();
    let handler_helpers = rendered_handlers
        .iter()
        .map(|handler| &handler.tokens)
        .collect::<Vec<_>>();
    let mut rendered_handler_entries = rendered_handlers.iter();
    let views_argument = has_views.then(|| quote! { #views_param, });
    let route_entries = route_entries(
        table,
        server,
        &routes_param,
        views_argument.as_ref(),
        &mut rendered_handler_entries,
    );
    let route_entries = vec_literal_tokens(route_entries);
    let router = server_router(has_websocket_routes, server, &routes_param, &route_entries);

    let inner_return = quote! {
        ::std::result::Result<
            margaret::framework::http::server_routes::ServerRoutes,
            margaret::framework::http::matchit::InsertError,
        >
    };
    let body = quote! { #router };
    let too_many_lines = too_many_lines_allow();
    quote! {
        #(#handler_helpers)*

        #too_many_lines
        pub(crate) fn #function_name(
            container: &super::super::container::Container,
            #routes_param: &::std::sync::Arc<super::super::routes::Routes>,
            #views_parameter
        ) -> #inner_return {
            #body
        }
    }
}

pub(crate) fn render(
    table: &HttpRouteTable,
    servers: &[HttpServer],
    has_views: bool,
    websocket_servers: &BTreeSet<String>,
    bindings: &ContainerBindings,
) -> Vec<GeneratedModuleTokens> {
    let server_declarations = servers.iter().map(|server| {
        let function_name = server.function_name();

        quote! {
            #[rustfmt::skip]
            pub mod #function_name;
        }
    });

    let http_tokens = quote! {
        #(#server_declarations)*
    };

    let mut modules = vec![GeneratedModuleTokens::new("http", http_tokens)];
    for server in servers {
        let has_websocket_routes = websocket_servers.contains(server.name());
        modules.push(GeneratedModuleTokens::new(
            format!("http/{}", server.function_name()),
            server_module(table, server, has_views, has_websocket_routes, bindings),
        ));
    }

    modules
}
