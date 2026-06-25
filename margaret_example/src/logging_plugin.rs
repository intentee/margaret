use margaret_macros::constructor;
use margaret_macros::singleton;

use crate::plugin::Plugin;

#[singleton(collection = Plugin)]
pub struct LoggingPlugin;

impl LoggingPlugin {
    #[constructor]
    pub fn create() -> Self {
        Self
    }
}

impl Plugin for LoggingPlugin {
    fn name(&self) -> String {
        "logging".to_string()
    }
}
