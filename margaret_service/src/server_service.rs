use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use tokio_util::sync::CancellationToken;
use trzcina::Service;

use margaret_http::bound_server::BoundServer;
use margaret_http::forward_targets::ForwardTargets;
use margaret_http::server::Server;

pub struct ServerService {
    forward_targets: Arc<ForwardTargets>,
    server: Arc<Server>,
}

impl ServerService {
    #[must_use]
    pub fn new(server: Arc<Server>, forward_targets: Arc<ForwardTargets>) -> Self {
        Self {
            forward_targets,
            server,
        }
    }
}

#[async_trait]
impl Service for ServerService {
    async fn run(self: Box<Self>, cancellation_token: CancellationToken) -> Result<()> {
        let Self {
            forward_targets,
            server,
        } = *self;
        let bound = BoundServer::bind(server, forward_targets).await?;

        bound.serve(cancellation_token).await;

        Ok(())
    }
}
