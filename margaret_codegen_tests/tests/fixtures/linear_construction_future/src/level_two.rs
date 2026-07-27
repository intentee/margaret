use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

use super::level_one::LevelOne;

#[singleton]
pub(crate) struct LevelTwo {
    _dependency: Arc<LevelOne>,
}

impl LevelTwo {
    #[constructor]
    pub(crate) fn create(dependency: Arc<LevelOne>) -> anyhow::Result<Self> {
        Ok(Self {
            _dependency: dependency,
        })
    }
}
