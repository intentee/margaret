use std::io::Error as IoError;

use thiserror::Error;
use tokio_tungstenite::tungstenite::Error as TungsteniteError;

#[derive(Debug, Error)]
pub enum WebSocketClientError {
    #[error("the websocket connection to '{url}' could not carry a frame: {source}")]
    SendFrame {
        url: String,
        #[source]
        source: TungsteniteError,
    },

    #[error("failed to establish a tcp connection to '{address}': {source}")]
    TcpConnect {
        address: String,
        #[source]
        source: IoError,
    },

    #[error("the tls handshake with '{host}' failed: {source}")]
    TlsHandshake {
        host: String,
        #[source]
        source: IoError,
    },

    #[error("the websocket handshake with '{url}' failed: {source}")]
    WebSocketHandshake {
        url: String,
        #[source]
        source: TungsteniteError,
    },

    #[error("'{host}' is not a valid tls server name: {source}")]
    TlsServerName {
        host: String,
        #[source]
        source: rustls::pki_types::InvalidDnsNameError,
    },

    #[error(
        "'{url}' does not address a secure websocket endpoint; a mutually authenticated client needs a 'wss' url carrying a host and a port"
    )]
    UnusableUrl { url: String },

    #[error("failed to serialize an outbound websocket frame: {source}")]
    SerializeFrame {
        #[source]
        source: serde_json::Error,
    },

    #[error("failed to deserialize a websocket response payload: {source}")]
    DeserializeResponse {
        #[source]
        source: serde_json::Error,
    },

    #[error(
        "the peer answered with the response method '{received}', but '{expected}' was requested"
    )]
    UnexpectedResponseMethod {
        expected: &'static str,
        received: String,
    },
}
