use async_trait::async_trait;
use url::Url;

#[async_trait]
pub trait ProvidesEndpoint: Send + Sync {
    async fn provide(&self) -> anyhow::Result<Url>;
}
