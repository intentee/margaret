use rustls::server::VerifierBuilderError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SvidError {
    #[error("the rustls client configuration could not be built from the svid: {source}")]
    ClientConfig {
        #[source]
        source: rustls::Error,
    },

    #[error("the svid client certificate verifier could not be built: {source}")]
    ClientVerifier {
        #[source]
        source: VerifierBuilderError,
    },

    #[error("the default rustls crypto provider is not installed")]
    CryptoProviderNotInstalled,

    #[error("the x509 context carries no ca bundle for trust domain '{trust_domain}'")]
    MissingCaBundle { trust_domain: String },

    #[error("the x509 context carries no default svid")]
    MissingDefaultSvid,

    #[error("a ca cert could not be added to the root certificate store: {source}")]
    RootStoreRejectedCaCert {
        #[source]
        source: rustls::Error,
    },

    #[error("the svid server certificate verifier could not be built: {source}")]
    ServerVerifier {
        #[source]
        source: VerifierBuilderError,
    },

    #[error("the svid private key was rejected as a signing key: {source}")]
    SigningKeyRejected {
        #[source]
        source: rustls::Error,
    },

    #[error("the svid private key is not in a supported format: {reason}")]
    UnsupportedPrivateKey { reason: &'static str },
}
