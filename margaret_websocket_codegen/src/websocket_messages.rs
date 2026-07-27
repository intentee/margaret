use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;

use crate::message_kind::MessageKind;
use crate::web_socket_codegen_error::WebSocketCodegenError;
use crate::web_socket_message::WebSocketMessage;

pub(crate) fn websocket_messages(
    index: &AttributeIndex,
) -> Result<Vec<WebSocketMessage>, WebSocketCodegenError> {
    let mut messages = Vec::new();

    for matched in index.select_framework_attribute(FrameworkAttribute::WebsocketMessage) {
        let item = matched.item();
        let message = item.canonical_path().to_string();

        if index.struct_identifier(item.canonical_path()).is_none() {
            return Err(WebSocketCodegenError::MessageNotAStruct { message });
        }

        let kind = MessageKind::parse(matched.args()?, &message)?;

        messages.push(WebSocketMessage {
            kind,
            path: item.canonical_path().clone(),
        });
    }

    messages.sort_by_key(|message| message.path.to_string());

    Ok(messages)
}
