use rustls::RootCertStore;

use margaret_sync_holder::sync_holder::SyncHolder;
use margaret_sync_holder::sync_holder_subscription::SyncHolderSubscription;

#[derive(Clone, Default)]
pub struct RootCertStoreHolder {
    inner: SyncHolder<RootCertStore>,
}

impl RootCertStoreHolder {
    #[must_use]
    pub fn get(&self) -> Option<RootCertStore> {
        self.inner.get()
    }

    pub fn set(&self, value: Option<RootCertStore>) {
        self.inner.set(value);
    }

    #[must_use]
    pub fn subscribe(&self) -> SyncHolderSubscription<RootCertStore> {
        self.inner.subscribe()
    }
}
