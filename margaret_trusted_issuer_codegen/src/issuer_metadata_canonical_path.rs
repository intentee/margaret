use margaret_attributes::canonical_path::CanonicalPath;
use margaret_umbrella_path::umbrella_item_path::umbrella_item_path;

use crate::trusted_issuers_module_name::TRUSTED_ISSUERS_MODULE_NAME;

#[must_use]
pub fn issuer_metadata_canonical_path(module_segment: &str) -> CanonicalPath {
    umbrella_item_path(&[
        TRUSTED_ISSUERS_MODULE_NAME,
        module_segment,
        "IssuerMetadata",
    ])
}
