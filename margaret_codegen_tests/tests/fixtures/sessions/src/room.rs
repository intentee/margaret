use margaret::framework::macros::build_for_session;
use margaret::framework::macros::websocket_session;

use crate::reader::Reader;

#[websocket_session(path = "/room", server = "public")]
pub struct Room {
    pub reader: Option<Reader>,
}

impl Room {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[build_for_session]
    pub fn assemble(#[authenticated_user] reader: Option<Reader>) -> anyhow::Result<Self> {
        Ok(Self { reader })
    }
}
