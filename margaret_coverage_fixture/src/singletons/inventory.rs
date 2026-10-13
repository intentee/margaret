use std::path::PathBuf;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
pub struct Inventory {
    titles: Vec<String>,
}

impl Inventory {
    /// # Errors
    ///
    /// Returns an error when the inventory file cannot be read.
    #[constructor]
    pub async fn create(
        #[console_argument(from = "inventory")] inventory: PathBuf,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            titles: tokio::fs::read_to_string(inventory)
                .await?
                .lines()
                .map(str::to_string)
                .collect(),
        })
    }

    #[must_use]
    pub fn titles(&self) -> &[String] {
        &self.titles
    }
}
