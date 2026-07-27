use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

use super::level_four::LevelFour;

#[singleton]
pub(crate) struct LevelFive {
    _dependency: Arc<LevelFour>,
}

impl LevelFive {
    #[constructor]
    pub(crate) fn create(dependency: Arc<LevelFour>) -> anyhow::Result<Self> {
        Ok(Self {
            _dependency: dependency,
        })
    }
}
