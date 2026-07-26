use std::sync::Arc;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use margaret_websocket::notification_envelope::NotificationEnvelope;
use margaret_websocket::responds_to_web_socket_notification::RespondsToWebSocketNotification;
use margaret_websocket::web_socket::WebSocket;

use crate::test_session::TestSession;
use crate::typing_notification::TypingNotification;

pub struct FailingNotificationHandler;

#[async_trait]
impl RespondsToWebSocketNotification for FailingNotificationHandler {
    type Message = TypingNotification;
    type Session = TestSession;

    async fn process(
        &self,
        _cancellation_token: CancellationToken,
        _session: Arc<TestSession>,
        _message: NotificationEnvelope<TypingNotification>,
        _socket: WebSocket,
    ) -> anyhow::Result<()> {
        Err(anyhow::anyhow!(
            "the notification handler could not reach its dependency"
        ))
    }
}
