use std::sync::Arc;

use rustls::crypto::CryptoProvider;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SvidError {
    #[error(
        "a rustls crypto provider is already installed; Margaret installs the process-wide provider itself"
    )]
    CryptoProviderAlreadyInstalled { installed: Arc<CryptoProvider> },

    #[error("the x509 context carries no ca bundle for trust domain '{trust_domain}'")]
    MissingCaBundle { trust_domain: String },

    #[error("the x509 context carries no default svid")]
    MissingDefaultSvid,

    #[error("a ca cert could not be added to the root certificate store: {source}")]
    RootStoreRejectedCaCert {
        #[source]
        source: rustls::Error,
    },

    #[error("the svid private key was rejected as a signing key: {source}")]
    SigningKeyRejected {
        #[source]
        source: rustls::Error,
    },

    #[error("the svid private key is not in a supported format: {reason}")]
    UnsupportedPrivateKey { reason: &'static str },
}
