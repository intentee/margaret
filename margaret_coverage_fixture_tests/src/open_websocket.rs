use std::net::SocketAddr;

use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::client_async;
use tokio_tungstenite::tungstenite::Error;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

/// # Errors
///
/// Returns the handshake error when the library refuses to upgrade the connection.
///
/// # Panics
///
/// Panics when the library does not accept the connection.
pub async fn open_websocket(
    address: SocketAddr,
    request: impl IntoClientRequest + Unpin,
) -> Result<WebSocketStream<TcpStream>, Error> {
    client_async(
        request,
        TcpStream::connect(address)
            .await
            .expect("the library accepts the connection"),
    )
    .await
    .map(|(stream, _handshake)| stream)
}
