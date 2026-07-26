use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::service;

use crate::identity_client::IdentityClient;
use crate::margaret::asset_bag::asset;

#[service]
pub struct Heartbeat {
    identity_client: Arc<IdentityClient>,
}

impl Heartbeat {
    #[constructor]
    pub fn create(identity_client: Arc<IdentityClient>) -> anyhow::Result<Self> {
        Ok(Self { identity_client })
    }

    #[process]
    pub async fn run(&self, cancellation_token: CancellationToken) -> anyhow::Result<()> {
        let _asset = asset!("resources/ts/app.ts");
        let _http_client = self.identity_client.http_client();

        cancellation_token.cancelled().await;

        Ok(())
    }
}
