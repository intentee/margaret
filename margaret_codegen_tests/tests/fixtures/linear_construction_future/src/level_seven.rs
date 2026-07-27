use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

use super::level_six::LevelSix;

#[singleton]
pub(crate) struct LevelSeven {
    _dependency: Arc<LevelSix>,
}

impl LevelSeven {
    #[constructor]
    pub(crate) fn create(dependency: Arc<LevelSix>) -> anyhow::Result<Self> {
        Ok(Self {
            _dependency: dependency,
        })
    }
}
