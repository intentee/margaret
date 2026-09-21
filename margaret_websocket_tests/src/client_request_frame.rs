use serde_json::Value;

use margaret_websocket_envelope::client_sent_frame::ClientSentFrame;
use margaret_websocket_envelope::request_id::RequestId;

/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
#[must_use]
pub fn client_request_frame(credit: usize, id: RequestId, method: &str, params: Value) -> String {
    serde_json::to_string(&ClientSentFrame::Request {
        credit,
        id,
        method: method.to_owned(),
        params,
    })
    .expect("a request frame serializes")
}
