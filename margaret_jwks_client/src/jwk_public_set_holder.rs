use std::sync::Arc;

use margaret_jwks_key_gen::jwk_public_set::JwkPublicSet;
use margaret_sync_holder::sync_holder::SyncHolder;

#[derive(Clone, Default)]
pub struct JwkPublicSetHolder {
    inner: SyncHolder<Arc<JwkPublicSet>>,
}

impl JwkPublicSetHolder {
    #[must_use]
    pub fn get(&self) -> Option<Arc<JwkPublicSet>> {
        self.inner.get()
    }

    #[must_use]
    pub fn is_ready(&self) -> bool {
        self.inner.get().is_some()
    }

    pub fn set(&self, value: Option<Arc<JwkPublicSet>>) {
        self.inner.set(value);
    }
}
