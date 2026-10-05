use std::sync::Arc;

use margaret_sync_holder::sync_holder::SyncHolder;
use margaret_sync_holder::sync_holder_subscription::SyncHolderSubscription;

use crate::jwks_secret::JwksSecret;

#[derive(Clone)]
pub struct JwksSecretHolder {
    inner: SyncHolder<Arc<JwksSecret>>,
}

impl JwksSecretHolder {
    #[must_use]
    pub fn new(secret: Arc<JwksSecret>) -> Self {
        Self {
            inner: SyncHolder::new(secret),
        }
    }

    #[must_use]
    pub fn get(&self) -> Arc<JwksSecret> {
        self.inner.get()
    }

    pub fn set(&self, secret: Arc<JwksSecret>) {
        self.inner.set(secret);
    }

    #[must_use]
    pub fn subscribe(&self) -> SyncHolderSubscription<Arc<JwksSecret>> {
        self.inner.subscribe()
    }
}
