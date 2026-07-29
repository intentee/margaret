use std::sync::Arc;

use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;

use crate::jwks_roller_server_bundle::JwksRollerServerBundle;
use crate::jwks_roller_server_bundle_params::JwksRollerServerBundleParams;
use crate::jwks_roller_server_error::JwksRollerServerError;
use crate::public_jwks_handler::PublicJwksHandler;

pub struct JwksRoller {
    bundle: JwksRollerServerBundle,
    public_jwks_handler: Arc<PublicJwksHandler>,
    server_secret_store: Arc<JwksSecretStore>,
}

impl JwksRoller {
    #[must_use]
    pub fn create(storage: Arc<dyn JwksSecretStorage>) -> Self {
        let bundle = JwksRollerServerBundle::new(JwksRollerServerBundleParams { storage });
        let public_jwks_handler = bundle.public_jwks_handler();
        let server_secret_store = Arc::new(JwksSecretStore::new(bundle.jwks_secret_holder()));

        Self {
            bundle,
            public_jwks_handler,
            server_secret_store,
        }
    }

    #[must_use]
    pub fn public_jwks_handler(&self) -> Arc<PublicJwksHandler> {
        self.public_jwks_handler.clone()
    }

    pub fn run(&self) -> Result<(), JwksRollerServerError> {
        self.bundle.roll_and_publish()
    }

    #[must_use]
    pub fn server_secret_store(&self) -> Arc<JwksSecretStore> {
        self.server_secret_store.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;

    use super::JwksRoller;

    fn roller() -> JwksRoller {
        JwksRoller::create(Arc::new(MemoryJwksSecretStorage))
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

    #[tokio::test]
    async fn exposes_a_secret_store_backed_by_the_live_secret() {
        let roller = roller();
        let store = roller.server_secret_store();
        let claims = serde_json::json!({ "sub": "subject" });

        assert!(store.sign(&claims).await.is_err());

        roller.run().expect("the first roll seeds the secret");

        assert!(store.sign(&claims).await.is_ok());
    }
}
