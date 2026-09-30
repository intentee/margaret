use margaret_attributes::canonical_path::CanonicalPath;
use margaret_umbrella_path::umbrella_item_path::umbrella_item_path;

use crate::oidc_provider_module_name::OIDC_PROVIDER_MODULE_NAME;
use crate::provider_endpoint_paths_module_name::PROVIDER_ENDPOINT_PATHS_MODULE_NAME;

#[must_use]
pub fn provider_endpoint_paths_path() -> CanonicalPath {
    umbrella_item_path(&[
        OIDC_PROVIDER_MODULE_NAME,
        PROVIDER_ENDPOINT_PATHS_MODULE_NAME,
        "PROVIDER_ENDPOINT_PATHS",
    ])
}

#[cfg(test)]
mod tests {
    use super::provider_endpoint_paths_path;

    #[test]
    fn roots_the_endpoint_paths_in_the_provider_module() {
        assert_eq!(
            provider_endpoint_paths_path().to_string(),
            "crate::margaret::oidc_provider::provider_endpoint_paths::PROVIDER_ENDPOINT_PATHS"
        );
    }
}
