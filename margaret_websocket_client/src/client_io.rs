use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;

pub(crate) trait ClientIo: AsyncRead + AsyncWrite + Send + Unpin {}

impl<Io> ClientIo for Io where Io: AsyncRead + AsyncWrite + Send + Unpin {}
