use std::sync::Arc;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;

#[must_use]
pub fn session_store() -> Arc<JwksSecretStore> {
    Arc::new(rolled_store(fresh_secret(SigningCurve::P256)))
}
