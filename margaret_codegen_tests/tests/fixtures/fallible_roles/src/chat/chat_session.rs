use failures as errors;
use margaret::framework::macros::build_for_session;
use margaret::framework::macros::websocket_session;

#[websocket_session(path = "/chat", server = "public")]
pub struct ChatSession;

impl ChatSession {
    #[build_for_session]
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub fn assemble() -> errors::Result<Self> {
        Ok(Self)
    }
}
