use margaret::framework::macros::build_for_session;
use margaret::framework::macros::websocket_session;

#[websocket_session(path = "/chat", server = "public")]
pub struct ChatSession;

impl ChatSession {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[build_for_session]
    pub fn build_for_session() -> anyhow::Result<Self> {
        Ok(Self)
    }
}
