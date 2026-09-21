use std::marker::PhantomData;
use std::sync::Arc;

use serde::de::DeserializeOwned;
use serde_json::Value;

use margaret_websocket_envelope::server_sent_frame::ServerSentFrame;
use margaret_websocket_envelope::web_socket_response_message::WebSocketResponseMessage;

use crate::exchange_outcome::ExchangeOutcome;
use crate::open_exchange::OpenExchange;
use crate::response_item::ResponseItem;
use crate::response_stream_state::ResponseStreamState;
use crate::web_socket_client_error::WebSocketClientError;

fn read_response<Payload>(
    method: &str,
    payload: Value,
) -> Result<ResponseItem<Payload>, WebSocketClientError>
where
    Payload: DeserializeOwned + WebSocketResponseMessage,
{
    if method == Payload::METHOD {
        serde_json::from_value(payload)
            .map(ResponseItem::Payload)
            .map_err(|source| WebSocketClientError::DeserializeResponse { source })
    } else {
        Err(WebSocketClientError::UnexpectedResponseMethod {
            expected: Payload::METHOD,
            received: method.to_string(),
        })
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

        let Some(frame) = self.exchange.receive().await else {
            self.state = ResponseStreamState::Ended;

            return match self.exchange.outcome() {
                ExchangeOutcome::Completed => None,
                ExchangeOutcome::Interrupted(interruption) => {
                    Some(Err(interruption.into_error(&self.url)))
                }
            };
        };

        match frame {
            ServerSentFrame::Error { error, .. } => Some(Ok(ResponseItem::Rejected(error))),
            ServerSentFrame::Response {
                is_done,
                method,
                payload,
                ..
            } => {
                if !is_done {
                    self.exchange.report_consumed();
                }

                Some(read_response(&method, payload))
            }
        }
    }
}
