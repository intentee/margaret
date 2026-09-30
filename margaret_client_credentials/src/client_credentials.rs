use std::sync::Arc;

use moka::future::Cache;

use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_authorization_server_client::authorization_server_client_error::AuthorizationServerClientError;
use margaret_authorization_server_client::token_target::TokenTarget;

use crate::acquired_token::AcquiredToken;
use crate::cached_acquisition::CachedAcquisition;
use crate::token_expiry::TokenExpiry;

pub struct ClientCredentials {
    server: Arc<AuthorizationServerClient>,
    tokens: Cache<TokenTarget, CachedAcquisition>,
}

impl ClientCredentials {
    #[must_use]
    pub fn create(server: Arc<AuthorizationServerClient>) -> Self {
        Self {
            server,
            tokens: Cache::builder().expire_after(TokenExpiry).build(),
        }
    }

    /// # Errors
    ///
    /// Returns `AuthorizationServerClientError::AssertionSigning` when the client assertion of a
    /// `private_key_jwt` client cannot be signed.
    pub async fn access_token(
        &self,
        target: &TokenTarget,
    ) -> Result<AcquiredToken, Arc<AuthorizationServerClientError>> {
        self.tokens
            .try_get_with(target.clone(), async {
                self.server
                    .client_credentials(target)
                    .await
                    .map(CachedAcquisition::of)
            })
            .await
            .map(|cached| cached.acquired)
    }
}
