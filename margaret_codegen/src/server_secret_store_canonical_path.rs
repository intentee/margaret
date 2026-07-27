use margaret_attributes::canonical_path::CanonicalPath;
use margaret_jwks_codegen::jwks_module_name::JWKS_MODULE_NAME;

use crate::umbrella_module_name::UMBRELLA_MODULE_NAME;

pub(crate) fn server_secret_store_canonical_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "crate".to_string(),
        UMBRELLA_MODULE_NAME.to_string(),
        JWKS_MODULE_NAME.to_string(),
        "JwksSecretStore".to_string(),
    ])
}
