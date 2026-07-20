#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JsonRpcErrorCode {
    IllegalMessageForState,
    InternalError,
    InvalidParams,
    InvalidRequest,
    MethodNotFound,
    ParseError,
}

impl JsonRpcErrorCode {
    #[must_use]
    pub fn as_i32(self) -> i32 {
        match self {
            Self::IllegalMessageForState => -32000,
            Self::InternalError => -32603,
            Self::InvalidParams => -32602,
            Self::InvalidRequest => -32600,
            Self::MethodNotFound => -32601,
            Self::ParseError => -32700,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::JsonRpcErrorCode;

    #[test]
    fn maps_each_code_to_its_json_rpc_wire_value() {
        assert_eq!(JsonRpcErrorCode::ParseError.as_i32(), -32700);
        assert_eq!(JsonRpcErrorCode::InvalidRequest.as_i32(), -32600);
        assert_eq!(JsonRpcErrorCode::MethodNotFound.as_i32(), -32601);
        assert_eq!(JsonRpcErrorCode::InvalidParams.as_i32(), -32602);
        assert_eq!(JsonRpcErrorCode::InternalError.as_i32(), -32603);
        assert_eq!(JsonRpcErrorCode::IllegalMessageForState.as_i32(), -32000);
    }
}
