use std::collections::BTreeMap;

use heck::ToUpperCamelCase;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_codegen_tokens::too_many_arguments_expect::too_many_arguments_expect;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::container_bindings::ContainerBindings;
use margaret_container::injected_dependency::InjectedDependency;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_request_binding_codegen::binding_reads_request::binding_reads_request;
use margaret_request_binding_codegen::extraction_context::ExtractionContext;
use margaret_request_binding_codegen::render_request_extraction::render_request_extraction;
use margaret_request_binding_codegen::request_binding::RequestBinding;

use crate::handler_binding::HandlerBinding;
use crate::session_console_arguments::dispatch_table_console_arguments;
use crate::session_console_arguments::session_console_arguments;
use crate::session_plan::SessionPlan;
use crate::websocket_session::WebSocketSession;

fn injected_field_type(dependency: &InjectedDependency) -> TokenStream {
    match dependency {
        InjectedDependency::SingleConcrete { concrete, .. } => {
            let concrete = path_tokens(concrete);

            quote! { ::std::sync::Arc<#concrete> }
        }
        InjectedDependency::SingleInterface { interface, .. } => {
            let interface = path_tokens(interface);

            quote! { ::std::sync::Arc<dyn #interface> }
        }
        InjectedDependency::Collection { trait_path, .. } => {
            let trait_path = path_tokens(trait_path);

            quote! { ::std::vec::Vec<::std::sync::Arc<dyn #trait_path>> }
        }
    }
}

fn injected_field_value(
    dependency: &InjectedDependency,
    bindings: &ContainerBindings,
) -> TokenStream {
    let accessor_arguments = bindings.injected_console_arguments(dependency);

    match dependency {
        InjectedDependency::SingleConcrete { field, .. }
        | InjectedDependency::SingleInterface { field, .. } => {
            let accessor = format_ident!("{}", field);
            let arguments = bindings.console_weaves(&accessor_arguments[0]);

            quote! { container.#accessor(#(#arguments),*).await }
        }
        InjectedDependency::Collection { member_fields, .. } => {
            let members =
                member_fields
                    .iter()
                    .zip(accessor_arguments)
                    .map(|(member, arguments)| {
                        let accessor = format_ident!("{}", member);
                        let arguments = bindings.console_weaves(&arguments);

                        quote! { container.#accessor(#(#arguments),*).await }
                    });

            quote! { ::std::vec::Vec::from([#(#members),*]) }
        }
    }
}

fn binder_fields(session: &WebSocketSession) -> BTreeMap<String, &CanonicalPath> {
    let mut fields = BTreeMap::new();

    for parameter in &session.parameters {
        if let RequestBinding::Bound {
            binder_field,
            binder_provider,
            ..
        } = &parameter.binding
        {
            fields.insert(binder_field.clone(), binder_provider);
        }
    }

    fields
}

fn factory_fields(session: &WebSocketSession) -> TokenStream {
    let holders = session.parameters.iter().filter_map(|parameter| {
        let holder = &parameter.holder;

        match &parameter.binding {
            RequestBinding::Injectable { dependency } => {
                let field_type = injected_field_type(dependency);

                Some(quote! { #holder: #field_type, })
            }
            RequestBinding::Routes => Some(quote! {
                #holder: ::std::sync::Arc<super::super::routes::Routes>,
            }),
            _ => None,
        }
    });
    let binders = binder_fields(session).into_iter().map(|(field, provider)| {
        let field = format_ident!("{field}");
        let provider = path_tokens(provider);

        quote! { #field: ::std::sync::Arc<#provider>, }
    });
    let providers = session
        .authenticated_user_providers()
        .into_iter()
        .map(|application| {
            let field = format_ident!("{}", application.field);
            let wrapper = &application.wrapper;

            quote! {
                #field: ::std::sync::Arc<super::super::authenticated_users::#wrapper>,
            }
        });

    quote! { #(#holders)* #(#binders)* #(#providers)* }
}

fn factory_initializers(session: &WebSocketSession, bindings: &ContainerBindings) -> TokenStream {
    let holders = session.parameters.iter().filter_map(|parameter| {
        let holder = &parameter.holder;

        match &parameter.binding {
            RequestBinding::Injectable { dependency } => {
                let value = injected_field_value(dependency, bindings);

                Some(quote! { #holder: #value, })
            }
            RequestBinding::Routes => Some(quote! { #holder: routes.clone(), }),
            _ => None,
        }
    });
    let binders = binder_fields(session).into_iter().map(|(field, provider)| {
        let field = format_ident!("{field}");
        let arguments = bindings.console_weaves(bindings.console_arguments(provider));

        quote! { #field: container.#field(#(#arguments),*).await, }
    });
    let providers = session
        .authenticated_user_providers()
        .into_iter()
        .map(|application| {
            let field = format_ident!("{}", application.field);
            let wrapper = &application.wrapper;
            let arguments =
                bindings.console_weaves(bindings.console_arguments(&application.concrete));
            let routes_init = application
                .injects_routes
                .then(|| quote! { routes: routes.clone(), });

            quote! {
                #field: ::std::sync::Arc::new(
                    super::super::authenticated_users::#wrapper {
                        inner: container.#field(#(#arguments),*).await,
                        #routes_init
                    },
                ),
            }
        });

    quote! { #(#holders)* #(#binders)* #(#providers)* }
}

fn create_extractions(session: &WebSocketSession, handshake: &Ident) -> TokenStream {
    let continuation_return = quote! { return ::std::result::Result::Err(response) };
    let response_return = quote! { return ::std::result::Result::Err(response.into()) };
    let provider_owner = quote! { self. };
    let extractions = session.parameters.iter().map(|parameter| {
        render_request_extraction(
            &parameter.binding,
            &parameter.holder,
            &ExtractionContext {
                continuation_return: &continuation_return,
                provider_owner: &provider_owner,
                request_local: handshake,
                response_return: &response_return,
            },
        )
    });

    quote! { #(#extractions)* }
}

fn build_arguments(session: &WebSocketSession) -> TokenStream {
    let arguments = session.parameters.iter().map(|parameter| {
        let holder = &parameter.holder;

        match &parameter.binding {
            RequestBinding::Injectable { .. } => quote! { self.#holder.clone() },
            RequestBinding::Routes => quote! { self.#holder.as_ref() },
            _ => quote! { #holder },
        }
    });

    quote! { #(#arguments),* }
}

fn render_factory(session: &WebSocketSession) -> TokenStream {
    let session_path = path_tokens(&session.session_path);
    let fields = factory_fields(session);
    let handshake = if session
        .parameters
        .iter()
        .any(|parameter| binding_reads_request(&parameter.binding))
    {
        format_ident!("handshake")
    } else {
        format_ident!("_handshake")
    };
    let extractions = create_extractions(session, &handshake);
    let arguments = build_arguments(session);
    let method_name = &session.method_name;

    quote! {
        struct Factory {
            #fields
        }

        #[async_trait::async_trait]
        impl margaret_websocket::web_socket_session_factory::WebSocketSessionFactory for Factory {
            type Session = #session_path;

            async fn create(
                &self,
                #handshake: &margaret_http::request::Request,
            ) -> ::std::result::Result<
                ::std::sync::Arc<Self::Session>,
                margaret_http::response_continuation::ResponseContinuation,
            > {
                #extractions

                ::std::result::Result::Ok(::std::sync::Arc::new(
                    #session_path::#method_name(#arguments),
                ))
            }
        }
    }
}

fn dispatch_struct_ident(binding: &HandlerBinding) -> Ident {
    format_ident!("{}Dispatch", binding.method.to_upper_camel_case())
}

fn render_request_dispatch(binding: &HandlerBinding, session_path: &TokenStream) -> TokenStream {
    let dispatch = dispatch_struct_ident(binding);
    let handler = path_tokens(&binding.handler_path);

    quote! {
        struct #dispatch {
            handler: ::std::sync::Arc<#handler>,
        }

        #[async_trait::async_trait]
        impl margaret_websocket::web_socket_message_dispatch::WebSocketMessageDispatch<#session_path>
            for #dispatch
        {
            async fn dispatch(
                &self,
                cancellation_token: tokio_util::sync::CancellationToken,
                session: ::std::sync::Arc<#session_path>,
                id: margaret_websocket::request_id::RequestId,
                params: serde_json::Value,
                socket: margaret_websocket::web_socket::WebSocket,
            ) {
                margaret_websocket::dispatch_request::dispatch_request(
                    &*self.handler,
                    cancellation_token,
                    session,
                    id,
                    params,
                    socket,
                )
                .await;
            }
        }
    }
}

fn render_notification_dispatch(
    binding: &HandlerBinding,
    session_path: &TokenStream,
) -> TokenStream {
    let dispatch = dispatch_struct_ident(binding);
    let handler = path_tokens(&binding.handler_path);

    quote! {
        struct #dispatch {
            handler: ::std::sync::Arc<#handler>,
        }

        #[async_trait::async_trait]
        impl margaret_websocket::web_socket_notification_dispatch::WebSocketNotificationDispatch<#session_path>
            for #dispatch
        {
            async fn dispatch(
                &self,
                cancellation_token: tokio_util::sync::CancellationToken,
                session: ::std::sync::Arc<#session_path>,
                params: serde_json::Value,
                socket: margaret_websocket::web_socket::WebSocket,
            ) {
                margaret_websocket::dispatch_notification::dispatch_notification(
                    &*self.handler,
                    cancellation_token,
                    session,
                    params,
                    socket,
                )
                .await;
            }
        }
    }
}

fn dispatch_insert(
    binding: &HandlerBinding,
    map: &Ident,
    container: &Ident,
    bindings: &ContainerBindings,
) -> TokenStream {
    let dispatch = dispatch_struct_ident(binding);
    let method = &binding.method;
    let accessor = format_ident!("{}", binding.handler_path.field_name());
    let arguments = bindings.console_weaves(bindings.console_arguments(&binding.handler_path));

    quote! {
        #map.insert(
            #method.to_string(),
            ::std::sync::Arc::new(#dispatch {
                handler: #container.#accessor(#(#arguments),*).await,
            }),
        );
    }
}

fn mutability(has_inserts: bool) -> TokenStream {
    if has_inserts {
        quote! { mut }
    } else {
        TokenStream::new()
    }
}

fn render_dispatch_table(
    plan: &SessionPlan,
    session_path: &TokenStream,
    dispatch_arguments: &[ConsoleArgument],
    bindings: &ContainerBindings,
) -> TokenStream {
    let requests = format_ident!("requests");
    let notifications = format_ident!("notifications");
    let container = if plan.request_handlers.is_empty() && plan.notification_handlers.is_empty() {
        format_ident!("_container")
    } else {
        format_ident!("container")
    };
    let parameters = bindings.console_parameters(dispatch_arguments);
    let request_inserts = plan
        .request_handlers
        .iter()
        .map(|binding| dispatch_insert(binding, &requests, &container, bindings));
    let notification_inserts = plan
        .notification_handlers
        .iter()
        .map(|binding| dispatch_insert(binding, &notifications, &container, bindings));

    let requests_mutability = mutability(!plan.request_handlers.is_empty());
    let notifications_mutability = mutability(!plan.notification_handlers.is_empty());
    let parameter_count = 1 + parameters.len();
    let too_many_arguments = too_many_arguments_expect(parameter_count);

    quote! {
        #too_many_arguments
        async fn dispatch_table(
            #container: &super::super::container::Container,
            #(#parameters)*
        ) -> ::std::sync::Arc<
            margaret_websocket::web_socket_dispatch_table::WebSocketDispatchTable<#session_path>,
        > {
            let #requests_mutability #requests: ::std::collections::HashMap<
                ::std::string::String,
                ::std::sync::Arc<
                    dyn margaret_websocket::web_socket_message_dispatch::WebSocketMessageDispatch<
                        #session_path,
                    >,
                >,
            > = ::std::collections::HashMap::new();
            #(#request_inserts)*

            let #notifications_mutability #notifications: ::std::collections::HashMap<
                ::std::string::String,
                ::std::sync::Arc<
                    dyn margaret_websocket::web_socket_notification_dispatch::WebSocketNotificationDispatch<
                        #session_path,
                    >,
                >,
            > = ::std::collections::HashMap::new();
            #(#notification_inserts)*

            ::std::sync::Arc::new(
                margaret_websocket::web_socket_dispatch_table::WebSocketDispatchTable::new(
                    #requests,
                    #notifications,
                ),
            )
        }
    }
}

fn render_session(plan: &SessionPlan, bindings: &ContainerBindings) -> TokenStream {
    let session_path = path_tokens(&plan.session.session_path);
    let factory = render_factory(&plan.session);
    let initializers = factory_initializers(&plan.session, bindings);
    let session_arguments = session_console_arguments(plan, bindings);
    let dispatch_arguments = dispatch_table_console_arguments(plan, bindings);
    let upgrade_parameters = bindings.console_parameters(&session_arguments);
    let dispatch_forward = bindings.console_forwards(&dispatch_arguments);
    let routes_parameter = plan.session.injects_routes().then(|| {
        quote! { routes: &::std::sync::Arc<super::super::routes::Routes>, }
    });
    let request_dispatches = plan
        .request_handlers
        .iter()
        .map(|binding| render_request_dispatch(binding, &session_path));
    let notification_dispatches = plan
        .notification_handlers
        .iter()
        .map(|binding| render_notification_dispatch(binding, &session_path));
    let dispatch_table = render_dispatch_table(plan, &session_path, &dispatch_arguments, bindings);
    let parameter_count = 1 + upgrade_parameters.len() + usize::from(routes_parameter.is_some());
    let too_many_arguments = too_many_arguments_expect(parameter_count);

    quote! {
        #factory

        #(#request_dispatches)*
        #(#notification_dispatches)*

        #dispatch_table

        #too_many_arguments
        pub async fn upgrade_entry(
            container: &super::super::container::Container,
            #(#upgrade_parameters)*
            #routes_parameter
        ) -> ::std::sync::Arc<dyn margaret_http::web_socket_upgrade::WebSocketUpgrade> {
            ::std::sync::Arc::new(
                margaret_websocket::web_socket_upgrade_entry::WebSocketUpgradeEntry::new(
                    Factory {
                        #initializers
                    },
                    dispatch_table(container, #(#dispatch_forward)*).await,
                ),
            )
        }
    }
}

pub(crate) fn render_sessions(
    sessions: &[SessionPlan],
    bindings: &ContainerBindings,
) -> Vec<GeneratedModuleTokens> {
    sessions
        .iter()
        .map(|plan| {
            GeneratedModuleTokens::new(
                format!("websocket/{}", plan.session.module_name),
                render_session(plan, bindings),
            )
        })
        .collect()
}
