use thiserror::Error;
use url::Url;

use margaret_jwks_secret_store::jwks_secret_store_error::JwksSecretStoreError;
use margaret_provider_state_storage::provider_state_error::ProviderStateError;

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
        "the issuer at '{issuer_origin}' is not served by the provider server at '{server_origin}'"
    )]
    IssuerNotServed {
        issuer_origin: String,
        server_origin: String,
    },

    #[error("the url '{server_url}' of the provider server is not a url: {source}")]
    ServerUrlMalformed {
        server_url: String,
        #[source]
        source: url::ParseError,
    },

    #[error("the provider could not sign a token: {0}")]
    Signing(#[source] JwksSecretStoreError),

    #[error("the provider could not reach its state: {0}")]
    State(#[source] ProviderStateError),

    #[error("the userinfo claims could not be serialized to json: {0}")]
    UserinfoClaimsSerialization(#[source] serde_json::Error),
}
