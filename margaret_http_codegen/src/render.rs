use std::collections::BTreeSet;

use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::name_allocator::NameAllocator;
use margaret_attributes::path_tokens::path_tokens;
use margaret_generated_module::generated_module::GeneratedModule;

use crate::form_request_extraction::FormRequestExtraction;
use crate::http_route::HttpRoute;
use crate::http_server::HttpServer;
use crate::interceptor_argument::InterceptorArgument;
use crate::interceptor_plan::InterceptorPlan;
use crate::middleware_argument::MiddlewareArgument;
use crate::middleware_plan::MiddlewarePlan;
use crate::responder_argument::ResponderArgument;
use crate::responder_argument_binding::ResponderArgumentBinding;
use crate::responder_output::ResponderOutput;

fn access(call: TokenStream) -> TokenStream {
    quote! { #call.await }
}

fn argument_needs_request(argument: &ResponderArgument) -> bool {
    !matches!(argument.binding, ResponderArgumentBinding::Routes)
}

fn holder_shadows_request(binding: &ResponderArgumentBinding) -> bool {
    matches!(
        binding,
        ResponderArgumentBinding::Raw { .. }
            | ResponderArgumentBinding::Bound { .. }
            | ResponderArgumentBinding::FormRequest { .. }
    )
}

fn responder_injects_routes(route: &HttpRoute) -> bool {
    route
        .arguments
        .iter()
        .any(|argument| matches!(argument.binding, ResponderArgumentBinding::Routes))
}

fn interceptor_injects_routes(route: &HttpRoute) -> bool {
    matches!(
        route.responder_output,
        ResponderOutput::Intercepted {
            injects_routes: true,
            ..
        }
    )
}

fn closure_captures_routes(route: &HttpRoute) -> bool {
    responder_injects_routes(route) || interceptor_injects_routes(route)
}

fn route_references_routes(route: &HttpRoute) -> bool {
    closure_captures_routes(route) || route.layers.iter().any(|layer| layer.injects_routes)
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
        if holder_shadows_request(&argument.binding) {
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
    let argument_values = route
        .arguments
        .iter()
        .map(|argument| argument_value(argument, &routes_local));
    let respond_call = quote! { #responder_binding.respond(#(#argument_values),*).await };
    let body_tail = match &route.responder_output {
        ResponderOutput::Plain => quote! {
            margaret_http::response_continuation::ResponseContinuation::from(#respond_call)
        },
        ResponderOutput::Intercepted {
            interceptor,
            injects_routes,
        } => {
            let wrapper = format_ident!("{}", index.type_name(interceptor));
            let field = format_ident!("{}", index.field_name(interceptor));
            let interceptor_expr = if *injects_routes {
                quote! { std::sync::Arc::new(super::#wrapper { inner: #field, routes: #routes_local.clone() }) }
            } else {
                quote! { std::sync::Arc::new(super::#wrapper { inner: #field }) }
            };

            quote! {
                margaret_http::response_continuation::ResponseContinuation::Intercept(Box::new(
                    margaret_http::interception::Interception::new(#interceptor_expr, #respond_call),
                ))
            }
        }
    };
    let body = quote! {
        #(#bindings)*
        #body_tail
    };

    let captures_routes = closure_captures_routes(route);
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

fn argument_value(argument: &ResponderArgument, routes_binding: &Ident) -> TokenStream {
    match &argument.binding {
        ResponderArgumentBinding::Routes => quote! { #routes_binding.as_ref() },
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
        ResponderArgumentBinding::Raw { path_key } => {
            quote! {
                let #holder = match #request.path_param(#path_key) {
                    Some(value) => value.to_string(),
                    None => return margaret_http::response::Response::not_found().into(),
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
        ResponderArgumentBinding::Routes => TokenStream::new(),
        ResponderArgumentBinding::Bound { binder, path_key } => {
            let binder_field = format_ident!("{}", index.field_name(binder));

            quote! {
                let #holder = match #request.path_param(#path_key) {
                    Some(value) => match #binder_field.bind(value.to_string()).await {
                        Some(value) => value,
                        None => return margaret_http::response::Response::not_found().into(),
                    },
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

fn capture_fields(route: &HttpRoute, index: &AttributeIndex) -> Vec<Ident> {
    let mut fields: BTreeSet<String> = BTreeSet::new();

    for argument in &route.arguments {
        if let ResponderArgumentBinding::Bound { binder, .. } = &argument.binding {
            fields.insert(index.field_name(binder).to_string());
        }
    }

    if let ResponderOutput::Intercepted { interceptor, .. } = &route.responder_output {
        fields.insert(index.field_name(interceptor).to_string());
    }

    fields
        .iter()
        .map(|field| format_ident!("{}", field))
        .collect()
}

fn interceptor_wrapper(plan: &InterceptorPlan, index: &AttributeIndex) -> TokenStream {
    let InterceptorPlan {
        arguments,
        injects_routes,
        interceptor,
        marker,
    } = plan;
    let wrapper = format_ident!("{}", index.type_name(interceptor));
    let concrete = path_tokens(interceptor);
    let marker = path_tokens(marker);
    let routes_field =
        injects_routes.then(|| quote! { routes: std::sync::Arc<super::routes::Routes>, });
    let request_binding = if arguments
        .iter()
        .any(|argument| matches!(argument, InterceptorArgument::CurrentRequest))
    {
        format_ident!("request")
    } else {
        format_ident!("_request")
    };
    let call_arguments = arguments.iter().map(|argument| match argument {
        InterceptorArgument::CurrentRequest => quote! { request },
        InterceptorArgument::Intercepted => quote! { intercepted },
        InterceptorArgument::Routes => quote! { &self.routes },
    });

    quote! {
        struct #wrapper {
            inner: std::sync::Arc<#concrete>,
            #routes_field
        }

        #[async_trait::async_trait]
        impl margaret_http::http_interceptor::HttpInterceptor for #wrapper {
            type Intercepted = dyn #marker;

            async fn intercept(
                &self,
                #request_binding: &margaret_http::request::Request,
                intercepted: std::boxed::Box<dyn #marker>,
            ) -> margaret_http::response_continuation::ResponseContinuation {
                margaret_http::response_continuation::ResponseContinuation::from(
                    self.inner.process(#(#call_arguments),*).await,
                )
            }
        }
    }
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
    let request_binding = if arguments
        .iter()
        .any(|argument| matches!(argument, MiddlewareArgument::CurrentRequest))
    {
        format_ident!("request")
    } else {
        format_ident!("_request")
    };
    let next_binding = if arguments
        .iter()
        .any(|argument| matches!(argument, MiddlewareArgument::Next))
    {
        format_ident!("next")
    } else {
        format_ident!("_next")
    };
    let call_arguments = arguments.iter().map(|argument| match argument {
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
                self.inner.process(#(#call_arguments),*).await
            }
        }
    }
}

fn server_module(routes: &[HttpRoute], server: &HttpServer, index: &AttributeIndex) -> TokenStream {
    let function_name = server.function_name();
    let server_routes: Vec<&HttpRoute> = routes
        .iter()
        .filter(|route| route.server == server.name())
        .collect();
    let binder_import = if server_routes.iter().copied().any(HttpRoute::is_bound) {
        quote! { use margaret_http::http_route_parameter_binder::HttpRouteParameterBinder; }
    } else {
        quote! {}
    };
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

    let route_calls = server_routes.iter().map(|route| {
        let method = &route.method;
        let path = &route.path;
        let handler = if route.name.is_some() {
            let binding = handler_binding(route);

            quote! { #binding.clone() }
        } else {
            onion(route, index)
        };

        quote! {
            .route(margaret_http::method::Method::#method, #path, #handler)
        }
    });

    let named_handlers = server_routes.iter().filter_map(|route| {
        route.name.as_ref().map(|name| {
            let binding = handler_binding(route);

            quote! { margaret_http::named_handler::NamedHandler::new(#name, #binding) }
        })
    });

    quote! {
        #binder_import

        pub async fn #function_name(
            container: &super::super::container::Container,
            #routes_param: &::std::sync::Arc<super::super::routes::Routes>,
        ) -> margaret_http::server_routes::ServerRoutes {
            #(#handler_bindings)*

            let router = margaret_http::router_builder::RouterBuilder::empty()
                #(#route_calls)*
                .build();

            margaret_http::server_routes::ServerRoutes::new(router, ::std::vec![#(#named_handlers),*])
        }
    }
}

pub(crate) fn unparse(tokens: TokenStream) -> String {
    let file =
        syn::parse2::<syn::File>(tokens).expect("the generated tokens form a valid Rust file");

    prettyplease::unparse(&file)
}

pub(crate) fn render(
    routes: &[HttpRoute],
    servers: &[HttpServer],
    interceptor_plans: &[InterceptorPlan],
    middleware_plans: &[MiddlewarePlan],
    index: &AttributeIndex,
) -> Vec<GeneratedModule> {
    let interceptor_wrappers = interceptor_plans
        .iter()
        .map(|plan| interceptor_wrapper(plan, index));
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
        #(#interceptor_wrappers)*
        #(#middleware_wrappers)*
    };

    let mut modules = vec![GeneratedModule::new("http", unparse(http_tokens))];

    for server in servers {
        modules.push(GeneratedModule::new(
            format!("http/{}", server.function_name()),
            unparse(server_module(routes, server, index)),
        ));
    }

    modules
}
