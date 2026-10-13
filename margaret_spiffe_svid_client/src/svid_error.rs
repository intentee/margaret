use rustls::server::VerifierBuilderError;
use spiffe::spiffe_id::SpiffeIdError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SvidError {
    #[error("the svid crypto provider supports none of the safe default tls versions: {source}")]
    ProtocolVersions {
        #[source]
        source: rustls::Error,
    },

    #[error("the reqwest client could not be built from the svid client configuration: {source}")]
    ReqwestClient {
        #[source]
        source: reqwest::Error,
    },

    #[error("the svid server certificate verifier could not be built: {source}")]
    ServerVerifier {
        #[source]
        source: VerifierBuilderError,
    },

    #[error("the configured spiffe trust domain is not a valid trust domain: {source}")]
    TrustDomain {
        #[source]
        source: SpiffeIdError,
    },
}
