use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use crate::protocol::Protocol;

pub(crate) fn render_handshake_handler(protocol: &Protocol<'_>) -> TokenStream {
    let module = format_ident!("{}", protocol.entry_field);

    quote! {
        margaret_http::responder_handler::responder_handler(
            ::std::sync::Arc::new(super::super::websocket::#module::build(container).await),
            |dispatcher: ::std::sync::Arc<super::super::websocket::#module::Dispatcher>,
             request: &margaret_http::request::Request|
             -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                        Output = margaret_http::response_continuation::ResponseContinuation,
                    > + ::std::marker::Send
                    + '_,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    match margaret_websocket::websocket_accept_key::websocket_accept_key(
                        request.inputs.server.header("upgrade").ok().flatten(),
                        request.inputs.server.header("connection").ok().flatten(),
                        request.inputs.server.header("sec-websocket-version").ok().flatten(),
                        request.inputs.server.header("sec-websocket-key").ok().flatten(),
                    ) {
                        ::std::option::Option::Some(accept_key) => {
                            margaret_http::response_continuation::ResponseContinuation::Upgrade(
                                ::std::boxed::Box::new(
                                    margaret_websocket::websocket_upgrade::WebsocketUpgrade::new(
                                        (*dispatcher).clone(),
                                        accept_key,
                                        margaret_websocket::websocket_channel_config::WebSocketChannelConfig::recommended(),
                                    ),
                                ),
                            )
                        }
                        ::std::option::Option::None => {
                            margaret_http::response_continuation::ResponseContinuation::Done(
                                margaret_http::response::Response::text(426u16, "Upgrade Required")
                                    .header("upgrade", "websocket")
                                    .header("connection", "Upgrade")
                                    .header("sec-websocket-version", "13"),
                            )
                        }
                    }
                })
            },
        )
    }
}
