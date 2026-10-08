use std::collections::BTreeSet;
use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use margaret::framework::authorization_server_client::target_audience::TargetAudience;
use margaret::framework::authorization_server_client::token_target::TokenTarget;
use margaret::framework::client_credentials::acquired_token::AcquiredToken;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::service;

use crate::margaret::oauth_clients::blog::ClientCredentials;
use crate::margaret::resource_tokens::attachments::AUDIENCE;

#[service]
pub struct AttachmentsTokenProbe {
    client_credentials: Arc<ClientCredentials>,
}

impl AttachmentsTokenProbe {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(client_credentials: Arc<ClientCredentials>) -> anyhow::Result<Self> {
        Ok(Self { client_credentials })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub async fn run(&self, cancellation_token: CancellationToken) -> anyhow::Result<()> {
        match self
            .client_credentials
            .access_token(&TokenTarget {
                audience: TargetAudience::Audience(AUDIENCE.to_string()),
                scopes: BTreeSet::new(),
            })
            .await
        {
            AcquiredToken::Acquired(_) => {
                println!("the blog acquired an access token for its attachments");
            }
            AcquiredToken::Refused(refusal) => {
                println!("the identity server refused an attachments token: {refusal}");
            }
            AcquiredToken::Unavailable(unavailability) => {
                println!("the identity server is unavailable: {unavailability}");
            }
        }

        cancellation_token.cancelled().await;

        Ok(())
    }
}
