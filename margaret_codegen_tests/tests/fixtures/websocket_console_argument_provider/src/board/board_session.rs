use margaret::framework::macros::build_for_session;
use margaret::framework::macros::websocket_session;

use crate::reader::Reader;

#[websocket_session(path = "/board/{topic}", server = "public")]
pub struct BoardSession {
    reader: Option<Reader>,
    topic: String,
}

impl BoardSession {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[build_for_session]
    pub fn assemble(
        #[route_parameter(from = "topic")] topic: String,
        #[authenticated_user] reader: Option<Reader>,
    ) -> anyhow::Result<Self> {
        Ok(Self { reader, topic })
    }

    #[must_use]
    pub fn reader_name(&self) -> Option<&str> {
        self.reader.as_ref().map(|reader| reader.name.as_str())
    }

    #[must_use]
    pub fn topic(&self) -> &str {
        &self.topic
    }
}
