use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;

use crate::protocol::Protocol;
use crate::render_dispatch::render_dispatch;
use crate::render_dispatch_internal::render_dispatch_internal;
use crate::websocket_injectable::WebsocketInjectable;

pub(crate) fn render_dispatcher(protocol: &Protocol<'_>) -> TokenStream {
    let uses_facts = protocol
        .transitions
        .iter()
        .any(|transition| transition.bindings.contains(&WebsocketInjectable::Facts));
    let facts_declaration = uses_facts.then(|| {
        quote! { facts: margaret_websocket::connection_facts::ConnectionFacts, }
    });
    let facts_initializer = uses_facts.then(|| {
        quote! { facts: margaret_websocket::connection_facts::ConnectionFacts, }
    });

    let field_declarations = protocol.transitions.iter().map(|transition| {
        let field = format_ident!("{}", transition.field);
        let concrete = path_tokens(&transition.transition);

        quote! { #field: ::std::sync::Arc<#concrete>, }
    });

    let field_initializers = protocol.transitions.iter().map(|transition| {
        let field = format_ident!("{}", transition.field);

        quote! { #field: container.#field().await, }
    });

    let entry_concrete = path_tokens(&protocol.entry);

    let terminal_patterns = protocol
        .states
        .iter()
        .filter(|state| state.role.is_terminal())
        .map(|state| {
            let variant = format_ident!("{}", state.variant);

            quote! { ProtocolState::#variant(_) }
        });

    let protocol_methods = protocol
        .transitions
        .iter()
        .filter_map(|transition| transition.trigger.wire_method());

    let dispatch = render_dispatch(protocol);
    let dispatch_internal = render_dispatch_internal(protocol);

    quote! {
        #[derive(Clone)]
        pub struct Dispatcher {
            #facts_declaration
            #(#field_declarations)*
        }

        impl Dispatcher {
            const PROTOCOL_METHODS: &'static [&'static str] = &[#(#protocol_methods),*];

            async fn reject(
                &self,
                method: &str,
                request_id: margaret_websocket::request_id::RequestId,
                current_state: &str,
                allowed_methods: &[&str],
                emit: &margaret_websocket::emit_core::EmitCore,
            ) {
                let frame = if Self::PROTOCOL_METHODS.contains(&method) {
                    margaret_websocket::json_rpc_error_frame::JsonRpcErrorFrame::illegal_message_for_state(
                        request_id,
                        current_state,
                        method,
                        allowed_methods,
                    )
                } else {
                    margaret_websocket::json_rpc_error_frame::JsonRpcErrorFrame::method_not_found(
                        request_id,
                        method,
                    )
                };

                emit.send(margaret_websocket::outbound_frame::OutboundFrame::Error(frame)).await;
            }
        }

        #[async_trait::async_trait]
        impl margaret_websocket::protocol::Protocol for Dispatcher {
            type Internal = Internal;
            type State = ProtocolState;

            fn initial_state(&self) -> ProtocolState {
                ProtocolState::from(<#entry_concrete as ::std::default::Default>::default())
            }

            fn is_terminal(&self, state: &ProtocolState) -> bool {
                matches!(state, #(#terminal_patterns)|*)
            }

            #dispatch

            #dispatch_internal
        }

        pub async fn build(container: &super::super::container::Container) -> Dispatcher {
            Dispatcher {
                #facts_initializer
                #(#field_initializers)*
            }
        }
    }
}
