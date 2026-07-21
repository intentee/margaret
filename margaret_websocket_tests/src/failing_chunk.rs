use serde::Serialize;
use serde::Serializer;
use serde::ser::Error;

use margaret_websocket::web_socket_response_message::WebSocketResponseMessage;

pub struct FailingChunk;

impl Serialize for FailingChunk {
    fn serialize<Target: Serializer>(
        &self,
        _serializer: Target,
    ) -> Result<Target::Ok, Target::Error> {
        Err(Target::Error::custom(
            "this response payload cannot be serialized",
        ))
    }
}

impl WebSocketResponseMessage for FailingChunk {
    const METHOD: &'static str = "failing_chunk";
}
