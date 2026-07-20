use std::collections::HashMap;
use std::collections::HashSet;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;

use crate::websocket_codegen_error::WebsocketCodegenError;
use crate::websocket_message::WebsocketMessage;

pub(crate) fn websocket_messages(
    index: &AttributeIndex,
) -> Result<Vec<WebsocketMessage>, WebsocketCodegenError> {
    let selector = AttributeSelector::from_marker("websocket_message");
    let mut messages = Vec::new();
    let mut seen_paths = HashSet::new();
    let mut method_owner: HashMap<String, String> = HashMap::new();

    for matched in index.select(&selector) {
        let canonical_path = matched.item().canonical_path().clone();
        let message = canonical_path.to_string();

        let Some(identifier) = index.struct_identifier(&canonical_path) else {
            return Err(WebsocketCodegenError::MessageNotAStruct { message });
        };
        let variant = identifier.type_name().to_owned();

        if !seen_paths.insert(canonical_path.clone()) {
            return Err(WebsocketCodegenError::DuplicateMessage { message });
        }

        let method = matched.args()?.string("method")?.ok_or_else(|| {
            WebsocketCodegenError::MessageMissingMethod {
                message: message.clone(),
            }
        })?;

        if method.is_empty() {
            return Err(WebsocketCodegenError::EmptyWireMethod { message });
        }

        if let Some(first) = method_owner.insert(method.clone(), message.clone()) {
            return Err(WebsocketCodegenError::DuplicateWireMethod {
                method,
                first,
                second: message,
            });
        }

        messages.push(WebsocketMessage {
            canonical_path,
            method,
            variant,
        });
    }

    Ok(messages)
}
