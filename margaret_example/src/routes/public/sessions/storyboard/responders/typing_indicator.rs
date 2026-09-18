use std::sync::Arc;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;
use margaret::framework::websocket_envelope::notification_envelope::NotificationEnvelope;
use margaret::framework::websocket::responds_to_web_socket_notification::RespondsToWebSocketNotification;
use margaret::framework::websocket::web_socket::WebSocket;

use crate::routes::public::sessions::storyboard::messages::typing::Typing;
use crate::routes::public::sessions::storyboard::storyboard_session::StoryboardSession;

#[singleton]
pub struct TypingIndicator;

impl TypingIndicator {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create() -> anyhow::Result<Self> {
        Ok(Self)
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
    ) -> anyhow::Result<()> {
        session
            .record(format!("{} is typing", message.message().who))
            .await;

        Ok(())
    }
}
