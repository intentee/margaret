use thiserror::Error;

use margaret_jwks_secret_store::jwks_secret_store_error::JwksSecretStoreError;
use margaret_provider_state_storage::provider_state_error::ProviderStateError;
use margaret_registered_claims::claims_merge_error::ClaimsMergeError;
use margaret_subject_token_exchange::subject_token_exchange_error::SubjectTokenExchangeError;

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error(
        "the issuer at '{issuer_origin}' is not served by the provider server at '{server_origin}'"
    )]
    IssuerNotServed {
        issuer_origin: &'static str,
        server_origin: String,
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
