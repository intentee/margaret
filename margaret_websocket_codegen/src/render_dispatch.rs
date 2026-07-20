use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;

use crate::alphabet_ident::alphabet_ident;
use crate::protocol::Protocol;
use crate::render_binding_arguments::render_binding_arguments;
use crate::websocket_injectable::WebsocketInjectable;
use crate::websocket_state::WebsocketState;
use crate::websocket_transition::WebsocketTransition;

fn render_emit_setup(transition: &WebsocketTransition) -> TokenStream {
    if transition.bindings.contains(&WebsocketInjectable::Emit) {
        let alphabet = alphabet_ident(transition);

        quote! {
            let typed_emit = margaret_websocket::emit::Emit::<#alphabet>::new(emit.clone());
        }
    } else {
        TokenStream::new()
    }
}

fn render_message_setup(transition: &WebsocketTransition) -> TokenStream {
    if transition.bindings.contains(&WebsocketInjectable::Envelope) {
        quote! {
            let envelope = margaret_websocket::envelope::Envelope::new(request_id, message);
        }
    } else {
        TokenStream::new()
    }
}

fn message_binding(transition: &WebsocketTransition) -> TokenStream {
    if transition.bindings.contains(&WebsocketInjectable::Envelope) {
        quote! { message }
    } else {
        quote! { _message }
    }
}

fn render_wire_arm(
    state_variant: &Ident,
    transition: &WebsocketTransition,
    method: &str,
) -> TokenStream {
    let message_path = path_tokens(transition.trigger.canonical_path());
    let field = format_ident!("{}", transition.field);
    let process = format_ident!("{}", transition.process_method);
    let arguments = render_binding_arguments(transition);
    let message_pattern = message_binding(transition);
    let message_setup = render_message_setup(transition);
    let emit_setup = render_emit_setup(transition);

    quote! {
        #method => {
            match margaret_validation::validate_json::validate_json::<#message_path>(params) {
                margaret_validation::validation_result::ValidationResult::Valid(#message_pattern) => {
                    #message_setup
                    #emit_setup

                    ProtocolState::from(self.#field.#process(#(#arguments),*).await)
                }
                margaret_validation::validation_result::ValidationResult::Invalid(details) => {
                    emit.send(margaret_websocket::outbound_frame::OutboundFrame::Error(
                        margaret_websocket::json_rpc_error_frame::JsonRpcErrorFrame::invalid_params(
                            request_id,
                            margaret_websocket::serde_json::to_value(&details).unwrap_or(margaret_websocket::serde_json::Value::Null),
                        ),
                    )).await;

                    ProtocolState::#state_variant(from_state)
                }
                margaret_validation::validation_result::ValidationResult::Malformed(_) => {
                    emit.send(margaret_websocket::outbound_frame::OutboundFrame::Error(
                        margaret_websocket::json_rpc_error_frame::JsonRpcErrorFrame::invalid_params(
                            request_id,
                            margaret_websocket::serde_json::Value::Null,
                        ),
                    )).await;

                    ProtocolState::#state_variant(from_state)
                }
            }
        }
    }
}

fn render_state_arm(protocol: &Protocol<'_>, state: &WebsocketState) -> TokenStream {
    let variant = format_ident!("{}", state.variant);
    let state_name = &state.variant;

    let has_wire_transitions = protocol.transitions.iter().any(|transition| {
        transition.from == state.canonical_path && transition.trigger.wire_method().is_some()
    });

    if !has_wire_transitions {
        return quote! {
            ProtocolState::#variant(from_state) => {
                self.reject(method, request_id, #state_name, &[], emit).await;

                ProtocolState::#variant(from_state)
            }
        };
    }

    let method_arms = protocol
        .transitions
        .iter()
        .filter(|transition| transition.from == state.canonical_path)
        .filter_map(|transition| {
            transition
                .trigger
                .wire_method()
                .map(|method| render_wire_arm(&variant, transition, method))
        });

    let allowed = protocol
        .transitions
        .iter()
        .filter(|transition| transition.from == state.canonical_path)
        .filter_map(|transition| transition.trigger.wire_method());

    quote! {
        ProtocolState::#variant(from_state) => match method {
            #(#method_arms)*
            other => {
                self.reject(other, request_id, #state_name, &[#(#allowed),*], emit).await;

                ProtocolState::#variant(from_state)
            }
        },
    }
}

pub(crate) fn render_dispatch(protocol: &Protocol<'_>) -> TokenStream {
    let state_arms = protocol
        .states
        .iter()
        .map(|state| render_state_arm(protocol, state));

    let spawner_used = protocol.transitions.iter().any(|transition| {
        transition.trigger.wire_method().is_some()
            && transition.bindings.contains(&WebsocketInjectable::Spawner)
    });
    let spawner = if spawner_used {
        format_ident!("spawner")
    } else {
        format_ident!("_spawner")
    };

    quote! {
        async fn dispatch(
            &self,
            state: ProtocolState,
            method: &str,
            params: ::std::option::Option<&margaret_websocket::serde_json::Value>,
            request_id: ::std::option::Option<margaret_websocket::request_id::RequestId>,
            emit: &margaret_websocket::emit_core::EmitCore,
            #spawner: &margaret_websocket::activity_spawner::ActivitySpawner<Internal>,
        ) -> ProtocolState {
            let request_id = match request_id {
                ::std::option::Option::Some(request_id) => request_id,
                ::std::option::Option::None => {
                    emit.send(margaret_websocket::outbound_frame::OutboundFrame::Error(
                        margaret_websocket::json_rpc_error_frame::JsonRpcErrorFrame::invalid_request(
                            ::std::option::Option::None,
                        ),
                    )).await;

                    return state;
                }
            };

            match state {
                #(#state_arms)*
            }
        }
    }
}
