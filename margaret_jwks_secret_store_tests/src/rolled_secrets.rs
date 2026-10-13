use std::sync::Arc;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;

#[must_use]
pub fn rolled_secrets(secret: JwksSecret) -> Arc<JwksSecretHolder> {
    Arc::new(JwksSecretHolder::new(Arc::new(secret)))
}
