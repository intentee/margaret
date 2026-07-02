use std::sync::Arc;

use margaret_example_plugin::BuildInfo;
use margaret_macros::constructor;
use margaret_macros::singleton;

use crate::clock::Clock;

#[singleton(provides = Clock)]
pub struct SystemClock {
    revision: String,
}

impl SystemClock {
    #[constructor]
    pub fn create(build: Arc<BuildInfo>) -> Self {
        Self {
            revision: build.revision().to_string(),
        }
    }
}

impl Clock for SystemClock {
    fn revision(&self) -> String {
        self.revision.clone()
    }
}
