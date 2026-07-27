use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

use super::level_two::LevelTwo;

#[singleton]
pub(crate) struct LevelThree {
    _dependency: Arc<LevelTwo>,
}

impl LevelThree {
    #[constructor]
    pub(crate) fn create(dependency: Arc<LevelTwo>) -> anyhow::Result<Self> {
        Ok(Self {
            _dependency: dependency,
        })
    }
}
