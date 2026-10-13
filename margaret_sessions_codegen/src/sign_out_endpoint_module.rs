use margaret_attributes::canonical_path::CanonicalPath;
use margaret_generated_module::module_path::ModulePath;
use margaret_umbrella_path::umbrella_item_path::umbrella_item_path;

use crate::constants::SESSIONS_MODULE_NAME;
use crate::constants::SIGN_OUT_ENDPOINTS_MODULE_NAME;
use crate::constants::SIGN_OUT_HANDLER_MODULE_NAME;
use crate::constants::SIGN_OUT_HANDLER_TYPE_NAME;

pub struct SignOutEndpointModule {
    handler_path: CanonicalPath,
    name: String,
}

impl SignOutEndpointModule {
    #[must_use]
    pub fn new(name: String) -> Self {
        Self {
            handler_path: umbrella_item_path(&[
                SESSIONS_MODULE_NAME,
                SIGN_OUT_ENDPOINTS_MODULE_NAME,
                &name,
                SIGN_OUT_HANDLER_MODULE_NAME,
                SIGN_OUT_HANDLER_TYPE_NAME,
            ]),
            name,
        }
    }

    #[must_use]
    pub fn handler_path(&self) -> &CanonicalPath {
        &self.handler_path
    }

    pub(crate) fn handler_module_path(&self) -> ModulePath {
        self.module_path().child(SIGN_OUT_HANDLER_MODULE_NAME)
    }

    pub(crate) fn module_path(&self) -> ModulePath {
        ModulePath::root(SESSIONS_MODULE_NAME)
            .child(SIGN_OUT_ENDPOINTS_MODULE_NAME)
            .child(self.name.as_str())
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::SignOutEndpointModule;

    #[test]
    fn places_the_handler_of_a_sign_out_route_in_a_module_of_its_own_under_the_sessions() {
        assert_eq!(
            SignOutEndpointModule::new("routes_post_sign_out_post_sign_out".to_string())
                .handler_path()
                .to_string(),
            "crate::margaret::sessions::sign_out_endpoints::routes_post_sign_out_post_sign_out::session_sign_out_endpoint::SessionSignOutEndpoint"
        );
    }
}
