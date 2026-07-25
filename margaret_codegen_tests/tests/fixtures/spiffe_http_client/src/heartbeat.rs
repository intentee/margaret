use std::convert::Infallible;
use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::service;

use crate::identity_client::IdentityClient;
use crate::margaret::asset_bag::asset;

#[service]
pub struct Heartbeat {
    identity_client: Arc<IdentityClient>,
}

impl Heartbeat {
    #[constructor]
    #[must_use]
    pub fn create(identity_client: Arc<IdentityClient>) -> Self {
        Self { identity_client }
    }

    #[process]
    pub async fn run(&self, cancellation_token: CancellationToken) -> Result<(), Infallible> {
        let _asset = asset!("resources/ts/app.ts");
        let _http_client = self.identity_client.http_client();

        cancellation_token.cancelled().await;

        Ok(())
    }
}
