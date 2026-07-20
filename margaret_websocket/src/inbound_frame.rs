use serde::Deserialize;
use serde_json::Value;

use crate::request_id::RequestId;

enum IdClass {
    Absent,
    Invalid,
    Present(RequestId),
}

#[derive(Deserialize)]
struct RawEnvelope {
    #[serde(default)]
    id: Option<Value>,
    jsonrpc: String,
    method: String,
    #[serde(default)]
    params: Option<Value>,
}

fn classify_id(id: Option<Value>) -> IdClass {
    match id {
        None => IdClass::Absent,
        Some(Value::Number(number)) => match number.as_i64() {
            Some(integer) => IdClass::Present(RequestId::Integer(integer)),
            None => IdClass::Invalid,
        },
        Some(Value::String(text)) => IdClass::Present(RequestId::Text(text)),
        Some(_) => IdClass::Invalid,
    }
}

pub enum InboundFrame {
    InvalidEnvelope {
        id: Option<RequestId>,
    },
    Notification {
        method: String,
        params: Option<Value>,
    },
    ParseError,
    Request {
        id: RequestId,
        method: String,
        params: Option<Value>,
    },
}

impl InboundFrame {
    #[must_use]
    pub fn decode(text: &str) -> Self {
        let value: Value = match serde_json::from_str(text) {
            Ok(value) => value,
            Err(_) => return Self::ParseError,
        };

        let RawEnvelope {
            id,
            jsonrpc,
            method,
            params,
        } = match serde_json::from_value(value) {
            Ok(envelope) => envelope,
            Err(_) => return Self::InvalidEnvelope { id: None },
        };

        if jsonrpc != "2.0" {
            return match classify_id(id) {
                IdClass::Present(id) => Self::InvalidEnvelope { id: Some(id) },
                IdClass::Absent | IdClass::Invalid => Self::InvalidEnvelope { id: None },
            };
        }

        match classify_id(id) {
            IdClass::Absent => Self::Notification { method, params },
            IdClass::Invalid => Self::InvalidEnvelope { id: None },
            IdClass::Present(id) => Self::Request { id, method, params },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::InboundFrame;
    use crate::request_id::RequestId;

    fn describe_request_id(id: RequestId) -> String {
        match id {
            RequestId::Integer(integer) => format!("int:{integer}"),
            RequestId::Text(text) => format!("text:{text}"),
        }
    }

    fn describe_id(id: Option<RequestId>) -> String {
        match id {
            None => "none".to_owned(),
            Some(id) => describe_request_id(id),
        }
    }

    fn summary(frame: InboundFrame) -> String {
        match frame {
            InboundFrame::ParseError => "parse_error".to_owned(),
            InboundFrame::InvalidEnvelope { id } => format!("invalid_envelope({})", describe_id(id)),
            InboundFrame::Notification { method, params } => {
                format!("notification({method}, has_params={})", params.is_some())
            }
            InboundFrame::Request { id, method, params } => format!(
                "request({}, {method}, has_params={})",
                describe_request_id(id),
                params.is_some()
            ),
        }
    }

    #[test]
    fn reports_a_parse_error_for_non_json_input() {
        assert_eq!(summary(InboundFrame::decode("not json")), "parse_error");
    }

    #[test]
    fn reports_an_invalid_envelope_for_json_that_is_not_a_request() {
        assert_eq!(
            summary(InboundFrame::decode("42")),
            "invalid_envelope(none)"
        );
    }

    #[test]
    fn reports_an_invalid_envelope_when_the_method_is_missing() {
        assert_eq!(
            summary(InboundFrame::decode(r#"{ "jsonrpc": "2.0", "id": 1 }"#)),
            "invalid_envelope(none)"
        );
    }

    #[test]
    fn echoes_the_id_when_the_jsonrpc_version_is_wrong() {
        assert_eq!(
            summary(InboundFrame::decode(
                r#"{ "jsonrpc": "1.0", "method": "go", "id": 7 }"#
            )),
            "invalid_envelope(int:7)"
        );
    }

    #[test]
    fn drops_the_id_for_a_wrong_version_notification() {
        assert_eq!(
            summary(InboundFrame::decode(r#"{ "jsonrpc": "1.0", "method": "go" }"#)),
            "invalid_envelope(none)"
        );
    }

    #[test]
    fn decodes_a_request_with_an_integer_id_and_params() {
        assert_eq!(
            summary(InboundFrame::decode(
                r#"{ "jsonrpc": "2.0", "method": "go", "id": 3, "params": {} }"#
            )),
            "request(int:3, go, has_params=true)"
        );
    }

    #[test]
    fn decodes_a_request_with_a_string_id_and_no_params() {
        assert_eq!(
            summary(InboundFrame::decode(
                r#"{ "jsonrpc": "2.0", "method": "go", "id": "abc" }"#
            )),
            "request(text:abc, go, has_params=false)"
        );
    }

    #[test]
    fn decodes_a_notification_without_an_id() {
        assert_eq!(
            summary(InboundFrame::decode(
                r#"{ "jsonrpc": "2.0", "method": "ping", "params": [] }"#
            )),
            "notification(ping, has_params=true)"
        );
    }

    #[test]
    fn rejects_a_fractional_id_as_an_invalid_envelope() {
        assert_eq!(
            summary(InboundFrame::decode(
                r#"{ "jsonrpc": "2.0", "method": "go", "id": 1.5 }"#
            )),
            "invalid_envelope(none)"
        );
    }

    #[test]
    fn rejects_a_structured_id_as_an_invalid_envelope() {
        assert_eq!(
            summary(InboundFrame::decode(
                r#"{ "jsonrpc": "2.0", "method": "go", "id": [1] }"#
            )),
            "invalid_envelope(none)"
        );
    }
}
