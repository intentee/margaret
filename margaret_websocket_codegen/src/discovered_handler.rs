use margaret_attributes::canonical_path::CanonicalPath;

use crate::handler_kind::HandlerKind;

pub(crate) struct DiscoveredHandler {
    pub(crate) handler_path: CanonicalPath,
    pub(crate) kind: HandlerKind,
    pub(crate) message_path: Option<CanonicalPath>,
    pub(crate) session_path: Option<CanonicalPath>,
}
