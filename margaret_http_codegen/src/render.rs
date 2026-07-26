use std::collections::BTreeMap;

use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::name_allocator::NameAllocator;
use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_codegen_tokens::too_many_arguments_expect::too_many_arguments_expect;
use margaret_codegen_tokens::vec_literal_tokens::vec_literal_tokens;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::accessor_failure::AccessorFailure;
use margaret_container::construction_error_path::construction_error_path;
use margaret_container::container_bindings::ContainerBindings;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_middleware_codegen::fold_layers::fold_layers;
use margaret_request_binding_codegen::authenticated_user_application::AuthenticatedUserApplication;
use margaret_request_binding_codegen::binding_console_arguments::binding_console_arguments;
use margaret_request_binding_codegen::binding_reads_request::binding_reads_request;
use margaret_request_binding_codegen::binding_shadows_request::binding_shadows_request;
use margaret_request_binding_codegen::bound_parameter::BoundParameter;
use margaret_request_binding_codegen::captured_provider::CapturedProvider;
use margaret_request_binding_codegen::captured_provider_kind::CapturedProviderKind;
use margaret_request_binding_codegen::captured_providers::CapturedProviders;
use margaret_request_binding_codegen::extraction_context::ExtractionContext;
use margaret_request_binding_codegen::injects_routes::injects_routes;
use margaret_request_binding_codegen::injects_views::injects_views;
use margaret_request_binding_codegen::render_request_extraction::render_request_extraction;
use margaret_request_binding_codegen::request_binding::RequestBinding;

use crate::http_route::HttpRoute;
use crate::http_route_table::HttpRouteTable;
use crate::http_server::HttpServer;
use crate::route_group::RouteGroup;

fn console_weave(path: &CanonicalPath, bindings: &ContainerBindings) -> Vec<TokenStream> {
    bindings.console_weaves(bindings.console_arguments(path))
}

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

fn handler_binding(route: &HttpRoute) -> Ident {
    format_ident!("{}_handler", route.responder_field)
}

fn onion(route: &HttpRoute, bindings: &ContainerBindings) -> TokenStream {
    let responder_type = path_tokens(&route.responder_path);
    let responder_woven = console_weave(&route.responder_path, bindings);
    let responder_access = bindings.accessor_invocation(
        &format_ident!("container"),
        &route.responder_field.to_string(),
        &responder_woven,
        &AccessorFailure::Propagate,
    );
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

    let bindings_tokens = route
        .arguments
        .iter()
        .map(|argument| argument_binding(argument, &request_binding, &captured));
    let server = format_ident!("{}", route.server);
    let argument_values = route
        .arguments
        .iter()
        .map(|argument| argument_value(argument, &routes_local, &views_local, &server));
    let method_name = &route.method_name;
    let respond_call = quote! { #responder_binding.#method_name(#(#argument_values),*).await };
    let body = quote! {
        #(#bindings_tokens)*
        margaret::framework::http::response_continuation::ResponseContinuation::from(#respond_call)
    };

    let captures_routes = responder_injects_routes(route);
    let captures_views = responder_injects_views(route);
    let outcome_future = quote! {
        std::pin::Pin<
            std::boxed::Box<
                dyn std::future::Future<
                    Output = margaret::framework::http::response_continuation::ResponseContinuation,
                > + Send
                + '_,
            >,
        >
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
            let woven = console_weave(&application.concrete, bindings);
            let inner_access = bindings.accessor_invocation(
                &container,
                kind.accessor(),
                &woven,
                &AccessorFailure::Propagate,
            );
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
        CapturedProviderKind::Binder { provider, .. } => {
            let woven = console_weave(provider, bindings);
            let binder_access = bindings.accessor_invocation(
                &container,
                kind.accessor(),
                &woven,
                &AccessorFailure::Propagate,
            );

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
            continuation_return: &quote! { return response },
            provider_access: &provider_access,
            request_local: request,
            response_return: &quote! { return response.into() },
        },
    )
}

fn server_routes<'table>(table: &'table HttpRouteTable, server: &str) -> Vec<&'table HttpRoute> {
    table
        .route_groups(server)
        .flat_map(RouteGroup::method_routes)
        .collect()
}

fn responder_and_binder_arguments(
    routes: &[&HttpRoute],
    bindings: &ContainerBindings,
) -> Vec<ConsoleArgument> {
    let mut collected: Vec<ConsoleArgument> = Vec::new();

    for route in routes {
        collected.extend_from_slice(bindings.console_arguments(&route.responder_path));

        for argument in &route.arguments {
            collected.extend(binding_console_arguments(&argument.binding, bindings));
        }

        for layer in &route.layers {
            collected.extend_from_slice(bindings.console_arguments(&layer.concrete));
        }
    }

    collected
}

pub(crate) fn server_console_arguments(
    table: &HttpRouteTable,
    servers: &[HttpServer],
    bindings: &ContainerBindings,
    websocket_server_arguments: &BTreeMap<String, Vec<ConsoleArgument>>,
) -> BTreeMap<String, Vec<ConsoleArgument>> {
    servers
        .iter()
        .map(|server| {
            let routes = server_routes(table, server.name());
            let mut collected = responder_and_binder_arguments(&routes, bindings);

            if let Some(websocket_arguments) = websocket_server_arguments.get(server.name()) {
                collected.extend_from_slice(websocket_arguments);
            }

            (
                server.name().to_string(),
                bindings.console_union(&collected),
            )
        })
        .collect()
}

