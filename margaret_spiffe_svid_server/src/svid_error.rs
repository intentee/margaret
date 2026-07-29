use rustls::server::VerifierBuilderError;
use spiffe::spiffe_id::SpiffeIdError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SvidError {
    #[error("the svid client certificate verifier could not be built: {source}")]
    ClientVerifier {
        #[source]
        source: VerifierBuilderError,
    },

    #[error("the configured spiffe trust domain is not a valid trust domain: {source}")]
    TrustDomain {
        #[source]
        source: SpiffeIdError,
    },
}
