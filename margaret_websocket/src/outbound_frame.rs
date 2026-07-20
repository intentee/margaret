use serde_json::Value;
use serde_json::json;
use tokio_tungstenite::tungstenite::Bytes;
use tokio_tungstenite::tungstenite::Message;

use crate::json_rpc_error_frame::JsonRpcErrorFrame;
use crate::request_id::RequestId;

pub enum OutboundFrame {
    Close,
    Error(JsonRpcErrorFrame),
    Notify { method: String, params: Value },
    Pong(Bytes),
    Result { id: RequestId, result: Value },
}

impl OutboundFrame {
    #[must_use]
    pub fn into_message(self) -> Message {
        match self {
            Self::Close => Message::Close(None),
            Self::Error(frame) => Message::text(frame.into_wire_value().to_string()),
            Self::Notify { method, params } => Message::text(
                json!({ "jsonrpc": "2.0", "method": method, "params": params }).to_string(),
            ),
            Self::Pong(payload) => Message::Pong(payload),
            Self::Result { id, result } => Message::text(
                json!({ "jsonrpc": "2.0", "id": id, "result": result }).to_string(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use serde_json::json;
    use tokio_tungstenite::tungstenite::Bytes;

    use super::OutboundFrame;
    use crate::json_rpc_error_frame::JsonRpcErrorFrame;
    use crate::request_id::RequestId;

    fn text_value(frame: OutboundFrame) -> Value {
        let message = frame.into_message();
        let text = message.into_text().expect("the frame serializes to a text message");

        serde_json::from_str(text.as_str()).expect("the text message is valid json")
    }

    #[test]
    fn a_result_frame_carries_the_request_id_and_payload() {
        assert_eq!(
            text_value(OutboundFrame::Result {
                id: RequestId::Integer(8),
                result: json!({ "ok": true }),
            }),
            json!({ "jsonrpc": "2.0", "id": 8, "result": { "ok": true } }),
        );
    }

    #[test]
    fn a_notify_frame_carries_the_method_and_params() {
        assert_eq!(
            text_value(OutboundFrame::Notify {
                method: "assistant.token".to_owned(),
                params: json!("hi"),
            }),
            json!({ "jsonrpc": "2.0", "method": "assistant.token", "params": "hi" }),
        );
    }

    #[test]
    fn an_error_frame_serializes_the_wire_error() {
        assert_eq!(
            text_value(OutboundFrame::Error(JsonRpcErrorFrame::method_not_found(
                RequestId::Integer(2),
                "nope",
            )))["error"]["code"],
            json!(-32601),
        );
    }

    #[test]
    fn a_pong_frame_becomes_a_pong_message() {
        assert!(
            OutboundFrame::Pong(Bytes::from_static(b"beat"))
                .into_message()
                .is_pong()
        );
    }

    #[test]
    fn a_close_frame_becomes_a_close_message() {
        assert!(OutboundFrame::Close.into_message().is_close());
    }
}
