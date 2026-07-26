use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::service;
use tokio_util::sync::CancellationToken;

use super::secrets::Secrets;

#[service]
pub struct Worker {
    secrets: Arc<Secrets>,
}

impl Worker {
    #[constructor]
    pub fn create(secrets: Arc<Secrets>) -> anyhow::Result<Self> {
        Ok(Self { secrets })
    }

    #[process]
    pub async fn run(&self, cancellation_token: CancellationToken) -> anyhow::Result<()> {
        let _ = self.secrets.token();

        cancellation_token.cancelled().await;

        Ok(())
    }
}
