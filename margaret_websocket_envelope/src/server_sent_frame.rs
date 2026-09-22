use serde::Deserialize;
use serde_json::Value;

use crate::envelope_error::EnvelopeError;
use crate::request_id::RequestId;

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case", tag = "kind")]
pub enum ServerSentFrame {
    Response {
        id: RequestId,
        #[serde(rename = "done")]
        is_done: bool,
        method: String,
        #[serde(rename = "result")]
        payload: Value,
    },
    Error {
        error: EnvelopeError,
        id: RequestId,
    },
}

impl ServerSentFrame {
    #[must_use]
    pub fn id(&self) -> &RequestId {
        match self {
            Self::Response { id, .. } | Self::Error { id, .. } => id,
        }
    }

    #[must_use]
    pub fn is_final(&self) -> bool {
        match self {
            Self::Response { is_done, .. } => *is_done,
            Self::Error { .. } => true,
        }
    }

    /// Flow control meters response frames only. A rejection must stay deliverable once the
    /// window is spent, as HTTP/2 exempts `RST_STREAM`.
    #[must_use]
    pub fn is_metered(&self) -> bool {
        match self {
            Self::Response { .. } => true,
            Self::Error { .. } => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use serde_json::json;

    use crate::envelope_error::EnvelopeError;
    use crate::envelope_error_code::EnvelopeErrorCode;
    use crate::outbound_error::OutboundError;
    use crate::outbound_response::OutboundResponse;
    use crate::request_id::RequestId;

    use super::ServerSentFrame;

    fn read(frame: &Value) -> Option<ServerSentFrame> {
        serde_json::from_value(frame.clone()).ok()
    }

    fn written<Frame: serde::Serialize>(frame: &Frame) -> Option<ServerSentFrame> {
        read(&serde_json::to_value(frame).expect("the server frame serializes"))
    }

    fn rejection() -> OutboundError {
        OutboundError {
            error: EnvelopeError {
                code: EnvelopeErrorCode::InternalError,
                details: Value::Null,
                message: "the handler gave up".to_string(),
            },
            id: RequestId::Number(1),
        }
    }

    #[test]
    fn reads_a_response_that_names_its_kind() {
        assert!(matches!(
            read(&json!({
                "kind": "response",
                "id": 1,
                "done": true,
                "method": "response_chunk",
                "result": {"text": "answered"},
            })),
            Some(ServerSentFrame::Response { is_done: true, ref method, .. })
                if method == "response_chunk"
        ));
    }

    #[test]
    fn reads_a_rejection_that_names_its_kind() {
        assert!(matches!(
            read(&json!({
                "kind": "error",
                "id": 1,
                "error": {"code": "internal_error", "details": null, "message": "gave up"},
            })),
            Some(ServerSentFrame::Error { ref error, .. }) if error.message == "gave up"
        ));
    }

    #[test]
    fn refuses_a_frame_that_names_no_kind() {
        assert!(
            read(&json!({
                "id": 1,
                "done": true,
                "method": "response_chunk",
                "result": {"text": "answered"},
            }))
            .is_none()
        );
    }

    #[test]
    fn refuses_a_frame_that_carries_both_a_result_and_an_error() {
        assert!(
            read(&json!({
                "kind": "response",
                "id": 1,
                "done": true,
                "method": "response_chunk",
                "result": {"text": "answered"},
                "error": {"code": "internal_error", "details": null, "message": "gave up"},
            }))
            .is_none()
        );
    }

    #[test]
    fn reads_back_the_response_the_server_writes() {
        assert!(matches!(
            written(&OutboundResponse {
                id: RequestId::Number(1),
                is_done: false,
                method: "response_chunk",
                payload: json!({"text": "chunk"}),
            }),
            Some(ServerSentFrame::Response { is_done: false, ref method, .. })
                if method == "response_chunk"
        ));
    }

    #[test]
    fn reads_back_the_rejection_the_server_writes() {
        assert!(matches!(
            written(&rejection()),
            Some(ServerSentFrame::Error { ref error, .. })
                if error.message == "the handler gave up"
        ));
    }
}
