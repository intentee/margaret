use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

use super::level_five::LevelFive;

#[singleton]
pub(crate) struct LevelSix {
    _dependency: Arc<LevelFive>,
}

impl LevelSix {
    #[constructor]
    pub(crate) fn create(dependency: Arc<LevelFive>) -> anyhow::Result<Self> {
        Ok(Self {
            _dependency: dependency,
        })
    }
}