fn server_module(
    table: &HttpRouteTable,
    server: &HttpServer,
    has_views: bool,
    has_websocket_routes: bool,
    bindings: &ContainerBindings,
    server_arguments: &[ConsoleArgument],
    websocket_arguments: &[ConsoleArgument],
) -> TokenStream {
    let function_name = server.function_name();
    let server_routes: Vec<&HttpRoute> = table
        .route_groups(server.name())
        .flat_map(RouteGroup::method_routes)
        .collect();
    let routes_param =
        if has_websocket_routes || server_routes.iter().copied().any(route_references_routes) {
            format_ident!("routes")
        } else {
            format_ident!("_routes")
        };
    let views_param = if server_routes.iter().copied().any(route_references_views) {
        format_ident!("views")
    } else {
        format_ident!("_views")
    };
    let views_parameter = has_views.then(|| {
        quote! {
            #views_param: &::std::sync::Arc<super::super::views::Views>,
        }
    });
    let console_parameters = bindings.console_parameters(server_arguments);

    let handler_bindings = server_routes
        .iter()
        .filter(|route| route.name.is_some())
        .map(|route| {
            let binding = handler_binding(route);
            let handler = onion(route, bindings);

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
                onion(route, bindings)
            };

            quote! {
                margaret::framework::http::method_handler::MethodHandler::new(#method, #handler)
            }
        });
        let method_handlers = vec_literal_tokens(method_handlers);

        quote! {
            margaret::framework::http::route_entry::RouteEntry::new(#path, #method_handlers)
        }
    });
    let route_entries = vec_literal_tokens(route_entries);

    let named_handlers = server_routes.iter().filter_map(|route| {
        route.name.as_ref().map(|name| {
            let binding = handler_binding(route);

            quote! { margaret::framework::http::named_handler::NamedHandler::new(#name, #binding) }
        })
    });
    let named_handlers = vec_literal_tokens(named_handlers);
    let has_fallible = bindings.has_fallible_accessors();
    let router = if has_websocket_routes {
        let websocket_routes = format_ident!("{}_routes", server.name());
        let websocket_forward = bindings.console_forwards(websocket_arguments);
        let websocket_call = quote! {
            super::super::websocket::#websocket_routes(container, #(#websocket_forward)* #routes_param).await
        };
        let websocket_extend = if has_fallible {
            quote! { route_entries.extend(#websocket_call?); }
        } else {
            quote! { route_entries.extend(#websocket_call); }
        };

        quote! {
            {
                let mut route_entries = #route_entries;

                #websocket_extend

                margaret::framework::http::router::Router::build(route_entries)
            }
        }
    } else {
        quote! { margaret::framework::http::router::Router::build(#route_entries) }
    };

    let parameter_count = 2 + console_parameters.len() + usize::from(views_parameter.is_some());
    let too_many_arguments = too_many_arguments_expect(parameter_count);
    let inner_return = quote! {
        ::std::result::Result<
            margaret::framework::http::server_routes::ServerRoutes,
            margaret::framework::http::matchit::InsertError,
        >
    };
    let body_value = quote! {
        #router.map(|router| {
            margaret::framework::http::server_routes::ServerRoutes::new(router, #named_handlers)
        })
    };
    let (return_type, body) = if has_fallible {
        let error = construction_error_path();

        (
            quote! { ::std::result::Result<#inner_return, #error> },
            quote! { Ok(#body_value) },
        )
    } else {
        (inner_return, body_value)
    };

    quote! {
        #too_many_arguments
        pub async fn #function_name(
            container: &super::super::container::Container,
            #(#console_parameters)*
            #routes_param: &::std::sync::Arc<super::super::routes::Routes>,
            #views_parameter
        ) -> #return_type {
            #(#handler_bindings)*

            #body
        }
    }
}

pub(crate) fn render(
    table: &HttpRouteTable,
    servers: &[HttpServer],
    has_views: bool,
    websocket_servers: &[String],
    bindings: &ContainerBindings,
    server_arguments: &BTreeMap<String, Vec<ConsoleArgument>>,
    websocket_server_arguments: &BTreeMap<String, Vec<ConsoleArgument>>,
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
    let empty: Vec<ConsoleArgument> = Vec::new();

    for server in servers {
        let has_websocket_routes = websocket_servers
            .iter()
            .any(|websocket_server| websocket_server == server.name());
        let server_union = server_arguments.get(server.name()).unwrap_or(&empty);
        let websocket_union = websocket_server_arguments
            .get(server.name())
            .unwrap_or(&empty);

        modules.push(GeneratedModuleTokens::new(
            format!("http/{}", server.function_name()),
            server_module(
                table,
                server,
                has_views,
                has_websocket_routes,
                bindings,
                server_union,
                websocket_union,
            ),
        ));
    }

    modules
}
