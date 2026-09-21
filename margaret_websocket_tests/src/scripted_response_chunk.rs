use tokio_tungstenite::tungstenite::Message;

use margaret_websocket_envelope::outbound_response::OutboundResponse;
use margaret_websocket_envelope::request_id::RequestId;
use margaret_websocket_envelope::web_socket_response_message::WebSocketResponseMessage;

use crate::response_chunk::ResponseChunk;
use crate::scripted_peer_step::ScriptedPeerStep;

/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
#[must_use]
pub fn scripted_response_chunk(id: RequestId, is_done: bool, text: &str) -> ScriptedPeerStep {
    ScriptedPeerStep::Send(Message::text(
        serde_json::to_string(&OutboundResponse {
            id,
            is_done,
            method: ResponseChunk::METHOD,
            payload: ResponseChunk {
                text: text.to_owned(),
            },
        })
        .expect("a scripted response chunk serializes"),
    ))
}
