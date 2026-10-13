use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::service;

use super::level_eight::LevelEight;

#[service]
pub struct LevelWatcher {
    _deepest: Arc<LevelEight>,
}

impl LevelWatcher {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(deepest: Arc<LevelEight>) -> anyhow::Result<Self> {
        Ok(Self { _deepest: deepest })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub async fn run(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
