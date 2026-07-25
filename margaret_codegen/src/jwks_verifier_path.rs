use margaret_attributes::canonical_path::CanonicalPath;
use margaret_jwks_codegen::public_jwks_verifier_suffix::public_jwks_verifier_suffix;

use crate::umbrella_module_name::UMBRELLA_MODULE_NAME;

pub(crate) fn public_jwks_verifier_canonical_path() -> CanonicalPath {
    let mut segments = vec!["crate".to_string(), UMBRELLA_MODULE_NAME.to_string()];

    segments.extend(
        public_jwks_verifier_suffix()
            .iter()
            .map(|segment| (*segment).to_string()),
    );

    CanonicalPath::new(segments)
}
