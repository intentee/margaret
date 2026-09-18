use proc_macro2::Ident;
use quote::format_ident;

#[must_use]
pub fn spiffe_web_socket_client_ident() -> Ident {
    format_ident!("spiffe_websocket_client")
}
