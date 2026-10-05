use bytes::Bytes;

use margaret_sync_holder::sync_holder::SyncHolder;
use margaret_sync_holder::sync_holder_subscription::SyncHolderSubscription;

#[derive(Clone)]
pub struct JwksDocumentHolder {
    inner: SyncHolder<Bytes>,
}

impl JwksDocumentHolder {
    #[must_use]
    pub fn new(document: Bytes) -> Self {
        Self {
            inner: SyncHolder::new(document),
        }
    }

    #[must_use]
    pub fn get(&self) -> Bytes {
        self.inner.get()
    }

    pub fn set(&self, document: Bytes) {
        self.inner.set(document);
    }

    #[must_use]
    pub fn subscribe(&self) -> SyncHolderSubscription<Bytes> {
        self.inner.subscribe()
    }
}
