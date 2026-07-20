use proc_macro2::TokenStream;
use quote::quote;

use crate::protocol::Protocol;
use crate::render_dispatcher::render_dispatcher;
use crate::render_internal::render_internal;
use crate::render_protocol_state::render_protocol_state;
use crate::render_transition_alphabet::render_transition_alphabet;
use crate::websocket_injectable::WebsocketInjectable;

pub(crate) fn render_protocol(protocol: &Protocol<'_>) -> TokenStream {
    let protocol_state = render_protocol_state(protocol);
    let internal = render_internal(protocol);

    let alphabets = protocol
        .transitions
        .iter()
        .filter(|transition| transition.bindings.contains(&WebsocketInjectable::Emit))
        .map(|transition| render_transition_alphabet(transition));

    let dispatcher = render_dispatcher(protocol);

    quote! {
        #protocol_state

        #internal

        #(#alphabets)*

        #dispatcher
    }
}
