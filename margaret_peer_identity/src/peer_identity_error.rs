use thiserror::Error;
use x509_parser::error::X509Error;

#[derive(Debug, Error)]
pub enum PeerIdentityError {
    #[error("the peer certificate DER could not be decoded: {source}")]
    CertificateEncoding {
        #[source]
        source: X509Error,
    },

    #[error("the peer certificate has no URI subject alternative name")]
    MissingUri,

    #[error("the peer certificate has more than one URI subject alternative name (found {count})")]
    MultipleUris { count: usize },

    #[error("the peer certificate URI is not a valid SPIFFE ID: {source}")]
    SpiffeId {
        #[source]
        source: spiffe::spiffe_id::SpiffeIdError,
    },
}
