use serde::Deserialize;
use serde::Serialize;

use margaret_websocket_envelope::web_socket_response_message::WebSocketResponseMessage;

#[derive(Debug, Deserialize, Serialize)]
pub struct StoryboardComplete {
    pub summary: String,
}

impl WebSocketResponseMessage for StoryboardComplete {
    const METHOD: &'static str = "storyboard_complete";
}
