use std::sync::Arc;

use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_roller_server::jwks_curve::JWKS_CURVE;

#[must_use]
pub fn fixture_secrets() -> Arc<JwksSecretHolder> {
    Arc::new(JwksSecretHolder::new(Arc::new(fresh_secret(JWKS_CURVE))))
}
