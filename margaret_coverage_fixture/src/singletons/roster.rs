use std::io;
use std::path::PathBuf;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
pub struct Roster {
    roster: PathBuf,
}

impl Roster {
    #[must_use]
    #[constructor]
    pub fn create(#[console_argument(from = "roster")] roster: PathBuf) -> Self {
        Self { roster }
    }

    /// # Errors
    ///
    /// Returns an error when the roster file cannot be read.
    pub async fn lists(&self, name: &str) -> io::Result<bool> {
        tokio::fs::read_to_string(&self.roster)
            .await
            .map(|roster| roster.lines().any(|listed| listed == name))
    }
}
