use crate::jwks_module_name::JWKS_MODULE_NAME;

#[must_use]
pub fn jwks_client_suffix() -> [&'static str; 2] {
    [JWKS_MODULE_NAME, "JwksClient"]
}
