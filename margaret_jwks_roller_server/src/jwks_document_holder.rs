use bytes::Bytes;

use margaret_sync_holder::sync_holder::SyncHolder;
use margaret_sync_holder::sync_holder_subscription::SyncHolderSubscription;

#[derive(Clone, Default)]
pub struct JwksDocumentHolder {
    inner: SyncHolder<Bytes>,
}

impl JwksDocumentHolder {
    #[must_use]
    pub fn get(&self) -> Option<Bytes> {
        self.inner.get()
    }

    pub fn set(&self, value: Option<Bytes>) {
        self.inner.set(value);
    }

    #[must_use]
    pub fn subscribe(&self) -> SyncHolderSubscription<Bytes> {
        self.inner.subscribe()
    }
}
