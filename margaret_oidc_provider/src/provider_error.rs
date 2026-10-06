use thiserror::Error;
use url::Origin;
use url::Url;

use margaret_jwks_secret_store::jwks_secret_store_error::JwksSecretStoreError;
use margaret_provider_state_storage::provider_state_error::ProviderStateError;
use margaret_registered_claims::claims_merge_error::ClaimsMergeError;
use margaret_subject_token_exchange::subject_token_exchange_error::SubjectTokenExchangeError;

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error(
        "the discovery route '{discovery_path}' is not served at the discovery location '{expected_location}' of the issuer"
    )]
    DiscoveryPathMismatch {
        discovery_path: &'static str,
        expected_location: Box<Url>,
    },

    #[error(
        "the issuer at '{}' is not served by the provider server at '{}'",
        .issuer_origin.ascii_serialization(),
        .server_origin.ascii_serialization()
    )]
    IssuerNotServed {
        issuer_origin: Origin,
        server_origin: Origin,
    },

    #[error("the provider could not sign a token: {0}")]
    Signing(#[source] JwksSecretStoreError),

    #[error("the provider could not reach its state: {0}")]
    State(#[source] ProviderStateError),

    #[error("the provider could not exchange the subject token: {0}")]
    SubjectTokenExchange(#[source] SubjectTokenExchangeError),

    #[error("the userinfo claims could not be merged: {0}")]
    UserinfoClaims(#[source] ClaimsMergeError),
}
