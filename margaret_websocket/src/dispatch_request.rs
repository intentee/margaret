use std::sync::Arc;

use serde::de::DeserializeOwned;
use serde_json::Value;
use tokio_util::sync::CancellationToken;
use validator::Validate;

use margaret_validation::validate_json::validate_json;
use margaret_validation::validation_result::ValidationResult;

use crate::envelope_error_code::EnvelopeErrorCode;
use crate::report_send_failure::report_send_failure;
use crate::request_id::RequestId;
use crate::responds_to_web_socket_message::RespondsToWebSocketMessage;
use crate::web_socket::WebSocket;
use crate::web_socket_request_message::WebSocketRequestMessage;

async fn report_request_result(result: anyhow::Result<()>, socket: WebSocket, id: RequestId) {
    if let Err(error) = result {
        eprintln!("margaret_websocket: request handler failed: {error:#}");
        report_send_failure(
            socket
                .send_error(
                    id,
                    EnvelopeErrorCode::InternalError,
                    "Internal error".to_string(),
                    Value::Null,
                )
                .await,
        );
    }
}

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
    match validate_json::<Handler::Message>(&params) {
        ValidationResult::Valid(message) => {
            let envelope = Handler::Message::envelope(id.clone(), message);
            let error_socket = socket.clone();

            report_request_result(
                handler
                    .process(cancellation_token, session, envelope, socket)
                    .await,
                error_socket,
                id,
            )
            .await;
        }
        ValidationResult::Invalid(_) => {
            report_send_failure(
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
            report_send_failure(
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
