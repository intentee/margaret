use margaret_attributes::canonical_path::CanonicalPath;

use crate::state_role::StateRole;

#[derive(Debug)]
pub struct WebsocketState {
    pub canonical_path: CanonicalPath,
    pub field: String,
    pub role: StateRole,
    pub variant: String,
}
