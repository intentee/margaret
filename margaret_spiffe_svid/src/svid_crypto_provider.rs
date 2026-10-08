use std::sync::Arc;

use rustls::crypto::CryptoProvider;
use rustls::crypto::aws_lc_rs::default_provider;

#[must_use]
pub fn svid_crypto_provider() -> Arc<CryptoProvider> {
    Arc::new(default_provider())
}
