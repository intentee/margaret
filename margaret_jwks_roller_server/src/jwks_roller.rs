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
    #[must_use]
    pub fn create(
        storage: Arc<dyn JwksSecretStorage>,
        rsa_keys: Arc<dyn ProvidesRsaSigningKeys>,
    ) -> Self {
        let bundle =
            JwksRollerServerBundle::new(JwksRollerServerBundleParams { rsa_keys, storage });
        let public_jwks_handler = bundle.public_jwks_handler();

        Self {
            bundle,
            public_jwks_handler,
        }
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
    use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;

    use super::JwksRoller;

    fn roller() -> JwksRoller {
        JwksRoller::create(
            Arc::new(MemoryJwksSecretStorage),
            Arc::new(FixtureRsaSigningKeys::default()),
        )
    }

    #[test]
    fn serves_no_document_before_the_first_roll() {
        assert_eq!(roller().public_jwks_handler().respond().status(), 503);
    }

    #[test]
    fn serves_the_rolled_document_after_a_run() {
        let roller = roller();

        roller.run().expect("the first roll publishes a document");

        assert_eq!(roller.public_jwks_handler().respond().status(), 200);
    }

    #[test]
    fn exposes_the_secret_it_rolls() {
        let roller = roller();
        let holder = roller.jwks_secret_holder();

        assert!(holder.get().is_none());

        roller.run().expect("the first roll seeds the secret");

        assert!(holder.get().is_some());
    }
}
