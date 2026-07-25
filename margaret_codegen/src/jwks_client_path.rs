use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::tag::Tag;
use margaret_jwks_codegen::jwks_client_module_segment::jwks_client_module_segment;
use margaret_jwks_codegen::jwks_module_name::JWKS_MODULE_NAME;

use crate::umbrella_module_name::UMBRELLA_MODULE_NAME;

pub(crate) fn jwks_client_canonical_path(tag: &Tag) -> CanonicalPath {
    CanonicalPath::new(vec![
        "crate".to_string(),
        UMBRELLA_MODULE_NAME.to_string(),
        JWKS_MODULE_NAME.to_string(),
        jwks_client_module_segment(tag),
        "JwksClient".to_string(),
    ])
}
