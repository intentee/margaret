use proc_macro2::TokenStream;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_codegen_tokens::path_tokens::path_tokens;

use crate::message_cardinality::MessageCardinality;
use crate::message_kind::MessageKind;
use crate::web_socket_message::WebSocketMessage;

fn render_notification_message(path: &CanonicalPath, method: &str) -> TokenStream {
    let message = path_tokens(path);

    quote! {
        impl margaret::framework::websocket_envelope::web_socket_notification_message::WebSocketNotificationMessage for #message {
            const METHOD: &'static str = #method;
        }
    }
}

fn render_request_message(
    path: &CanonicalPath,
    cardinality: &MessageCardinality,
    method: &str,
) -> TokenStream {
    let message = path_tokens(path);
    let envelope = match cardinality {
        MessageCardinality::Single => {
            quote! { margaret::framework::websocket_envelope::request_envelope::RequestEnvelope<Self> }
        }
        MessageCardinality::Stream => {
            quote! { margaret::framework::websocket_envelope::streaming_request_envelope::StreamingRequestEnvelope<Self> }
        }
    };
    let construction = match cardinality {
        MessageCardinality::Single => {
            quote! { margaret::framework::websocket_envelope::request_envelope::RequestEnvelope::new(id, message) }
        }
        MessageCardinality::Stream => {
            quote! { margaret::framework::websocket_envelope::streaming_request_envelope::StreamingRequestEnvelope::new(id, message) }
        }
    };

    quote! {
        impl margaret::framework::websocket_envelope::web_socket_request_message::WebSocketRequestMessage for #message {
            const METHOD: &'static str = #method;

            type Envelope = #envelope;

            fn envelope(
                id: margaret::framework::websocket_envelope::request_id::RequestId,
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
        impl margaret::framework::websocket_envelope::web_socket_response_message::WebSocketResponseMessage for #message {
            const METHOD: &'static str = #method;
        }
    }
}

pub(crate) fn render_messages(messages: &[WebSocketMessage]) -> TokenStream {
    let implementations = messages.iter().map(|message| match &message.kind {
        MessageKind::Notification { method } => {
            render_notification_message(&message.path, method)
        }
        MessageKind::Request {
            cardinality,
            method,
        } => render_request_message(&message.path, cardinality, method),
        MessageKind::Response { method } => render_response_message(&message.path, method),
    });

    quote! {
        #(#implementations)*
    }
}
