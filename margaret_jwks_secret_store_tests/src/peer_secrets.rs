use std::sync::Arc;

use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;
use margaret_jwks_roller_server::jwks_curve::JWKS_CURVE;

pub struct PeerSecrets {
    pub lagging: Arc<JwksSecretHolder>,
    pub rolled: Arc<JwksSecretHolder>,
}

impl PeerSecrets {
    #[must_use]
    pub fn one_generation_apart() -> Self {
        let lagging = fresh_secret(JWKS_CURVE);
        let rolled = rolled_secret(&lagging);

        Self {
            lagging: Arc::new(JwksSecretHolder::new(Arc::new(lagging))),
            rolled: Arc::new(JwksSecretHolder::new(Arc::new(rolled))),
        }
    }
}
