use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::alphabet_ident::alphabet_ident;
use crate::protocol::Protocol;
use crate::render_binding_arguments::render_binding_arguments;
use crate::render_internal::internal_event_triggers;
use crate::websocket_injectable::WebsocketInjectable;
use crate::websocket_transition::WebsocketTransition;

fn render_internal_arm(
    transition: &WebsocketTransition,
    state_variant_by_path: &HashMap<&CanonicalPath, &str>,
) -> TokenStream {
    let from_variant = format_ident!(
        "{}",
        state_variant_by_path
            .get(&transition.from)
            .copied()
            .unwrap_or_default()
    );
    let event_variant = format_ident!("{}", transition.trigger.variant());
    let field = format_ident!("{}", transition.field);
    let process = format_ident!("{}", transition.process_method);
    let arguments = render_binding_arguments(transition);

    let state_pattern = if transition.bindings.contains(&WebsocketInjectable::FromState) {
        quote! { from_state }
    } else {
        quote! { _ }
    };

    let event_pattern = if transition.bindings.contains(&WebsocketInjectable::EventValue) {
        quote! { event_value }
    } else {
        quote! { _ }
    };

    let emit_setup = if transition.bindings.contains(&WebsocketInjectable::Emit) {
        let alphabet = alphabet_ident(transition);

        quote! {
            let typed_emit = margaret_websocket::emit::Emit::<#alphabet>::new(emit.clone());
        }
    } else {
        TokenStream::new()
    };

    quote! {
        (ProtocolState::#from_variant(#state_pattern), Internal::#event_variant(#event_pattern)) => {
            #emit_setup

            ProtocolState::from(self.#field.#process(#(#arguments),*).await)
        }
    }
}

pub(crate) fn render_dispatch_internal(protocol: &Protocol<'_>) -> TokenStream {
    let triggers = internal_event_triggers(protocol);

    if triggers.is_empty() {
        return quote! {
            async fn dispatch_internal(
                &self,
                _state: ProtocolState,
                event: Internal,
                _emit: &margaret_websocket::emit_core::EmitCore,
                _spawner: &margaret_websocket::activity_spawner::ActivitySpawner<Internal>,
            ) -> ProtocolState {
                match event {}
            }
        };
    }

    let state_variant_by_path: HashMap<&CanonicalPath, &str> = protocol
        .states
        .iter()
        .map(|state| (&state.canonical_path, state.variant.as_str()))
        .collect();

    let arms = protocol
        .transitions
        .iter()
        .filter(|transition| transition.trigger.wire_method().is_none())
        .map(|transition| render_internal_arm(transition, &state_variant_by_path));

    let internal_transitions = || {
        protocol
            .transitions
            .iter()
            .filter(|transition| transition.trigger.wire_method().is_none())
    };
    let emit = if internal_transitions()
        .any(|transition| transition.bindings.contains(&WebsocketInjectable::Emit))
    {
        format_ident!("emit")
    } else {
        format_ident!("_emit")
    };
    let spawner = if internal_transitions()
        .any(|transition| transition.bindings.contains(&WebsocketInjectable::Spawner))
    {
        format_ident!("spawner")
    } else {
        format_ident!("_spawner")
    };

    quote! {
        async fn dispatch_internal(
            &self,
            state: ProtocolState,
            event: Internal,
            #emit: &margaret_websocket::emit_core::EmitCore,
            #spawner: &margaret_websocket::activity_spawner::ActivitySpawner<Internal>,
        ) -> ProtocolState {
            match (state, event) {
                #(#arms)*
                (state, _) => state,
            }
        }
    }
}
