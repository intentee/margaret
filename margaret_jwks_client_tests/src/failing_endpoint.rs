use async_trait::async_trait;
use url::Url;

use margaret_jwks_endpoint::provides_endpoint::ProvidesEndpoint;

pub struct FailingEndpoint;

#[async_trait]
impl ProvidesEndpoint for FailingEndpoint {
    async fn provide(&self) -> anyhow::Result<Url> {
        Err(anyhow::anyhow!(
            "the issuer endpoint could not be discovered"
        ))
    }
}
