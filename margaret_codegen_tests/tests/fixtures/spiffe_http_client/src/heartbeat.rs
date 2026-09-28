use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::service;

use crate::identity_client::IdentityClient;

#[service]
pub struct Heartbeat {
    identity_client: Arc<IdentityClient>,
}

impl Heartbeat {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(identity_client: Arc<IdentityClient>) -> anyhow::Result<Self> {
        Ok(Self { identity_client })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub async fn run(&self, cancellation_token: CancellationToken) -> anyhow::Result<()> {
        let _http_client = self.identity_client.http_client();

        cancellation_token.cancelled().await;

        Ok(())
    }
}
