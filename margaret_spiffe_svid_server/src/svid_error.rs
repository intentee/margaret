use rustls::server::VerifierBuilderError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SvidError {
    #[error("the svid client certificate verifier could not be built: {source}")]
    ClientVerifier {
        #[source]
        source: VerifierBuilderError,
    },
}
