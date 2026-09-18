use std::sync::Arc;

use rustls::ClientConfig;
use tokio_rustls::TlsConnector;
use url::Url;

use crate::connect_tls_stream::connect_tls_stream;
use crate::web_socket_client_error::WebSocketClientError;
use crate::web_socket_connection::WebSocketConnection;

#[derive(Clone)]
pub struct WebSocketClient {
    connector: TlsConnector,
}

impl WebSocketClient {
    #[must_use]
    pub fn new(client_config: ClientConfig) -> Self {
        Self {
            connector: TlsConnector::from(Arc::new(client_config)),
        }
    }

    /// # Errors
    ///
    /// Returns `WebSocketClientError` when the connection cannot be established.
    pub async fn connect(&self, url: &Url) -> Result<WebSocketConnection, WebSocketClientError> {
        let tls_stream = connect_tls_stream(&self.connector, url).await?;
        let (stream, _response) = tokio_tungstenite::client_async(url.as_str(), tls_stream)
            .await
            .map_err(|source| WebSocketClientError::WebSocketHandshake {
                source,
                url: url.to_string(),
            })?;

        Ok(WebSocketConnection::new(stream, url.to_string()))
    }
}
