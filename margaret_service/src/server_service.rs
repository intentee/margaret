use anyhow::Result;
use async_trait::async_trait;
use tokio_util::sync::CancellationToken;
use trzcina::Service;

use margaret_http::server::Server;
use margaret_http::upload_config::UploadConfig;

pub struct ServerService {
    address: String,
    server: Server,
    upload_config: UploadConfig,
}

impl ServerService {
    pub fn new(server: Server, address: String, upload_config: UploadConfig) -> Self {
        Self {
            address,
            server,
            upload_config,
        }
    }
}

#[async_trait]
impl Service for ServerService {
    async fn run(self: Box<Self>, cancellation_token: CancellationToken) -> Result<()> {
        let Self {
            address,
            server,
            upload_config,
        } = *self;
        let bound = server.bind(&address, upload_config).await?;

        bound.serve(cancellation_token).await;

        Ok(())
    }
}
