use std::sync::Arc;

use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_authorization_server_client::token_target::TokenTarget;

use crate::acquired_token::AcquiredToken;
use crate::client_credentials::ClientCredentials;

pub struct ResourceCredentials {
    credentials: ClientCredentials,
    target: TokenTarget,
}

impl ResourceCredentials {
    #[must_use]
    pub fn create(server: Arc<AuthorizationServerClient>, target: TokenTarget) -> Self {
        Self {
            credentials: ClientCredentials::create(server),
            target,
        }
    }

    pub async fn access_token(&self) -> AcquiredToken {
        self.credentials.access_token(&self.target).await
    }
}
