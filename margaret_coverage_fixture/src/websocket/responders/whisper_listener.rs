use std::sync::Arc;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use margaret::framework::macros::singleton;
use margaret::framework::websocket::notification_envelope::NotificationEnvelope;
use margaret::framework::websocket::responds_to_web_socket_notification::RespondsToWebSocketNotification;
use margaret::framework::websocket::web_socket::WebSocket;

use crate::websocket::messages::whisper::Whisper;
use crate::websocket::sessions::shelf_talk_session::ShelfTalkSession;

#[singleton]
pub struct WhisperListener;

#[async_trait]
impl RespondsToWebSocketNotification for WhisperListener {
    type Message = Whisper;
    type Session = ShelfTalkSession;

    async fn process(
        &self,
        _cancellation_token: CancellationToken,
        session: Arc<ShelfTalkSession>,
        _message: NotificationEnvelope<Whisper>,
        _socket: WebSocket,
    ) -> anyhow::Result<()> {
        session.hear_whisper().await;

        Ok(())
    }
}
