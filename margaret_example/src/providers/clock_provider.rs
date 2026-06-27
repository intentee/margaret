use std::sync::Arc;

use margaret_example_plugin::BuildInfo;
use margaret_macros::constructor;
use margaret_macros::provide;
use margaret_macros::provider;

use crate::clock::Clock;
use crate::system_clock::SystemClock;

#[provider(provides = Clock)]
pub struct ClockProvider {
    build: Arc<BuildInfo>,
}

impl ClockProvider {
    #[constructor]
    pub fn create(build: Arc<BuildInfo>) -> Self {
        Self { build }
    }

    #[provide]
    pub async fn provide(&self) -> Arc<dyn Clock> {
        Arc::new(SystemClock::new(self.build.revision().to_string()))
    }
}
