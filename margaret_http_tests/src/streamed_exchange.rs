use std::net::SocketAddr;

use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub struct StreamedExchange {
    client: TcpStream,
    received: Vec<u8>,
}

impl StreamedExchange {
    /// # Panics
    ///
    /// Panics when the fixture server cannot be reached.
    pub async fn start(address: SocketAddr, request: &[u8]) -> Self {
        let mut client = TcpStream::connect(address)
            .await
            .expect("the client connects to the fixture server");

        client
            .write_all(request)
            .await
            .expect("the request reaches the fixture server");

        Self {
            client,
            received: Vec::new(),
        }
    }

    /// # Panics
    ///
    /// Panics when the server ends the response before sending `fragment`.
    pub async fn receive_until(&mut self, fragment: &str) {
        while !String::from_utf8_lossy(&self.received).contains(fragment) {
            let mut buffer = [0_u8; 1024];
            let read = self
                .client
                .read(&mut buffer)
                .await
                .expect("the server keeps the response open");

            assert_ne!(read, 0, "the server ended the response before `{fragment}`");
            self.received.extend_from_slice(&buffer[..read]);
        }
    }

    /// # Panics
    ///
    /// Panics when the rest of the response cannot be read.
    pub async fn finish(mut self) -> String {
        self.client
            .read_to_end(&mut self.received)
            .await
            .expect("the rest of the response is read");

        String::from_utf8_lossy(&self.received).into_owned()
    }
}
