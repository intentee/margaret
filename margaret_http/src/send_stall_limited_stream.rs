use std::io;
use std::io::IoSlice;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;
use tokio::io::ReadBuf;

use crate::client_send_timeout::CLIENT_SEND_TIMEOUT;
use crate::stall_outcome::StallOutcome;
use crate::stall_timer::StallTimer;

pub(crate) struct SendStallLimitedStream<TStream> {
    stall_timer: StallTimer,
    stream: TStream,
}

impl<TStream> SendStallLimitedStream<TStream> {
    pub(crate) fn new(stream: TStream) -> Self {
        Self {
            stall_timer: StallTimer::new(CLIENT_SEND_TIMEOUT),
            stream,
        }
    }

    fn sent<TOutput>(
        &mut self,
        context: &mut Context<'_>,
        outcome: Poll<io::Result<TOutput>>,
    ) -> Poll<io::Result<TOutput>> {
        self.stall_timer
            .poll_progress(context, outcome)
            .map(|outcome| match outcome {
                StallOutcome::Progressed(result) => result,
                StallOutcome::Stalled => Err(io::ErrorKind::TimedOut.into()),
            })
    }
}

impl<TStream: AsyncRead + Unpin> AsyncRead for SendStallLimitedStream<TStream> {
    fn poll_read(
        self: Pin<&mut Self>,
        context: &mut Context<'_>,
        buffer: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().stream).poll_read(context, buffer)
    }
}

impl<TStream: AsyncWrite + Unpin> AsyncWrite for SendStallLimitedStream<TStream> {
    fn poll_write(
        self: Pin<&mut Self>,
        context: &mut Context<'_>,
        buffer: &[u8],
    ) -> Poll<io::Result<usize>> {
        let this = self.get_mut();
        let outcome = Pin::new(&mut this.stream).poll_write(context, buffer);

        this.sent(context, outcome)
    }

    fn poll_write_vectored(
        self: Pin<&mut Self>,
        context: &mut Context<'_>,
        buffers: &[IoSlice<'_>],
    ) -> Poll<io::Result<usize>> {
        let this = self.get_mut();
        let outcome = Pin::new(&mut this.stream).poll_write_vectored(context, buffers);

        this.sent(context, outcome)
    }

    fn is_write_vectored(&self) -> bool {
        self.stream.is_write_vectored()
    }

    fn poll_flush(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<io::Result<()>> {
        let this = self.get_mut();
        let outcome = Pin::new(&mut this.stream).poll_flush(context);

        this.sent(context, outcome)
    }

    fn poll_shutdown(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<io::Result<()>> {
        let this = self.get_mut();
        let outcome = Pin::new(&mut this.stream).poll_shutdown(context);

        this.sent(context, outcome)
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use tokio::io::AsyncWriteExt;

    use super::SendStallLimitedStream;

    #[tokio::test(start_paused = true)]
    async fn fails_a_write_the_peer_leaves_pending() {
        let (local, _peer) = tokio::io::duplex(1);
        let mut stream = SendStallLimitedStream::new(local);
        let failure = stream
            .write_all(b"more than the peer accepts")
            .await
            .expect_err("the stalled write fails");

        assert_eq!(failure.kind(), io::ErrorKind::TimedOut);
    }
}
