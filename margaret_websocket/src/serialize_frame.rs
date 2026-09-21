use serde::Serialize;

use crate::web_socket_error::WebSocketError;

/// # Errors
///
/// Returns `WebSocketError::SerializeResponse` when the frame cannot be written as JSON.
pub(crate) fn serialize_frame<Frame: Serialize>(frame: &Frame) -> Result<String, WebSocketError> {
    serde_json::to_string(frame).map_err(|source| WebSocketError::SerializeResponse { source })
}

#[cfg(test)]
mod tests {
    use serde::Serialize;
    use serde::Serializer;

    use super::serialize_frame;
    use crate::web_socket_error::WebSocketError;

    struct Unserializable;

    impl Serialize for Unserializable {
        fn serialize<Target>(&self, _target: Target) -> Result<Target::Ok, Target::Error>
        where
            Target: Serializer,
        {
            Err(serde::ser::Error::custom("this payload never serializes"))
        }
    }

    #[test]
    fn writes_a_frame_as_json() {
        assert_eq!(
            serialize_frame(&"answered").expect("a string frame serializes"),
            "\"answered\""
        );
    }

    #[test]
    fn reports_a_frame_that_cannot_be_written() {
        let refused = serialize_frame(&Unserializable).expect_err("the payload never serializes");

        assert!(matches!(
            refused,
            WebSocketError::SerializeResponse { ref source } if source.is_data()
        ));
    }
}
