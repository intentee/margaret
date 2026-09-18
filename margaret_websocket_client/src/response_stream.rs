use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use tokio::sync::mpsc::Receiver;

use margaret_websocket_envelope::request_id::RequestId;
use margaret_websocket_envelope::server_sent_frame::ServerSentFrame;
use margaret_websocket_envelope::web_socket_response_message::WebSocketResponseMessage;

use crate::pending_responses::PendingResponses;
use crate::response_item::ResponseItem;
use crate::web_socket_client_error::WebSocketClientError;

pub struct ResponseStream<Payload> {
    id: RequestId,
    payload: PhantomData<Payload>,
    pending: PendingResponses,
    receiver: Receiver<ServerSentFrame>,
}

impl<Payload> ResponseStream<Payload>
where
    Payload: DeserializeOwned + WebSocketResponseMessage,
{
    pub(crate) fn new(
        id: RequestId,
        pending: PendingResponses,
        receiver: Receiver<ServerSentFrame>,
    ) -> Self {
        Self {
            id,
            payload: PhantomData,
            pending,
            receiver,
        }
    }

    /// # Errors
    ///
    /// Returns `WebSocketClientError` when a frame cannot be read as `Payload`.
    pub async fn next(&mut self) -> Option<Result<ResponseItem<Payload>, WebSocketClientError>> {
        self.receiver.recv().await.map(|frame| match frame {
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
        })
    }
}

impl<Payload> Drop for ResponseStream<Payload> {
    fn drop(&mut self) {
        self.pending.forget(&self.id);
    }
}
