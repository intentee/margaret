use std::sync::Arc;

use margaret_jwks_key_gen::jwks_secret::JwksSecret;
use margaret_jwks_key_gen::jwks_secret_holder::JwksSecretHolder;
use margaret_macros::constructor;
use margaret_macros::singleton;

#[singleton]
pub struct SigningKeyStore {
    holder: JwksSecretHolder,
}

impl SigningKeyStore {
    #[constructor]
    pub fn create() -> Self {
        Self {
            holder: JwksSecretHolder::default(),
        }
    }

    #[must_use]
    pub fn current(&self) -> Option<Arc<JwksSecret>> {
        self.holder.get()
    }

    #[must_use]
    pub fn holder(&self) -> &JwksSecretHolder {
        &self.holder
    }
}
