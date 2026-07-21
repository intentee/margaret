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
            quote! { margaret_websocket::request_envelope::RequestEnvelope::new(id, message) }
        }
        MessageCardinality::Stream => {
            quote! { margaret_websocket::streaming_request_envelope::StreamingRequestEnvelope::new(id, message) }
        }
    };

    quote! {
        impl margaret_websocket::web_socket_request_message::WebSocketRequestMessage for #message {
            type Envelope = #envelope;

            fn envelope(
                id: margaret_websocket::request_id::RequestId,
                message: Self,
            ) -> Self::Envelope {
                #construction
            }
        }
    }
}

fn render_response_message(path: &CanonicalPath, method: &str) -> TokenStream {
    let message = path_tokens(path);

    quote! {
        impl margaret_websocket::web_socket_response_message::WebSocketResponseMessage for #message {
            const METHOD: &'static str = #method;
        }
    }
}

pub(crate) fn render_messages(messages: &[WebSocketMessage]) -> TokenStream {
    let implementations = messages.iter().filter_map(|message| match &message.kind {
        MessageKind::Request { cardinality, .. } => {
            Some(render_request_message(&message.path, cardinality))
        }
        MessageKind::Response { method } => Some(render_response_message(&message.path, method)),
        MessageKind::Notification { .. } => None,
    });

    quote! {
        #(#implementations)*
    }
}
