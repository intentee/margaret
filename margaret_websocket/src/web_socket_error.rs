use thiserror::Error;
use tokio::sync::mpsc::error::SendError;
use tokio_tungstenite::tungstenite::Message;

#[derive(Debug, Error)]
pub enum WebSocketError {
    #[error(
        "the websocket connection is closed and can no longer accept outbound frames: {source}"
    )]
    Send {
        #[source]
        source: SendError<Message>,
    },

    #[error("failed to serialize a websocket response payload: {source}")]
    SerializeResponse {
        #[source]
        source: serde_json::Error,
    },

    #[error("a user-implemented websocket handler returned an error: {0:#}")]
    UserError(anyhow::Error),
}
