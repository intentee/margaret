use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use tokio_util::sync::CancellationToken;

use margaret_websocket_envelope::request_id::RequestId;

use crate::web_socket::WebSocket;

#[async_trait]
pub trait WebSocketMessageDispatch<Session>: Send + Sync {
    async fn dispatch(
        &self,
        cancellation_token: CancellationToken,
        session: Arc<Session>,
        id: RequestId,
        params: Value,
        socket: WebSocket,
    );
}
