use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use tokio_util::sync::CancellationToken;

use margaret_websocket::dispatch_request::dispatch_request;
use margaret_websocket_envelope::request_id::RequestId;
use margaret_websocket::web_socket::WebSocket;
use margaret_websocket::web_socket_message_dispatch::WebSocketMessageDispatch;

use crate::ping_handler::PingHandler;
use crate::test_session::TestSession;

pub struct PingDispatch {
    pub handler: Arc<PingHandler>,
}

#[async_trait]
impl WebSocketMessageDispatch<TestSession> for PingDispatch {
    async fn dispatch(
        &self,
        cancellation_token: CancellationToken,
        session: Arc<TestSession>,
        id: RequestId,
        params: Value,
        socket: WebSocket,
    ) {
        dispatch_request(
            &*self.handler,
            cancellation_token,
            session,
            id,
            params,
            socket,
        )
        .await;
    }
}
