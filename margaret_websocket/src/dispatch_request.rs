use std::sync::Arc;

use serde::de::DeserializeOwned;
use serde_json::Value;
use tokio_util::sync::CancellationToken;
use validator::Validate;

use margaret_validation::validate_json::validate_json;
use margaret_validation::validation_result::ValidationResult;

use crate::envelope_error_code::EnvelopeErrorCode;
use crate::report_web_socket_error::report_web_socket_error;
use crate::request_id::RequestId;
use crate::responds_to_web_socket_message::RespondsToWebSocketMessage;
use crate::web_socket::WebSocket;
use crate::web_socket_error::WebSocketError;
use crate::web_socket_request_message::WebSocketRequestMessage;

pub async fn dispatch_request<Handler>(
    handler: &Handler,
    cancellation_token: CancellationToken,
    session: Arc<Handler::Session>,
    id: RequestId,
    params: Value,
    socket: WebSocket,
) where
    Handler: RespondsToWebSocketMessage,
    Handler::Message: DeserializeOwned + Validate,
{
    match validate_json::<Handler::Message>(Some(&params)) {
        ValidationResult::Valid(message) => {
            let envelope = Handler::Message::envelope(id.clone(), message);

            if let Err(error) = handler
                .process(cancellation_token, session, envelope, socket.clone())
                .await
            {
                report_web_socket_error(Err(WebSocketError::UserError(error)));
                report_web_socket_error(
                    socket
                        .send_error(
                            id,
                            EnvelopeErrorCode::InternalError,
                            "the request handler failed".to_string(),
                            Value::Null,
                        )
                        .await,
                );
            }
        }
        ValidationResult::Invalid(_) => {
            report_web_socket_error(
                socket
                    .send_error(
                        id,
                        EnvelopeErrorCode::InvalidParams,
                        "the message parameters failed validation".to_string(),
                        Value::Null,
                    )
                    .await,
            );
        }
        ValidationResult::Malformed(malformation) => {
            report_web_socket_error(
                socket
                    .send_error(
                        id,
                        EnvelopeErrorCode::InvalidParams,
                        malformation.to_string(),
                        Value::Null,
                    )
                    .await,
            );
        }
    }
}
