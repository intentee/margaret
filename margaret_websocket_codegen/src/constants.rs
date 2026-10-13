use margaret_attributes::framework_vocabulary::FrameworkVocabulary;

use crate::message_cardinality::MessageCardinality;

fn web_socket_response_name(cardinality: MessageCardinality) -> &'static str {
    match cardinality {
        MessageCardinality::Single => "Single",
        MessageCardinality::Stream => "Stream",
    }
}

pub(crate) const WEB_SOCKET_RESPONSES: FrameworkVocabulary<MessageCardinality> =
    FrameworkVocabulary {
        enum_path: &[
            "margaret",
            "framework",
            "websocket",
            "web_socket_response",
            "WebSocketResponse",
        ],
        name: web_socket_response_name,
        variants: &[MessageCardinality::Single, MessageCardinality::Stream],
    };
