use anyhow::Result;
use async_trait::async_trait;
use tokio_util::sync::CancellationToken;
use trzcina::Service;

use margaret_http::server::Server;

pub struct ServerService {
    address: String,
    server: Server,
}

impl ServerService {
    pub fn new(server: Server, address: String) -> Self {
        Self { address, server }
    }
}

#[async_trait]
impl Service for ServerService {
    async fn run(self: Box<Self>, cancellation_token: CancellationToken) -> Result<()> {
        let Self { address, server } = *self;
        let bound = server.bind(&address).await?;

        bound.serve(cancellation_token).await;

        Ok(())
    }
}
