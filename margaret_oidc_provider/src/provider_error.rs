use thiserror::Error;

use margaret_accepted_clients::accepted_clients_error::AcceptedClientsError;
use margaret_jwks_secret_store::jwks_secret_store_error::JwksSecretStoreError;
use margaret_registered_claims::claims_merge_error::ClaimsMergeError;
use margaret_subject_token_exchange::subject_token_exchange_error::SubjectTokenExchangeError;

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("the provider could not authenticate the client: {0}")]
    ClientAuthentication(#[source] AcceptedClientsError),

    #[error("the application could not look up a refresh token: {0}")]
    FindRefreshToken(#[source] anyhow::Error),

    #[error("the application could not hold a pending authorization: {0}")]
    HoldPendingAuthorization(#[source] anyhow::Error),

    #[error("the application could not issue an authorization code: {0}")]
    IssueCode(#[source] anyhow::Error),

    #[error("the application could not open a refresh token family: {0}")]
    OpenRefreshFamily(#[source] anyhow::Error),

    #[error("the application could not redeem an authorization code: {0}")]
    RedeemCode(#[source] anyhow::Error),

    #[error("the application could not revoke a refresh token family: {0}")]
    RevokeRefreshFamily(#[source] anyhow::Error),

    #[error("the application could not rotate a refresh token: {0}")]
    RotateRefreshToken(#[source] anyhow::Error),

    #[error("the provider could not sign a token: {0}")]
    Signing(#[source] JwksSecretStoreError),

    #[error("the provider could not exchange the subject token: {0}")]
    SubjectTokenExchange(#[source] SubjectTokenExchangeError),

    #[error("the application could not take a pending authorization: {0}")]
    TakePendingAuthorization(#[source] anyhow::Error),

    #[error(
        "the subject token exchanger granted the scope '{scope}', which the client '{client_id}' does not declare in its token_exchange scopes"
    )]
    UndeclaredExchangeScope {
        client_id: &'static str,
        scope: String,
    },

    #[error("the userinfo claims could not be merged: {0}")]
    UserinfoClaims(#[source] ClaimsMergeError),
}
