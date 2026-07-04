use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use tokio_util::sync::CancellationToken;
use trzcina::Service;

use margaret_http::bound_server::BoundServer;
use margaret_http::forward_targets::ForwardTargets;
use margaret_http::server_registry::ServerRegistry;

pub struct ServerService {
    forward_targets: Arc<ForwardTargets>,
    name: Arc<str>,
    server_registry: Arc<ServerRegistry>,
}

impl ServerService {
    pub fn new(
        server_registry: Arc<ServerRegistry>,
        forward_targets: Arc<ForwardTargets>,
        name: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            forward_targets,
            name: name.into(),
            server_registry,
        }
    }
}

#[async_trait]
impl Service for ServerService {
    async fn run(self: Box<Self>, cancellation_token: CancellationToken) -> Result<()> {
        let Self {
            forward_targets,
            name,
            server_registry,
        } = *self;
        let bound = BoundServer::bind(server_registry, forward_targets, name).await?;

        bound.serve(cancellation_token).await;

        Ok(())
    }
}
