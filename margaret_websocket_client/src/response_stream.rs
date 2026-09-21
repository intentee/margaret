use std::marker::PhantomData;
use std::sync::Arc;

use serde::de::DeserializeOwned;

use margaret_websocket_envelope::server_sent_frame::ServerSentFrame;
use margaret_websocket_envelope::web_socket_response_message::WebSocketResponseMessage;

use crate::exchange_outcome::ExchangeOutcome;
use crate::open_exchange::OpenExchange;
use crate::response_item::ResponseItem;
use crate::response_stream_state::ResponseStreamState;
use crate::web_socket_client_error::WebSocketClientError;

fn read_frame<Payload>(
    frame: ServerSentFrame,
) -> Result<ResponseItem<Payload>, WebSocketClientError>
where
    Payload: DeserializeOwned + WebSocketResponseMessage,
{
    match frame {
        ServerSentFrame::Error { error, .. } => Ok(ResponseItem::Rejected(error)),
        ServerSentFrame::Response {
            method, payload, ..
        } => {
            if method == Payload::METHOD {
                serde_json::from_value(payload)
                    .map(ResponseItem::Payload)
                    .map_err(|source| WebSocketClientError::DeserializeResponse { source })
            } else {
                Err(WebSocketClientError::UnexpectedResponseMethod {
                    expected: Payload::METHOD,
                    received: method,
                })
            }
        }
    }
}

pub struct ResponseStream<Payload> {
    exchange: OpenExchange,
    payload: PhantomData<Payload>,
    state: ResponseStreamState,
    url: Arc<str>,
}

impl<Payload> ResponseStream<Payload>
where
    Payload: DeserializeOwned + WebSocketResponseMessage,
{
    pub(crate) fn new(exchange: OpenExchange, url: Arc<str>) -> Self {
        Self {
            exchange,
            payload: PhantomData,
            state: ResponseStreamState::Open,
            url,
        }
    }

    /// # Errors
    ///
    /// Returns `WebSocketClientError` when a frame cannot be read as `Payload`, or when the
    /// exchange ended before the peer completed it.
    pub async fn next(&mut self) -> Option<Result<ResponseItem<Payload>, WebSocketClientError>> {
        if self.state == ResponseStreamState::Ended {
            return None;
        }

        if let Some(frame) = self.exchange.receive().await {
            return Some(read_frame(frame));
        }

        self.state = ResponseStreamState::Ended;

        match self.exchange.outcome() {
            ExchangeOutcome::Completed => None,
            ExchangeOutcome::Interrupted(interruption) => {
                Some(Err(interruption.into_error(&self.url)))
            }
        }
    }
}
