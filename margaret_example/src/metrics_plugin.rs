use margaret_macros::constructor;
use margaret_macros::singleton;

use crate::plugin::Plugin;

#[singleton(collection = Plugin)]
pub struct MetricsPlugin;

impl MetricsPlugin {
    #[constructor]
    pub fn create() -> Self {
        Self
    }
}

impl Plugin for MetricsPlugin {
    fn name(&self) -> String {
        "metrics".to_string()
    }
}
