use std::sync::Arc;

use margaret_jwks_keygen::jwks_secret::JwksSecret;

#[derive(Clone)]
pub enum HeldSecret {
    Held(Arc<JwksSecret>),
    Unheld,
}
