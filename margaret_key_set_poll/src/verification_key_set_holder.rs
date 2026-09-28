use std::sync::Arc;

use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_sync_holder::sync_holder::SyncHolder;
use margaret_sync_holder::sync_holder_subscription::SyncHolderSubscription;

#[derive(Clone, Default)]
pub struct VerificationKeySetHolder {
    inner: SyncHolder<Arc<VerificationKeySet>>,
}

impl VerificationKeySetHolder {
    #[must_use]
    pub fn get(&self) -> Option<Arc<VerificationKeySet>> {
        self.inner.get()
    }

    #[must_use]
    pub fn is_ready(&self) -> bool {
        self.inner.get().is_some()
    }

    pub fn set(&self, value: Option<Arc<VerificationKeySet>>) {
        self.inner.set(value);
    }

    #[must_use]
    pub fn subscribe(&self) -> SyncHolderSubscription<Arc<VerificationKeySet>> {
        self.inner.subscribe()
    }
}
