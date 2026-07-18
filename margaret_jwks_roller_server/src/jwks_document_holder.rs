use bytes::Bytes;

use margaret_sync_holder::sync_holder::SyncHolder;

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
}
