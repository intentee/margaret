use std::convert::Infallible;

use tokio_util::sync::CancellationToken;

use margaret::framework::macros::process;
use margaret::framework::macros::service;

#[service]
pub struct Pulse;

impl Pulse {
    #[process]
    pub async fn run(&self, cancellation_token: CancellationToken) -> Result<(), Infallible> {
        cancellation_token.cancelled().await;

        Ok(())
    }
}
