use margaret_macros::constructor;
use margaret_macros::singleton;

pub trait Plugin {
    fn name(&self) -> String;
}

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
