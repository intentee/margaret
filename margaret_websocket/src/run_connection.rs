use futures_util::StreamExt;
use futures_util::stream::SplitStream;
use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::channel;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_util::sync::CancellationToken;

use crate::activity_spawner::ActivitySpawner;
use crate::emit_core::EmitCore;
use crate::inbound_frame::InboundFrame;
use crate::json_rpc_error_frame::JsonRpcErrorFrame;
use crate::outbound_frame::OutboundFrame;
use crate::protocol::Protocol;
use crate::websocket_channel_config::WebSocketChannelConfig;
use crate::websocket_connection_error::WebSocketConnectionError;
use crate::writer_task::run_writer_loop;

async fn dispatch_wire<TProtocol>(
    protocol: &TProtocol,
    state: TProtocol::State,
    text: &str,
    emit: &EmitCore,
    spawner: &ActivitySpawner<TProtocol::Internal>,
) -> TProtocol::State
where
    TProtocol: Protocol,
{
    match InboundFrame::decode(text) {
        InboundFrame::ParseError => {
            emit.send(OutboundFrame::Error(JsonRpcErrorFrame::parse_error()))
                .await;

            state
        }
        InboundFrame::InvalidEnvelope { id } => {
            emit.send(OutboundFrame::Error(JsonRpcErrorFrame::invalid_request(id)))
                .await;

            state
        }
        InboundFrame::Notification { method, params } => {
            protocol
                .dispatch(state, &method, params.as_ref(), None, emit, spawner)
                .await
        }
        InboundFrame::Request {
            id,
            method,
            params,
        } => {
            protocol
                .dispatch(state, &method, params.as_ref(), Some(id), emit, spawner)
                .await
        }
    }
}

async fn handle_message<TProtocol>(
    protocol: &TProtocol,
    state: TProtocol::State,
    message: Message,
    emit: &EmitCore,
    spawner: &ActivitySpawner<TProtocol::Internal>,
) -> TProtocol::State
where
    TProtocol: Protocol,
{
    match message {
        Message::Text(text) => {
            dispatch_wire(protocol, state, text.as_str(), emit, spawner).await
        }
        Message::Ping(payload) => {
            emit.send(OutboundFrame::Pong(payload)).await;

            state
        }
        Message::Binary(_) | Message::Close(_) | Message::Pong(_) | Message::Frame(_) => state,
    }
}

async fn run_read_loop<TProtocol, Io>(
    protocol: TProtocol,
    mut read: SplitStream<WebSocketStream<Io>>,
    mut internal_receiver: Receiver<TProtocol::Internal>,
    emit: EmitCore,
    spawner: ActivitySpawner<TProtocol::Internal>,
    cancellation_token: CancellationToken,
) -> Result<(), WebSocketConnectionError>
where
    TProtocol: Protocol,
    Io: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    let mut state = protocol.initial_state();

    let outcome = loop {
        if protocol.is_terminal(&state) {
            emit.send(OutboundFrame::Close).await;

            break Ok(());
        }

        tokio::select! {
            biased;
            () = cancellation_token.cancelled() => break Ok(()),
            Some(event) = internal_receiver.recv() => {
                state = protocol.dispatch_internal(state, event, &emit, &spawner).await;
            }
            frame = read.next() => match frame {
                None => break Ok(()),
                Some(Ok(message)) => {
                    state = handle_message(&protocol, state, message, &emit, &spawner).await;
                }
                Some(Err(error)) => break Err(WebSocketConnectionError::Transport(error)),
            },
        }
    };

    cancellation_token.cancel();

    outcome
}

