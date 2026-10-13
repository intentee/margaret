use std::collections::BTreeSet;
use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use zeroize::Zeroizing;

use margaret::framework::authorization_server_client::target_audience::TargetAudience;
use margaret::framework::authorization_server_client::token_target::TokenTarget;
use margaret::framework::client_credentials::acquired_token::AcquiredToken;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::service;
use margaret::framework::macros::singleton;
use margaret::framework::oauth_vocabulary::subject_token_type::SubjectTokenType;
use margaret::framework::token_exchange_client::subject_token::SubjectToken;

use crate::margaret::oauth_clients::partner_client::ClientCredentials;
use crate::margaret::oauth_clients::partner_client::TokenExchange;

#[singleton]
#[service]
pub struct TokenRefresher {
    client_credentials: Arc<ClientCredentials>,
    token_exchange: Arc<TokenExchange>,
}

impl TokenRefresher {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        client_credentials: Arc<ClientCredentials>,
        token_exchange: Arc<TokenExchange>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            client_credentials,
            token_exchange,
        })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub async fn run(&self, cancellation_token: CancellationToken) -> anyhow::Result<()> {
        let target = TokenTarget {
            audience: TargetAudience::Audience("artifacts".to_string()),
            scopes: BTreeSet::new(),
        };

        if let AcquiredToken::Acquired(token) = self.client_credentials.access_token(&target).await
        {
            self.token_exchange
                .exchange(
                    &SubjectToken {
                        token: Zeroizing::new(token.secret().clone()),
                        token_type: SubjectTokenType::AccessToken,
                    },
                    &target,
                )
                .await;
        }

        cancellation_token.cancelled().await;

        Ok(())
    }
}
