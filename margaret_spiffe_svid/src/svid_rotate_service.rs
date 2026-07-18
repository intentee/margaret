use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use log::error;
use log::info;
use spiffe::WorkloadApiClient;
use spiffe::X509Context;
use tokio::sync::broadcast;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use trzcina::Service;

use crate::svid_rotate_loop::svid_rotate_loop;

const SPIRE_AGENT_RECONNECT_INTERVAL: Duration = Duration::from_secs(1);

pub struct SvidRotateService {
    pub spire_agent_addr: String,
    pub x509_context_tx: broadcast::Sender<X509Context>,
}

impl SvidRotateService {
    pub async fn stream_from_agent(&self, cancellation_token: CancellationToken) {
        if let Err(err) = self.try_stream_from_agent(cancellation_token).await {
            error!("Unable to stream SVID contexts from SPIRE agent: {err:#?}");
        }
    }

    async fn try_stream_from_agent(&self, cancellation_token: CancellationToken) -> Result<()> {
        let mut client = WorkloadApiClient::new_from_path(&self.spire_agent_addr).await?;

        info!("Connected to SPIRE agent");

        let x509_context_stream = client.stream_x509_contexts().await?;
        let x509_context_stream =
            futures_util::stream::StreamExt::map(x509_context_stream, |result| {
                result.map_err(anyhow::Error::from)
            });

        svid_rotate_loop(
            Box::pin(x509_context_stream),
            &self.x509_context_tx,
            cancellation_token,
        )
        .await;

        Ok(())
    }
}

#[async_trait]
impl Service for SvidRotateService {
    async fn run(self: Box<Self>, cancellation_token: CancellationToken) -> Result<()> {
        loop {
            tokio::select! {
                () = cancellation_token.cancelled() => return Ok(()),
                () = self.stream_from_agent(cancellation_token.clone()) => {
                    tokio::select! {
                        () = cancellation_token.cancelled() => return Ok(()),
                        () = sleep(SPIRE_AGENT_RECONNECT_INTERVAL) => continue,
                    }
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::sync::broadcast;
    use tokio_util::sync::CancellationToken;

    use super::SvidRotateService;

    #[tokio::test]
    async fn stream_from_agent_returns_when_socket_path_invalid() {
        let (x509_context_tx, _x509_context_rx) = broadcast::channel(1);
        let service = SvidRotateService {
            spire_agent_addr: "unix:///nonexistent/spire-agent.sock".to_string(),
            x509_context_tx,
        };

        service.stream_from_agent(CancellationToken::new()).await;
    }
}