pub async fn run_connection<TProtocol, Io>(
    protocol: TProtocol,
    stream: WebSocketStream<Io>,
    cancellation_token: CancellationToken,
    config: WebSocketChannelConfig,
) -> Result<(), WebSocketConnectionError>
where
    TProtocol: Protocol,
    Io: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    let (sink, read) = stream.split();
    let (outbound_sender, outbound_receiver) = channel(config.outbound_capacity);
    let (internal_sender, internal_receiver) = channel(config.internal_capacity);
    let emit = EmitCore::new(cancellation_token.clone(), outbound_sender);
    let spawner = ActivitySpawner::new(cancellation_token.clone(), internal_sender);

    let (read_result, ()) = tokio::join!(
        run_read_loop(
            protocol,
            read,
            internal_receiver,
            emit,
            spawner,
            cancellation_token.clone(),
        ),
        run_writer_loop(sink, outbound_receiver, cancellation_token),
    );

    read_result
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use futures_util::SinkExt;
    use futures_util::StreamExt;
    use serde_json::Value;
    use serde_json::json;
    use tokio::io::AsyncWriteExt;
    use tokio::io::DuplexStream;
    use tokio::io::duplex;
    use tokio::task::JoinHandle;
    use tokio_tungstenite::WebSocketStream;
    use tokio_tungstenite::tungstenite::Message;
    use tokio_tungstenite::tungstenite::protocol::Role;
    use tokio_util::sync::CancellationToken;

    use super::run_connection;
    use crate::activity_spawner::ActivitySpawner;
    use crate::emit_core::EmitCore;
    use crate::json_rpc_error_frame::JsonRpcErrorFrame;
    use crate::outbound_frame::OutboundFrame;
    use crate::protocol::Protocol;
    use crate::request_id::RequestId;
    use crate::websocket_channel_config::WebSocketChannelConfig;
    use crate::websocket_connection_error::WebSocketConnectionError;

    type Client = WebSocketStream<DuplexStream>;
    type Serving = JoinHandle<Result<(), WebSocketConnectionError>>;

    enum StoryboardInternal {
        GenerationComplete,
    }

    enum StoryboardState {
        Chatting,
        Ended,
        Fresh,
        Thinking,
    }

    struct Storyboard;

    #[async_trait]
    impl Protocol for Storyboard {
        type Internal = StoryboardInternal;
        type State = StoryboardState;

        fn initial_state(&self) -> StoryboardState {
            StoryboardState::Fresh
        }

        fn is_terminal(&self, state: &StoryboardState) -> bool {
            matches!(state, StoryboardState::Ended)
        }

        async fn dispatch(
            &self,
            state: StoryboardState,
            method: &str,
            _params: Option<&Value>,
            request_id: Option<RequestId>,
            emit: &EmitCore,
            spawner: &ActivitySpawner<StoryboardInternal>,
        ) -> StoryboardState {
            match (state, method) {
                (StoryboardState::Fresh, "start") => {
                    let deliver = spawner.internal_sender();
                    let streaming = emit.clone();

                    spawner.spawn(async move {
                        streaming
                            .send(OutboundFrame::Notify {
                                method: "assistant.accepted".to_owned(),
                                params: json!(null),
                            })
                            .await;

                        deliver
                            .send(StoryboardInternal::GenerationComplete)
                            .await
                            .expect("the completion event reaches the loop");
                    });

                    StoryboardState::Thinking
                }
                (StoryboardState::Chatting, "finish") => StoryboardState::Ended,
                (other, _) => {
                    if let Some(id) = request_id {
                        emit.send(OutboundFrame::Error(JsonRpcErrorFrame::method_not_found(
                            id, method,
                        )))
                        .await;
                    }

                    other
                }
            }
        }

        async fn dispatch_internal(
            &self,
            _state: StoryboardState,
            event: StoryboardInternal,
            emit: &EmitCore,
            _spawner: &ActivitySpawner<StoryboardInternal>,
        ) -> StoryboardState {
            match event {
                StoryboardInternal::GenerationComplete => {
                    emit.send(OutboundFrame::Notify {
                        method: "assistant.reply".to_owned(),
                        params: json!("done"),
                    })
                    .await;

                    StoryboardState::Chatting
                }
            }
        }
    }

    fn config() -> WebSocketChannelConfig {
        WebSocketChannelConfig {
            internal_capacity: 8,
            outbound_capacity: 8,
        }
    }

    async fn connect() -> (Client, CancellationToken, Serving) {
        let (server_io, client_io) = duplex(4096);
        let server = WebSocketStream::from_raw_socket(server_io, Role::Server, None).await;
        let client = WebSocketStream::from_raw_socket(client_io, Role::Client, None).await;
        let cancellation_token = CancellationToken::new();
        let serving = tokio::spawn(run_connection(
            Storyboard,
            server,
            cancellation_token.clone(),
            config(),
        ));

        (client, cancellation_token, serving)
    }

    async fn send(client: &mut Client, request: &str) {
        client
            .send(Message::text(request.to_owned()))
            .await
            .expect("the request reaches the server");
    }

    async fn next_message(client: &mut Client) -> Message {
        client
            .next()
            .await
            .expect("a message arrives")
            .expect("without a transport error")
    }

    async fn next_json(client: &mut Client) -> Value {
        let text = next_message(client)
            .await
            .into_text()
            .expect("the frame is a text message");

        serde_json::from_str(text.as_str()).expect("the frame is valid json")
    }

    #[tokio::test]
    async fn drives_a_conversation_through_a_spawned_activity() {
        let (mut client, _token, serving) = connect().await;

        send(&mut client, r#"{ "jsonrpc": "2.0", "method": "start", "id": 1 }"#).await;

        assert_eq!(next_json(&mut client).await["method"], "assistant.accepted");
        assert_eq!(next_json(&mut client).await["method"], "assistant.reply");

        client
            .close(None)
            .await
            .expect("the close frame reaches the server");

        serving
            .await
            .expect("the task joins")
            .expect("the connection ends cleanly");
    }

    #[tokio::test]
    async fn reports_method_not_found_for_an_unknown_request() {
        let (mut client, _token, _serving) = connect().await;

        send(&mut client, r#"{ "jsonrpc": "2.0", "method": "nope", "id": 2 }"#).await;

        let value = next_json(&mut client).await;

        assert_eq!(value["id"], json!(2));
        assert_eq!(value["error"]["code"], json!(-32601));
    }

    #[tokio::test]
    async fn reports_a_parse_error_for_non_json_text() {
        let (mut client, _token, _serving) = connect().await;

        send(&mut client, "this is not json").await;

        assert_eq!(next_json(&mut client).await["error"]["code"], json!(-32700));
    }

    #[tokio::test]
    async fn reports_invalid_request_for_a_malformed_envelope() {
        let (mut client, _token, _serving) = connect().await;

        send(&mut client, r#"{ "jsonrpc": "2.0" }"#).await;

        assert_eq!(next_json(&mut client).await["error"]["code"], json!(-32600));
    }

    #[tokio::test]
    async fn ignores_an_unknown_notification_and_keeps_serving() {
        let (mut client, _token, _serving) = connect().await;

        send(&mut client, r#"{ "jsonrpc": "2.0", "method": "nope" }"#).await;
        send(&mut client, r#"{ "jsonrpc": "2.0", "method": "nope", "id": 5 }"#).await;

        assert_eq!(next_json(&mut client).await["id"], json!(5));
    }

    #[tokio::test]
    async fn answers_a_ping_with_a_pong() {
        let (mut client, _token, _serving) = connect().await;

        client
            .send(Message::Ping(Vec::from(b"beat").into()))
            .await
            .expect("the ping reaches the server");

        assert!(next_message(&mut client).await.is_pong());
    }

    #[tokio::test]
    async fn ignores_a_binary_message_and_keeps_serving() {
        let (mut client, _token, _serving) = connect().await;

        client
            .send(Message::Binary(Vec::from(b"raw").into()))
            .await
            .expect("the binary message reaches the server");
        send(&mut client, r#"{ "jsonrpc": "2.0", "method": "nope", "id": 6 }"#).await;

        assert_eq!(next_json(&mut client).await["id"], json!(6));
    }

    #[tokio::test]
    async fn closes_when_the_client_closes() {
        let (mut client, _token, serving) = connect().await;

        client
            .close(None)
            .await
            .expect("the close frame reaches the server");

        serving
            .await
            .expect("the task joins")
            .expect("the connection ends cleanly");
    }

    #[tokio::test]
    async fn closes_the_connection_when_a_terminal_state_is_reached() {
        let (mut client, _token, serving) = connect().await;

        send(&mut client, r#"{ "jsonrpc": "2.0", "method": "start", "id": 1 }"#).await;

        assert_eq!(next_json(&mut client).await["method"], "assistant.accepted");
        assert_eq!(next_json(&mut client).await["method"], "assistant.reply");

        send(&mut client, r#"{ "jsonrpc": "2.0", "method": "finish", "id": 2 }"#).await;

        assert!(next_message(&mut client).await.is_close());

        serving
            .await
            .expect("the task joins")
            .expect("the connection ends cleanly");
    }

    #[tokio::test]
    async fn stops_the_connection_on_cancellation() {
        let (_client, cancellation_token, serving) = connect().await;

        cancellation_token.cancel();

        serving
            .await
            .expect("the task joins")
            .expect("the connection ends cleanly");
    }

    #[tokio::test]
    async fn returns_a_transport_error_on_a_malformed_frame() {
        let (server_io, mut client_io) = duplex(4096);
        let server = WebSocketStream::from_raw_socket(server_io, Role::Server, None).await;
        let cancellation_token = CancellationToken::new();
        let serving = tokio::spawn(run_connection(
            Storyboard,
            server,
            cancellation_token,
            config(),
        ));

        client_io
            .write_all(&[0xFF, 0xFF, 0x00, 0x01])
            .await
            .expect("the raw bytes reach the server");
        drop(client_io);

        assert!(serving.await.expect("the task joins").is_err());
    }
}
