use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;

use crate::message_kind::MessageKind;
use crate::websocket_codegen_error::WebSocketCodegenError;
use crate::websocket_message::WebSocketMessage;

pub(crate) fn websocket_messages(
    index: &AttributeIndex,
) -> Result<Vec<WebSocketMessage>, WebSocketCodegenError> {
    let selector = AttributeSelector::from_marker("websocket_message");
    let mut messages = Vec::new();

    for matched in index.select(&selector) {
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
