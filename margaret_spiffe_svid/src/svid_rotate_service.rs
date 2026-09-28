use anyhow::Result;
use async_trait::async_trait;
use futures_util::stream::StreamExt;
use log::error;
use log::info;
use spiffe::WorkloadApiClient;
use spiffe::X509Context;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use trzcina::TickContext;
use trzcina::Ticker;

use crate::svid_rotate_loop::svid_rotate_loop;

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
        let x509_context_stream = StreamExt::map(x509_context_stream, |result| {
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
impl Ticker for SvidRotateService {
    async fn handle_tick(
        &mut self,
        cancellation_token: CancellationToken,
        _tick_context: TickContext,
    ) -> Result<()> {
        self.stream_from_agent(cancellation_token).await;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tokio::sync::broadcast;
    use tokio::time::sleep;
    use tokio_util::sync::CancellationToken;
    use trzcina::Service as _;
    use trzcina::Ticker as _;

    use super::SvidRotateService;

    #[tokio::test(start_paused = true)]
    async fn keeps_reconnecting_until_cancelled_when_agent_is_unreachable() {
        let (x509_context_tx, _x509_context_rx) = broadcast::channel(1);
        let cancellation_token = CancellationToken::new();
        let cancellation_token_for_service = cancellation_token.clone();

        let service = SvidRotateService {
            spire_agent_addr: "unix:///nonexistent/spire-agent.sock".to_string(),
            x509_context_tx,
        };
        let tick_interval = service.tick_interval();

        let service_task =
            tokio::spawn(
                async move { Box::new(service).run(cancellation_token_for_service).await },
            );

        sleep(tick_interval * 3).await;

        assert!(!service_task.is_finished());

        cancellation_token.cancel();
        service_task.await.unwrap().unwrap();
    }
}
