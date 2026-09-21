use std::sync::Arc;
use std::sync::atomic::AtomicI64;
use std::sync::atomic::Ordering;

use futures_util::StreamExt;
use futures_util::stream::SplitStream;
use serde::Serialize;
use serde::de::DeserializeOwned;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio_tungstenite::tungstenite::Message;
use tokio_util::sync::CancellationToken;

use margaret_websocket_envelope::client_sent_frame::ClientSentFrame;
use margaret_websocket_envelope::request_id::RequestId;
use margaret_websocket_envelope::server_sent_frame::ServerSentFrame;
use margaret_websocket_envelope::web_socket_notification_message::WebSocketNotificationMessage;
use margaret_websocket_envelope::web_socket_request_message::WebSocketRequestMessage;
use margaret_websocket_envelope::web_socket_response_message::WebSocketResponseMessage;

use crate::client_stream::ClientStream;
use crate::exchange_interruption::ExchangeInterruption;
use crate::exchange_outcome::ExchangeOutcome;
use crate::exchange_registration::ExchangeRegistration;
use crate::exchange_registry::ExchangeRegistry;
use crate::exchange_signal::ExchangeSignal;
use crate::exchange_signals::ExchangeSignals;
use crate::exchange_termination::ExchangeTermination;
use crate::granted_credit::GrantedCredit;
use crate::open_exchange::OpenExchange;
use crate::outbound_frames::OutboundFrames;
use crate::pending_exchange::PendingExchange;
use crate::response_credit_window::RESPONSE_CREDIT_WINDOW;
use crate::response_stream::ResponseStream;
use crate::route_server_frame::route_server_frame;
use crate::web_socket_client_error::WebSocketClientError;

async fn read_server_frames(
    cancellation_token: CancellationToken,
    mut source: SplitStream<ClientStream>,
    exchanges: ExchangeRegistry,
    frames: OutboundFrames,
    mut signals: UnboundedReceiver<ExchangeSignal>,
) {
    let interruption = loop {
        tokio::select! {
            biased;
            () = cancellation_token.cancelled() => break ExchangeInterruption::ConnectionDropped,
            Some(signal) = signals.recv() => frames.report_exchange_signal(signal).await,
            message = source.next() => match message {
                Some(Ok(Message::Text(text))) => {
                    match serde_json::from_str::<ServerSentFrame>(&text) {
                        Ok(frame) => route_server_frame(frame, &exchanges),
                        Err(error) => {
                            eprintln!(
                                "margaret_websocket_client: the peer sent an unreadable frame: {error}"
                            );

                            break ExchangeInterruption::PeerSentUnreadableFrame;
                        }
                    }
                }
                Some(Ok(Message::Close(_))) => {
                    tokio::select! {
                        biased;
                        () = cancellation_token.cancelled() => {
                            break ExchangeInterruption::ConnectionDropped;
                        }
                        () = frames.answer_close() => {}
                    }
                }
                Some(Ok(_)) => {}
                Some(Err(error)) => {
                    eprintln!("margaret_websocket_client: the connection failed: {error}");

                    break ExchangeInterruption::ConnectionFailed;
                }
                None => break ExchangeInterruption::PeerClosed,
            },
        }
    };

    exchanges.interrupt_all(interruption);
}

pub struct WebSocketConnection {
    cancellation_token: CancellationToken,
    exchanges: ExchangeRegistry,
    frames: OutboundFrames,
    next_request_id: AtomicI64,
    signals: ExchangeSignals,
    url: Arc<str>,
}

