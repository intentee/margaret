use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use tokio_util::sync::CancellationToken;

use margaret_websocket::dispatch_request::dispatch_request;
use margaret_websocket::request_id::RequestId;
use margaret_websocket::web_socket::WebSocket;
use margaret_websocket::web_socket_message_dispatch::WebSocketMessageDispatch;

use crate::storyboard_handler::StoryboardHandler;
use crate::test_session::TestSession;

pub struct StoryboardDispatch {
    pub handler: Arc<StoryboardHandler>,
}

#[async_trait]
impl WebSocketMessageDispatch<TestSession> for StoryboardDispatch {
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
