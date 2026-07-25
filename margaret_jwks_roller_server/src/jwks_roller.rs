use std::sync::Arc;

use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;

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
    pub fn create() -> Self {
        let bundle = JwksRollerServerBundle::new(JwksRollerServerBundleParams {
            storage: Arc::new(MemoryJwksSecretStorage),
        });
        let public_jwks_handler = bundle.public_jwks_handler();

        Self {
            bundle,
            public_jwks_handler,
        }
    }

    #[must_use]
    pub fn public_jwks_handler(&self) -> Arc<PublicJwksHandler> {
        self.public_jwks_handler.clone()
    }

    pub async fn run(&self) -> Result<(), JwksRollerServerError> {
        self.bundle.roll_and_publish()
    }
}

#[cfg(test)]
mod tests {
    use super::JwksRoller;

    #[test]
    fn serves_no_document_before_the_first_roll() {
        let roller = JwksRoller::create();

        assert_eq!(roller.public_jwks_handler().respond().status(), 503);
    }

    #[tokio::test]
    async fn serves_the_rolled_document_after_a_run() {
        let roller = JwksRoller::create();

        roller.run().await.expect("the first roll publishes a document");

        assert_eq!(roller.public_jwks_handler().respond().status(), 200);
    }
}
