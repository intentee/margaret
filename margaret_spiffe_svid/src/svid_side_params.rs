use std::sync::Arc;

use rustls::crypto::CryptoProvider;

use crate::root_cert_store_holder::RootCertStoreHolder;
use crate::svid_certified_key_holder::SvidCertifiedKeyHolder;

pub struct SvidSideParams {
    pub crypto_provider: Arc<CryptoProvider>,
    pub root_cert_store_holder: RootCertStoreHolder,
    pub spiffe_trust_domain: String,
    pub svid_certified_key_holder: SvidCertifiedKeyHolder,
}
