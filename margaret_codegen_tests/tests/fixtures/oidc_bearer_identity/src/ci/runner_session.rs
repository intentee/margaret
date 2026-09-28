use margaret::framework::macros::build_for_session;
use margaret::framework::macros::websocket_session;

use crate::ci::ci_runner::CiRunner;

#[websocket_session(path = "/runner/session", server = "public")]
pub struct RunnerSession {
    runner: CiRunner,
}

impl RunnerSession {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[build_for_session]
    pub fn build_for_session(#[authenticated_user] runner: CiRunner) -> anyhow::Result<Self> {
        Ok(Self { runner })
    }

    #[must_use]
    pub fn repository(&self) -> &str {
        &self.runner.repository
    }
}
