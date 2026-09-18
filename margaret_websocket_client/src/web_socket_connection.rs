use std::sync::atomic::AtomicI64;
use std::sync::atomic::Ordering;

use futures_util::SinkExt;
use futures_util::StreamExt;
use futures_util::stream::SplitSink;
use futures_util::stream::SplitStream;
use serde::Serialize;
use serde::de::DeserializeOwned;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tokio_rustls::client::TlsStream;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_util::sync::CancellationToken;

use margaret_websocket_envelope::client_sent_frame::ClientSentFrame;
use margaret_websocket_envelope::request_id::RequestId;
use margaret_websocket_envelope::server_sent_frame::ServerSentFrame;
use margaret_websocket_envelope::web_socket_notification_message::WebSocketNotificationMessage;
use margaret_websocket_envelope::web_socket_request_message::WebSocketRequestMessage;
use margaret_websocket_envelope::web_socket_response_message::WebSocketResponseMessage;

use crate::pending_responses::PendingResponses;
use crate::response_stream::ResponseStream;
use crate::route_server_frame::route_server_frame;
use crate::web_socket_client_error::WebSocketClientError;

type ClientStream = WebSocketStream<TlsStream<TcpStream>>;

/// One in-flight frame per exchange, matching the outbound buffer the server serves with.
const RESPONSE_BUFFER_CAPACITY: usize = 1;

async fn read_server_frames(
    cancellation_token: CancellationToken,
    mut source: SplitStream<ClientStream>,
    pending: PendingResponses,
) {
    loop {
        tokio::select! {
            biased;
            () = cancellation_token.cancelled() => break,
            message = source.next() => match message {
                Some(Ok(Message::Text(text))) => {
                    match serde_json::from_str::<ServerSentFrame>(&text) {
                        Ok(frame) => route_server_frame(frame, &pending).await,
                        Err(error) => {
                            eprintln!(
                                "margaret_websocket_client: the peer sent an unreadable frame: {error}"
                            );
                        }
                    }
                }
                Some(Ok(_)) => {}
                Some(Err(error)) => {
                    eprintln!("margaret_websocket_client: the connection failed: {error}");

                    break;
                }
                None => break,
            },
        }
    }
}

pub struct WebSocketConnection {
    cancellation_token: CancellationToken,
    next_request_id: AtomicI64,
    pending: PendingResponses,
    sink: Mutex<SplitSink<ClientStream, Message>>,
    url: String,
}

impl WebSocketConnection {
    pub(crate) fn new(stream: ClientStream, url: String) -> Self {
        let (sink, source) = stream.split();
        let cancellation_token = CancellationToken::new();
        let pending = PendingResponses::default();

        drop(tokio::spawn(read_server_frames(
            cancellation_token.clone(),
            source,
            pending.clone(),
        )));

        Self {
            cancellation_token,
            next_request_id: AtomicI64::new(0),
            pending,
            sink: Mutex::new(sink),
            url,
        }
    }

    /// # Errors
    ///
    /// Returns `WebSocketClientError` when the notification cannot be sent.
    pub async fn notify<Notification>(
        &self,
        notification: Notification,
    ) -> Result<(), WebSocketClientError>
    where
        Notification: Serialize + WebSocketNotificationMessage,
    {
        let params = serde_json::to_value(notification)
            .map_err(|source| WebSocketClientError::SerializeFrame { source })?;

        self.send_frame(&ClientSentFrame::Notification {
            method: Notification::METHOD.to_string(),
            params,
        })
        .await
    }

    /// # Errors
    ///
    /// Returns `WebSocketClientError` when the request cannot be sent.
    pub async fn request<Request, Payload>(
        &self,
        request: Request,
    ) -> Result<ResponseStream<Payload>, WebSocketClientError>
    where
        Request: Serialize + WebSocketRequestMessage,
        Payload: DeserializeOwned + WebSocketResponseMessage,
    {
        let params = serde_json::to_value(request)
            .map_err(|source| WebSocketClientError::SerializeFrame { source })?;
        let id = RequestId::Number(self.next_request_id.fetch_add(1, Ordering::Relaxed));
        let (sender, receiver) = mpsc::channel(RESPONSE_BUFFER_CAPACITY);

        self.pending.remember(id.clone(), sender);

        if let Err(error) = self
            .send_frame(&ClientSentFrame::Request {
                id: id.clone(),
                method: Request::METHOD.to_string(),
                params,
            })
            .await
        {
            self.pending.forget(&id);

            return Err(error);
        }

        Ok(ResponseStream::new(id, self.pending.clone(), receiver))
    }

    async fn send_frame(&self, frame: &ClientSentFrame) -> Result<(), WebSocketClientError> {
        let text = serde_json::to_string(frame)
            .map_err(|source| WebSocketClientError::SerializeFrame { source })?;

        self.sink
            .lock()
            .await
            .send(Message::text(text))
            .await
            .map_err(|source| WebSocketClientError::SendFrame {
                source,
                url: self.url.clone(),
            })
    }
}

impl Drop for WebSocketConnection {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
    }
}
