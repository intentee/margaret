pub mod on_conversation_message;

use margaret::framework::macros::build_for_session;
use margaret::framework::macros::websocket_session;

#[websocket_session(path = "/beta", server = "public")]
pub struct BetaSession;

impl BetaSession {
    #[build_for_session]
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self)
    }
}
