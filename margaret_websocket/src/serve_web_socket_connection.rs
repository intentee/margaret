use std::sync::Arc;

use futures_util::SinkExt;
use futures_util::StreamExt;
use futures_util::stream::SplitSink;
use futures_util::stream::SplitStream;
use serde_json::Value;
use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_util::sync::CancellationToken;

use crate::envelope_error_code::EnvelopeErrorCode;
use crate::inbound_frame::InboundFrame;
use crate::report_web_socket_error::report_web_socket_error;
use crate::web_socket::WebSocket;
use crate::web_socket_dispatch_table::WebSocketDispatchTable;

const OUTBOUND_BUFFER_CAPACITY: usize = 1;

async fn drain_outbound<Io>(
    mut sink: SplitSink<WebSocketStream<Io>, Message>,
    mut outbound: Receiver<Message>,
) where
    Io: AsyncRead + AsyncWrite + Unpin,
{
    while let Some(message) = outbound.recv().await {
        if sink.send(message).await.is_err() {
            return;
        }
    }

    drop(sink.send(Message::Close(None)).await);
}

async fn dispatch_frame<Session>(
    cancellation_token: &CancellationToken,
    session: &Arc<Session>,
    dispatch_table: &Arc<WebSocketDispatchTable<Session>>,
    socket: &WebSocket,
    frame: InboundFrame,
) where
    Session: Send + Sync + 'static,
{
    match frame {
        InboundFrame::Request { id, method, params } => match dispatch_table.request(&method) {
            Some(dispatch) => {
                let dispatch = dispatch.clone();
                let cancellation_token = cancellation_token.child_token();
                let session = session.clone();
                let socket = socket.request_scope();

                tokio::spawn(async move {
                    dispatch
                        .dispatch(cancellation_token, session, id, params, socket)
                        .await;
                });
            }
            None => {
                report_web_socket_error(
                    socket
                        .send_error(
                            id,
                            EnvelopeErrorCode::UnknownMethod,
                            format!("no handler is registered for the request method '{method}'"),
                            Value::Null,
                        )
                        .await,
                );
            }
        },
        InboundFrame::Notification { method, params } => {
            if let Some(dispatch) = dispatch_table.notification(&method) {
                let dispatch = dispatch.clone();
                let cancellation_token = cancellation_token.child_token();
                let session = session.clone();
                let socket = socket.clone();

                tokio::spawn(async move {
                    dispatch
                        .dispatch(cancellation_token, session, params, socket)
                        .await;
                });
            }
        }
    }
}

async fn run_source<Io, Session>(
    cancellation_token: CancellationToken,
    session: Arc<Session>,
    dispatch_table: Arc<WebSocketDispatchTable<Session>>,
    socket: WebSocket,
    mut source: SplitStream<WebSocketStream<Io>>,
) where
    Io: AsyncRead + AsyncWrite + Unpin,
    Session: Send + Sync + 'static,
{
    loop {
        tokio::select! {
            biased;
            () = cancellation_token.cancelled() => break,
            inbound = source.next() => match inbound {
                Some(Ok(Message::Text(text))) => match serde_json::from_str::<InboundFrame>(text.as_str()) {
                    Ok(frame) => {
                        dispatch_frame(&cancellation_token, &session, &dispatch_table, &socket, frame)
                            .await;
                    }
                    Err(_) => break,
                },
                Some(Ok(Message::Close(_))) | Some(Err(_)) | None => break,
                Some(Ok(_)) => {}
            },
        }
    }
}

pub async fn serve_web_socket_connection<Io, Session>(
    cancellation_token: CancellationToken,
    session: Arc<Session>,
    dispatch_table: Arc<WebSocketDispatchTable<Session>>,
    stream: WebSocketStream<Io>,
) where
    Io: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    Session: Send + Sync + 'static,
{
    let (outbound_sender, outbound_receiver) = mpsc::channel::<Message>(OUTBOUND_BUFFER_CAPACITY);
    let (sink, source) = stream.split();

    tokio::join!(
        run_source(
            cancellation_token,
            session,
            dispatch_table,
            WebSocket::new(outbound_sender),
            source,
        ),
        drain_outbound(sink, outbound_receiver),
    );
}
