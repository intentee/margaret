use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use tokio_util::sync::CancellationToken;

use margaret_websocket::dispatch_notification::dispatch_notification;
use margaret_websocket::web_socket::WebSocket;
use margaret_websocket::web_socket_notification_dispatch::WebSocketNotificationDispatch;

use crate::test_session::TestSession;
use crate::typing_handler::TypingHandler;

pub struct TypingDispatch {
    pub handler: Arc<TypingHandler>,
}

#[async_trait]
impl WebSocketNotificationDispatch<TestSession> for TypingDispatch {
    async fn dispatch(
        &self,
        cancellation_token: CancellationToken,
        session: Arc<TestSession>,
        params: Value,
        socket: WebSocket,
    ) {
        dispatch_notification(&*self.handler, cancellation_token, session, params, socket).await;
    }
}
