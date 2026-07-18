use std::sync::Arc;

use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;
use margaret_jwks_roller_server::JwksRollerServerBundle;
use margaret_jwks_roller_server::JwksRollerServerBundleParams;
use margaret_jwks_roller_server::jwk_public_set_handler::JwkPublicSetHandler;
use margaret_jwks_roller_server::jwks_roller_server_error::JwksRollerServerError;
use margaret_macros::constructor;
use margaret_macros::singleton;

#[singleton]
pub struct JwksRolling {
    bundle: JwksRollerServerBundle,
    jwk_public_set_handler: Arc<JwkPublicSetHandler>,
}

impl JwksRolling {
    #[constructor]
    #[must_use]
    pub fn create() -> Self {
        let bundle = JwksRollerServerBundle::new(JwksRollerServerBundleParams {
            storage: Arc::new(MemoryJwksSecretStorage),
        });
        let jwk_public_set_handler = bundle.jwk_public_set_handler();

        Self {
            bundle,
            jwk_public_set_handler,
        }
    }

    #[must_use]
    pub fn jwk_public_set_handler(&self) -> &JwkPublicSetHandler {
        &self.jwk_public_set_handler
    }

    pub fn roll_and_publish(&self) -> Result<(), JwksRollerServerError> {
        self.bundle.roll_and_publish()
    }
}
