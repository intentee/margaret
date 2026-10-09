use std::collections::BTreeSet;
use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use margaret::framework::authorization_server_client::target_audience::TargetAudience;
use margaret::framework::authorization_server_client::token_target::TokenTarget;
use margaret::framework::client_credentials::acquired_token::AcquiredToken;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::service;

use crate::margaret::oauth_clients::cluster::ClientCredentials;
use crate::margaret::resource_tokens::notes::AUDIENCE;
use crate::models::token_acquisition_outcome::TokenAcquisitionOutcome;
use crate::stores::token_acquisition_store::TokenAcquisitionStore;

#[service]
pub struct TokenAcquirer {
    acquisitions: Arc<TokenAcquisitionStore>,
    client_credentials: Arc<ClientCredentials>,
}

impl TokenAcquirer {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        acquisitions: Arc<TokenAcquisitionStore>,
        client_credentials: Arc<ClientCredentials>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            acquisitions,
            client_credentials,
        })
    }

    /// # Errors
    ///
    /// Returns an error when the acquisition cannot be recorded.
    #[process]
    pub async fn run(&self, cancellation_token: CancellationToken) -> anyhow::Result<()> {
        let outcome = match self
            .client_credentials
            .access_token(&TokenTarget {
                audience: TargetAudience::Audience(AUDIENCE.to_string()),
                scopes: BTreeSet::new(),
            })
            .await
        {
            AcquiredToken::Acquired(_) => TokenAcquisitionOutcome::Acquired,
            AcquiredToken::Refused(_) => TokenAcquisitionOutcome::Refused,
            AcquiredToken::Unavailable(_) => TokenAcquisitionOutcome::Unavailable,
        };

        self.acquisitions.record(&outcome).await?;
        cancellation_token.cancelled().await;

        Ok(())
    }
}
