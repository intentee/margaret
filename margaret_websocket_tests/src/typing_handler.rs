use std::sync::Arc;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use margaret_websocket_envelope::notification_envelope::NotificationEnvelope;
use margaret_websocket::responds_to_web_socket_notification::RespondsToWebSocketNotification;
use margaret_websocket::web_socket::WebSocket;

use crate::test_session::TestSession;
use crate::typing_notification::TypingNotification;

pub struct TypingHandler;

#[async_trait]
impl RespondsToWebSocketNotification for TypingHandler {
    type Message = TypingNotification;
    type Session = TestSession;

    async fn process(
        &self,
        _cancellation_token: CancellationToken,
        session: Arc<TestSession>,
        message: NotificationEnvelope<TypingNotification>,
        _socket: WebSocket,
    ) -> anyhow::Result<()> {
        session
            .notifications
            .lock()
            .expect("the notification log is not poisoned")
            .push(message.message().who.clone());

        Ok(())
    }
}
