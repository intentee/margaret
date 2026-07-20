use proc_macro2::Ident;
use quote::format_ident;

use crate::websocket_transition::WebsocketTransition;

pub(crate) fn alphabet_ident(transition: &WebsocketTransition) -> Ident {
    format_ident!("{}Emit", transition.type_name)
}