impl WebSocketConnection {
    pub(crate) fn new(stream: ClientStream, url: Arc<str>) -> Self {
        let (sink, source) = stream.split();
        let cancellation_token = CancellationToken::new();
        let exchanges = ExchangeRegistry::default();
        let frames = OutboundFrames::new(Arc::new(Mutex::new(sink)), url.clone());
        let (reporter, reported_signals) = mpsc::unbounded_channel();

        drop(tokio::spawn(read_server_frames(
            cancellation_token.clone(),
            source,
            exchanges.clone(),
            frames.clone(),
            reported_signals,
        )));

        Self {
            cancellation_token,
            exchanges,
            frames,
            next_request_id: AtomicI64::new(0),
            signals: ExchangeSignals::new(reporter),
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
        self.refuse_when_terminated()?;

        self.frames
            .send_serialization(serde_json::to_string(&ClientSentFrame::Notification {
                method: Notification::METHOD.to_string(),
                params: notification,
            }))
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
        let url = self.url.clone();
        let serialized = serde_json::to_string(&ClientSentFrame::Request {
            credit: RESPONSE_CREDIT_WINDOW,
            id: id.clone(),
            method: Request::METHOD.to_string(),
            params: request,
        });

        self.start_exchange(id, serialized)
            .await
            .map(|exchange| ResponseStream::new(exchange, url))
    }

    fn refuse_when_terminated(&self) -> Result<(), WebSocketClientError> {
        match self.exchanges.outcome() {
            ExchangeOutcome::Completed => Ok(()),
            ExchangeOutcome::Interrupted(interruption) => Err(interruption.into_error(&self.url)),
        }
    }

    async fn start_exchange(
        &self,
        id: RequestId,
        serialized: Result<String, serde_json::Error>,
    ) -> Result<OpenExchange, WebSocketClientError> {
        let (sender, receiver) = mpsc::unbounded_channel();
        let granted = GrantedCredit::new(RESPONSE_CREDIT_WINDOW);
        let termination = ExchangeTermination::default();
        let exchange = OpenExchange::new(
            granted.clone(),
            id.clone(),
            receiver,
            self.exchanges.clone(),
            self.signals.clone(),
            termination.clone(),
        );

        if let ExchangeRegistration::Refused(interruption) = self.exchanges.register(
            &id,
            PendingExchange {
                granted,
                sender,
                termination,
            },
        ) {
            return Err(interruption.into_error(&self.url));
        }

        self.frames.send_serialization(serialized).await?;

        Ok(exchange)
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

    use futures_util::FutureExt;
    use futures_util::SinkExt;
    use futures_util::StreamExt;
    use futures_util::stream::SplitStream;
    use margaret_websocket_envelope::request_id::RequestId;
    use margaret_websocket_envelope::web_socket_notification_message::WebSocketNotificationMessage;
    use margaret_websocket_envelope::web_socket_request_message::WebSocketRequestMessage;
    use margaret_websocket_envelope::web_socket_response_message::WebSocketResponseMessage;
    use serde::Deserialize;
    use serde::Serialize;
    use tokio::io::duplex;
    use tokio::sync::Mutex;
    use tokio::sync::mpsc;
    use tokio_tungstenite::WebSocketStream;
    use tokio_tungstenite::tungstenite::Message;
    use tokio_tungstenite::tungstenite::protocol::Role;
    use tokio_util::sync::CancellationToken;

    use super::ClientStream;
    use super::ExchangeRegistry;
    use super::OutboundFrames;
    use super::WebSocketConnection;
    use super::read_server_frames;
    use crate::client_io::ClientIo;
    use crate::response_item::ResponseItem;
    use crate::web_socket_client_error::WebSocketClientError;

    /// The duplex buffer the framework's own websocket harness serves connections over.
    const DUPLEX_CAPACITY: usize = 65536;

    const PEER_URL: &str = "wss://peer.test/ws";

    /// A duplex too small to accept a whole frame, so the write parks instead of completing.
    const PARKING_DUPLEX_CAPACITY: usize = 1;

    struct Halves {
        frames: OutboundFrames,
        peer: ClientStream,
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
            frames: OutboundFrames::new(Arc::new(Mutex::new(sink)), Arc::from(PEER_URL)),
            peer: WebSocketStream::from_raw_socket(peer, Role::Server, None).await,
            source,
        }
    }

    async fn read_until_the_peer_is_gone(
        frames: OutboundFrames,
        source: SplitStream<ClientStream>,
    ) -> ExchangeRegistry {
        let exchanges = ExchangeRegistry::default();

        let (_signals, reported_signals) = mpsc::unbounded_channel();

        read_server_frames(
            CancellationToken::new(),
            source,
            exchanges.clone(),
            frames,
            reported_signals,
        )
        .await;

        exchanges
    }

    #[tokio::test]
    async fn stops_reading_when_the_peer_completes_the_closing_handshake() {
        let Halves {
            frames,
            mut peer,
            source,
        } = halves().await;

        peer.send(Message::Close(None))
            .await
            .expect("the peer sends a close frame");

        tokio::join!(read_until_the_peer_is_gone(frames, source), async move {
            while peer.next().await.is_some() {}
        });
    }

    #[tokio::test]
    async fn reports_a_closing_handshake_the_peer_will_not_receive() {
        let Halves {
            frames,
            mut peer,
            source,
        } = halves().await;

        peer.send(Message::Close(None))
            .await
            .expect("the peer sends a close frame");

        drop(peer);

        read_until_the_peer_is_gone(frames, source).await;
    }

    #[tokio::test]
    async fn stops_reading_when_the_connection_is_cancelled() {
        let Halves {
            frames,
            peer,
            source,
        } = halves().await;
        let cancellation_token = CancellationToken::new();

        cancellation_token.cancel();

        let (_signals, reported_signals) = mpsc::unbounded_channel();

        read_server_frames(
            cancellation_token,
            source,
            ExchangeRegistry::default(),
            frames,
            reported_signals,
        )
        .await;

        drop(peer);
    }

    #[derive(Serialize)]
    struct Echo {
        label: String,
    }

    impl WebSocketRequestMessage for Echo {
        const METHOD: &'static str = "echo";
    }

    #[derive(Debug, Deserialize)]
    struct Echoed {
        label: String,
    }

    impl WebSocketResponseMessage for Echoed {
        const METHOD: &'static str = "echoed";
    }

    #[derive(Serialize)]
    struct Waved;

    impl WebSocketNotificationMessage for Waved {
        const METHOD: &'static str = "waved";
    }

    struct Unserializable;

    impl Serialize for Unserializable {
        fn serialize<Serializer>(
            &self,
            _serializer: Serializer,
        ) -> Result<Serializer::Ok, Serializer::Error>
        where
            Serializer: serde::Serializer,
        {
            Err(serde::ser::Error::custom("this payload never serializes"))
        }
    }

    impl WebSocketNotificationMessage for Unserializable {
        const METHOD: &'static str = "unserializable";
    }

    impl WebSocketRequestMessage for Unserializable {
        const METHOD: &'static str = "unserializable";
    }

    struct Connected {
        connection: WebSocketConnection,
        peer: ClientStream,
    }

    async fn connected() -> Connected {
        connected_over(DUPLEX_CAPACITY).await
    }

    async fn connected_over(capacity: usize) -> Connected {
        let (client_io, peer_io) = duplex(capacity);
        let client: Box<dyn ClientIo> = Box::new(client_io);
        let peer: Box<dyn ClientIo> = Box::new(peer_io);

        Connected {
            connection: WebSocketConnection::new(
                WebSocketStream::from_raw_socket(client, Role::Client, None).await,
                Arc::from(PEER_URL),
            ),
            peer: WebSocketStream::from_raw_socket(peer, Role::Server, None).await,
        }
    }

    async fn next_text(peer: &mut ClientStream) -> String {
        peer.next()
            .await
            .expect("the peer receives a frame")
            .expect("the frame reads cleanly")
            .into_text()
            .expect("the frame carries text")
            .to_string()
    }

    #[tokio::test]
    async fn carries_a_request_and_reads_the_answer_back() {
        let Connected {
            connection,
            mut peer,
        } = connected().await;
        let mut responses = connection
            .request::<Echo, Echoed>(Echo {
                label: "here".to_string(),
            })
            .await
            .expect("the request is sent");

        assert!(next_text(&mut peer).await.contains("\"echo\""));

        peer.send(Message::text(
            r#"{"id":0,"done":true,"method":"echoed","result":{"label":"here"}}"#,
        ))
        .await
        .expect("the peer answers");

        let answer = responses
            .next()
            .await
            .expect("the answer arrives")
            .expect("the answer reads as the requested payload");

        assert!(matches!(answer, ResponseItem::Payload(echoed) if echoed.label == "here"));
        assert!(responses.next().await.is_none());
    }

    #[tokio::test]
    async fn reports_a_notification_that_cannot_be_serialized() {
        let Connected {
            connection,
            peer: _peer,
        } = connected().await;

        let refused = connection
            .notify(Unserializable)
            .await
            .expect_err("an unserializable payload never reaches the peer");

        assert!(matches!(
            refused,
            WebSocketClientError::SerializeFrame { source } if source.is_data()
        ));
    }

    #[tokio::test]
    async fn reports_a_request_that_cannot_be_serialized() {
        let Connected {
            connection,
            peer: _peer,
        } = connected().await;
        let refused = connection
            .request::<Unserializable, Echoed>(Unserializable)
            .await
            .map(drop)
            .expect_err("an unserializable payload never reaches the peer");

        assert!(matches!(
            refused,
            WebSocketClientError::SerializeFrame { source } if source.is_data()
        ));
    }

    #[tokio::test]
    async fn forgets_an_exchange_whose_request_was_abandoned_mid_flight() {
        let Connected {
            connection,
            peer: _peer,
        } = connected_over(PARKING_DUPLEX_CAPACITY).await;

        let abandoned = RequestId::Number(0);

        assert!(
            connection
                .request::<Echo, Echoed>(Echo {
                    label: "abandoned".to_string(),
                })
                .now_or_never()
                .is_none()
        );
        assert!(connection.exchanges.peek(&abandoned).is_none());
    }

    #[tokio::test]
    async fn reports_a_connection_that_was_dropped_mid_exchange() {
        let Connected {
            connection,
            peer: _peer,
        } = connected().await;
        let mut responses = connection
            .request::<Echo, Echoed>(Echo {
                label: "dropped".to_string(),
            })
            .await
            .expect("the request is sent");

        drop(connection);

        let interruption = responses
            .next()
            .await
            .expect("a dropped connection reports why the exchange ended")
            .expect_err("the connection was dropped before the peer answered");

        assert!(matches!(
            interruption,
            WebSocketClientError::ExchangeConnectionDropped { url } if url == PEER_URL
        ));
    }

    #[tokio::test]
    async fn stops_answering_a_close_when_the_connection_is_dropped() {
        let Connected {
            connection,
            mut peer,
        } = connected().await;
        let mut answered = connection
            .request::<Echo, Echoed>(Echo {
                label: "answered".to_string(),
            })
            .await
            .expect("the first request is sent");
        let mut responses = connection
            .request::<Echo, Echoed>(Echo {
                label: "outstanding".to_string(),
            })
            .await
            .expect("the second request is sent");
        let sink = connection.frames.sink.clone();
        let held = sink.lock().await;

        peer.send(Message::text(
            r#"{"id":0,"done":true,"method":"echoed","result":{"label":"first"}}"#,
        ))
        .await
        .expect("the peer answers the first request");
        peer.send(Message::Close(None))
            .await
            .expect("the peer closes");

        assert!(answered.next().await.is_some());

        drop(connection);

        let interruption = responses
            .next()
            .await
            .expect("a dropped connection reports why the exchange ended")
            .expect_err("the connection was dropped while the close was being answered");

        drop(held);

        assert!(matches!(
            interruption,
            WebSocketClientError::ExchangeConnectionDropped { url } if url == PEER_URL
        ));
    }

    #[tokio::test]
    async fn carries_a_notification() {
        let Connected {
            connection,
            mut peer,
        } = connected().await;

        connection
            .notify(Waved)
            .await
            .expect("the notification is sent");

        assert!(next_text(&mut peer).await.contains("\"waved\""));
    }
}
