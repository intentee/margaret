use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

use super::level_three::LevelThree;

#[singleton]
pub(crate) struct LevelFour {
    _dependency: Arc<LevelThree>,
}

impl LevelFour {
    #[constructor]
    pub(crate) fn create(dependency: Arc<LevelThree>) -> anyhow::Result<Self> {
        Ok(Self {
            _dependency: dependency,
        })
    }
}
