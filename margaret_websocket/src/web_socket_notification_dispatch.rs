use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use tokio_util::sync::CancellationToken;

use crate::web_socket::WebSocket;

#[async_trait]
pub trait WebSocketNotificationDispatch<Session>: Send + Sync {
    async fn dispatch(
        &self,
        cancellation_token: CancellationToken,
        session: Arc<Session>,
        params: Value,
        socket: WebSocket,
    );
}
