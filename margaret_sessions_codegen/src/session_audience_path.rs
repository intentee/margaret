use margaret_attributes::canonical_path::CanonicalPath;
use margaret_umbrella_path::umbrella_item_path::umbrella_item_path;

use crate::sessions_module_name::SESSIONS_MODULE_NAME;

#[must_use]
pub fn session_audience_path() -> CanonicalPath {
    umbrella_item_path(&[SESSIONS_MODULE_NAME, "session_audience", "SESSION_AUDIENCE"])
}
