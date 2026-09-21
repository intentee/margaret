use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;

use margaret_websocket_envelope::envelope_error::EnvelopeError;
use margaret_websocket_envelope::envelope_error_code::EnvelopeErrorCode;
use margaret_websocket_envelope::outbound_error::OutboundError;
use margaret_websocket_envelope::request_id::RequestId;

use crate::scripted_peer_step::ScriptedPeerStep;

/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
#[must_use]
pub fn scripted_response_rejection(id: RequestId, message: &str) -> ScriptedPeerStep {
    ScriptedPeerStep::Send(Message::text(
        serde_json::to_string(&OutboundError {
            error: EnvelopeError {
                code: EnvelopeErrorCode::InternalError,
                details: Value::Null,
                message: message.to_owned(),
            },
            id,
        })
        .expect("a scripted rejection serializes"),
    ))
}
