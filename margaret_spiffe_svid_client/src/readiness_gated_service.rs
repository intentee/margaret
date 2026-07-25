use anyhow::Result;
use async_trait::async_trait;
use tokio_util::sync::CancellationToken;
use trzcina::Service;

use margaret_sync_holder::sync_holder_presence::SyncHolderPresence;

use crate::svid_client_readiness::SvidClientReadiness;

pub struct ReadinessGatedService {
    inner: Box<dyn Service>,
    readiness: SvidClientReadiness,
}

impl ReadinessGatedService {
    #[must_use]
    pub fn new(readiness: SvidClientReadiness, inner: impl Service) -> Self {
        Self {
            inner: Box::new(inner),
            readiness,
        }
    }
}

#[async_trait]
impl Service for ReadinessGatedService {
    fn name(&self) -> &'static str {
        self.inner.name()
    }

    async fn run(self: Box<Self>, cancellation_token: CancellationToken) -> Result<()> {
        let Self {
            inner,
            mut readiness,
        } = *self;

        match readiness.wait_until_ready(&cancellation_token).await {
            SyncHolderPresence::Present => inner.run(cancellation_token).await,
            SyncHolderPresence::Cancelled => Ok(()),
        }
    }
}
