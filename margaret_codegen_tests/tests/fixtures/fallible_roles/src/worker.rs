use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::service;

use super::secrets::Secrets;

type WorkerResult<Value> = failures::Result<Value>;
type ChainedWorkerResult<Value> = WorkerResult<Value>;

#[service]
pub struct Worker {
    secrets: Arc<Secrets>,
}

impl Worker {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(secrets: Arc<Secrets>) -> WorkerResult<Self> {
        Ok(Self { secrets })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub async fn run(&self, cancellation_token: CancellationToken) -> ChainedWorkerResult<()> {
        let _ = self.secrets.token();

        cancellation_token.cancelled().await;

        Ok(())
    }
}
