use std::sync::Arc;

use margaret_jwks_keygen::public_jwks::PublicJwks;
use margaret_sync_holder::sync_holder::SyncHolder;
use margaret_sync_holder::sync_holder_subscription::SyncHolderSubscription;

#[derive(Clone, Default)]
pub struct PublicJwksHolder {
    inner: SyncHolder<Arc<PublicJwks>>,
}

impl PublicJwksHolder {
    #[must_use]
    pub fn get(&self) -> Option<Arc<PublicJwks>> {
        self.inner.get()
    }

    #[must_use]
    pub fn is_ready(&self) -> bool {
        self.inner.get().is_some()
    }

    pub fn set(&self, value: Option<Arc<PublicJwks>>) {
        self.inner.set(value);
    }

    #[must_use]
    pub fn subscribe(&self) -> SyncHolderSubscription<Arc<PublicJwks>> {
        self.inner.subscribe()
    }
}
