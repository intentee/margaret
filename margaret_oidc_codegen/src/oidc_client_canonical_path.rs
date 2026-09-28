use margaret_attributes::canonical_path::CanonicalPath;
use margaret_umbrella_path::umbrella_item_path::umbrella_item_path;

use crate::oidc_module_name::OIDC_MODULE_NAME;

#[must_use]
pub fn oidc_client_canonical_path(module_segment: &str) -> CanonicalPath {
    umbrella_item_path(&[OIDC_MODULE_NAME, module_segment, "OidcClient"])
}
