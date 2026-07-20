use std::sync::Arc;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use margaret_macros::constructor;
use margaret_macros::singleton;
use margaret_websocket::notification_envelope::NotificationEnvelope;
use margaret_websocket::responds_to_web_socket_notification::RespondsToWebSocketNotification;
use margaret_websocket::web_socket::WebSocket;

use crate::websocket::storyboard_session::StoryboardSession;
use crate::websocket::typing::Typing;

#[singleton]
pub struct TypingIndicator;

impl TypingIndicator {
    #[constructor]
    #[must_use]
    pub fn create() -> Self {
        Self
    }
}

#[async_trait]
impl RespondsToWebSocketNotification for TypingIndicator {
    type Message = Typing;
    type Session = StoryboardSession;

    async fn process(
        &self,
        _cancellation_token: CancellationToken,
        session: Arc<StoryboardSession>,
        message: NotificationEnvelope<Typing>,
        _socket: WebSocket,
    ) {
        session.record(format!("{} is typing", message.message().who)).await;
    }
}
