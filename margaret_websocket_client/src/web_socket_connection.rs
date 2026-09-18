use std::sync::Arc;
use std::sync::atomic::AtomicI64;
use std::sync::atomic::Ordering;

use futures_util::SinkExt;
use futures_util::StreamExt;
use futures_util::stream::SplitSink;
use futures_util::stream::SplitStream;
use serde::Serialize;
use serde::de::DeserializeOwned;
use tokio::sync::Mutex;
use serde_json::Value;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio_tungstenite::tungstenite::Message;
use tokio_util::sync::CancellationToken;

use margaret_websocket_envelope::client_sent_frame::ClientSentFrame;
use margaret_websocket_envelope::request_id::RequestId;
use margaret_websocket_envelope::server_sent_frame::ServerSentFrame;
use margaret_websocket_envelope::web_socket_notification_message::WebSocketNotificationMessage;
use margaret_websocket_envelope::web_socket_request_message::WebSocketRequestMessage;
use margaret_websocket_envelope::web_socket_response_message::WebSocketResponseMessage;

use crate::client_stream::ClientStream;
use crate::pending_responses::PendingResponses;
use crate::response_stream::ResponseStream;
use crate::route_server_frame::route_server_frame;
use crate::web_socket_client_error::WebSocketClientError;

/// One in-flight frame per exchange, matching the outbound buffer the server serves with.
const RESPONSE_BUFFER_CAPACITY: usize = 1;

type ClientSink = Arc<Mutex<SplitSink<ClientStream, Message>>>;

async fn answer_close(sink: &ClientSink) {
    if let Err(error) = sink.lock().await.close().await {
        eprintln!("margaret_websocket_client: the closing handshake failed: {error}");
    }
}

async fn read_server_frames(
    cancellation_token: CancellationToken,
    mut source: SplitStream<ClientStream>,
    pending: PendingResponses,
    sink: ClientSink,
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
                Some(Ok(Message::Close(_))) => answer_close(&sink).await,
                Some(Ok(_)) => {}
                Some(Err(error)) => {
                    eprintln!("margaret_websocket_client: the connection failed: {error}");

                    break;
                }
                None => break,
            },
        }
    }

    pending.clear();
}

pub struct WebSocketConnection {
    cancellation_token: CancellationToken,
    next_request_id: AtomicI64,
    pending: PendingResponses,
    sink: ClientSink,
    url: String,
}

impl WebSocketConnection {
    pub(crate) fn new(stream: ClientStream, url: String) -> Self {
        let (sink, source) = stream.split();
        let cancellation_token = CancellationToken::new();
        let pending = PendingResponses::default();
        let sink = Arc::new(Mutex::new(sink));

        drop(tokio::spawn(read_server_frames(
            cancellation_token.clone(),
            source,
            pending.clone(),
            sink.clone(),
        )));

        Self {
            cancellation_token,
            next_request_id: AtomicI64::new(0),
            pending,
            sink,
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
        self.send_notification(Notification::METHOD, serde_json::to_value(notification))
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
        let id = RequestId::Number(self.next_request_id.fetch_add(1, Ordering::Relaxed));
        let pending = self.pending.clone();

        self.open_exchange(id.clone(), Request::METHOD, serde_json::to_value(request))
            .await
            .map(|receiver| ResponseStream::new(id, pending, receiver))
    }

    async fn open_exchange(
        &self,
        id: RequestId,
        method: &'static str,
        params: Result<Value, serde_json::Error>,
    ) -> Result<Receiver<ServerSentFrame>, WebSocketClientError> {
        let params = params.map_err(|source| WebSocketClientError::SerializeFrame { source })?;
        let (sender, receiver) = mpsc::channel(RESPONSE_BUFFER_CAPACITY);

        self.pending.remember(id.clone(), sender);

        if let Err(error) = self
            .send_frame(&ClientSentFrame::Request {
                id: id.clone(),
                method: method.to_string(),
                params,
            })
            .await
        {
            self.pending.forget(&id);

            return Err(error);
        }

        Ok(receiver)
    }

    async fn send_notification(
        &self,
        method: &'static str,
        params: Result<Value, serde_json::Error>,
    ) -> Result<(), WebSocketClientError> {
        let params = params.map_err(|source| WebSocketClientError::SerializeFrame { source })?;

        self.send_frame(&ClientSentFrame::Notification {
            method: method.to_string(),
            params,
        })
        .await
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use futures_util::SinkExt;
    use futures_util::StreamExt;
    use futures_util::stream::SplitStream;
    use tokio::io::duplex;
    use tokio::sync::Mutex;
    use tokio_tungstenite::WebSocketStream;
    use tokio_tungstenite::tungstenite::Message;
    use tokio_tungstenite::tungstenite::protocol::Role;
    use tokio_util::sync::CancellationToken;

    use super::ClientSink;
    use super::ClientStream;
    use super::PendingResponses;
    use super::read_server_frames;
    use crate::client_io::ClientIo;

    /// The duplex buffer the framework's own websocket harness serves connections over.
    const DUPLEX_CAPACITY: usize = 65536;

    struct Halves {
        peer: ClientStream,
        sink: ClientSink,
        source: SplitStream<ClientStream>,
    }

    async fn halves() -> Halves {
        let (client_io, peer_io) = duplex(DUPLEX_CAPACITY);
        let client: Box<dyn ClientIo> = Box::new(client_io);
        let peer: Box<dyn ClientIo> = Box::new(peer_io);
        let (sink, source) = WebSocketStream::from_raw_socket(client, Role::Client, None)
            .await
            .split();

        Halves {
            peer: WebSocketStream::from_raw_socket(peer, Role::Server, None).await,
            sink: Arc::new(Mutex::new(sink)),
            source,
        }
    }

    async fn read_until_the_peer_is_gone(
        sink: ClientSink,
        source: SplitStream<ClientStream>,
    ) -> PendingResponses {
        let pending = PendingResponses::default();

        read_server_frames(
            CancellationToken::new(),
            source,
            pending.clone(),
            sink,
        )
        .await;

        pending
    }

    #[tokio::test]
    async fn stops_reading_when_the_peer_completes_the_closing_handshake() {
        let Halves {
            mut peer,
            sink,
            source,
        } = halves().await;

        peer.send(Message::Close(None))
            .await
            .expect("the peer sends a close frame");

        tokio::join!(read_until_the_peer_is_gone(sink, source), async move {
            while peer.next().await.is_some() {}
        });
    }

    #[tokio::test]
    async fn reports_a_closing_handshake_the_peer_will_not_receive() {
        let Halves {
            mut peer,
            sink,
            source,
        } = halves().await;

        peer.send(Message::Close(None))
            .await
            .expect("the peer sends a close frame");

        drop(peer);

        read_until_the_peer_is_gone(sink, source).await;
    }

    #[tokio::test]
    async fn stops_reading_when_the_connection_is_cancelled() {
        let Halves {
            peer,
            sink,
            source,
        } = halves().await;
        let cancellation_token = CancellationToken::new();

        cancellation_token.cancel();

        read_server_frames(
            cancellation_token,
            source,
            PendingResponses::default(),
            sink,
        )
        .await;

        drop(peer);
    }
}
