use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;

use crate::outbound_frame::OutboundFrame;

#[derive(Clone)]
pub struct EmitCore {
    cancellation_token: CancellationToken,
    sender: Sender<OutboundFrame>,
}

impl EmitCore {
    #[must_use]
    pub fn new(cancellation_token: CancellationToken, sender: Sender<OutboundFrame>) -> Self {
        Self {
            cancellation_token,
            sender,
        }
    }

    pub async fn send(&self, frame: OutboundFrame) {
        if self.sender.send(frame).await.is_err() {
            self.cancellation_token.cancel();
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc::channel;
    use tokio_util::sync::CancellationToken;

    use super::EmitCore;
    use crate::outbound_frame::OutboundFrame;

    #[tokio::test]
    async fn send_delivers_the_frame_to_the_writer() {
        let (sender, mut receiver) = channel(1);
        let core = EmitCore::new(CancellationToken::new(), sender);

        core.send(OutboundFrame::Close).await;

        assert!(
            receiver
                .recv()
                .await
                .expect("a frame reaches the writer")
                .into_message()
                .is_close()
        );
    }

    #[tokio::test]
    async fn send_cancels_the_connection_when_the_writer_is_gone() {
        let (sender, receiver) = channel(1);

        drop(receiver);

        let cancellation_token = CancellationToken::new();
        let core = EmitCore::new(cancellation_token.clone(), sender);

        core.send(OutboundFrame::Close).await;

        assert!(cancellation_token.is_cancelled());
    }
}
