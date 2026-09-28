use margaret_attributes::canonical_path::CanonicalPath;
use margaret_jwks_codegen::jwks_module_name::JWKS_MODULE_NAME;
use margaret_umbrella_path::umbrella_item_path::umbrella_item_path;

pub(crate) fn public_jwks_handler_canonical_path() -> CanonicalPath {
    umbrella_item_path(&[JWKS_MODULE_NAME, "PublicJwksHandler"])
}
