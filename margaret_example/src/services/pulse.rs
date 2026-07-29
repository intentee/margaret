use tokio_util::sync::CancellationToken;

use margaret::framework::macros::process;
use margaret::framework::macros::service;

#[service]
pub struct Pulse;

impl Pulse {
    #[process]
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub async fn run(&self, cancellation_token: CancellationToken) -> anyhow::Result<()> {
        cancellation_token.cancelled().await;

        Ok(())
    }
}
