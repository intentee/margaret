use std::sync::Arc;

use futures_util::SinkExt;
use tokio_tungstenite::tungstenite::Message;

use margaret_websocket_envelope::client_sent_frame::ClientSentFrame;
use margaret_websocket_envelope::credit_grant::CreditGrant;

use crate::client_sink::ClientSink;
use crate::exchange_signal::ExchangeSignal;
use crate::web_socket_client_error::WebSocketClientError;

/// One consumed response frame grants the peer exactly one frame of the window back.
const GRANTED_BY_ONE_FRAME: CreditGrant = CreditGrant::from_frames(1);

fn signal_frame(signal: ExchangeSignal) -> ClientSentFrame<()> {
    match signal {
        ExchangeSignal::Cancelled(id) => ClientSentFrame::Cancel { id },
        ExchangeSignal::Consumed(id) => ClientSentFrame::Credit {
            credit: GRANTED_BY_ONE_FRAME,
            id,
        },
    }
}

#[derive(Clone)]
pub(crate) struct OutboundFrames {
    pub(crate) sink: ClientSink,
    url: Arc<str>,
}

impl OutboundFrames {
    pub(crate) fn new(sink: ClientSink, url: Arc<str>) -> Self {
        Self { sink, url }
    }

    pub(crate) async fn answer_close(&self) {
        if let Err(error) = self.sink.lock().await.close().await {
            eprintln!("margaret_websocket_client: the closing handshake failed: {error}");
        }
    }

    pub(crate) async fn report_exchange_signal(&self, signal: ExchangeSignal) {
        if let Err(error) = self
            .send_serialization(serde_json::to_string(&signal_frame(signal)))
            .await
        {
            eprintln!(
                "margaret_websocket_client: an exchange signal never reached the peer: {error}"
            );
        }
    }

    pub(crate) async fn send_serialization(
        &self,
        serialized: Result<String, serde_json::Error>,
    ) -> Result<(), WebSocketClientError> {
        let text = serialized.map_err(|source| WebSocketClientError::SerializeFrame { source })?;

        self.sink
            .lock()
            .await
            .send(Message::text(text))
            .await
            .map_err(|source| WebSocketClientError::SendFrame {
                source,
                url: self.url.to_string(),
            })
    }
}
