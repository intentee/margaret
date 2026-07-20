use futures_util::SinkExt;
use futures_util::stream::SplitSink;
use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;
use tokio::sync::mpsc::Receiver;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_util::sync::CancellationToken;

use crate::outbound_frame::OutboundFrame;

pub async fn run_writer_loop<Io>(
    mut sink: SplitSink<WebSocketStream<Io>, Message>,
    mut receiver: Receiver<OutboundFrame>,
    cancellation_token: CancellationToken,
) where
    Io: AsyncRead + AsyncWrite + Unpin,
{
    while let Some(frame) = receiver.recv().await {
        let closes = matches!(frame, OutboundFrame::Close);

        if sink.send(frame.into_message()).await.is_err() {
            cancellation_token.cancel();

            return;
        }

        if closes {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use futures_util::StreamExt;
    use tokio::io::duplex;
    use tokio::sync::mpsc::channel;
    use tokio_tungstenite::WebSocketStream;
    use tokio_tungstenite::tungstenite::protocol::Role;
    use tokio_util::sync::CancellationToken;

    use super::run_writer_loop;
    use crate::outbound_frame::OutboundFrame;

    #[tokio::test]
    async fn cancels_the_connection_when_the_sink_is_dead() {
        let (server_io, client_io) = duplex(64);
        let server = WebSocketStream::from_raw_socket(server_io, Role::Server, None).await;
        let (sink, _server_read) = server.split();
        let (sender, receiver) = channel(2);
        let cancellation_token = CancellationToken::new();

        drop(client_io);
        sender
            .send(OutboundFrame::Close)
            .await
            .expect("the frame is queued for the writer");
        drop(sender);

        run_writer_loop(sink, receiver, cancellation_token.clone()).await;

        assert!(cancellation_token.is_cancelled());
    }
}
