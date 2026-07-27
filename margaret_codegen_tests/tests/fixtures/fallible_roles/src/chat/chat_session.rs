use failures as errors;
use margaret::framework::macros::build_for_session;
use margaret::framework::macros::websocket_session;

#[websocket_session(origin = "https://example.test", path = "/chat", server = "public")]
pub struct ChatSession;

impl ChatSession {
    #[build_for_session]
    pub fn assemble() -> errors::Result<Self> {
        Ok(Self)
    }
}
