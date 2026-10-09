use margaret_attributes::canonical_path::CanonicalPath;
use margaret_umbrella_path::umbrella_item_path::umbrella_item_path;

use crate::authorization_endpoint_url_module_name::AUTHORIZATION_ENDPOINT_URL_MODULE_NAME;
use crate::oidc_provider_module_name::OIDC_PROVIDER_MODULE_NAME;
use crate::provider_endpoints_module_name::PROVIDER_ENDPOINTS_MODULE_NAME;

#[must_use]
pub fn authorization_endpoint_url_path() -> CanonicalPath {
    umbrella_item_path(&[
        OIDC_PROVIDER_MODULE_NAME,
        PROVIDER_ENDPOINTS_MODULE_NAME,
        AUTHORIZATION_ENDPOINT_URL_MODULE_NAME,
        "AUTHORIZATION_ENDPOINT_URL",
    ])
}

#[cfg(test)]
mod tests {
    use super::authorization_endpoint_url_path;

    #[test]
    fn locates_the_authorization_endpoint_url_beside_the_provider_endpoints() {
        assert_eq!(
            authorization_endpoint_url_path().to_string(),
            "crate::margaret::oidc_provider::provider_endpoints::authorization_endpoint_url::AUTHORIZATION_ENDPOINT_URL"
        );
    }
}
