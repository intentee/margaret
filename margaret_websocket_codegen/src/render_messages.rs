use proc_macro2::TokenStream;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_codegen_tokens::path_tokens::path_tokens;

use crate::message_cardinality::MessageCardinality;
use crate::message_kind::MessageKind;
use crate::websocket_message::WebSocketMessage;

fn render_request_message(path: &CanonicalPath, cardinality: &MessageCardinality) -> TokenStream {
    let message = path_tokens(path);
    let envelope = match cardinality {
        MessageCardinality::Single => {
            quote! { margaret_websocket::request_envelope::RequestEnvelope<Self> }
        }
        MessageCardinality::Stream => {
            quote! { margaret_websocket::streaming_request_envelope::StreamingRequestEnvelope<Self> }
        }
    };
    let construction = match cardinality {
        MessageCardinality::Single => {
            quote! { margaret_websocket::request_envelope::RequestEnvelope::new(id, method, message) }
        }
        MessageCardinality::Stream => {
            quote! { margaret_websocket::streaming_request_envelope::StreamingRequestEnvelope::new(id, method, message) }
        }
    };

    quote! {
        impl margaret_websocket::web_socket_request_message::WebSocketRequestMessage for #message {
            type Envelope = #envelope;

            fn envelope(
                id: margaret_websocket::request_id::RequestId,
                method: ::std::string::String,
                message: Self,
            ) -> Self::Envelope {
                #construction
            }
        }
    }
}

pub(crate) fn render_messages(messages: &[WebSocketMessage]) -> TokenStream {
    let implementations = messages.iter().filter_map(|message| match &message.kind {
        MessageKind::Request { cardinality, .. } => {
            Some(render_request_message(&message.path, cardinality))
        }
        MessageKind::Notification { .. } | MessageKind::Response => None,
    });

    quote! {
        #(#implementations)*
    }
}
