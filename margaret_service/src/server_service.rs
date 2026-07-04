use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use tokio_util::sync::CancellationToken;
use trzcina::Service;

use margaret_http::bound_server::BoundServer;
use margaret_http::servers::Servers;

pub struct ServerService {
    name: Arc<str>,
    servers: Arc<Servers>,
}

impl ServerService {
    pub fn new(servers: Arc<Servers>, name: impl Into<Arc<str>>) -> Self {
        Self {
            name: name.into(),
            servers,
        }
    }
}

#[async_trait]
impl Service for ServerService {
    async fn run(self: Box<Self>, cancellation_token: CancellationToken) -> Result<()> {
        let Self { name, servers } = *self;
        let bound = BoundServer::bind(servers, name).await?;

        bound.serve(cancellation_token).await;

        Ok(())
    }
}
