use serde_json::Value;
use serde_json::json;

use crate::json_rpc_error_code::JsonRpcErrorCode;
use crate::request_id::RequestId;

pub struct JsonRpcErrorFrame {
    code: JsonRpcErrorCode,
    data: Value,
    id: Option<RequestId>,
    message: String,
}

impl JsonRpcErrorFrame {
    #[must_use]
    pub fn illegal_message_for_state(
        id: RequestId,
        current_state: &str,
        method: &str,
        allowed_methods: &[&str],
    ) -> Self {
        Self {
            code: JsonRpcErrorCode::IllegalMessageForState,
            data: json!({
                "currentState": current_state,
                "method": method,
                "allowedMethods": allowed_methods,
            }),
            id: Some(id),
            message: "the message is not accepted in the current protocol state".to_owned(),
        }
    }

    #[must_use]
    pub fn internal_error(id: Option<RequestId>) -> Self {
        Self {
            code: JsonRpcErrorCode::InternalError,
            data: Value::Null,
            id,
            message: "the message could not be processed".to_owned(),
        }
    }

    #[must_use]
    pub fn invalid_params(id: RequestId, data: Value) -> Self {
        Self {
            code: JsonRpcErrorCode::InvalidParams,
            data,
            id: Some(id),
            message: "the message parameters are not valid".to_owned(),
        }
    }

    #[must_use]
    pub fn invalid_request(id: Option<RequestId>) -> Self {
        Self {
            code: JsonRpcErrorCode::InvalidRequest,
            data: Value::Null,
            id,
            message: "the message is not a valid json-rpc request".to_owned(),
        }
    }

    #[must_use]
    pub fn method_not_found(id: RequestId, method: &str) -> Self {
        Self {
            code: JsonRpcErrorCode::MethodNotFound,
            data: json!({ "method": method }),
            id: Some(id),
            message: "the method is not part of this protocol".to_owned(),
        }
    }

    #[must_use]
    pub fn parse_error() -> Self {
        Self {
            code: JsonRpcErrorCode::ParseError,
            data: Value::Null,
            id: None,
            message: "the message is not valid json".to_owned(),
        }
    }

    #[must_use]
    pub fn into_wire_value(self) -> Value {
        json!({
            "jsonrpc": "2.0",
            "id": self.id,
            "error": {
                "code": self.code.as_i32(),
                "message": self.message,
                "data": self.data,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use serde_json::json;

    use super::JsonRpcErrorFrame;
    use crate::request_id::RequestId;

    #[test]
    fn parse_error_has_a_null_id_and_the_parse_code() {
        assert_eq!(
            JsonRpcErrorFrame::parse_error().into_wire_value(),
            json!({
                "jsonrpc": "2.0",
                "id": Value::Null,
                "error": {
                    "code": -32700,
                    "message": "the message is not valid json",
                    "data": Value::Null,
                },
            }),
        );
    }

    #[test]
    fn invalid_request_carries_the_offending_id_when_known() {
        assert_eq!(
            JsonRpcErrorFrame::invalid_request(Some(RequestId::Integer(4))).into_wire_value()["id"],
            json!(4),
        );
    }

    #[test]
    fn method_not_found_reports_the_unknown_method() {
        let value = JsonRpcErrorFrame::method_not_found(RequestId::Text("c1".to_owned()), "unknown")
            .into_wire_value();

        assert_eq!(value["id"], json!("c1"));
        assert_eq!(value["error"]["code"], json!(-32601));
        assert_eq!(value["error"]["data"]["method"], json!("unknown"));
    }

    #[test]
    fn illegal_message_for_state_lists_the_allowed_methods() {
        let value = JsonRpcErrorFrame::illegal_message_for_state(
            RequestId::Integer(9),
            "Chatting",
            "start",
            &["reply"],
        )
        .into_wire_value();

        assert_eq!(value["error"]["code"], json!(-32000));
        assert_eq!(value["error"]["data"]["currentState"], json!("Chatting"));
        assert_eq!(value["error"]["data"]["allowedMethods"], json!(["reply"]));
    }

    #[test]
    fn invalid_params_carries_the_validation_data() {
        let value =
            JsonRpcErrorFrame::invalid_params(RequestId::Integer(1), json!({ "text": ["too short"] }))
                .into_wire_value();

        assert_eq!(value["error"]["code"], json!(-32602));
        assert_eq!(value["error"]["data"]["text"], json!(["too short"]));
    }

    #[test]
    fn internal_error_uses_the_internal_code() {
        assert_eq!(
            JsonRpcErrorFrame::internal_error(None).into_wire_value()["error"]["code"],
            json!(-32603),
        );
    }
}
