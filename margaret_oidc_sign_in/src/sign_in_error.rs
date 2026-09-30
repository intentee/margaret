use thiserror::Error;

use margaret_authorization_server_client::authorization_server_client_error::AuthorizationServerClientError;
use margaret_jwks_secret_store::jwks_secret_store_error::JwksSecretStoreError;

#[derive(Debug, Error)]
pub enum SignInError {
    #[error("the client cannot authenticate to the authorization server: {0}")]
    ClientAuthentication(#[source] AuthorizationServerClientError),

    #[error("the sign-in transaction cannot be signed or verified: {0}")]
    TransactionSecret(#[source] JwksSecretStoreError),
}
