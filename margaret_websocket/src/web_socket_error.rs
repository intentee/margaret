use thiserror::Error;

#[derive(Debug, Error)]
pub enum WebSocketError {
    #[error("the websocket connection is closed and can no longer accept outbound frames")]
    Send,

    #[error("failed to serialize a websocket response payload: {source}")]
    SerializeResponse {
        #[source]
        source: serde_json::Error,
    },
}
