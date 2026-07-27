use margaret_attributes::canonical_path::CanonicalPath;

use crate::message_kind::MessageKind;

pub(crate) struct WebSocketMessage {
    pub(crate) kind: MessageKind,
    pub(crate) path: CanonicalPath,
}
