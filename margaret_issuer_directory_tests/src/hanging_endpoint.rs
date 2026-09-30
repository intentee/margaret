use std::future::pending;

use async_trait::async_trait;
use url::Url;

use margaret_jwks_endpoint::provides_endpoint::ProvidesEndpoint;

pub struct HangingEndpoint;

#[async_trait]
impl ProvidesEndpoint for HangingEndpoint {
    async fn provide(&self) -> anyhow::Result<Url> {
        pending().await
    }
}
