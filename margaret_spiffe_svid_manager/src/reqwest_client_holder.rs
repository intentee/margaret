use reqwest::Client;

use margaret_sync_holder::sync_holder::SyncHolder;
use margaret_sync_holder::sync_holder_subscription::SyncHolderSubscription;

#[derive(Clone, Default)]
pub struct ReqwestClientHolder {
    inner: SyncHolder<Client>,
}

impl ReqwestClientHolder {
    #[must_use]
    pub fn get(&self) -> Option<Client> {
        self.inner.get()
    }

    pub fn set(&self, value: Option<Client>) {
        self.inner.set(value);
    }

    #[must_use]
    pub fn subscribe(&self) -> SyncHolderSubscription<Client> {
        self.inner.subscribe()
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Client;

    use super::ReqwestClientHolder;

    #[test]
    fn returns_client_after_set() {
        let holder = ReqwestClientHolder::default();
        let client = Client::new();

        holder.set(Some(client));

        assert!(holder.get().is_some());
    }

    #[test]
    fn returns_none_before_set() {
        let holder = ReqwestClientHolder::default();

        assert!(holder.get().is_none());
    }
}
