use std::sync::Arc;

use futures_util::FutureExt;
use futures_util::SinkExt;
use futures_util::StreamExt;
use futures_util::future::BoxFuture;
use futures_util::stream::FuturesUnordered;
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

use margaret_websocket_envelope::client_sent_frame::ClientSentFrame;
use margaret_websocket_envelope::envelope_error_code::EnvelopeErrorCode;

use crate::credit_grant_outcome::CreditGrantOutcome;
use crate::exchange_admission::ExchangeAdmission;
use crate::exchange_credit::ExchangeCredit;
use crate::exchange_flow_control::ExchangeFlowControl;
use crate::outbound_frames::OutboundFrames;
use crate::report_send_failure::report_send_failure;
use crate::web_socket::WebSocket;
use crate::web_socket_dispatch_table::WebSocketDispatchTable;

const OUTBOUND_BUFFER_CAPACITY: usize = 1;

fn report_close_failure(outcome: Result<(), tokio_tungstenite::tungstenite::Error>) {
    if let Err(error) = outcome {
        eprintln!("margaret_websocket: unable to close a websocket connection: {error}");
    }
}

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

    report_close_failure(sink.send(Message::Close(None)).await);
}

async fn dispatch_frame<Session>(
    cancellation_token: &CancellationToken,
    session: &Arc<Session>,
    dispatch_table: &Arc<WebSocketDispatchTable<Session>>,
    credit: &ExchangeCredit,
    frames: &OutboundFrames,
    pending: &mut FuturesUnordered<BoxFuture<'static, ()>>,
    frame: ClientSentFrame,
) where
    Session: Send + Sync + 'static,
{
    match frame {
        ClientSentFrame::Cancel { id } => credit.cancel(&id),
        ClientSentFrame::Credit {
            credit: granted,
            id,
        } => match credit.grant(&id, granted) {
            CreditGrantOutcome::ExchangeIsOver | CreditGrantOutcome::Granted => {}
            CreditGrantOutcome::WindowExceeded => {
                credit.cancel(&id);
                report_send_failure(
                    frames
                        .send_error(
                            id,
                            EnvelopeErrorCode::CreditWindowExceeded,
                            "the grant would take this exchange past the credit window the protocol allows"
                                .to_string(),
                            Value::Null,
                        )
                        .await,
                );
            }
        },
        ClientSentFrame::Request {
            credit: granted,
            id,
            method,
            params,
        } => match dispatch_table.request(&method) {
            Some(dispatch) => {
                let exchange_token = cancellation_token.child_token();

                match credit.open(id.clone(), granted, exchange_token.clone()) {
                    ExchangeAdmission::Admitted(exchange) => {
                        let dispatch = dispatch.clone();
                        let session = session.clone();
                        let socket = WebSocket::new(
                            ExchangeFlowControl::Metered(exchange.credit.clone()),
                            frames.clone(),
                        );
                        let credit = credit.clone();

                        pending.push(
                            async move {
                                dispatch
                                    .dispatch(exchange_token, session, id.clone(), params, socket)
                                    .await;
                                credit.complete(&id, &exchange);
                            }
                            .boxed(),
                        );
                    }
                    ExchangeAdmission::AlreadyOpen => {
                        report_send_failure(
                            frames
                                .send_error(
                                    id,
                                    EnvelopeErrorCode::DuplicateRequestId,
                                    "another exchange is already open under this request id"
                                        .to_string(),
                                    Value::Null,
                                )
                                .await,
                        );
                    }
                }
            }
            None => {
                report_send_failure(
                    frames
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
        ClientSentFrame::Notification { method, params } => {
            if let Some(dispatch) = dispatch_table.notification(&method) {
                let dispatch = dispatch.clone();
                let cancellation_token = cancellation_token.child_token();
                let session = session.clone();
                let socket = WebSocket::new(ExchangeFlowControl::Unmetered, frames.clone());

                pending.push(
                    async move {
                        dispatch
                            .dispatch(cancellation_token, session, params, socket)
                            .await;
                    }
                    .boxed(),
                );
            }
        }
    }
}

async fn run_source<Io, Session>(
    cancellation_token: CancellationToken,
    session: Arc<Session>,
    dispatch_table: Arc<WebSocketDispatchTable<Session>>,
    credit: ExchangeCredit,
    frames: OutboundFrames,
    mut source: SplitStream<WebSocketStream<Io>>,
) where
    Io: AsyncRead + AsyncWrite + Unpin,
    Session: Send + Sync + 'static,
{
    let mut pending = FuturesUnordered::new();

    loop {
        tokio::select! {
            biased;
            () = cancellation_token.cancelled() => break,
            Some(()) = pending.next(), if !pending.is_empty() => {}
            inbound = source.next() => match inbound {
                Some(Ok(Message::Text(text))) => match serde_json::from_str::<ClientSentFrame>(text.as_str()) {
                    Ok(frame) => {
                        dispatch_frame(
                            &cancellation_token,
                            &session,
                            &dispatch_table,
                            &credit,
                            &frames,
                            &mut pending,
                            frame,
                        )
                            .await;
                    }
                    Err(error) => {
                        eprintln!("margaret_websocket: the client sent an unreadable frame: {error}");

                        break;
                    }
                },
                Some(Ok(Message::Close(_)) | Err(_)) | None => break,
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
            ExchangeCredit::default(),
            OutboundFrames::new(outbound_sender),
            source,
        ),
        drain_outbound(sink, outbound_receiver),
    );
}

#[cfg(test)]
mod tests {
    use tokio_tungstenite::tungstenite::Error;

    use super::report_close_failure;

    #[test]
    fn accepts_a_successful_websocket_close() {
        report_close_failure(Ok(()));
    }

    #[test]
    fn reports_a_failed_websocket_close() {
        report_close_failure(Err(Error::ConnectionClosed));
    }
}
