use std::sync::Arc;

use margaret_sync_holder::sync_holder::SyncHolder;
use margaret_sync_holder::sync_holder_subscription::SyncHolderSubscription;

use crate::jwks_secret::JwksSecret;

#[derive(Clone, Default)]
pub struct JwksSecretHolder {
    inner: SyncHolder<Arc<JwksSecret>>,
}

impl JwksSecretHolder {
    #[must_use]
    pub fn get(&self) -> Option<Arc<JwksSecret>> {
        self.inner.get()
    }

    pub fn set(&self, value: Option<Arc<JwksSecret>>) {
        self.inner.set(value);
    }

    #[must_use]
    pub fn subscribe(&self) -> SyncHolderSubscription<Arc<JwksSecret>> {
        self.inner.subscribe()
    }
}
