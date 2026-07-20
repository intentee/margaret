use margaret_attributes::canonical_path::CanonicalPath;

use crate::session_parameter::SessionParameter;

pub(crate) struct WebSocketSession {
    pub(crate) module_name: String,
    pub(crate) parameters: Vec<SessionParameter>,
    pub(crate) path: String,
    pub(crate) server: String,
    pub(crate) session_path: CanonicalPath,
}
