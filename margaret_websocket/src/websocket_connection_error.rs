use thiserror::Error;
use tokio_tungstenite::tungstenite::Error as TungsteniteError;

#[derive(Debug, Error)]
pub enum WebSocketConnectionError {
    #[error("the websocket transport failed while reading a frame: {0}")]
    Transport(#[source] TungsteniteError),
}
