use std::net::SocketAddr;
use std::sync::Arc;

use futures_util::TryFutureExt as _;
use rustls::ServerConfig;
use tokio::io::copy_bidirectional;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::task::JoinHandle;
use tokio_rustls::TlsAcceptor;
use tokio_util::sync::CancellationToken;

use crate::relay_destination::RelayDestination;
use crate::relay_error::RelayError;

async fn relayed(
    acceptor: TlsAcceptor,
    stream: TcpStream,
    destination: Arc<RelayDestination>,
) -> Result<(), RelayError> {
    acceptor
        .accept(stream)
        .map_err(RelayError::Handshake)
        .and_then(async |mut client| match destination.next() {
            None => Err(RelayError::NoAdmittedBackend),
            Some(backend) => {
                TcpStream::connect(backend)
                    .map_err(RelayError::BackendConnect)
                    .and_then(async |mut upstream| {
                        copy_bidirectional(&mut client, &mut upstream)
                            .await
                            .map_err(RelayError::Transfer)
                            .map(|_transferred| ())
                    })
                    .await
            }
        })
        .await
}

async fn accept_connections(
    listener: TcpListener,
    acceptor: TlsAcceptor,
    destination: Arc<RelayDestination>,
    cancellation_token: CancellationToken,
) {
    while let Some(accepted) = cancellation_token
        .run_until_cancelled(listener.accept())
        .await
    {
        let (stream, _peer) = accepted.expect("the relay accepts a connection");
        let acceptor = acceptor.clone();
        let destination = Arc::clone(&destination);

        tokio::spawn(async move {
            if let Err(error) = relayed(acceptor, stream, destination).await {
                eprintln!("{error}");
            }
        });
    }
}

pub struct TlsRelay {
    address: SocketAddr,
    cancellation_token: CancellationToken,
    task: JoinHandle<()>,
}

impl TlsRelay {
    /// # Panics
    ///
    /// Panics when the relay cannot listen on the address.
    pub async fn open(
        address: SocketAddr,
        server_config: Arc<ServerConfig>,
        destination: RelayDestination,
    ) -> Self {
        let listener = TcpListener::bind(address)
            .await
            .expect("the relay listens on its address");
        let address = listener.local_addr().expect("the relay knows its address");
        let cancellation_token = CancellationToken::new();
        let task = tokio::spawn(accept_connections(
            listener,
            TlsAcceptor::from(server_config),
            Arc::new(destination),
            cancellation_token.clone(),
        ));

        Self {
            address,
            cancellation_token,
            task,
        }
    }

    #[must_use]
    pub fn port(&self) -> u16 {
        self.address.port()
    }

    /// # Panics
    ///
    /// Panics when the relay stopped accepting connections by panicking.
    pub async fn close(self) {
        self.cancellation_token.cancel();
        self.task.await.expect("the relay closes cleanly");
    }
}
