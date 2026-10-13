use margaret_attributes::canonical_path::CanonicalPath;
use margaret_umbrella_path::umbrella_item_path::umbrella_item_path;

use crate::jwks_module_name::JWKS_MODULE_NAME;

#[must_use]
pub fn public_jwks_handler_canonical_path() -> CanonicalPath {
    umbrella_item_path(&[JWKS_MODULE_NAME, "PublicJwksHandler"])
}
