pub mod on_conversation_message;

use margaret::framework::macros::build_for_session;
use margaret::framework::macros::websocket_session;

#[websocket_session(path = "/beta", server = "public")]
pub struct BetaSession;

impl BetaSession {
    #[build_for_session]
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}
