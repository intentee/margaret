use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;
use tokio_rustls::client::TlsStream;
use rustls::pki_types::ServerName;
use url::Url;

use crate::web_socket_client_error::WebSocketClientError;

const SECURE_WEB_SOCKET_SCHEME: &str = "wss";

pub(crate) async fn connect_tls_stream(
    connector: &TlsConnector,
    url: &Url,
) -> Result<TlsStream<TcpStream>, WebSocketClientError> {
    if url.scheme() != SECURE_WEB_SOCKET_SCHEME {
        return Err(WebSocketClientError::UnsupportedUrlScheme {
            scheme: url.scheme().to_string(),
            url: url.to_string(),
        });
    }

    let Some(host) = url.host_str() else {
        return Err(WebSocketClientError::UrlWithoutHost {
            url: url.to_string(),
        });
    };
    let Some(port) = url.port_or_known_default() else {
        return Err(WebSocketClientError::UrlWithoutPort {
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
    let tcp_stream = TcpStream::connect(&address).await.map_err(|source| {
        WebSocketClientError::TcpConnect {
            address: address.clone(),
            source,
        }
    })?;

    connector
        .connect(server_name, tcp_stream)
        .await
        .map_err(|source| WebSocketClientError::TlsHandshake {
            host: host.to_string(),
            source,
        })
}
