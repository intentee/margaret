use std::path::PathBuf;
use std::sync::Arc;

use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::macros::console_command;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::singleton;

use crate::singletons::inventory::Inventory;

#[singleton]
#[console_command(
    name = "count-books",
    description = "Writes how many books the inventory lists"
)]
pub struct CountBooks {
    inventory: Arc<Inventory>,
    to: PathBuf,
}

impl CountBooks {
    #[must_use]
    #[constructor]
    pub fn create(inventory: Arc<Inventory>, #[console_argument(from = "to")] to: PathBuf) -> Self {
        Self { inventory, to }
    }

    /// # Errors
    ///
    /// Returns an error when the count cannot be written to its destination.
    #[process]
    pub async fn run(&self) -> anyhow::Result<CommandOutcome> {
        tokio::fs::write(&self.to, self.inventory.titles().len().to_string())
            .await
            .map(|()| CommandOutcome::Succeeded)
            .map_err(anyhow::Error::from)
    }
}
