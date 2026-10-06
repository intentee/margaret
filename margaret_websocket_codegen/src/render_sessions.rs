use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::name_allocator::NameAllocator;
use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_codegen_tokens::too_many_lines_allow::too_many_lines_allow;
use margaret_container::container_bindings::ContainerBindings;
use margaret_container::injected_dependency::InjectedDependency;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_request_binding_codegen::binding_reads_request::binding_reads_request;
use margaret_request_binding_codegen::captured_provider::CapturedProvider;
use margaret_request_binding_codegen::captured_provider_kind::CapturedProviderKind;
use margaret_request_binding_codegen::captured_providers::CapturedProviders;
use margaret_request_binding_codegen::head_extraction_context::HeadExtractionContext;
use margaret_request_binding_codegen::render_authenticated_user_wrapper_construction::render_authenticated_user_wrapper_construction;
use margaret_request_binding_codegen::render_head_extractions::render_head_extractions;
use margaret_request_binding_codegen::request_binding::RequestBinding;

use crate::handler_binding::HandlerBinding;
use crate::session_plan::SessionPlan;
use crate::web_socket_session::WebSocketSession;

fn injected_field_type(dependency: &InjectedDependency) -> TokenStream {
    let concrete = path_tokens(&dependency.concrete);

    quote! { ::std::sync::Arc<#concrete> }
}

fn injected_field_value(
    dependency: &InjectedDependency,
    bindings: &ContainerBindings,
) -> TokenStream {
    bindings.accessor_invocation(&format_ident!("container"), &dependency.field)
}

fn captured_providers(session: &WebSocketSession) -> CapturedProviders {
    let mut allocator = NameAllocator::new();

    for parameter in &session.parameters {
        if matches!(
            parameter.binding,
            RequestBinding::Injectable { .. } | RequestBinding::Routes
        ) {
            allocator.reserve(&parameter.holder.to_string());
        }
    }

    CapturedProviders::capture(&session.parameters, &mut allocator)
}

fn factory_fields(session: &WebSocketSession, captured: &CapturedProviders) -> TokenStream {
    let holders = session.parameters.iter().filter_map(|parameter| {
        let holder = &parameter.holder;

        match &parameter.binding {
            RequestBinding::Injectable { dependency } => {
                let field_type = injected_field_type(dependency);

                Some(quote! { #holder: #field_type, })
            }
            RequestBinding::Routes => Some(quote! {
                #holder: ::std::sync::Arc<super::super::super::routes::Routes>,
            }),
            _ => None,
        }
    });
    let fields = captured
        .entries()
        .map(|CapturedProvider { kind, local }| match kind {
            CapturedProviderKind::AuthenticatedUser { application } => {
                let wrapper = &application.wrapper;

                quote! {
                    #local: ::std::sync::Arc<super::super::super::authenticated_users::#wrapper>,
                }
            }
            CapturedProviderKind::Binder { provider, .. } => {
                let provider = path_tokens(provider);

                quote! { #local: ::std::sync::Arc<#provider>, }
            }
        });

    quote! { #(#holders)* #(#fields)* }
}

fn factory_initializers(
    session: &WebSocketSession,
    bindings: &ContainerBindings,
    captured: &CapturedProviders,
) -> TokenStream {
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
    let container = format_ident!("container");
    let initializers = captured
        .entries()
        .map(|CapturedProvider { kind, local }| match kind {
            CapturedProviderKind::AuthenticatedUser { application } => {
                let construction = render_authenticated_user_wrapper_construction(
                    application,
                    &quote! { super::super::super::authenticated_users },
                    &container,
                    bindings,
                );

                quote! { #local: #construction, }
            }
            CapturedProviderKind::Binder { .. } => {
                let access = bindings.accessor_invocation(&container, kind.accessor());

                quote! { #local: #access, }
            }
        });

    quote! { #(#holders)* #(#initializers)* }
}

fn create_extractions(
    session: &WebSocketSession,
    handshake: &Ident,
    captured: &CapturedProviders,
) -> TokenStream {
    let continuation_return = quote! {
        return ::std::result::Result::Ok(
            margaret::framework::websocket_session::web_socket_session_creation_outcome::WebSocketSessionCreationOutcome::Interrupted(
                response,
            ),
        )
    };
    let error_return = quote! {
        return ::std::result::Result::Err(
            margaret::framework::websocket_session::web_socket_session_creation_error::WebSocketSessionCreationError::consumer(
                error,
            ),
        )
    };

    render_head_extractions(
        &session.parameters,
        captured,
        &HeadExtractionContext {
            continuation_return: &continuation_return,
            error_return: &error_return,
            owner: &quote! { self. },
            request_local: handshake,
        },
    )
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

fn render_factory(session: &WebSocketSession, captured: &CapturedProviders) -> TokenStream {
    let session_path = path_tokens(&session.session_path);
    let fields = factory_fields(session, captured);
    let handshake = if session
        .parameters
        .iter()
        .any(|parameter| binding_reads_request(&parameter.binding))
    {
        format_ident!("handshake")
    } else {
        format_ident!("_handshake")
    };
    let extractions = create_extractions(session, &handshake, captured);
    let arguments = build_arguments(session);
    let method_name = &session.method_name;
    let create_too_many_lines = too_many_lines_allow();

    quote! {
        struct Factory {
            #fields
        }

        #[async_trait::async_trait]
        impl margaret::framework::websocket_session::web_socket_session_factory::WebSocketSessionFactory for Factory {
            type Session = #session_path;

            #create_too_many_lines
            async fn create(
                &self,
                #handshake: &margaret::framework::http::request::Request,
            ) -> ::std::result::Result<
                margaret::framework::websocket_session::web_socket_session_creation_outcome::WebSocketSessionCreationOutcome<Self::Session>,
                margaret::framework::websocket_session::web_socket_session_creation_error::WebSocketSessionCreationError,
            > {
                #extractions

                #session_path::#method_name(#arguments)
                    .map(::std::sync::Arc::new)
                    .map(margaret::framework::websocket_session::web_socket_session_creation_outcome::WebSocketSessionCreationOutcome::Created)
                    .map_err(
                        margaret::framework::websocket_session::web_socket_session_creation_error::WebSocketSessionCreationError::consumer,
                    )
            }
        }
    }
}

fn dispatch_struct_ident(binding: &HandlerBinding) -> Ident {
    format_ident!("{}", binding.dispatch_ident)
}

fn render_request_dispatch(binding: &HandlerBinding, session_path: &TokenStream) -> TokenStream {
    let dispatch = dispatch_struct_ident(binding);
    let handler = path_tokens(&binding.handler_path);

    quote! {
        struct #dispatch {
            handler: ::std::sync::Arc<#handler>,
        }

        #[async_trait::async_trait]
        impl margaret::framework::websocket::web_socket_message_dispatch::WebSocketMessageDispatch<#session_path>
            for #dispatch
        {
            async fn dispatch(
                &self,
                cancellation_token: tokio_util::sync::CancellationToken,
                session: ::std::sync::Arc<#session_path>,
                id: margaret::framework::websocket::request_id::RequestId,
                params: serde_json::Value,
                socket: margaret::framework::websocket::web_socket::WebSocket,
            ) {
                margaret::framework::websocket::dispatch_request::dispatch_request(
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
        impl margaret::framework::websocket::web_socket_notification_dispatch::WebSocketNotificationDispatch<#session_path>
            for #dispatch
        {
            async fn dispatch(
                &self,
                cancellation_token: tokio_util::sync::CancellationToken,
                session: ::std::sync::Arc<#session_path>,
                params: serde_json::Value,
                socket: margaret::framework::websocket::web_socket::WebSocket,
            ) {
                margaret::framework::websocket::dispatch_notification::dispatch_notification(
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
    let handler = bindings.accessor_invocation(container, &binding.handler_field);

    quote! {
        #map.insert(
            #method.to_string(),
            ::std::sync::Arc::new(#dispatch {
                handler: #handler,
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
    bindings: &ContainerBindings,
) -> TokenStream {
    let requests = format_ident!("requests");
    let notifications = format_ident!("notifications");
    let container = if plan.request_handlers.is_empty() && plan.notification_handlers.is_empty() {
        format_ident!("_container")
    } else {
        format_ident!("container")
    };
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
    let dispatch_too_many_lines = too_many_lines_allow();
    let table_type = quote! {
        ::std::sync::Arc<
            margaret::framework::websocket::web_socket_dispatch_table::WebSocketDispatchTable<#session_path>,
        >
    };
    let table_value = quote! {
        ::std::sync::Arc::new(
            margaret::framework::websocket::web_socket_dispatch_table::WebSocketDispatchTable::new(
                #requests,
                #notifications,
            ),
        )
    };
    quote! {
        #dispatch_too_many_lines
        fn dispatch_table(
            #container: &super::super::super::container::Container,
        ) -> #table_type {
            let #requests_mutability #requests: ::std::collections::HashMap<
                ::std::string::String,
                ::std::sync::Arc<
                    dyn margaret::framework::websocket::web_socket_message_dispatch::WebSocketMessageDispatch<
                        #session_path,
                    >,
                >,
            > = ::std::collections::HashMap::new();
            #(#request_inserts)*

            let #notifications_mutability #notifications: ::std::collections::HashMap<
                ::std::string::String,
                ::std::sync::Arc<
                    dyn margaret::framework::websocket::web_socket_notification_dispatch::WebSocketNotificationDispatch<
                        #session_path,
                    >,
                >,
            > = ::std::collections::HashMap::new();
            #(#notification_inserts)*

            #table_value
        }
    }
}

fn render_session(plan: &SessionPlan, bindings: &ContainerBindings) -> TokenStream {
    let session_path = path_tokens(&plan.session.session_path);
    let captured = captured_providers(&plan.session);
    let factory = render_factory(&plan.session, &captured);
    let initializers = factory_initializers(&plan.session, bindings, &captured);
    let routes_parameter = plan.session.injects_routes().then(|| {
        quote! { routes: &::std::sync::Arc<super::super::super::routes::Routes>, }
    });
    let request_dispatches = plan
        .request_handlers
        .iter()
        .map(|binding| render_request_dispatch(binding, &session_path));
    let notification_dispatches = plan
        .notification_handlers
        .iter()
        .map(|binding| render_notification_dispatch(binding, &session_path));
    let dispatch_table = render_dispatch_table(plan, &session_path, bindings);
    let dispatch_call = quote! { dispatch_table(container) };
    let upgrade_type = quote! {
        ::std::sync::Arc<dyn margaret::framework::http::web_socket_upgrade::WebSocketUpgrade>
    };
    let upgrade_value = quote! {
        ::std::sync::Arc::new(
            margaret::framework::websocket::web_socket_upgrade_entry::WebSocketUpgradeEntry::new(
                Factory {
                    #initializers
                },
                #dispatch_call,
            ),
        )
    };
    quote! {
        #factory

        #(#request_dispatches)*
        #(#notification_dispatches)*

        #dispatch_table

        #[must_use]
        pub fn upgrade_entry(
            container: &super::super::super::container::Container,
            #routes_parameter
        ) -> #upgrade_type {
            #upgrade_value
        }
    }
}

pub(crate) fn render_sessions(
    sessions: &[SessionPlan],
    bindings: &ContainerBindings,
) -> Vec<GeneratedModuleTokens> {
    sessions
        .iter()
        .flat_map(|plan| {
            let module_name = &plan.session.module_name;

            [
                GeneratedModuleTokens::new(
                    format!("websocket/{module_name}"),
                    quote! {
                        mod upgrade_entry;
                        pub use upgrade_entry::upgrade_entry;
                    },
                ),
                GeneratedModuleTokens::new(
                    format!("websocket/{module_name}/upgrade_entry"),
                    render_session(plan, bindings),
                ),
            ]
        })
        .collect()
}
