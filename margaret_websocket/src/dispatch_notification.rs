use std::sync::Arc;

use serde::de::DeserializeOwned;
use serde_json::Value;
use tokio_util::sync::CancellationToken;
use validator::Validate;

use margaret_validation::validate_json::validate_json;
use margaret_validation::validation_result::ValidationResult;

use crate::notification_envelope::NotificationEnvelope;
use crate::responds_to_web_socket_notification::RespondsToWebSocketNotification;
use crate::web_socket::WebSocket;

fn report_notification_result(result: anyhow::Result<()>) {
    if let Err(error) = result {
        eprintln!("margaret_websocket: notification handler failed: {error:#}");
    }
}

pub async fn dispatch_notification<Handler>(
    handler: &Handler,
    cancellation_token: CancellationToken,
    session: Arc<Handler::Session>,
    params: Value,
    socket: WebSocket,
) where
    Handler: RespondsToWebSocketNotification,
    Handler::Message: DeserializeOwned + Validate,
{
    if let ValidationResult::Valid(message) = validate_json::<Handler::Message>(Some(&params)) {
        report_notification_result(
            handler
                .process(
                    cancellation_token,
                    session,
                    NotificationEnvelope::new(message),
                    socket,
                )
                .await,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::report_notification_result;

    #[test]
    fn reports_a_notification_handler_failure() {
        report_notification_result(Err(anyhow::anyhow!("database unavailable")));
    }
}
