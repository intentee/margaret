use margaret_attributes::canonical_path::CanonicalPath;
use margaret_umbrella_path::umbrella_item_path::umbrella_item_path;

use crate::oidc_provider_module_name::OIDC_PROVIDER_MODULE_NAME;
use crate::provider_endpoints_module_name::PROVIDER_ENDPOINTS_MODULE_NAME;

#[must_use]
pub fn provider_endpoints_path() -> CanonicalPath {
    umbrella_item_path(&[
        OIDC_PROVIDER_MODULE_NAME,
        PROVIDER_ENDPOINTS_MODULE_NAME,
        "PROVIDER_ENDPOINTS",
    ])
}

#[cfg(test)]
mod tests {
    use super::provider_endpoints_path;

    #[test]
    fn roots_the_endpoints_in_the_provider_module() {
        assert_eq!(
            provider_endpoints_path().to_string(),
            "crate::margaret::oidc_provider::provider_endpoints::PROVIDER_ENDPOINTS"
        );
    }
}
