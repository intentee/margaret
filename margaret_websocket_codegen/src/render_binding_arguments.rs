use proc_macro2::TokenStream;
use quote::quote;

use crate::websocket_injectable::WebsocketInjectable;
use crate::websocket_transition::WebsocketTransition;

pub(crate) fn render_binding_arguments(transition: &WebsocketTransition) -> Vec<TokenStream> {
    transition
        .bindings
        .iter()
        .map(|binding| match binding {
            WebsocketInjectable::Emit => quote! { &typed_emit },
            WebsocketInjectable::Envelope => quote! { envelope },
            WebsocketInjectable::EventValue => quote! { event_value },
            WebsocketInjectable::Facts => quote! { &self.facts },
            WebsocketInjectable::FromState => quote! { from_state },
            WebsocketInjectable::Spawner => quote! { spawner },
        })
        .collect()
}
