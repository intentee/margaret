use serde_json::Value;

use margaret_websocket_envelope::client_sent_frame::ClientSentFrame;

/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
#[must_use]
pub fn client_notification_frame(method: &str, params: Value) -> String {
    serde_json::to_string(&ClientSentFrame::Notification {
        method: method.to_owned(),
        params,
    })
    .expect("a notification frame serializes")
}
