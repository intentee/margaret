use serde::Deserialize;
use serde::Serialize;
use validator::Validate;

use margaret_websocket_envelope::web_socket_notification_message::WebSocketNotificationMessage;

#[derive(Deserialize, Serialize, Validate)]
pub struct TypingNotification {
    #[validate(length(min = 1))]
    pub who: String,
}

impl WebSocketNotificationMessage for TypingNotification {
    const METHOD: &'static str = "typing";
}
