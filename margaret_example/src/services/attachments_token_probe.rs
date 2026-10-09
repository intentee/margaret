use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use margaret::framework::client_credentials::acquired_token::AcquiredToken;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::service;

use crate::margaret::oauth_clients::blog::resources::attachments::ResourceCredentials;

#[service]
pub struct AttachmentsTokenProbe {
    credentials: Arc<ResourceCredentials>,
}

impl AttachmentsTokenProbe {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(credentials: Arc<ResourceCredentials>) -> anyhow::Result<Self> {
        Ok(Self { credentials })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub async fn run(&self, cancellation_token: CancellationToken) -> anyhow::Result<()> {
        match self.credentials.access_token().await {
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
