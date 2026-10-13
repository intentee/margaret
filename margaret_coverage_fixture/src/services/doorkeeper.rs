use tokio_util::sync::CancellationToken;

use margaret::framework::macros::process;
use margaret::framework::macros::service;

#[service]
pub struct Doorkeeper;

impl Doorkeeper {
    /// # Errors
    ///
    /// Never returns an error; the doors stay open until the library is stopped.
    #[process]
    pub async fn run(&self, cancellation_token: CancellationToken) -> anyhow::Result<()> {
        cancellation_token.cancelled().await;

        Ok(())
    }
}
