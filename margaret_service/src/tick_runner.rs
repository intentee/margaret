use anyhow::Result;
use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

#[async_trait]
pub trait TickRunner: Send + Sync + 'static {
    async fn tick(&self, cancellation_token: CancellationToken) -> Result<()>;
}
