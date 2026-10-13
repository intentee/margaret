use margaret_attributes::canonical_path::CanonicalPath;
use margaret_umbrella_path::umbrella_item_path::umbrella_item_path;

use crate::authorization_handler_module_name::AUTHORIZATION_HANDLER_MODULE_NAME;
use crate::oidc_provider_module_name::OIDC_PROVIDER_MODULE_NAME;

#[must_use]
pub fn authorization_handler_path() -> CanonicalPath {
    umbrella_item_path(&[
        OIDC_PROVIDER_MODULE_NAME,
        AUTHORIZATION_HANDLER_MODULE_NAME,
        "AuthorizationHandler",
    ])
}

#[cfg(test)]
mod tests {
    use super::authorization_handler_path;

    #[test]
    fn locates_the_authorization_handler_beside_the_provider_exports() {
        assert_eq!(
            authorization_handler_path().to_string(),
            "crate::margaret::oidc_provider::authorization_handler::AuthorizationHandler"
        );
    }
}
