use std::sync::Arc;

use rustls::SignatureScheme;
use rustls::client::ResolvesClientCert;
use rustls::server::ClientHello;
use rustls::server::ResolvesServerCert;
use rustls::sign::CertifiedKey;

use margaret_sync_holder::sync_holder::SyncHolder;
use margaret_sync_holder::sync_holder_subscription::SyncHolderSubscription;

use crate::svid_certified_key::SvidCertifiedKey;

#[derive(Clone, Debug, Default)]
pub struct SvidCertifiedKeyHolder {
    inner: SyncHolder<Arc<SvidCertifiedKey>>,
}

impl SvidCertifiedKeyHolder {
    #[must_use]
    pub fn get(&self) -> Option<Arc<SvidCertifiedKey>> {
        self.inner.get()
    }

    pub fn set(&self, value: Option<Arc<SvidCertifiedKey>>) {
        self.inner.set(value);
    }

    #[must_use]
    pub fn subscribe(&self) -> SyncHolderSubscription<Arc<SvidCertifiedKey>> {
        self.inner.subscribe()
    }

    fn certified_key(&self) -> Option<Arc<CertifiedKey>> {
        self.get()
            .map(|svid_certified_key| svid_certified_key.certified_key.clone())
    }
}

impl ResolvesClientCert for SvidCertifiedKeyHolder {
    fn has_certs(&self) -> bool {
        self.get().is_some()
    }

    fn resolve(
        &self,
        _root_hint_subjects: &[&[u8]],
        _sigschemes: &[SignatureScheme],
    ) -> Option<Arc<CertifiedKey>> {
        self.certified_key()
    }
}

impl ResolvesServerCert for SvidCertifiedKeyHolder {
    fn resolve(&self, _client_hello: ClientHello<'_>) -> Option<Arc<CertifiedKey>> {
        self.certified_key()
    }
}
