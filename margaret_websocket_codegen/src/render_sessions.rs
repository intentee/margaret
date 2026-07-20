use heck::ToUpperCamelCase;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_container::injected_dependency::InjectedDependency;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::handler_binding::HandlerBinding;
use crate::session_parameter::SessionParameter;
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

fn injected_field_value(dependency: &InjectedDependency) -> TokenStream {
    match dependency {
        InjectedDependency::SingleConcrete { field, .. }
        | InjectedDependency::SingleInterface { field, .. } => {
            let accessor = format_ident!("{}", field);

            quote! { container.#accessor().await }
        }
        InjectedDependency::Collection { member_fields, .. } => {
            let members = member_fields.iter().map(|member| {
                let accessor = format_ident!("{}", member);

                quote! { container.#accessor().await }
            });

            quote! { ::std::vec::Vec::from([#(#members),*]) }
        }
    }
}

fn factory_fields(session: &WebSocketSession) -> TokenStream {
    let fields = session.parameters.iter().filter_map(|parameter| match parameter {
        SessionParameter::Injectable { dependency, holder } => {
            let field_type = injected_field_type(dependency);

            Some(quote! { #holder: #field_type, })
        }
        SessionParameter::Route { .. } => None,
    });

    quote! { #(#fields)* }
}

fn factory_initializers(session: &WebSocketSession) -> TokenStream {
    let initializers = session.parameters.iter().filter_map(|parameter| match parameter {
        SessionParameter::Injectable { dependency, holder } => {
            let value = injected_field_value(dependency);

            Some(quote! { #holder: #value, })
        }
        SessionParameter::Route { .. } => None,
    });

    quote! { #(#initializers)* }
}

fn route_extractions(session: &WebSocketSession) -> TokenStream {
    let extractions = session.parameters.iter().filter_map(|parameter| match parameter {
        SessionParameter::Route { from, holder } => Some(quote! {
            let #holder = match margaret_http::require_route_parameter::require_route_parameter(
                handshake,
                #from,
            ) {
                ::std::result::Result::Ok(value) => value,
                ::std::result::Result::Err(response) => {
                    return ::std::result::Result::Err(response);
                }
            };
        }),
        SessionParameter::Injectable { .. } => None,
    });

    quote! { #(#extractions)* }
}

fn build_arguments(session: &WebSocketSession) -> TokenStream {
    let arguments = session.parameters.iter().map(|parameter| match parameter {
        SessionParameter::Injectable { holder, .. } => quote! { self.#holder.clone() },
        SessionParameter::Route { holder, .. } => quote! { #holder },
    });

    quote! { #(#arguments),* }
}

fn render_factory(session: &WebSocketSession) -> TokenStream {
    let session_path = path_tokens(&session.session_path);
    let fields = factory_fields(session);
    let extractions = route_extractions(session);
    let arguments = build_arguments(session);

    quote! {
        struct Factory {
            #fields
        }

        #[async_trait::async_trait]
        impl margaret_websocket::web_socket_session_factory::WebSocketSessionFactory for Factory {
            type Session = #session_path;

            async fn create(
                &self,
                handshake: &margaret_http::request::Request,
            ) -> ::std::result::Result<
                ::std::sync::Arc<Self::Session>,
                margaret_http::response::Response,
            > {
                #extractions

                ::std::result::Result::Ok(::std::sync::Arc::new(
                    #session_path::build_for_session(#arguments),
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
                method: ::std::string::String,
                params: serde_json::Value,
                socket: margaret_websocket::web_socket::WebSocket,
            ) {
                margaret_websocket::dispatch_request::dispatch_request(
                    &*self.handler,
                    cancellation_token,
                    session,
                    id,
                    method,
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

fn dispatch_insert(binding: &HandlerBinding, map: &Ident) -> TokenStream {
    let dispatch = dispatch_struct_ident(binding);
    let method = &binding.method;
    let accessor = format_ident!("{}", binding.handler_path.field_name());

    quote! {
        #map.insert(
            #method.to_string(),
            ::std::sync::Arc::new(#dispatch {
                handler: container.#accessor().await,
            }),
        );
    }
}

fn render_dispatch_table(plan: &SessionPlan, session_path: &TokenStream) -> TokenStream {
    let requests = format_ident!("requests");
    let notifications = format_ident!("notifications");
    let request_inserts = plan
        .request_handlers
        .iter()
        .map(|binding| dispatch_insert(binding, &requests));
    let notification_inserts = plan
        .notification_handlers
        .iter()
        .map(|binding| dispatch_insert(binding, &notifications));

    quote! {
        async fn dispatch_table(
            container: &super::super::container::Container,
        ) -> ::std::sync::Arc<
            margaret_websocket::web_socket_dispatch_table::WebSocketDispatchTable<#session_path>,
        > {
            let mut #requests: ::std::collections::HashMap<
                ::std::string::String,
                ::std::sync::Arc<
                    dyn margaret_websocket::web_socket_message_dispatch::WebSocketMessageDispatch<
                        #session_path,
                    >,
                >,
            > = ::std::collections::HashMap::new();
            #(#request_inserts)*

            let mut #notifications: ::std::collections::HashMap<
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

fn render_session(plan: &SessionPlan) -> TokenStream {
    let session_path = path_tokens(&plan.session.session_path);
    let factory = render_factory(&plan.session);
    let initializers = factory_initializers(&plan.session);
    let request_dispatches = plan
        .request_handlers
        .iter()
        .map(|binding| render_request_dispatch(binding, &session_path));
    let notification_dispatches = plan
        .notification_handlers
        .iter()
        .map(|binding| render_notification_dispatch(binding, &session_path));
    let dispatch_table = render_dispatch_table(plan, &session_path);

    quote! {
        #factory

        #(#request_dispatches)*
        #(#notification_dispatches)*

        #dispatch_table

        pub async fn upgrade_entry(
            container: &super::super::container::Container,
        ) -> ::std::sync::Arc<dyn margaret_http::web_socket_upgrade::WebSocketUpgrade> {
            ::std::sync::Arc::new(
                margaret_websocket::web_socket_upgrade_entry::WebSocketUpgradeEntry::new(
                    Factory {
                        #initializers
                    },
                    dispatch_table(container).await,
                ),
            )
        }
    }
}

pub(crate) fn render_sessions(sessions: &[SessionPlan]) -> Vec<GeneratedModuleTokens> {
    sessions
        .iter()
        .map(|plan| {
            GeneratedModuleTokens::new(
                format!("websocket/{}", plan.session.module_name),
                render_session(plan),
            )
        })
        .collect()
}
