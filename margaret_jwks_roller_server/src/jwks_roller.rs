use std::sync::Arc;

use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_keygen::provides_rsa_signing_keys::ProvidesRsaSigningKeys;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;

use crate::jwks_roller_server_bundle::JwksRollerServerBundle;
use crate::jwks_roller_server_bundle_params::JwksRollerServerBundleParams;
use crate::jwks_roller_server_error::JwksRollerServerError;
use crate::public_jwks_handler::PublicJwksHandler;

pub struct JwksRoller {
    bundle: JwksRollerServerBundle,
    public_jwks_handler: Arc<PublicJwksHandler>,
}

impl JwksRoller {
    /// # Errors
    ///
    /// Returns `JwksRollerServerError` when the first secret cannot be rolled and published.
    pub fn create(
        storage: Arc<dyn JwksSecretStorage>,
        rsa_keys: Arc<dyn ProvidesRsaSigningKeys>,
    ) -> Result<Self, JwksRollerServerError> {
        JwksRollerServerBundle::new(JwksRollerServerBundleParams { rsa_keys, storage }).map(
            |bundle| Self {
                public_jwks_handler: bundle.public_jwks_handler(),
                bundle,
            },
        )
    }

    #[must_use]
    pub fn jwks_secret_holder(&self) -> JwksSecretHolder {
        self.bundle.jwks_secret_holder()
    }

    #[must_use]
    pub fn public_jwks_handler(&self) -> Arc<PublicJwksHandler> {
        self.public_jwks_handler.clone()
    }

    /// # Errors
    ///
    /// Returns `JwksRollerServerError` propagated from the work it performs.
    pub fn run(&self) -> Result<(), JwksRollerServerError> {
        self.bundle.roll_and_publish()
    }
}
