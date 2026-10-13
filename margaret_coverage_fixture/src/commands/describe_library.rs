use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::macros::console_command;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::singleton;

use crate::config::library_profile::LibraryProfile;

#[singleton]
#[console_command(
    name = "describe-library",
    description = "Writes the profile of the library"
)]
pub struct DescribeLibrary {
    profile: Arc<LibraryProfile>,
    to: PathBuf,
}

impl DescribeLibrary {
    #[must_use]
    #[constructor]
    pub fn create(
        profile: Arc<LibraryProfile>,
        #[console_argument(from = "to")] to: PathBuf,
    ) -> Self {
        Self { profile, to }
    }

    /// # Errors
    ///
    /// Returns an error when the profile cannot be written to its destination.
    #[process]
    pub fn run(&self) -> anyhow::Result<CommandOutcome> {
        fs::write(&self.to, self.profile.description())
            .map(|()| CommandOutcome::Succeeded)
            .map_err(anyhow::Error::from)
    }
}
