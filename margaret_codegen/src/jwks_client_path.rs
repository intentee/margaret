use margaret_attributes::canonical_path::CanonicalPath;
use margaret_jwks_codegen::jwks_client_suffix::jwks_client_suffix;

use crate::umbrella_module_name::UMBRELLA_MODULE_NAME;

pub(crate) fn jwks_client_canonical_path() -> CanonicalPath {
    let mut segments = vec!["crate".to_string(), UMBRELLA_MODULE_NAME.to_string()];

    segments.extend(
        jwks_client_suffix()
            .iter()
            .map(|segment| (*segment).to_string()),
    );

    CanonicalPath::new(segments)
}
