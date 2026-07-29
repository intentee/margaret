use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::service;
use tokio_util::sync::CancellationToken;

use super::secrets::Secrets;

type WorkerResult<T> = failures::Result<T>;
type ChainedWorkerResult<T> = WorkerResult<T>;

#[service]
pub struct Worker {
    secrets: Arc<Secrets>,
}

impl Worker {
    #[constructor]
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub fn create(secrets: Arc<Secrets>) -> WorkerResult<Self> {
        Ok(Self { secrets })
    }

    #[process]
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub async fn run(&self, cancellation_token: CancellationToken) -> ChainedWorkerResult<()> {
        let _ = self.secrets.token();

        cancellation_token.cancelled().await;

        Ok(())
    }
}
