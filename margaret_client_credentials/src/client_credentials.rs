use std::sync::Arc;

use moka::future::Cache;

use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
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

    pub async fn access_token(&self, target: &TokenTarget) -> AcquiredToken {
        self.tokens
            .get_with(target.clone(), async {
                CachedAcquisition::of(self.server.client_credentials(target).await)
            })
            .await
            .acquired
    }
}
