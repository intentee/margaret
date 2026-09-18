use rustls::pki_types::ServerName;
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;
use tokio_rustls::client::TlsStream;
use url::Url;

use crate::web_socket_client_error::WebSocketClientError;

const SECURE_WEB_SOCKET_SCHEME: &str = "wss";

pub(crate) async fn connect_tls_stream(
    connector: &TlsConnector,
    url: &Url,
) -> Result<TlsStream<TcpStream>, WebSocketClientError> {
    let (SECURE_WEB_SOCKET_SCHEME, Some(host), Some(port)) =
        (url.scheme(), url.host_str(), url.port_or_known_default())
    else {
        return Err(WebSocketClientError::UnusableUrl {
            url: url.to_string(),
        });
    };

    let server_name = ServerName::try_from(host.to_string()).map_err(|source| {
        WebSocketClientError::TlsServerName {
            host: host.to_string(),
            source,
        }
    })?;
    let address = format!("{host}:{port}");
    let tcp_stream =
        TcpStream::connect(&address)
            .await
            .map_err(|source| WebSocketClientError::TcpConnect {
                address: address.clone(),
                source,
            })?;

    connector
        .connect(server_name, tcp_stream)
        .await
        .map_err(|source| WebSocketClientError::TlsHandshake {
            host: host.to_string(),
            source,
        })
}
