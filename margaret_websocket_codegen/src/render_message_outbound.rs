use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;

use crate::emitted_message::EmittedMessage;

pub(crate) fn render_message_outbound(message: &EmittedMessage) -> TokenStream {
    let concrete = path_tokens(&message.canonical_path);
    let method = &message.method;

    quote! {
        impl margaret_websocket::websocket_outbound::WebsocketOutbound for #concrete {
            fn wire_method(&self) -> &'static str {
                #method
            }
        }
    }
}
