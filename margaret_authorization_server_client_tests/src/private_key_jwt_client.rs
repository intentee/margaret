use std::sync::Arc;

use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_oauth_client::client_authentication::ClientAuthentication;
use margaret_oauth_client::oauth_client::OAuthClient;

/// # Panics
///
/// Panics when the fixture client identifier is rejected.
#[must_use]
pub fn private_key_jwt_client(signer: JwksSecretStore) -> OAuthClient {
    OAuthClient {
        authentication: ClientAuthentication::PrivateKeyJwt(Arc::new(signer)),
        client_id: "client:id"
            .parse()
            .expect("the fixture client identifier is visible"),
    }
}
