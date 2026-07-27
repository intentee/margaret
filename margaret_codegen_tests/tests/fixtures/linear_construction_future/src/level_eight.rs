use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

use super::level_seven::LevelSeven;

#[singleton]
pub(crate) struct LevelEight {
    _dependency: Arc<LevelSeven>,
}

impl LevelEight {
    #[constructor]
    pub(crate) fn create(dependency: Arc<LevelSeven>) -> anyhow::Result<Self> {
        Ok(Self {
            _dependency: dependency,
        })
    }
}
