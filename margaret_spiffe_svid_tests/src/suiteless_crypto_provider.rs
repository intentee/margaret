use std::sync::Arc;

use rustls::crypto::CryptoProvider;

use margaret_spiffe_svid::svid_crypto_provider::svid_crypto_provider;

#[must_use]
pub fn suiteless_crypto_provider() -> Arc<CryptoProvider> {
    Arc::new(CryptoProvider {
        cipher_suites: Vec::new(),
        ..CryptoProvider::clone(&svid_crypto_provider())
    })
}
