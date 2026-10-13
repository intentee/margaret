use thiserror::Error;

use margaret_accepted_clients::accepted_clients_error::AcceptedClientsError;
use margaret_authorization_grants::authorization_grants_error::AuthorizationGrantsError;
use margaret_claims_merge::claims_merge_error::ClaimsMergeError;
use margaret_jwks_secret_store::jwks_secret_store_error::JwksSecretStoreError;
use margaret_subject_token_exchange::subject_token_exchange_error::SubjectTokenExchangeError;

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("the application could not keep its authorization grants: {0}")]
    AuthorizationGrants(#[source] AuthorizationGrantsError),

    #[error("the provider could not authenticate the client: {0}")]
    ClientAuthentication(#[source] AcceptedClientsError),

    #[error("the provider could not sign a token: {0}")]
    Signing(#[source] JwksSecretStoreError),

    #[error("the provider could not exchange the subject token: {0}")]
    SubjectTokenExchange(#[source] SubjectTokenExchangeError),

    #[error(
        "the subject token exchanger granted the scope '{scope}', which the client '{client_id}' does not declare in its token_exchange scopes"
    )]
    UndeclaredExchangeScope {
        client_id: &'static str,
        scope: String,
    },

    #[error("the userinfo claims could not be merged: {0}")]
    UserinfoClaims(#[source] ClaimsMergeError),

    #[error("the userinfo claims provider failed: {0:#}")]
    UserinfoClaimsProvider(#[source] anyhow::Error),
}
